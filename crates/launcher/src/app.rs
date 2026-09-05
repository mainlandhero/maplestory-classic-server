//! The window: sign in, register, or recover a password - and a log pane that says what
//! happened, collapsed until something in it is worth reading.
//!
//! The owner asked for exactly this to begin with: *"input of email and password and server IP to
//! be able to direct the client. There should be two buttons, Login then Start Game. Login
//! validates the session and then enables the 'Start Game' button."* So **Start Game is
//! disabled until a sign-in succeeds**, and every server call runs on a worker thread -
//! argon2id is deliberately slow, and a window that stops repainting during it looks hung.
//!
//! Two more screens since 2026-09-05, both behind a single-use code an administrator mints
//! (in game: `!registrationcode`, `!recoverycode <email|username>`): **Register** - username,
//! email, password, code - and **Forgot password** - email or username, code, new password. A
//! client machine has no database and no `maplecw-useradd`, so these are the only way a
//! player gets an account or gets back into one. The password rule is checked here first, so
//! the sentence appears before a round trip, and again on the server, which is the check that
//! counts.

use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use egui::{Color32, RichText};

use crate::http::{RecoverReply, RegisterReply};
use crate::paths::Layout;
use crate::prepare::{self, Level, Plan};
use crate::session::{self, SignIn};

pub const WINDOW_TITLE: &str = "MapleCW Launcher";

/// The sentence this project is required to say whenever it reports progress, in the place it
/// matters most: `CLAUDE.md`, standing constraints - *"Nothing authenticates. The game socket
/// carries no credentials."*
const NO_CREDENTIALS_NOTE: &str = "The game socket carries no credentials. Signing in verifies \
                                   your password and selects which account the next game \
                                   connection is served as - it does not authenticate the \
                                   client.";

/// Messages from the worker threads back to the UI.
enum Msg {
    Log(Level, String),
    SignedIn(SignIn),
    Registered(RegisterReply),
    Recovered(RecoverReply),
    LaunchFinished(Result<(), String>),
}

/// Which of the three things the window is doing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    SignIn,
    Register,
    Recover,
}

struct LogLine {
    level: Level,
    text: String,
}

pub struct LauncherApp {
    layout: Layout,
    screen: Screen,

    identity: String,
    password: String,
    server_ip: String,
    port_text: String,
    /// The client directory, editable. The owner, 2026-08-29: *"We should probably let the player
    /// choose where the MapleStory.exe is."* Held as text rather than a `PathBuf` so a
    /// half-typed path is a half-typed path and not a resolution failure on every keystroke.
    client_dir_text: String,

    // The Register screen.
    reg_username: String,
    reg_email: String,
    reg_password: String,
    reg_confirm: String,
    reg_code: String,

    // The Forgot-password screen.
    rec_identity: String,
    rec_code: String,
    rec_password: String,
    rec_confirm: String,

    signing_in: bool,
    launching: bool,
    /// A registration or recovery is in flight.
    working: bool,
    /// `Some` once a sign-in has succeeded. This is the gate on **Start Game**.
    signed_in: Option<SignIn>,
    status: Option<(Level, String)>,

    /// Open the log pane on the next frame. Set whenever a warning or error is pushed, so a
    /// collapsed log never hides a problem - the status line carries the headline, the pane
    /// carries the detail, and the pane opens itself the moment there is detail worth reading.
    reveal_log: bool,
    log: Vec<LogLine>,
    tx: Sender<Msg>,
    rx: Receiver<Msg>,
}

