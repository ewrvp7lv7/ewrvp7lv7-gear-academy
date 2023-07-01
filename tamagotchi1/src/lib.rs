//! #3. Homework "Tamagotchi"
#![no_std]

use gstd::{exec, msg, debug, prelude::*};
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};

static mut STATE: Option<Tamagotchi> = None;

#[no_mangle]
extern "C" fn init() {
    let name: String = msg::load().expect("Error in loading name");
    let date_of_birth = exec::block_timestamp();
    unsafe {
        STATE = Some(Tamagotchi {
            name,
            date_of_birth,
        })
    };

    msg::reply(String::from("Success!"), 0)
        .expect("Failed to share initialization result");
}

// Good job! Commented in the PR. The main concern i that you have not implemented the handle() function.
// I am sorry, but I don't see the handle() function that receives TmgAction::Name and TmgAction::Age. Also, we need to cover this function with tests.
#[no_mangle]
extern "C" fn handle() {
    let action: TmgAction = msg::load().expect("Error in loading TmgAction");
    let state: &Tamagotchi = unsafe { STATE.as_ref().expect("failed to get contract state") };

    let event: TmgEvent = match action {
        TmgAction::Age => {
            let age_min = (exec::block_timestamp() - state.date_of_birth) / 60_000 as u64;
            debug!("Message: Age in min: {}", age_min);
            TmgEvent::Age(age_min)
        },
        TmgAction::Name => TmgEvent::Name(state.name.clone())
    };

    msg::reply(event, 0).expect("Failed to share TmgEvent");
}

#[no_mangle]
extern "C" fn state() {
    let state: &Tamagotchi = unsafe { STATE.as_ref().expect("failed to get contract state") };
    msg::reply(state, 0).expect("Failed to share state");
}

// It returns the hash of metadata.
// .metahash is generating automatically
// while you are using build.rs
#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0)
        .expect("Failed to share metahash");
}





