//! `maplecw-launcher` - the executable. Everything real is in the library beside it.
//!
//! # Why this file is three lines
//!
//! The launcher is built with `requireAdministrator` embedded in its manifest (see
//! `build.rs`), so Windows raises UAC when it is double-clicked - which is the point. But a
//! `#[cfg(test)]` module inside a **binary** target is compiled into a test harness built
//! from this same target, and that harness inherits the manifest. `cargo test` then cannot
//! start it:
//!
//! ```text
//! could not execute process ...\maplecw_launcher-<hash>.exe (never executed)
//! Caused by: The requested operation requires elevation. (os error 740)
//! ```
//!
//! So the code and its tests live in the library, whose harness carries no manifest, and the
//! binary is a shell with `test = false` in `Cargo.toml`. Moving the tests to another module
//! would not have helped - they would still have been the *binary's* harness.

// A GUI launcher should not flash a console window on a test machine. Debug builds keep the
// console, so a panic during development still says where it came from.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    launcher::run();
}
