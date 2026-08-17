//! Print a packet body as hex, in the form `tools/test-one.ps1 -ReplySeq` wants.
//!
//! The harness is driven with hex typed on a command line, and the bodies are now long
//! enough that typing them is a real risk: the world-list pin was once two characters too
//! long, and the only reason it was caught is that a test happened to compare against the
//! builder. This closes that gap by making the builder itself the source of the string.
//!
//! ```text
//! cargo run --release -p net --bin packet-hex -- account-info maplecw "wisp****@example.com"
//! ```

use net::opcode::{
    account_info, enter_creation_permitted, login_result, world_list_end, world_list_entry,
    Character, ACCOUNT_INFO, ENTER_CREATION_RESULT, LOGIN_RESULT, WORLD_LIST,
};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: packet-hex <account-info LOGIN ACCOUNT | world-entry [NAME] | world-end \
                 | login-result [CHARACTER_NAME...] | enter-creation>";

    let (opcode, body) = match args.first().map(String::as_str) {
        Some("account-info") => {
            let login = args.get(1).map(String::as_str).unwrap_or("maplecw");
            let account = args.get(2).map(String::as_str).unwrap_or("");
            (ACCOUNT_INFO, account_info(login, account))
        }
        Some("world-entry") => {
            let name = args.get(1).map(String::as_str).unwrap_or("Scania");
            (WORLD_LIST, world_list_entry(0, name, 1))
        }
        Some("world-end") => (WORLD_LIST, world_list_end()),
        Some("enter-creation") => (ENTER_CREATION_RESULT, enter_creation_permitted()),
        // No names means the empty list the client has been getting all along, which is
        // the control this run is measured against.
        Some("login-result") => {
            let chars: Vec<Character> = args[1..]
                .iter()
                .enumerate()
                .map(|(i, name)| Character {
                    id: 100 + i as u32,
                    name: name.clone(),
                    ..Character::default()
                })
                .collect();
            (LOGIN_RESULT, login_result(0, 0, &chars))
        }
        _ => {
            eprintln!("{usage}");
            std::process::exit(2);
        }
    };

    // The `-ReplySeq` element, ready to paste, plus the length as a sanity check the eye
    // can use - a body that is suddenly the wrong size is the failure mode that matters.
    println!("{:04x}:{}", opcode, hex(&body));
    eprintln!("({} bytes)", body.len());
}
