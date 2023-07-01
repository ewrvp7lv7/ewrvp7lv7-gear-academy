use gtest::{Log, Program, System};
use tamagotchi_io::{TmgAction, TmgEvent};

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let t_name = "7Tamagotchi42";

    // The first message to the Program structure is always the initialization message even if the program does not have the init function.
    let sender_id = 2;
    let res = program.send(sender_id, String::from(t_name));
    assert!(!res.main_failed());
    assert!(!res.log().is_empty());

    //test init()
    let expected_log = Log::builder()
        .dest(sender_id)
        .payload(String::from("Success!"));

    assert!(res.contains(&expected_log));

    // test for TmgAction::Name
    let sender_id = 3;
    let res = program.send(sender_id, TmgAction::Name);

    let expected_log = Log::builder()
        .dest(sender_id)
        .payload(TmgEvent::Name(String::from(t_name)));

    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());

    // test for TmgAction::Age
    let sender_id = 3;
    let res = program.send(sender_id, TmgAction::Age);

    assert!(!res.main_failed());
}








