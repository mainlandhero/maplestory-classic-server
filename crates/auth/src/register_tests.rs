//! Registration and recovery through the service, end to end against an in-memory store.
//!
//! The important assertions are the negative ones: which failures spend the code and which do
//! not. `crate::register`'s module doc promises that everything checkable is checked first; a
//! test that only exercised the happy path would say nothing about that promise.

use std::sync::Arc;

use store::Store;

use crate::ratelimit::Limiter;
use crate::{
    AuthService, LoginRequest, LoginResponse, RecoverRequest, RecoverResponse, RegisterRequest,
    RegisterResponse,
};

fn service() -> (AuthService, Arc<Store>) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    (AuthService::new(store.clone()), store)
}

fn reg(username: &str, email: &str, password: &str, code: &str) -> RegisterRequest {
    RegisterRequest {
        username: username.into(),
        email: email.into(),
        password: password.into(),
        code: code.into(),
    }
}

fn rec(identity: &str, code: &str, new_password: &str) -> RecoverRequest {
    RecoverRequest { identity: identity.into(), code: code.into(), new_password: new_password.into() }
}

fn signs_in(svc: &AuthService, identity: &str, password: &str) -> bool {
    matches!(
        svc.login(&LoginRequest { username: identity.into(), password: password.into() }, None),
        LoginResponse::Ok { .. }
    )
}

#[test]
fn a_valid_code_registers_an_account_with_its_email_and_is_spent() {
    let (svc, store) = service();
    let code = store.create_invite_code(store::INVITE_TTL_SECS).unwrap().code;

    let r = svc.register(&reg("newbie", "newbie@example.test", "Passw0rd", &code), Some("10.0.0.5"));
    let RegisterResponse::Ok { username, .. } = r else { panic!("{r:?}") };
    assert_eq!(username, "newbie");
    assert!(signs_in(&svc, "newbie", "Passw0rd"), "by name");
    assert!(signs_in(&svc, "newbie@example.test", "Passw0rd"), "and by the email it registered");

    // Spent: the same code registers nobody else.
    let again = svc.register(&reg("another", "another@example.test", "Passw0rd", &code), Some("10.0.0.5"));
    assert_eq!(again, RegisterResponse::InvalidCode);
}

#[test]
fn bad_input_is_refused_before_the_code_is_spent() {
    let (svc, store) = service();
    let code = store.create_invite_code(store::INVITE_TTL_SECS).unwrap().code;

    let weak = svc.register(&reg("newbie", "n@example.test", "password", &code), None);
    assert!(matches!(weak, RegisterResponse::WeakPassword { .. }), "{weak:?} - no digit");
    let short = svc.register(&reg("newbie", "n@example.test", "Pass1", &code), None);
    assert!(matches!(short, RegisterResponse::WeakPassword { .. }), "{short:?}");
    let email = svc.register(&reg("newbie", "not-an-email", "Passw0rd", &code), None);
    assert!(matches!(email, RegisterResponse::InvalidEmail { .. }), "{email:?}");
    let name = svc.register(&reg("no spaces!", "n@example.test", "Passw0rd", &code), None);
    assert!(matches!(name, RegisterResponse::InvalidUsername { .. }), "{name:?}");

    // Four refusals, and the code is still live: the fifth attempt, correct, succeeds.
    let ok = svc.register(&reg("newbie", "n@example.test", "Passw0rd", &code), None);
    assert!(matches!(ok, RegisterResponse::Ok { .. }), "{ok:?}");
}

#[test]
fn a_taken_username_or_email_is_refused_without_spending_the_code() {
    let (svc, store) = service();
    store.create_account("taken", "Whatever1").unwrap();
    store.set_email("taken", Some("taken@example.test")).unwrap();
    let code = store.create_invite_code(store::INVITE_TTL_SECS).unwrap().code;

    assert_eq!(
        svc.register(&reg("TAKEN", "fresh@example.test", "Passw0rd", &code), None),
        RegisterResponse::UsernameTaken,
        "names are case-insensitive in the store, so the check must be too"
    );
    let by_email = svc.register(&reg("fresh", "taken@example.test", "Passw0rd", &code), None);
    assert!(matches!(by_email, RegisterResponse::InvalidEmail { .. }), "{by_email:?}");
    assert!(store.redeem_invite_code(&code).unwrap(), "neither refusal touched the code");
}

#[test]
fn an_unknown_code_is_refused_and_a_recovery_code_is_not_an_invite() {
    let (svc, store) = service();
    store.create_account("someone", "Whatever1").unwrap();
    let recovery = store.create_recovery_code("someone", store::RECOVERY_TTL_SECS).unwrap().code;
    assert_eq!(
        svc.register(&reg("newbie", "n@example.test", "Passw0rd", "2222-3333"), None),
        RegisterResponse::InvalidCode
    );
    assert_eq!(
        svc.register(&reg("newbie", "n@example.test", "Passw0rd", &recovery), None),
        RegisterResponse::InvalidCode,
        "a recovery code lives in another table and cannot register anybody"
    );
}