impl LauncherApp {
    pub fn new(cc: &eframe::CreationContext<'_>, layout: Layout) -> LauncherApp {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut app = LauncherApp {
            server_ip: layout.server_ip.clone(),
            port_text: layout.port.to_string(),
            client_dir_text: layout.client_dir.display().to_string(),
            layout,
            screen: Screen::SignIn,
            identity: String::new(),
            password: String::new(),
            reg_username: String::new(),
            reg_email: String::new(),
            reg_password: String::new(),
            reg_confirm: String::new(),
            reg_code: String::new(),
            rec_identity: String::new(),
            rec_code: String::new(),
            rec_password: String::new(),
            rec_confirm: String::new(),
            signing_in: false,
            launching: false,
            working: false,
            signed_in: None,
            status: None,
            reveal_log: false,
            log: Vec::new(),
            tx,
            rx,
        };
        app.announce_layout();
        // Slightly roomier text than the default; the log pane is the point of the window.
        cc.egui_ctx.style_mut(|s| {
            s.spacing.item_spacing.y = 6.0;
        });
        app
    }

    /// Say what was found, and what is missing, before anybody presses anything. A wrong path
    /// guess has to be visible rather than mysterious.
    fn announce_layout(&mut self) {
        self.push(Level::Info, format!("launcher: {}", self.layout.exe_dir.display()));
        self.push(Level::Info, format!("paths from: {}", self.layout.source.label()));
        // The game folder gets its own line naming its source, because a remembered choice
        // that has gone stale looks exactly like a wrong guess until the line says which.
        self.push(
            Level::Info,
            format!(
                "game folder: {} ({})",
                self.layout.client_dir.display(),
                self.layout.client_dir_from
            ),
        );
        if let Some(cfg) = self.layout.config_file.clone() {
            self.push(Level::Info, format!("config: {}", cfg.display()));
            let applied = self.layout.config_applied.join(", ");
            if applied.is_empty() {
                self.push(Level::Warn, "the config file set nothing".into());
            } else {
                self.push(Level::Info, format!("config set: {applied}"));
            }
        }
        for problem in self.layout.config_problems.clone() {
            self.push(Level::Warn, format!("config: {problem}"));
        }
        for problem in self.layout.problems() {
            self.push(Level::Warn, problem);
        }
    }

    fn push(&mut self, level: Level, text: String) {
        if matches!(level, Level::Warn | Level::Error) {
            self.reveal_log = true;
        }
        self.log.push(LogLine { level, text });
    }

    fn fail(&mut self, text: String) {
        self.push(Level::Error, text.clone());
        self.status = Some((Level::Error, text));
    }

    fn port(&self) -> Result<u16, String> {
        match self.port_text.trim().parse::<u16>() {
            Ok(0) | Err(_) => Err(format!("{:?} is not a port (1..=65535)", self.port_text.trim())),
            Ok(p) => Ok(p),
        }
    }

    fn busy(&self) -> bool {
        self.signing_in || self.launching || self.working
    }

    fn can_sign_in(&self) -> bool {
        !self.busy() && !self.identity.trim().is_empty() && !self.password.is_empty()
    }

    fn can_register(&self) -> bool {
        !self.busy()
            && !self.reg_username.trim().is_empty()
            && !self.reg_email.trim().is_empty()
            && !self.reg_password.is_empty()
            && !self.reg_confirm.is_empty()
            && !self.reg_code.trim().is_empty()
    }

    fn can_recover(&self) -> bool {
        !self.busy()
            && !self.rec_identity.trim().is_empty()
            && !self.rec_code.trim().is_empty()
            && !self.rec_password.is_empty()
            && !self.rec_confirm.is_empty()
    }

    /// Where the sign-in service is and which certificate it must present - or the sentence
    /// saying why nothing will be sent. Every server call starts here.
    fn service_target(&mut self) -> Option<(String, u16, tlspin::Fingerprint)> {
        let host = self.server_ip.trim().to_string();
        let port = self.layout.auth_port;
        match self.layout.auth_fingerprint {
            Some(pin) => {
                self.push(Level::Info, format!("{host}:{port} over TLS, pinned to {}", pin.short()));
                Some((host, port, pin))
            }
            None => {
                self.fail(session::NOT_PINNED.to_string());
                None
            }
        }
    }

    fn start_sign_in(&mut self, ctx: &egui::Context) {
        // A copy goes to the worker and is dropped there. The field itself is wiped on
        // SUCCESS, not here: a failed sign-in that also silently emptied the box would have
        // The owner retyping a password to find out they had typed it right the first time.
        // The SERVER checks the password, not this machine. A client machine has no
        // database to read - `crate::http` has the whole reasoning. It goes over TLS to the
        // one certificate this launcher has pinned; with no pin it does not go at all.
        let Some((host, auth_port, pin)) = self.service_target() else { return };
        let identity = self.identity.trim().to_string();
        let password = self.password.clone();
        let tx = self.tx.clone();
        let ctx = ctx.clone();

        self.signing_in = true;
        self.signed_in = None;
        self.status = Some((Level::Info, "checking the password (argon2id is slow on purpose)…".into()));

        thread::spawn(move || {
            let outcome = session::sign_in(&host, auth_port, Some(&pin), &identity, &password);
            drop(password);
            let _ = tx.send(Msg::SignedIn(outcome));
            ctx.request_repaint();
        });
    }

    /// Everything about a registration that can be decided without the server, decided
    /// first - so the sentence appears at once and the round trip is not spent on a typo.
    fn register_checks(&self) -> Result<(), String> {
        if self.reg_password != self.reg_confirm {
            return Err("the two passwords are not the same".into());
        }
        store::check_password_policy(&self.reg_password)?;
        if !self.reg_email.trim().contains('@') {
            return Err("that does not look like an email address".into());
        }
        Ok(())
    }

    fn start_register(&mut self, ctx: &egui::Context) {
        if let Err(why) = self.register_checks() {
            self.fail(why);
            return;
        }
        let Some((host, auth_port, pin)) = self.service_target() else { return };
        let username = self.reg_username.trim().to_string();
        let email = self.reg_email.trim().to_string();
        let password = self.reg_password.clone();
        let code = self.reg_code.trim().to_string();
        let tx = self.tx.clone();
        let ctx = ctx.clone();

        self.working = true;
        self.status = Some((Level::Info, "registering…".into()));
        self.push(Level::Info, format!("--- registering {username} ---"));

        thread::spawn(move || {
            let reply = crate::http::register(&host, auth_port, &pin, &username, &email, &password, &code);
            drop(password);
            let _ = tx.send(Msg::Registered(reply));
            ctx.request_repaint();
        });
    }

    fn recover_checks(&self) -> Result<(), String> {
        if self.rec_password != self.rec_confirm {
            return Err("the two passwords are not the same".into());
        }
        store::check_password_policy(&self.rec_password)?;
        Ok(())
    }

    fn start_recover(&mut self, ctx: &egui::Context) {
        if let Err(why) = self.recover_checks() {
            self.fail(why);
            return;
        }
        let Some((host, auth_port, pin)) = self.service_target() else { return };
        let identity = self.rec_identity.trim().to_string();
        let code = self.rec_code.trim().to_string();
        let password = self.rec_password.clone();
        let tx = self.tx.clone();
        let ctx = ctx.clone();

        self.working = true;
        self.status = Some((Level::Info, "setting the new password…".into()));
        self.push(Level::Info, format!("--- password recovery for {identity} ---"));

        thread::spawn(move || {
            let reply = crate::http::recover(&host, auth_port, &pin, &identity, &code, &password);
            drop(password);
            let _ = tx.send(Msg::Recovered(reply));
            ctx.request_repaint();
        });
    }

    /// Ask for `MapleStory.exe` and keep the directory it is in.
    ///
    /// The dialog picks the **executable** rather than a folder, because "where is
    /// MapleStory.exe" is the question a person can answer - a folder picker asks them to
    /// know which of several folders is the right one.
    fn browse_for_client(&mut self) {
        let start = std::path::PathBuf::from(self.client_dir_text.trim());
        let start = start.is_dir().then_some(start);
        let Some(exe) = crate::launch::pick_client_exe(start.as_deref()) else {
            // Cancelled, or the dialog failed. Indistinguishable through this API and not
            // worth interrupting anybody over - the field is untouched either way.
            return;
        };
        match exe.parent() {
            Some(dir) => {
                self.client_dir_text = dir.display().to_string();
                self.push(Level::Good, format!("game folder set to {}", dir.display()));
                self.commit_client_dir();
            }
            None => self.push(Level::Warn, format!("{} has no parent folder", exe.display())),
        }
    }

    /// Take whatever is in the box and make it the layout's client directory.
    ///
    /// Called before a launch as well as after Browse, so a **typed** path counts - somebody
    /// pasting a path and pressing Start Game should not silently launch the old one.
    fn commit_client_dir(&mut self) {
        let typed = self.client_dir_text.trim();
        if typed.is_empty() || self.layout.client_dir == std::path::Path::new(typed) {
            return;
        }
        self.layout.client_dir = std::path::PathBuf::from(typed);
        self.remember_client_dir();
    }

    /// Save the chosen folder beside the executable, so the next start opens on it.
    ///
    /// The owner, 2026-09-05: *"Setting it every time is going to be very frustrating for users."*
    /// Until this, Browse changed the running launcher and nothing else.
    ///
    /// Only a folder that actually holds `MapleStory.exe` is remembered. Browse cannot produce
    /// any other kind, and a typed one that does not is a typo rather than a choice -
    /// remembering it would greet the next start with a red line about a folder nobody meant.
    /// The launch that follows a bad typed path fails with its own message, so the line here
    /// is information, not a second alarm.
    fn remember_client_dir(&mut self) {
        if !self.layout.client_exe().is_file() {
            self.push(
                Level::Info,
                format!(
                    "no {} in {} - not remembered for next time",
                    crate::paths::CLIENT_EXE_NAME,
                    self.layout.client_dir.display()
                ),
            );
            return;
        }
        match crate::remembered::save_client_dir(&self.layout.exe_dir, &self.layout.client_dir) {
            Ok(path) => self.push(
                Level::Info,
                format!("game folder remembered for next time in {}", path.display()),
            ),
            // A read-only install directory, most likely. The launch still works with the
            // folder in the box; only the memory of it is lost, and the line says so.
            Err(e) => self.push(
                Level::Warn,
                format!(
                    "could not remember the game folder - {} is not writable: {e}",
                    self.layout.exe_dir.join(crate::remembered::FILE_NAME).display()
                ),
            ),
        }
    }

    fn start_launch(&mut self, ctx: &egui::Context, plan: Plan) {
        let layout = self.layout.clone();
        let tx = self.tx.clone();
        let ctx = ctx.clone();
        // Carried into the worker so the launch can be registered the instant the client
        // starts. `Start Game` is gated on a successful sign-in, so this is `Some` in every
        // path a person can reach - and `prepare` says so out loud if it ever is not.
        let launch_id = self.signed_in.as_ref().and_then(|s| s.launch_id()).cloned();
        // And the credential the CLIENT will carry. `None` when the server issued none, which
        // `prepare` turns into a deletion rather than an empty file - a stale token is refused
        // by the login server, not ignored, so leaving one behind is worse than having none.
        let client_token = self.signed_in.as_ref().and_then(|s| s.client_token()).cloned();

        self.launching = true;
        self.status = Some((Level::Info, "preparing the client…".into()));
        self.push(Level::Info, "--- Start Game ---".into());

        thread::spawn(move || {
            let mut emit = |level: Level, text: String| {
                let _ = tx.send(Msg::Log(level, text));
                ctx.request_repaint();
            };
            let result = prepare::prepare_and_launch(
                &layout,
                &plan,
                launch_id.as_ref(),
                client_token.as_ref(),
                &mut emit,
            );
            let _ = tx.send(Msg::LaunchFinished(result));
            ctx.request_repaint();
        });
    }

    fn drain(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                Msg::Log(level, text) => self.push(level, text),
                Msg::SignedIn(outcome) => {
                    self.signing_in = false;
                    let text = outcome.message();
                    if outcome.is_ok() {
                        wipe(&mut self.password);
                        self.push(Level::Good, text.clone());
                        self.status = Some((Level::Good, text));
                        self.signed_in = Some(outcome);
                    } else {
                        self.fail(text);
                        self.signed_in = None;
                    }
                }
                Msg::Registered(reply) => {
                    self.working = false;
                    let text = reply.message();
                    if let RegisterReply::Ok { username, .. } = &reply {
                        // Straight to the sign-in screen with the name filled in. The code is
                        // spent and the passwords are gone; the log says what to do next.
                        self.identity = username.clone();
                        wipe(&mut self.reg_password);
                        wipe(&mut self.reg_confirm);
                        self.reg_code.clear();
                        self.screen = Screen::SignIn;
                        self.push(Level::Good, text.clone());
                        self.status = Some((Level::Good, text));
                    } else {
                        self.fail(text);
                    }
                }
                Msg::Recovered(reply) => {
                    self.working = false;
                    let text = reply.message();
                    if let RecoverReply::Ok { username } = &reply {
                        self.identity = username.clone();
                        wipe(&mut self.rec_password);
                        wipe(&mut self.rec_confirm);
                        self.rec_code.clear();
                        self.screen = Screen::SignIn;
                        self.push(Level::Good, text.clone());
                        self.status = Some((Level::Good, text));
                    } else {
                        self.fail(text);
                    }
                }
                Msg::LaunchFinished(result) => {
                    self.launching = false;
                    match result {
                        Ok(()) => {
                            self.status = Some((Level::Good, "the client is starting".into()));
                        }
                        Err(e) => self.fail(e),
                    }
                }
            }
        }
    }

    fn text_row(ui: &mut egui::Ui, enabled: bool, label: &str, value: &mut String, password: bool, hint: &str) {
        ui.label(label);
        let mut edit = egui::TextEdit::singleline(value).desired_width(300.0).password(password);
        if !hint.is_empty() {
            edit = edit.hint_text(hint);
        }
        ui.add_enabled(enabled, edit);
        ui.end_row();
    }
}

