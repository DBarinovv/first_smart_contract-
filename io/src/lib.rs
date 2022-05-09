#![no_std]

use codec::{Decode, Encode};

use gstd::ActorId;
// use primitive_types::U256;
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub enum SaleAction {
    AddTokens,
    Buy(u128),
    EndSale,
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub enum SaleEvent {
    AddedTokens(u128),
    Bought {
        buyer: ActorId,
        amount: u128,
    },
    EndedSale,
    ToManyTokens,
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub struct SaleInit {
    pub price: u128, 
    pub token_id: ActorId,
    pub token_decimals: u32,
    pub helper_send_id: ActorId,
}