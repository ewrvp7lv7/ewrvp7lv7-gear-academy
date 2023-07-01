//! # escrow-state
//!
//! independent crate for reading the state
//! Once we've built the crate, we'll have a file called `escrow_state.meta.wasm` that we can use in our UI applications to interact with the smart contract.

#![no_std]

use escrow_io::*;
use gmeta::metawasm;
use gstd::{prelude::*, ActorId};

#[metawasm]
pub mod metafns {
    pub type State = Escrow;

    pub fn seller(state: State) -> ActorId {
        state.seller
    }

    pub fn buyer(state: State) -> ActorId {
        state.buyer
    }

    pub fn escrow_state(state: State) -> EscrowState {
        state.state
    }
}
