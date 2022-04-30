#![no_std]

use codec::{Decode, Encode};

use gstd::ActorId;
// use primitive_types::U256;
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum SaleAction {
    Send(u128),
    EndSale
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum SaleEvent {
    Bought {
        buyer: ActorId,
        amount: u128,
    },
    EndedSale,
}