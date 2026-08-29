//! The window: three inputs, two buttons, and a log pane that says what happened.
//!
//! The owner asked for exactly this: *"input of email and password and server IP to be able to
//! direct the client. There should be two buttons, Login then Start Game. Login validates the
//! session and then enables the 'Start Game' button."* So **Start Game is disabled until a
//! sign-in succeeds**, and both buttons do their work on a worker thread - argon2id is
//! deliberately slow, and a window that stops repainting during it looks hung.

use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use egui::{Color32, RichText};

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
    LaunchFinished(Result<(), String>),
}

struct LogLine {
    level: Level,
    text: String,
}

pub struct LauncherApp {
    layout: Layout,

    identity: String,
    password: String,
    server_ip: String,
    port_text: String,

    signing_in: bool,
    launching: bool,
    /// `Some` once a sign-in has succeeded. This is the gate on **Start Game**.
    signed_in: Option<SignIn>,
    status: Option<(Level, String)>,

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
            layout,
            identity: String::new(),
            password: String::new(),
            signing_in: false,
            launching: false,
            signed_in: None,
            status: None,
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
        self.log.push(LogLine { level, text });
    }

    fn port(&self) -> Result<u16, String> {
        match self.port_text.trim().parse::<u16>() {
            Ok(0) | Err(_) => Err(format!("{:?} is not a port (1..=65535)", self.port_text.trim())),
            Ok(p) => Ok(p),
        }
    }

    fn can_sign_in(&self) -> bool {
        !self.signing_in
            && !self.launching
            && !self.identity.trim().is_empty()
            && !self.password.is_empty()
    }

    fn start_sign_in(&mut self, ctx: &egui::Context) {
        // A copy goes to the worker and is dropped there. The field itself is wiped on
        // SUCCESS, not here: a failed sign-in that also silently emptied the box would have
        // The owner retyping a password to find out they had typed it right the first time.
        let db = self.layout.db_path.clone();
        let identity = self.identity.trim().to_string();
        let password = self.password.clone();
        let tx = self.tx.clone();
        let ctx = ctx.clone();

        self.signing_in = true;
        self.signed_in = None;
        self.status = Some((Level::Info, "checking the password (argon2id is slow on purpose)…".into()));
        self.push(Level::Info, format!("signing in against {}", db.display()));

        thread::spawn(move || {
            let outcome = session::sign_in(&db, &identity, &password);
            drop(password);
            let _ = tx.send(Msg::SignedIn(outcome));
            ctx.request_repaint();
        });
    }

    fn start_launch(&mut self, ctx: &egui::Context, plan: Plan) {
        let layout = self.layout.clone();
        let tx = self.tx.clone();
        let ctx = ctx.clone();

        self.launching = true;
        self.status = Some((Level::Info, "preparing the client…".into()));
        self.push(Level::Info, "--- Start Game ---".into());

        thread::spawn(move || {
            let mut emit = |level: Level, text: String| {
                let _ = tx.send(Msg::Log(level, text));
                ctx.request_repaint();
            };
            let result = prepare::prepare_and_launch(&layout, &plan, &mut emit);
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
                        self.push(Level::Error, text.clone());
                        self.status = Some((Level::Error, text));
                        self.signed_in = None;
                    }
                }
                Msg::LaunchFinished(result) => {
                    self.launching = false;
                    match result {
                        Ok(()) => {
                            self.status = Some((Level::Good, "the client is starting".into()));
                        }
                        Err(e) => {
                            self.push(Level::Error, e.clone());
                            self.status = Some((Level::Error, e));
                        }
                    }
                }
            }
        }
    }
}

/// Empty a password field, overwriting the bytes it held first.
///
/// **Not a security boundary, and it should not be described as one.** egui keeps its own
/// undo history for a `TextEdit`, the worker thread had a copy, and any process can be
/// dumped. What is actually guaranteed is narrower and is what matters here: the password is
/// never written to a file, never put in the log pane, and never leaves the process except
/// into `store`'s argon2id verification.
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

            let busy = self.signing_in || self.launching;

            egui::Grid::new("fields")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Email or account name");
                    ui.add_enabled(
                        !busy,
                        egui::TextEdit::singleline(&mut self.identity).desired_width(300.0),
                    );
                    ui.end_row();

                    ui.label("Password");
                    ui.add_enabled(
                        !busy,
                        egui::TextEdit::singleline(&mut self.password)
                            .password(true)
                            .desired_width(300.0),
                    );
                    ui.end_row();

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
                });

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui
                    .add_enabled(self.can_sign_in(), egui::Button::new("Login"))
                    .clicked()
                {
                    self.start_sign_in(ctx);
                }

                // Disabled until a sign-in has succeeded - the whole point of the two-button
                // shape the owner asked for.
                let ready = self.signed_in.is_some() && !busy;
                let start = ui.add_enabled(ready, egui::Button::new("Start Game"));
                if start.clicked() {
                    match self.port() {
                        Ok(port) => {
                            let plan = Plan {
                                ip: self.server_ip.trim().to_string(),
                                port,
                            };
                            self.start_launch(ctx, plan);
                        }
                        Err(e) => {
                            self.push(Level::Error, e.clone());
                            self.status = Some((Level::Error, e));
                        }
                    }
                }
                if self.signed_in.is_none() {
                    let _ = start.on_disabled_hover_text("sign in first");
                }

                if busy {
                    ui.spinner();
                }
            });

            if let Some((level, text)) = &self.status {
                ui.add_space(2.0);
                ui.label(RichText::new(text).color(colour(*level)));
            }

            ui.add_space(4.0);
            egui::CollapsingHeader::new("Where the launcher is looking")
                .default_open(false)
                .show(ui, |ui| {
                    egui::Grid::new("paths")
                        .num_columns(2)
                        .spacing([10.0, 4.0])
                        .show(ui, |ui| {
                            let rows = [
                                ("source", self.layout.source.label().to_string()),
                                ("launcher", self.layout.exe_dir.display().to_string()),
                                ("client", self.layout.client_dir.display().to_string()),
                                ("database", self.layout.db_path.display().to_string()),
                                ("stub", self.layout.stub_path.display().to_string()),
                                ("output", self.layout.data_root.display().to_string()),
                                (
                                    "archives",
                                    self.layout.previous_runs_dir().display().to_string(),
                                ),
                                (
                                    "config",
                                    match &self.layout.config_file {
                                        Some(p) => p.display().to_string(),
                                        None => format!(
                                            "none ({} beside the launcher would be read)",
                                            crate::config::CONFIG_FILE_NAME
                                        ),
                                    },
                                ),
                            ];
                            for (label, value) in rows {
                                ui.label(label);
                                ui.label(RichText::new(value).monospace().small());
                                ui.end_row();
                            }
                        });
                    for problem in self.layout.problems() {
                        ui.label(RichText::new(format!("• {problem}")).color(colour(Level::Warn)));
                    }
                });

            ui.separator();
            ui.label(RichText::new("Log").strong());
            egui::ScrollArea::vertical()
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
    }
}
