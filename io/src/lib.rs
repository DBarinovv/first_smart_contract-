#![no_std]

use codec::{Decode, Encode};

use gstd::ActorId;
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub enum IcoAction {
    StartSale,
    Buy(u128),
    EndSale,
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub enum IcoEvent {
    SaleStarted,
    Bought {
        buyer: ActorId,
        amount: u128,
    },
    SaleEnded,
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub struct IcoInit {
    pub tokens_goal: u128,
    pub price: u128,
    pub price_step: u128,
    pub tokens_step: u128,
    pub owner: ActorId,
    pub token_id: ActorId,
}