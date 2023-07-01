#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Escrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
    pub state: EscrowState,
}

// #[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
// pub enum EscrowState {
//     AwaitingPayment,
//     AwaitingDelivery,
//     Closed,
// }
//
// impl Default for EscrowState {
//     fn default() -> Self {
//         Self::AwaitingPayment
//     }
// }

#[derive(Default, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum EscrowState {
    #[default]
    AwaitingPayment,
    AwaitingDelivery,
    Closed,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitEscrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowAction {
    Deposit,
    ConfirmDelivery,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowEvent {
    FundsDeposited,
    DeliveryConfirmed,
}


pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    // type Init = InOut<InitEscrow, ()>;
    type Init = In<InitEscrow>;
    type Handle = InOut<EscrowAction, EscrowEvent>;
    type Reply = InOut<(), ()>;
    type Others = InOut<(), ()>;
    type Signal = ();
    type State = EscrowState;
}
