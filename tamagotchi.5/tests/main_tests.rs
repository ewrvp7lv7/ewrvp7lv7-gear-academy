use gtest::{Log, Program, System};
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};

const OWNER: u64 = 100;
const PROGRAM_ID: u64 = 1;

pub fn init_test(owner: u64) -> System {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send_bytes(owner, String::from("Satoshi"));
    let expected_log = Log::builder()
        .dest(owner)
        .payload(String::from("Success!"));

    assert!(!res.log().is_empty());
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    sys
}

#[test]
fn tamagotchi_name() {
    let sys = init_test(OWNER);
    let program = sys.get_program(PROGRAM_ID);

    // test for TmgAction::Name
    let res = program.send(OWNER, TmgAction::Name);

    let expected_log = Log::builder()
        .dest(OWNER)
        .payload(TmgEvent::Name(String::from("Satoshi")));

    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}

#[test]
fn tamagotchi_mood() {
    let sys = init_test(OWNER);
    let program = sys.get_program(PROGRAM_ID);

    // read the state
    let state: Tamagotchi = program.read_state().expect("Error while reading the state");

    // check the fed value before feeding
    assert_eq!(state.fed, 500, "Invalid fed value");

    // feed the tamagotchi
    let res = program.send(OWNER, TmgAction::Feed);
    let expected_log = Log::builder()
        .dest(OWNER)
        .payload(TmgEvent::Fed);
    assert!(!res.main_failed(), "System is failed");
    assert!(res.contains(&expected_log));

    // play with tamagotchi
    let res = program.send(OWNER, TmgAction::Play);
    let expected_log = Log::builder()
        .dest(OWNER)
        .payload(TmgEvent::Entertained);
    assert!(!res.main_failed(), "System is failed");
    assert!(res.contains(&expected_log));

    // rest with tamagotchi
    let res = program.send(OWNER, TmgAction::Sleep);
    let expected_log = Log::builder()
        .dest(OWNER)
        .payload(TmgEvent::Slept);
    assert!(!res.main_failed(), "System is failed");
    assert!(res.contains(&expected_log));


    // read the state
    // assert state is changed
    let state: Tamagotchi = program.read_state().expect("Error while reading the state");
    assert_eq!(state.fed, 500 + 1000, "Invalid fed value");
    assert_eq!(state.entertained, 500 + 1000, "Invalid happy value");
    assert_eq!(state.rested, 500 + 1000, "Invalid happy value");


    let num_b = 150;
    sys.spend_blocks(num_b);


    program.send(OWNER, TmgAction::Feed);
    program.send(OWNER, TmgAction::Play);
    program.send(OWNER, TmgAction::Sleep);

    let state: Tamagotchi = program.read_state().expect("Error while reading the state");
    assert_eq!(state.fed, 500 + 1000*2 - num_b, "Invalid fed value");
    assert_eq!(state.entertained, 500 + 1000*2 - num_b * 2, "Invalid happy value");
    assert_eq!(state.rested, 500 + 1000*2 - num_b * 2, "Invalid happy value");
}

