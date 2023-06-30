// use gtest::{Log, Program, System};
//
// #[test]
// fn hello_test() {
//     let sys = System::new();
//     sys.init_logger();
//     let program = Program::current(&sys);
// //Note that we added sys.init_logger() to initialize
// //printing logs into stdout,
// //and we sent a message from the user with id 2 (id 2 transforms to ActorId equal to 0x020000…00 ).
//     program.send(2, String::from("INIT MESSAGE"));
//
//     let expected_log = Log::builder()
//         .dest(2)
//         .payload(String::from("Hello"));
//     assert!(res.contains(&expected_log));
// }


use gtest::{Log, Program, System};
// use hello_world::InputMessages;
use hello_world_io::InputMessages;

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    // The first message to the Program structure is always the initialization message even if the program does not have the init function.
    //let res = program.send_bytes(2, String::from("Hello"));
    let res = program.send(2, String::from("Hello"));
    assert!(!res.main_failed());
    assert!(res.log().is_empty());

    // test `SendHelloTo`
    let hello_recipient: u64 = 4;
    let res = program.send(
        2,
        InputMessages::SendHelloTo(hello_recipient.into()),
    );

    let expected_log = Log::builder()
        .dest(hello_recipient)
        .payload(String::from("Hello"));

    assert!(res.contains(&expected_log))
}


// What is the purpose of the init() function in the smart contract code?
// +To emulate the node behaviour for running programs
// -To handle messages received by a program
// -To create a new program
// -To handle messages received by a program









