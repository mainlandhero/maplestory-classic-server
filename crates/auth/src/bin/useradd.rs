//! Local account administration.
//!
//!   maplecw-useradd <name> [--email ADDR]   create an account
//!   maplecw-useradd --list                  list accounts
//!   maplecw-useradd --passwd <name|email>   change a password
//!   maplecw-useradd --email <name> <addr>   set or clear an email ("" clears)
//!   maplecw-useradd --disable <name>
//!   maplecw-useradd --enable  <name>
//!   maplecw-useradd --gm <name>             allow the ! GM commands
//!   maplecw-useradd --no-gm <name>          take that away
//!   maplecw-useradd --claims                who is signed in, one row per launch
//!   maplecw-useradd --clear-claims          forget which accounts are "playing"
//!
//! GM status is authorisation, **not authentication**. The game socket carries no
//! credentials, so it says which *account* may use `!item`; which account a connection is
//! served as comes from a launcher claim. It keeps a second account on the machine out of the
//! commands. It keeps nothing out of the port.
//!
//! The email is a second **identity**, not a second credential: `maplecw-launcher`'s
//! sign-in field accepts either it or the account name, and `Store::validate_name` allows
//! only `[A-Za-z0-9_]` so the two namespaces cannot collide. An account without one signs
//! in by name exactly as before.
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

    // Optional --db, anywhere in the arguments. `--email ADDR` is the same shape, but only
    // when it is not the leading verb - `--email <name> <addr>` is its own command below.
    let mut db_path = String::from(DEFAULT_DB);
    let mut new_email: Option<String> = None;
    let mut rest: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--db" && i + 1 < args.len() {
            db_path = args[i + 1].clone();
            i += 2;
        // `--email` is two different things and they are told apart by POSITION:
        //   maplecw-useradd <name> --email <addr>     an option on `create`
        //   maplecw-useradd --email <name> <addr>     its own command
        //
        // The test is "has a verb been seen yet", i.e. is `rest` non-empty - NOT the raw
        // argument index. The first version used `i > 0`, which looks equivalent and is not:
        // `--db <path>` is stripped out here too, so in
        //   --db C:\maplecw.db --email maplecw wisp@example.com
        // the command form sits at index 2 and was read as the option form. It silently took
        // "maplecw" as an email address and then tried to CREATE an account called
        // "wisp@example.com", which failed on a password prompt with no terminal - an error
        // three steps removed from the mistake.
        } else if args[i] == "--email" && !rest.is_empty() && i + 1 < args.len() {
            new_email = Some(args[i + 1].clone());
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
            Some(identity) => set_password(&store, identity),
            None => Err("--passwd needs an account name or email".into()),
        },
        Some("--disable") => match rest.get(1) {
            Some(name) => toggle(&store, name, false),
            None => Err("--disable needs an account name".into()),
        },
        Some("--enable") => match rest.get(1) {
            Some(name) => toggle(&store, name, true),
            None => Err("--enable needs an account name".into()),
        },
        Some("--gm") => match rest.get(1) {
            Some(name) => set_gm(&store, name, true),
            None => Err("--gm needs an account name".into()),
        },
        Some("--no-gm") => match rest.get(1) {
            Some(name) => set_gm(&store, name, false),
            None => Err("--no-gm needs an account name".into()),
        },
        Some("--claims") => list_claims(&store),
        Some("--clear-claims") => clear_claims(&store),
        Some("--email") => match (rest.get(1), rest.get(2)) {
            (Some(name), Some(addr)) => set_email(&store, name, addr),
            _ => Err("--email needs an account name and an address (\"\" clears it)".into()),
        },
        Some(flag) if flag.starts_with('-') => Err(format!("unknown option: {flag}")),
        Some(name) => create(&store, name, new_email.as_deref()),
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
         maplecw-useradd <name> [--email ADDR]   create an account\n  \
         maplecw-useradd --list                  list accounts\n  \
         maplecw-useradd --passwd <name|email>   set a NEW password (the old one is not asked for)\n  \
         maplecw-useradd --email <name> <addr>   set an email (\"\" clears it)\n  \
         maplecw-useradd --gm <name>             allow the ! GM commands\n  \
         maplecw-useradd --no-gm <name>          take that away\n  \
         maplecw-useradd --disable <name>\n  \
         maplecw-useradd --enable  <name>\n  \
         maplecw-useradd --claims                who is signed in, one row per launch\n  \
         maplecw-useradd --clear-claims          forget which accounts are playing\n\n\
         options:\n  \
         --db <path>                             database file (default {DEFAULT_DB})\n\n\
         Passwords are read from a prompt or stdin, never from an argument.\n  \
         The email is a second identity for the launcher's sign-in field, not a second\n  \
         credential; an account without one signs in by name.\n  \
         GM status is authorisation, not authentication: the game socket carries no\n  \
         credentials, so it gates the ACCOUNT rather than whoever is connected."
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

fn create(store: &Store, name: &str, email: Option<&str>) -> Result<(), String> {
    let pw = read_password(&format!("Password for {name}: "), true)?;
    let id = store
        .create_account(name, &pw)
        .map_err(|e| format!("could not create account: {e}"))?;
    println!("created account {name:?} (id {id})");
    // After the account exists, so a rejected address leaves a usable account rather than
    // no account at all - the password has already been typed twice by this point.
    if let Some(addr) = email.filter(|a| !a.is_empty()) {
        store
            .set_email(name, Some(addr))
            .map_err(|e| format!("account created, but the email was NOT set: {e}"))?;
        println!("  email {addr:?} - the launcher accepts this or the name");
    }
    Ok(())
}

/// Drop every login claim, so the login server falls back to its `--account`.
///
/// A claim says "serve the next game connection as this account" and lives for hours, which
/// is right when the launcher is driving. It is a trap when it is not: a run started with an
/// explicit `--account` would quietly serve yesterday's launcher account instead, and on
/// screen that is someone else's characters with no explanation. `tools/test-server.ps1`
/// calls this on the path that does not use the launcher, so explicit configuration wins.
/// Grant or revoke GM status.
///
/// Separate from account creation on purpose: granting is a decision, and a decision belongs
/// in one obvious place rather than falling out of a default. A fresh account has no GM
/// status, which is the safe direction.
fn set_gm(store: &Store, name: &str, is_gm: bool) -> Result<(), String> {
    store
        .set_gm(name, is_gm)
        .map_err(|e| format!("could not change GM status: {e}"))?;
    if is_gm {
        println!("account {name:?} may now use the ! GM commands");
        println!("  this is authorisation, not authentication: the game socket carries no");
        println!("  credentials, so it gates the ACCOUNT, not whoever is connected");
    } else {
        println!("account {name:?} may no longer use the ! GM commands");
    }
    Ok(())
}

/// **Who is signed in, one row per launch.**
///
/// Worth its own command for the same reason `--list` is: it answers a question that
/// otherwise costs one of the owner's manual client launches. A claim used to be one global row,
/// so "which account is playing" had exactly one answer and the login server's startup banner
/// was enough. It is now one row per launch, and the interesting states - two people signed
/// in, or one person signed in twice - are invisible anywhere else.
///
/// The **count** is the number that matters. With more than one live claim, a connection the
/// server cannot attribute to a launch is served as the fallback `--account` rather than
/// guessed at, so "2 live claims" plus "the wrong characters on screen" is a complete
/// diagnosis rather than the start of an investigation.
fn list_claims(store: &Store) -> Result<(), String> {
    let claims = store
        .live_login_claims()
        .map_err(|e| format!("could not read login claims: {e}"))?;
    if claims.is_empty() {
        println!("no live login claim - every connection is served as the login server's --account");
        return Ok(());
    }
    println!("{:<24} {:<12} {}", "account", "staked", "expires");
    for c in &claims {
        println!("{:<24} {:<12} {}", c.account_name, c.created_at, c.expires_at);
    }
    println!();
    match claims.len() {
        1 => println!(
            "1 live claim. Any connection is served as it, including one the server cannot \
             attribute to a launch."
        ),
        n => println!(
            "{n} live claims. A connection the server CAN attribute to a launch - same machine, \
             registered by the launcher - is served as that launch's account. One it cannot is \
             served as the --account fallback, deliberately: serving the newest is how one \
             player gets served as another."
        ),
    }
    Ok(())
}

fn clear_claims(store: &Store) -> Result<(), String> {
    let n = store
        .clear_login_claims()
        .map_err(|e| format!("could not clear login claims: {e}"))?;
    match n {
        0 => println!("no login claim was live; the login server was already using --account"),
        // The plural is load-bearing now: this takes EVERY launch, including other people's.
        // `Store::clear_login_claims_for` is the scoped one.
        _ => println!(
            "cleared {n} login claim(s) - every launch, not just one. The login server now uses \
             its --account until somebody signs in again."
        ),
    }
    Ok(())
}

fn set_email(store: &Store, name: &str, addr: &str) -> Result<(), String> {
    let value = if addr.is_empty() { None } else { Some(addr) };
    store
        .set_email(name, value)
        .map_err(|e| format!("could not set email: {e}"))?;
    match value {
        Some(a) => println!("account {name:?} now signs in as {name:?} or {a:?}"),
        None => println!("account {name:?} has no email; it signs in by name only"),
    }
    Ok(())
}

/// Reset a password. `identity` may be the account **name or its email**.
///
/// The account is resolved BEFORE the password is typed, so a mistyped identity costs one
/// line rather than two careful password entries - and so the prompt can name the account
/// that is actually about to change, which matters when the identity given was an email and
/// the account is called something else.
fn set_password(store: &Store, identity: &str) -> Result<(), String> {
    let account = store
        .get_account_by_identity(identity)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("no account with the name or email {identity:?}"))?;

    let label = match &account.email {
        Some(email) if !identity.eq_ignore_ascii_case(&account.name) => {
            format!("{} ({email})", account.name)
        }
        _ => account.name.clone(),
    };
    let pw = read_password(&format!("New password for {label}: "), true)?;
    store
        .set_password(&account.name, &pw)
        .map_err(|e| format!("could not set password: {e}"))?;
    println!("password updated for {:?}", account.name);
    println!("  existing sessions revoked, and any login claim on this account cleared");
    println!("  sign in again in the launcher to decide who plays");
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
    println!("{:<5} {:<24} {:<28} {:<9} {:<4} last login", "id", "name", "email", "state", "gm");
    for a in accounts {
        let last = a
            .last_login
            .map(|t| format!("{t}"))
            .unwrap_or_else(|| "never".into());
        println!(
            "{:<5} {:<24} {:<28} {:<9} {:<4} {}",
            a.id,
            a.name,
            a.email.as_deref().unwrap_or("-"),
            if a.enabled { "enabled" } else { "disabled" },
            if a.is_gm { "GM" } else { "-" },
            last
        );
    }
    Ok(())
}
