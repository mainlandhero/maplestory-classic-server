//! Local account administration.
//!
//!   maplecw-useradd <name>              create an account
//!   maplecw-useradd --list              list accounts
//!   maplecw-useradd --passwd <name>     change a password
//!   maplecw-useradd --disable <name>
//!   maplecw-useradd --enable  <name>
//!
//! Passwords are read from a hidden prompt, or from stdin when it is piped. They are
//! **never** accepted as a command-line argument: arguments land in shell history and
//! are visible to any other process on the machine via the process list.

use std::io::IsTerminal;

use store::Store;

const DEFAULT_DB: &str = "maplecw.db";

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args[0] == "-h" || args[0] == "--help" {
        usage();
        return std::process::ExitCode::SUCCESS;
    }

    // Optional --db, anywhere in the arguments.
    let mut db_path = String::from(DEFAULT_DB);
    let mut rest: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--db" && i + 1 < args.len() {
            db_path = args[i + 1].clone();
            i += 2;
        } else {
            rest.push(args[i].clone());
            i += 1;
        }
    }

    let store = match Store::open(&db_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("could not open database {db_path}: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };

    let result = match rest.first().map(String::as_str) {
        Some("--list") => list(&store),
        Some("--passwd") => match rest.get(1) {
            Some(name) => set_password(&store, name),
            None => Err("--passwd needs an account name".into()),
        },
        Some("--disable") => match rest.get(1) {
            Some(name) => toggle(&store, name, false),
            None => Err("--disable needs an account name".into()),
        },
        Some("--enable") => match rest.get(1) {
            Some(name) => toggle(&store, name, true),
            None => Err("--enable needs an account name".into()),
        },
        Some(flag) if flag.starts_with('-') => Err(format!("unknown option: {flag}")),
        Some(name) => create(&store, name),
        None => {
            usage();
            Ok(())
        }
    };

    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn usage() {
    println!(
        "usage:\n  \
         maplecw-useradd <name>            create an account\n  \
         maplecw-useradd --list            list accounts\n  \
         maplecw-useradd --passwd <name>   change a password\n  \
         maplecw-useradd --disable <name>\n  \
         maplecw-useradd --enable  <name>\n\n\
         options:\n  \
         --db <path>                       database file (default {DEFAULT_DB})\n\n\
         Passwords are read from a prompt or stdin, never from an argument."
    );
}

/// Read a password without echoing it. Falls back to stdin when piped, so the tool can
/// be scripted without ever putting the password on a command line.
fn read_password(prompt: &str, confirm: bool) -> Result<String, String> {
    if !std::io::stdin().is_terminal() {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .map_err(|e| format!("could not read password from stdin: {e}"))?;
        let pw = buf.trim_end_matches(['\r', '\n']).to_string();
        if pw.is_empty() {
            return Err("no password supplied on stdin".into());
        }
        return Ok(pw);
    }

    let pw = rpassword::prompt_password(prompt)
        .map_err(|e| format!("could not read password: {e}"))?;
    if confirm {
        let again = rpassword::prompt_password("Confirm password: ")
            .map_err(|e| format!("could not read password: {e}"))?;
        if pw != again {
            return Err("passwords did not match".into());
        }
    }
    Ok(pw)
}

fn create(store: &Store, name: &str) -> Result<(), String> {
    let pw = read_password(&format!("Password for {name}: "), true)?;
    let id = store
        .create_account(name, &pw)
        .map_err(|e| format!("could not create account: {e}"))?;
    println!("created account {name:?} (id {id})");
    Ok(())
}

fn set_password(store: &Store, name: &str) -> Result<(), String> {
    if store
        .get_account(name)
        .map_err(|e| e.to_string())?
        .is_none()
    {
        return Err(format!("no such account: {name}"));
    }
    let pw = read_password(&format!("New password for {name}: "), true)?;
    store
        .set_password(name, &pw)
        .map_err(|e| format!("could not set password: {e}"))?;
    println!("password updated for {name:?}; existing sessions revoked");
    Ok(())
}

fn toggle(store: &Store, name: &str, enabled: bool) -> Result<(), String> {
    store
        .set_enabled(name, enabled)
        .map_err(|e| format!("could not update account: {e}"))?;
    println!(
        "account {name:?} {}",
        if enabled { "enabled" } else { "disabled" }
    );
    Ok(())
}

fn list(store: &Store) -> Result<(), String> {
    let accounts = store.list_accounts().map_err(|e| e.to_string())?;
    if accounts.is_empty() {
        println!("no accounts yet");
        return Ok(());
    }
    println!("{:<5} {:<24} {:<9} last login", "id", "name", "state");
    for a in accounts {
        let last = a
            .last_login
            .map(|t| format!("{t}"))
            .unwrap_or_else(|| "never".into());
        println!(
            "{:<5} {:<24} {:<9} {}",
            a.id,
            a.name,
            if a.enabled { "enabled" } else { "disabled" },
            last
        );
    }
    Ok(())
}
