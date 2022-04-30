#![no_std]

use codec::{Decode, Encode};

use gstd::ActorId;
// use primitive_types::U256;
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub enum SaleAction {
    Buy(u128),
    EndSale,
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub enum SaleEvent {
    Bought {
        buyer: ActorId,
        amount: u128,
    },
    EndedSale,
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
pub struct SaleInit {
    pub price: u128, 
    pub token_id: ActorId,
    pub token_decimals: u32,
}