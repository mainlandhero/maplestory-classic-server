//! MapleCW launcher.
//!
//! Sign in, stake the login claim, neutralise GameGuard, arm the hook, start the client.
//!
//! **Nothing here authenticates the game socket.** The client never sends a credential on it.
//! A successful sign-in verifies a password against an argon2id hash and stakes a claim -
//! a row the login server reads to decide which account to serve the next connection as. That
//! is account *selection*, not authentication, and the window says so.
//!
//! The modules, in the order the buttons use them:
//!
//! | module | what it owns |
//! |---|---|
//! | [`config`] | the optional `maplecw-launcher.toml` beside the exe |
//! | [`paths`] | resolving the client, database, stub and output directories - **never hard-coded** |
//! | [`session`] | Login: argon2id verification, then staking the login claim |
//! | [`client`] | the GameGuard stub, the hook markers, archiving the previous run's log |
//! | [`launch`] | `ShellExecuteW`, because the client has an elevation manifest |
//! | [`prepare`] | Start Game: the whole sequence, in order |
//! | [`app`] | the window |

// A GUI launcher should not flash a console window on a test machine. Debug builds keep the
// console, so a panic during development still says where it came from - and `cargo test`
// needs one.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod client;
mod config;
mod launch;
mod paths;
mod prepare;
mod session;

#[cfg(test)]
mod testutil;

const USAGE: &str = "\
maplecw-launcher [--print-paths]

  (no arguments)  open the launcher window
  --print-paths   print the resolved client, database, stub and output paths, then exit
  --help          this

The launcher takes no server address on the command line: the window has fields for it, and
a maplecw-launcher.toml beside the executable can set defaults (client_dir, db_path,
stub_path, server_ip, port).";

/// Print, or put it in a window when there is nothing to print to.
///
/// The order matters both ways. A modal box in front of a script that just wanted the paths
/// would be worse than no output at all - and `println!` **panics** ("failed printing to
/// stdout") when the handle is null, which is exactly what a release build started from
/// Explorer has. So the handle is checked first rather than written to and hoped for.
fn report(text: &str) {
    if launch::stdout_is_usable() {
        println!("{text}");
    } else {
        launch::info_box(app::WINDOW_TITLE, text);
    }
}

fn main() {
    // A release build is a `windows`-subsystem binary with no console, so an unhandled panic
    // would close the window and say nothing at all - the failure mode this project spends
    // the most effort avoiding everywhere else. The hook covers worker threads too.
    std::panic::set_hook(Box::new(|info| {
        let message = format!("The launcher hit a bug and stopped.\n\n{info}");
        // Guarded: `eprintln!` panics on a missing handle, and a panic inside a panic hook
        // aborts the process before the box below can be shown.
        if launch::stderr_is_usable() {
            eprintln!("{message}");
        }
        launch::message_box(app::WINDOW_TITLE, &message);
    }));

    // Resolved before the window exists, so the window can show what it found - including a
    // wrong guess, which is the failure this has to make visible rather than mysterious.
    let layout = paths::resolve();

    // `--print-paths` is the check an installed launcher can be scripted into giving and a
    // window cannot: "did this machine's copy find the right client?" answered without a GUI
    // and without launching anything.
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--print-paths" || a == "--paths") {
        report(&layout.report());
        return;
    }
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        report(USAGE);
        return;
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([560.0, 640.0])
            .with_min_inner_size([460.0, 420.0])
            .with_title(app::WINDOW_TITLE),
        ..Default::default()
    };

    let result = eframe::run_native(
        app::WINDOW_TITLE,
        options,
        Box::new(move |cc| Ok(Box::new(app::LauncherApp::new(cc, layout)))),
    );

    if let Err(e) = result {
        // In a release build there is no console to print into, so a failure this early would
        // otherwise be completely silent.
        let message = format!("The launcher window could not be created.\n\n{e}");
        if launch::stderr_is_usable() {
            eprintln!("{message}");
        }
        launch::message_box(app::WINDOW_TITLE, &message);
    }
}
