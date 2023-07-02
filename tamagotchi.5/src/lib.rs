//! #5. Homework "Tamagotchi NFT"
#![no_std]
use gstd::{msg, debug, prelude::*, exec::{block_timestamp, block_height}};
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};

mod nftt;
use nftt::NFTamagotchi;

static mut STATE: Option<Tamagotchi> = None;
static mut MOOD: Option<Mood> = None;

//Mood: Fed (from 1 to 10000), Happy (from 1 to 10000) and Rested (from 1 to 10000).
pub struct Mood {
    pub fed: u32,
    pub happy: u32,
    pub rested: u32,
}

const INT_NUM: u32 = 500;
const MAX_NUM: u32 = 10000;

impl Mood {
    const HUNGER_PER_BLOCK: u32 = 1;
    const ENERGY_PER_BLOCK: u32 = 2;
    const BOREDOM_PER_BLOCK: u32 = 2;

    const FILL_PER_FEED: u32 = 1000;
    const FILL_PER_ENTERTAINMENT: u32 = 1000;
    const FILL_PER_SLEEP: u32 = 1000;


    fn feed(&mut self) {
        let tamagotchi = unsafe {
            STATE.as_mut().expect("The contract is not initialized")
        };

        assert_eq!(msg::source(), tamagotchi.owner, "Only owner can feed the tamagotchi");
        assert!(self.fed < 5000, "Tamagotchi not enough hungry");


        // calculating and normalizing hunger level
        let lost = (block_height() - tamagotchi.fed_block) * Self::HUNGER_PER_BLOCK;

        self.fed += Self::FILL_PER_FEED;

        self.fed = match lost {
            hl if self.fed < hl => 0,
            hl if MAX_NUM < hl => MAX_NUM,
            _ => self.fed - lost,
        };

        debug!("feed: {}", self.fed);

        tamagotchi.fed = self.fed;
        tamagotchi.fed_block = block_height();

        msg::reply(
            TmgEvent::Fed,
            0,
        ).expect("Failed to share TmgEvent");
    }

    fn play(&mut self) {
        let tamagotchi = unsafe {
            STATE.as_mut().expect("The contract is not initialized")
        };

        assert_eq!(msg::source(), tamagotchi.owner, "Only owner can feed the tamagotchi");
        assert!(self.happy < 5000, "Tamagotchi not enough hungry");

        let lost = (block_height() - tamagotchi.entertained_block) * Self::ENERGY_PER_BLOCK;

        self.happy += Self::FILL_PER_ENTERTAINMENT;

        self.happy = match lost {
            hl if self.happy < hl => 0,
            hl if MAX_NUM < hl => MAX_NUM,
            _ => self.happy - lost,
        };

        debug!("play: {}", self.happy);

        tamagotchi.entertained = self.happy;
        tamagotchi.entertained_block = block_height();

        msg::reply(
            TmgEvent::Entertained,
            0,
        ).expect("Failed to share TmgEvent");
    }

    fn sleep(&mut self) {
        let tamagotchi = unsafe {
            STATE.as_mut().expect("The contract is not initialized")
        };

        assert_eq!(msg::source(), tamagotchi.owner, "Only owner can feed the tamagotchi");
        assert!(self.rested < 5000, "Tamagotchi not enough hungry");

        // calculating and normalizing hunger level
        let lost = (block_height() - tamagotchi.rested_block) * Self::BOREDOM_PER_BLOCK;


        self.rested += Self::FILL_PER_SLEEP;
        self.rested = match lost {
            hl if self.rested < hl => 0,
            hl if MAX_NUM < hl => MAX_NUM,
            _ => self.rested - lost,
        };

        debug!("sleep: {}", self.rested);

        tamagotchi.rested = self.rested;
        tamagotchi.rested_block = block_height();

        msg::reply(
            TmgEvent::Slept,
            0,
        ).expect("Failed to share TmgEvent");
    }
}

#[no_mangle]
extern "C" fn init() {
    let owner = msg::source();

    let name: String = String::from_utf8(
        msg::load_bytes().expect("Can't load tamagotchi name")
    ).expect("Can't decode tamagotchi name");

    // let name: String = msg::load().expect("Can't load tamagotchi name");

    let date_of_birth = block_timestamp();

    let bh = block_height();


    unsafe {
        MOOD = Some(Mood {
            fed: INT_NUM,
            happy: INT_NUM,
            rested: INT_NUM,
        });
    };

    unsafe {
        STATE = Some(Tamagotchi {
            owner,
            name,
            date_of_birth,
            fed: INT_NUM,
            rested: INT_NUM,
            entertained: INT_NUM,
            fed_block: bh.clone(),
            entertained_block: bh.clone(),
            rested_block: bh.clone(),
            allowed_account: None,
        });
    };

    msg::reply(String::from("Success!"), 0)
        .expect("Failed to share initialization result");
}

#[no_mangle]
extern "C" fn handle() {
    let mood = unsafe {
        MOOD.as_mut().expect("The contract is not initialized")
    };

    let tamagotchi = unsafe {
        STATE.as_mut().expect("The contract is not initialized")
    };

    let action: TmgAction = msg::load().expect("Error in loading TmgAction");

    // matching pattern
    match action {
        TmgAction::Age => {
            msg::reply(
                TmgEvent::Age(block_timestamp() - tamagotchi.date_of_birth),
                0,
            ).expect("Failed to share the TmgEvent");
        }
        TmgAction::Name => {
            msg::reply(
                TmgEvent::Name(tamagotchi.name.clone()),
                0,
            ).expect("Failed to share the TmgEvent");
        }
        TmgAction::Feed => mood.feed(),
        TmgAction::Play => mood.play(),
        TmgAction::Sleep => mood.sleep(),
        TmgAction::RevokeApproval => tamagotchi.revoke_approval(),
        TmgAction::Approve(actor_id) => tamagotchi.approve(actor_id),
        TmgAction::Transfer(actor_id) => tamagotchi.transfer(actor_id)
    };
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