#[test]
fn recovery_sets_a_new_password_by_email_and_the_old_one_stops_working() {
    let (svc, store) = service();
    store.create_account("player", "OldPass1").unwrap();
    store.set_email("player", Some("player@example.test")).unwrap();
    // Minted by the email, which is what `!recoverycode <email>` does.
    let code = store.create_recovery_code("player@example.test", store::RECOVERY_TTL_SECS).unwrap().code;

    let r = svc.recover(&rec("player@example.test", &code, "NewPass2"), Some("10.0.0.9"));
    assert_eq!(r, RecoverResponse::Ok { username: "player".into() });
    assert!(signs_in(&svc, "player", "NewPass2"));
    assert!(!signs_in(&svc, "player", "OldPass1"), "the old password is gone");
    assert_eq!(svc.recover(&rec("player", &code, "NewPass3"), None), RecoverResponse::InvalidCode, "single use");
}

#[test]
fn recovery_with_the_wrong_identity_does_not_burn_the_code() {
    let (svc, store) = service();
    store.create_account("otter", "OtterPw1").unwrap();
    store.create_account("owl", "OwlPw111").unwrap();
    let code = store.create_recovery_code("otter", store::RECOVERY_TTL_SECS).unwrap().code;

    assert_eq!(svc.recover(&rec("owl", &code, "Hijack01"), None), RecoverResponse::InvalidCode);
    assert!(signs_in(&svc, "owl", "OwlPw111"), "owl's password is untouched");
    assert_eq!(svc.recover(&rec("nobody@example.test", &code, "Hijack01"), None), RecoverResponse::InvalidCode);
    // Otter's code survived both.
    assert_eq!(svc.recover(&rec("otter", &code, "OtterNew2"), None), RecoverResponse::Ok { username: "otter".into() });
    assert!(signs_in(&svc, "otter", "OtterNew2"));
}

#[test]
fn a_weak_new_password_is_refused_before_the_recovery_code_is_spent() {
    let (svc, store) = service();
    store.create_account("otter", "OtterPw1").unwrap();
    let code = store.create_recovery_code("otter", store::RECOVERY_TTL_SECS).unwrap().code;
    let weak = svc.recover(&rec("otter", &code, "lettersonly"), None);
    assert!(matches!(weak, RecoverResponse::WeakPassword { .. }), "{weak:?}");
    assert_eq!(svc.recover(&rec("otter", &code, "Fine1234"), None), RecoverResponse::Ok { username: "otter".into() });
}

#[test]
fn too_many_bad_codes_from_one_peer_are_refused_and_another_peer_is_not() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let svc = AuthService::with_limiter(store.clone(), Limiter::new(3, 100, std::time::Duration::from_secs(60)));
    for _ in 0..3 {
        assert_eq!(
            svc.register(&reg("newbie", "n@example.test", "Passw0rd", "9999-9999"), Some("203.0.113.9")),
            RegisterResponse::InvalidCode
        );
    }
    assert_eq!(
        svc.register(&reg("newbie", "n@example.test", "Passw0rd", "9999-9999"), Some("203.0.113.9")),
        RegisterResponse::TooManyAttempts
    );
    // The budget is per peer; and a REAL code from another peer still works, because only
    // failures were ever counted.
    let code = store.create_invite_code(store::INVITE_TTL_SECS).unwrap().code;
    let ok = svc.register(&reg("newbie", "n@example.test", "Passw0rd", &code), Some("198.51.100.4"));
    assert!(matches!(ok, RegisterResponse::Ok { .. }), "{ok:?}");
    // The limited peer is refused even with the right code, until the window passes.
    let code2 = store.create_invite_code(store::INVITE_TTL_SECS).unwrap().code;
    assert_eq!(
        svc.register(&reg("other", "o@example.test", "Passw0rd", &code2), Some("203.0.113.9")),
        RegisterResponse::TooManyAttempts
    );
}

#[test]
fn the_http_layer_routes_both_endpoints_with_the_right_status_codes() {
    use crate::http::{handle, Request};
    let (svc, store) = service();
    let code = store.create_invite_code(store::INVITE_TTL_SECS).unwrap().code;
    let post = |path: &str, body: String| {
        handle(
            &svc,
            &Request {
                method: "POST".into(),
                path: path.into(),
                body,
                peer: Some("127.0.0.1".into()),
                query: String::new(),
            },
        )
    };
    let r = post("/register", format!(r#"{{"username":"newbie","email":"n@example.test","password":"Passw0rd","code":"{code}"}}"#));
    assert_eq!(r.status, 200, "{}", r.body);
    assert!(r.body.contains(r#""status":"ok""#), "{}", r.body);
    let r = post("/register", format!(r#"{{"username":"newbie","email":"n@example.test","password":"Passw0rd","code":"{code}"}}"#));
    assert_eq!(r.status, 409, "taken now: {}", r.body);
    let r = post("/recover", r#"{"identity":"newbie","code":"0000-0000","new_password":"NewPass9"}"#.into());
    assert_eq!(r.status, 401, "{}", r.body);
    assert!(r.body.contains("invalid_code"), "{}", r.body);
    let r = post("/recover", r#"{"identity":"newbie","code":"0000-0000","new_password":"short"}"#.into());
    assert_eq!(r.status, 400, "{}", r.body);
    assert!(r.body.contains("weak_password"), "{}", r.body);
    assert_eq!(post("/register", "not json".into()).status, 400);
}
