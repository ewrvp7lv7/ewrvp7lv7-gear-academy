#![no_std]
//https://academy.gear.rs/inner/hello-world-advanced/more-advanced/
use gstd::{msg, prelude::*, debug}; //ERROR
use tamagotchi_io::InputMessages;

static mut GREETING: Option<String> = None;

// #[derive(Encode, Decode, TypeInfo)]
// pub enum InputMessages {
//     SendHelloTo(ActorId),
//     SendHelloReply,
// }

// #[no_mangle]
// extern "C" fn handle() {
//     msg::reply(String::from("Hello"), 0)
//         .expect("Error in sending a reply message");
// }
//
// #[no_mangle]
// extern "C" fn init() {
//     // let init_message: String = msg::load()
//     //     .expect("Can't load init message");
//     // debug!("Program was initialized with message {:?}",
//     //     init_message);
// }

#[no_mangle]
extern "C" fn init() {
    let greeting: String = msg::load()
        .expect("Can't decode an init message");
    debug!("Program was initialized with message {:?}",
        greeting);
    unsafe { GREETING = Some(greeting) };
}

//https://academy.gear.rs/inner/hello-world-advanced/more-advanced/
#[no_mangle]
extern "C" fn handle() {
    //https://docs.gear.rs/gstd/msg/fn.load.html
    let input_message: InputMessages = msg::load()
        .expect("Error in loading InputMessages");

    let greeting = unsafe {
        GREETING
            //.as_mut()
             .as_ref()
            .expect("The contract is not initialized")
    };

    match input_message {
        InputMessages::SendHelloTo(account) => {
            debug!("Message: SendHelloTo {:?}", account);
            msg::send(account, greeting, 0)
                .expect("Error in sending Hello message to account");
        }
        InputMessages::SendHelloReply => {
            debug!("Message: SendHelloReply");
            msg::reply(greeting, 0)
                .expect("Error in sending reply");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let greeting = unsafe {
        GREETING
            .as_ref()
            .expect("The contract is not initialized")
    };
    msg::reply(greeting, 0).expect("Failed to share state");
}

#[no_mangle]
// It returns the hash of metadata.
// .metahash is generating automatically
// while you are using build.rs
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0)
        .expect("Failed to share metahash");
}





