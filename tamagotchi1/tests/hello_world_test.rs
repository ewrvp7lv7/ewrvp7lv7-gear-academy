use gtest::{Log, Program, System};

//3. Homework "Tamagotchi"
//0x869387a9adb6201662bc39b740b289077b8228fbeb1c4bd4e64e746cc812c1b7

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    // The first message to the Program structure is always the initialization message even if the program does not have the init function.
    let res = program.send(2, String::from("tamagotchi142"));
    assert!(!res.main_failed());
    assert!(!res.log().is_empty());

    let expected_log = Log::builder()
        .dest(2)
        .payload(String::from("Success!"));

    assert!(res.contains(&expected_log))
}







