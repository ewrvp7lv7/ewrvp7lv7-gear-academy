#![no_std]

use gstd::{exec, msg, prelude::*};
// use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};
use tamagotchi_io::{Tamagotchi};

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





