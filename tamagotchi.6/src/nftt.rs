use gstd::{msg, prelude::*, ActorId, ReservationId};
use ft_main_io::{FTokenAction, FTokenEvent, LogicAction};
use store_io::{StoreAction, StoreEvent};
use async_trait::async_trait;
use tamagotchi6_io::*;

pub trait NFTamagotchi {
    fn transfer(&mut self, actor_id: ActorId);
    fn approve(&mut self, actor_id: ActorId);
    fn revoke_approval(&mut self);

    fn reserve_gas(&mut self, reservation_amount: u64, duration: u32);
}

impl NFTamagotchi for Tamagotchi {
    fn transfer(&mut self, actor_id: ActorId) {
        let sender = msg::source();
        assert!(
            sender == self.owner || self.allowed_account == Some(sender),
            "Only owner or allowed account can transfer ownership"
        );
        self.owner = actor_id;

        msg::reply(
            TmgEvent::Transfer(actor_id),
            0,
        ).expect("Failed to share the TmgEvent");
    }

    fn approve(&mut self, actor_id: ActorId) {
        assert!(msg::source() == self.owner, "Only owner can approve");
        self.allowed_account = Some(actor_id);

        msg::reply(
            TmgEvent::Approve(actor_id),
            0,
        ).expect("Failed to share the TmgEvent");
    }

    fn revoke_approval(&mut self) {
        assert!(msg::source() == self.owner, "Only owner can revoke approval");
        self.allowed_account = None;

        msg::reply(
            TmgEvent::RevokeApproval,
            0,
        ).expect("Failed to share the TmgEvent");
    }

    fn reserve_gas(&mut self, reservation_amount: u64, duration: u32) {
        let reservation_id = ReservationId::reserve(
            reservation_amount,
            duration,
        ).expect("reservation across executions");
        self.reservations.push(reservation_id);

        msg::reply(
            TmgEvent::GasReserved,
            0
        ).expect("Failed to share TmgEvent");
    }
}

#[async_trait]
pub trait TamagotchiShop {
    async fn approve_tokens(&mut self, account: &ActorId, amount: u128);
    async fn buy_attribute(
        &mut self,
        store_id: &ActorId,
        attribute_id: AttributeId,
    );
}

#[async_trait]
impl TamagotchiShop for Tamagotchi {
    async fn approve_tokens(&mut self, account: &ActorId, amount: u128) {
        assert!(msg::source() == self.owner, "Only owner can revoke approval");

        let result = msg::send_for_reply_as::<_, FTokenEvent>(
            self.ft_contract_id,
            FTokenAction::Message {
                transaction_id: self.transaction_id,
                payload: LogicAction::Approve {
                    approved_account: *account,
                    amount,
                },
            },
            0,
        )
            .expect("Error in sending a message `FTokenAction::Message`")
            .await;

        match result {
            Ok(FTokenEvent::Ok) => {
                let _ = self.transaction_id.wrapping_add(1);
                msg::reply(
                    TmgEvent::ApproveTokens { account: *account, amount },
                    0,
                ).expect("Failed to share the TmgEvent");
            }
            _ => {
                msg::reply(
                    TmgEvent::ApprovalError,
                    0,
                ).expect("Failed to share TmgEvent");
            }
        }

    }

    async fn buy_attribute(
        &mut self,
        store_id: &ActorId,
        attribute_id: AttributeId
    ) {
        assert!(msg::source() == self.owner, "Only owner can revoke approval");

        let result = msg::send_for_reply_as::<_, StoreEvent>(
            *store_id,
            StoreAction::BuyAttribute {
                attribute_id
            },
            0
        )
            .expect("Error in sending a message `StoreAction::BuyAttribute`")
            .await;

        match result {
            Ok(StoreEvent::CompletePrevTx{attribute_id}) => {
                msg::reply(
                    TmgEvent::CompletePrevPurchase(attribute_id),
                    0
                ).expect("Failed to share TmgEvent");
            },
            Ok(StoreEvent::AttributeSold{success: true}) => {
                msg::reply(
                    TmgEvent::AttributeBought(attribute_id),
                    0
                ).expect("Failed to share TmgEvent");
            },
            _ => {
                msg::reply(
                    TmgEvent::ErrorDuringPurchase,
                    0
                ).expect("Failed to share TmgEvent");
            }
        }
    }

}
