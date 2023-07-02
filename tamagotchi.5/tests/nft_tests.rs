use gtest::{Log};
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};

mod main_tests;
use main_tests::{init_test};

const OWNER: u64 = 100;
const PROGRAM_ID: u64 = 1;
const USER: u64 = 101;
const USER2: u64 = 102;

#[test]
fn tamagotchi_transfer() {
    let sys = init_test(OWNER);
    let program = sys.get_program(PROGRAM_ID);

    let from = 32;
    // must fail since `from` is not OWNER
    let res = program.send(from, TmgAction::Transfer(USER.into()));
    assert!(res.main_failed());

    // successful transfer
    let res = program.send(OWNER, TmgAction::Transfer(USER.into()));
    let expected_log = Log::builder()
        .dest(OWNER)
        .payload(TmgEvent::Transfer(USER.into()));
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));

    // check the state
    let state: Tamagotchi = program.read_state().expect("Failed to read the state");
    assert!(state.owner == USER.into());
}

#[test]
fn tamagotchi_approve() {
    let sys = init_test(OWNER);
    let program = sys.get_program(PROGRAM_ID);

    let from = 32;

    // must fail since `from` is not owner
    let res = program.send(from, TmgAction::Approve(USER.into()));
    assert!(res.main_failed());

    // successful approve
    let res = program.send(OWNER, TmgAction::Approve(USER.into()));
    let expected_log = Log::builder()
        .dest(OWNER)
        .payload(TmgEvent::Approve(USER.into()));
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));

    // check the state after approval
    let state: Tamagotchi = program.read_state().expect("Failed to read the state");
    assert!(state.allowed_account.unwrap() == USER.into());

    // successful transfer from user
    let res = program.send(USER, TmgAction::Transfer(USER.into()));
    let expected_log = Log::builder()
        .dest(USER)
        .payload(TmgEvent::Transfer(USER.into()));
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));

    // check the state after transfer
    let state: Tamagotchi = program.read_state().expect("Failed to read the state");
    assert!(state.owner == USER.into());
}

#[test]
fn tamagotchi_revoke_approval() {
    let sys = init_test(OWNER);
    let program = sys.get_program(PROGRAM_ID);

    // approve user to transfer ownership
    let res = program.send(OWNER, TmgAction::Approve(USER.into()));
    assert!(!res.main_failed());

    // must fail since user2 is not owner
    let res = program.send(USER2, TmgAction::RevokeApproval);
    assert!(res.main_failed());

    let state: Tamagotchi = program.read_state().expect("Failed to read the state");
    assert!(state.allowed_account.unwrap() == USER.into());

    let res = program.send(OWNER, TmgAction::RevokeApproval);
    let expected_log = Log::builder()
        .dest(OWNER)
        .payload(TmgEvent::RevokeApproval);
    assert!(!res.main_failed());
    assert!(!res.log().is_empty());
    assert!(res.contains(&expected_log));

    let state: Tamagotchi = program.read_state().expect("Failed to read the state");
    assert!(state.allowed_account.unwrap_or_default() != USER.into());
}
