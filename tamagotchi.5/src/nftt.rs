use gstd::{msg, prelude::*, ActorId};
use tamagotchi_io::{Tamagotchi, TmgEvent};

pub trait NFTamagotchi {
    fn transfer(&mut self, actor_id: ActorId);
    fn approve(&mut self, actor_id: ActorId);
    fn revoke_approval(&mut self);
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
}
