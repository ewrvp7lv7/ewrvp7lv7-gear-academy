//! #5. Homework "Tamagotchi NFT"
#![no_std]

use gstd::{msg, debug, prelude::*, ActorId, exec::{block_timestamp, block_height}};
use tamagotchi6_io::{Tamagotchi, TmgAction, TmgEvent};

mod nftt;

use nftt::{NFTamagotchi, TamagotchiShop};

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

    fn set_ft_contract(&mut self, actor_id: &ActorId) {
        let tamagotchi = unsafe {
            STATE.as_mut().expect("The contract is not initialized")
        };
        assert_eq!(msg::source(), tamagotchi.owner, "Only owner can feed the tamagotchi");

        tamagotchi.ft_contract_id = *actor_id;

        msg::reply(
            TmgEvent::SetFTokenContract,
            0,
        ).expect("Failed to share the TmgEvent");
    }
}

#[no_mangle]
extern "C" fn init() {
    let owner = msg::source();

    let name: String = String::from_utf8(
        msg::load_bytes().expect("Can't load tamagotchi name")
    ).expect("Can't decode tamagotchi name");

    // let name: String = msg::load().expect("Can't load tamagotchi name");

// [*] Upload code "FT Storage"
//     [*] Code hash: 0x5bf27311ebedd5052f5109518d83221977449d6898c4040c94ba9c2ac271ec77
//     [*] Code is already uploaded in block
// 0x1bd29a68ea48c5acde9abb6c4081cd763d8f46e5255d2eaa7317080bac79ce3e
//
//     [*] Upload code "FT Logic"
//     [*] Code hash: 0x33a860e1e62e5c4a6aa0cb31ab8216662c5c5b2603c85e1b3e0c96983e8b75e7
//     [*] Code is already uploaded in block 0x06f2002aa7a0d1b281fcdfaad5f8fa0399a6f77e7cf076aedf965f147a36fd73
//
//     [*] Upload Fungible Token
//     [*] Calculated gas: 391,031,092
//     [*] Program id:
// 0xfa282c3f5bc1f120873235f2cbe27b700c8f0d58666b93f6ba59f442f619b006

    // 0xe6eb8542ac1b1c09ff1269d5eaaecbd455d7cdaaa43b3db1ccaac1f874d6d68d

    let bytes: [u8; 32] = [
        0xfa, 0x28, 0x2c, 0x3f, 0x5b, 0xc1, 0xf1, 0x20,
        0x87, 0x32, 0x35, 0xf2, 0xcb, 0xe2, 0x7b, 0x70,
        0x0c, 0x8f, 0x0d, 0x58, 0x66, 0x6b, 0x93, 0xf6,
        0xba, 0x59, 0xf4, 0x42, 0xf6, 0x19, 0xb0, 0x06];

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
            date_of_birth: block_timestamp(),
            fed: INT_NUM,
            rested: INT_NUM,
            entertained: INT_NUM,
            fed_block: bh.clone(),
            entertained_block: bh.clone(),
            rested_block: bh.clone(),
            allowed_account: None,
            ft_contract_id: ActorId::new(bytes),
            transaction_id: 0,
        });
    };

    msg::reply(String::from("Success!"), 0)
        .expect("Failed to share initialization result");
}

#[gstd::async_main]
async fn main() {
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
        TmgAction::Transfer(actor_id) => tamagotchi.transfer(actor_id),
        TmgAction::ApproveTokens {
            account,
            amount
        } => tamagotchi.approve_tokens(&account, amount).await,
        TmgAction::BuyAttribute {
            store_id,
            attribute_id,
        } => tamagotchi.buy_attribute(&store_id, attribute_id).await,
        TmgAction::SetFTokenContract(actor_id) => mood.set_ft_contract(&actor_id)
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