/// Empty a password field, overwriting the bytes it held first.
///
/// **Not a security boundary, and it should not be described as one.** egui keeps its own
/// undo history for a `TextEdit`, the worker thread had a copy, and any process can be
/// dumped. What is actually guaranteed is narrower and is what matters here: the password is
/// never written to a file, never put in the log pane, and never leaves the process except
/// over the pinned TLS connection to the sign-in service.
fn wipe(password: &mut String) {
    let len = password.len();
    if len > 0 {
        // Same byte length, so this overwrites in place rather than reallocating and leaving
        // the old buffer behind.
        password.replace_range(.., &"\0".repeat(len));
    }
    password.clear();
}

fn colour(level: Level) -> Color32 {
    match level {
        Level::Info => Color32::from_rgb(0x9a, 0x9a, 0x9a),
        Level::Good => Color32::from_rgb(0x6d, 0xc0, 0x6d),
        Level::Warn => Color32::from_rgb(0xd8, 0xa6, 0x3a),
        Level::Error => Color32::from_rgb(0xe0, 0x6c, 0x6c),
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.drain();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(WINDOW_TITLE);
            ui.label(
                RichText::new(NO_CREDENTIALS_NOTE)
                    .small()
                    .color(colour(Level::Info)),
            );
            ui.separator();

            let busy = self.busy();

            // Which of the three things this window does. Register and Forgot password exist
            // because a client machine has no database and no useradd: the only way in is a
            // code from the administrator, typed here.
            ui.horizontal(|ui| {
                for (screen, label) in [
                    (Screen::SignIn, "Sign in"),
                    (Screen::Register, "Register"),
                    (Screen::Recover, "Forgot password"),
                ] {
                    let selected = self.screen == screen;
                    if ui.add_enabled(!busy, egui::SelectableLabel::new(selected, label)).clicked()
                        && !selected
                    {
                        self.screen = screen;
                        self.status = None;
                    }
                }
            });
            ui.add_space(4.0);

            egui::Grid::new("fields")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    match self.screen {
                        Screen::SignIn => {
                            Self::text_row(ui, !busy, "Email or account name", &mut self.identity, false, "");
                            Self::text_row(ui, !busy, "Password", &mut self.password, true, "");
                        }
                        Screen::Register => {
                            Self::text_row(ui, !busy, "Username", &mut self.reg_username, false, "3-24 letters, digits, underscore");
                            Self::text_row(ui, !busy, "Email", &mut self.reg_email, false, "you@example.com");
                            Self::text_row(ui, !busy, "Password", &mut self.reg_password, true, store::PASSWORD_POLICY);
                            Self::text_row(ui, !busy, "Confirm password", &mut self.reg_confirm, true, "");
                            Self::text_row(ui, !busy, "Registration code", &mut self.reg_code, false, "from the administrator, e.g. 7K3M-PQ2X");
                        }
                        Screen::Recover => {
                            Self::text_row(ui, !busy, "Email or account name", &mut self.rec_identity, false, "");
                            Self::text_row(ui, !busy, "Recovery code", &mut self.rec_code, false, "from the administrator, e.g. 7K3M-PQ2X");
                            Self::text_row(ui, !busy, "New password", &mut self.rec_password, true, store::PASSWORD_POLICY);
                            Self::text_row(ui, !busy, "Confirm new password", &mut self.rec_confirm, true, "");
                        }
                    }

                    ui.label("Server IP");
                    ui.add_enabled(
                        !busy,
                        egui::TextEdit::singleline(&mut self.server_ip).desired_width(300.0),
                    );
                    ui.end_row();

                    ui.label("Port");
                    ui.add_enabled(
                        !busy,
                        egui::TextEdit::singleline(&mut self.port_text).desired_width(90.0),
                    );
                    ui.end_row();

                    // **Where MapleStory.exe is.** Editable and browsable, because every
                    // install is somewhere different and the resolver can only guess.
                    ui.label("Game folder");
                    ui.horizontal(|ui| {
                        ui.add_enabled(
                            !busy,
                            egui::TextEdit::singleline(&mut self.client_dir_text)
                                .desired_width(224.0),
                        );
                        if ui.add_enabled(!busy, egui::Button::new("Browse…")).clicked() {
                            self.browse_for_client();
                        }
                    });
                    ui.end_row();
                });

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                match self.screen {
                    Screen::SignIn => {
                        if ui
                            .add_enabled(self.can_sign_in(), egui::Button::new("Login"))
                            .clicked()
                        {
                            self.start_sign_in(ctx);
                        }

                        // **Sign out, so a second account does not need a second launcher.**
                        //
                        // The owner, 2026-09-02, on the run where two clients first worked: *"I had
                        // to close and reopen the launcher to be able to login to another
                        // account since there's no logout button."* Two clients means two
                        // accounts, and the shape that was fine for one player is a restart for
                        // every swap.
                        //
                        // It clears the sign-in and the password, and **leaves the identity**,
                        // which is a deliberate asymmetry: the next sign-in is usually the
                        // OTHER account, so the field wants replacing rather than preserving -
                        // but retyping a name you can see is cheap, and losing what you typed
                        // is annoying.
                        //
                        // No server call. A claim is keyed per launch and expires on its own;
                        // nothing here can revoke one, and pretending otherwise in the UI would
                        // be a lie about what the button does.
                        if ui
                            .add_enabled(self.signed_in.is_some() && !busy, egui::Button::new("Sign out"))
                            .clicked()
                        {
                            self.signed_in = None;
                            wipe(&mut self.password);
                            self.status = Some((
                                Level::Info,
                                "signed out - type another account and press Login".into(),
                            ));
                            self.push(
                                Level::Info,
                                "--- signed out. The login claim from that sign-in is NOT revoked: it \
                                 is keyed per launch and expires on its own. A client already running \
                                 keeps its own session ---"
                                    .into(),
                            );
                        }

                        // Disabled until a sign-in has succeeded - the whole point of the
                        // two-button shape the owner asked for.
                        let ready = self.signed_in.is_some() && !busy;
                        let start = ui.add_enabled(ready, egui::Button::new("Start Game"));
                        if start.clicked() {
                            // A typed path counts, not only a browsed one.
                            self.commit_client_dir();
                            match self.port() {
                                Ok(port) => {
                                    let plan = Plan {
                                        ip: self.server_ip.trim().to_string(),
                                        port,
                                    };
                                    self.start_launch(ctx, plan);
                                }
                                Err(e) => self.fail(e),
                            }
                        }
                        if self.signed_in.is_none() {
                            let _ = start.on_disabled_hover_text("sign in first");
                        }
                    }
                    Screen::Register => {
                        if ui
                            .add_enabled(self.can_register(), egui::Button::new("Create account"))
                            .clicked()
                        {
                            self.start_register(ctx);
                        }
                        ui.label(
                            RichText::new("The code is single use and comes from the administrator.")
                                .small()
                                .color(colour(Level::Info)),
                        );
                    }
                    Screen::Recover => {
                        if ui
                            .add_enabled(self.can_recover(), egui::Button::new("Set new password"))
                            .clicked()
                        {
                            self.start_recover(ctx);
                        }
                        ui.label(
                            RichText::new("The code is single use, lasts a day, and is minted for your account only.")
                                .small()
                                .color(colour(Level::Info)),
                        );
                    }
                }

                if busy {
                    ui.spinner();
                }
            });

            if let Some((level, text)) = &self.status {
                ui.add_space(2.0);
                ui.label(RichText::new(text).color(colour(*level)));
            }

            // The "Where the launcher is looking" panel was here and is GONE. The owner,
            // 2026-08-29: *"just remove the entire section."*
            //
            // It listed source, launcher, client, stub, output, archives and config - and
            // most of those are not things a player has any use for. The three that matter
            // are the fields above: where MapleStory.exe is, which server, which ports.
            //
            // **Nothing was hidden by removing it.** The panel also rendered
            // `Layout::problems()`, and `announce_layout` already pushes exactly those into
            // the Log pane at startup - so a missing MapleStory.exe still says so, in the
            // place the rest of the run is reported. `--print-paths` still prints everything
            // for a machine where a window cannot be scripted into answering.

            ui.separator();

            // **The log is collapsed by default.** The owner, 2026-09-05: *"can we hide it inside
            // a collapsible panel since the average user will not care?"* The status line
            // under the buttons is what a player reads; the pane is for the run that went
            // wrong. Two things keep a collapsed log from hiding a problem: the header counts
            // warnings, and `push` opens the pane whenever a warning or error arrives - so
            // `announce_layout`'s "MapleStory.exe not found" at startup is still on screen,
            // in the place the rest of the run is reported.
            let warnings = self
                .log
                .iter()
                .filter(|l| matches!(l.level, Level::Warn | Level::Error))
                .count();
            let title = if warnings == 0 {
                format!("Log ({} lines)", self.log.len())
            } else {
                format!("Log ({} lines, {warnings} warnings)", self.log.len())
            };
            // Forced open for one frame when something worth reading arrived; otherwise the
            // header keeps whatever the person last set it to.
            let force_open = std::mem::take(&mut self.reveal_log).then_some(true);
            egui::CollapsingHeader::new(RichText::new(title).strong())
                // A fixed id, because the title changes with every line and egui would
                // otherwise key the open state on the text and forget it each time.
                .id_salt("log-pane")
                .default_open(false)
                .open(force_open)
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(260.0)
                        .auto_shrink([false, false])
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            for line in &self.log {
                                ui.label(
                                    RichText::new(&line.text)
                                        .monospace()
                                        .size(11.0)
                                        .color(colour(line.level)),
                                );
                            }
                        });
                });
        });
    }
}
