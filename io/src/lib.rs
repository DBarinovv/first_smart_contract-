#![no_std]

use codec::{Decode, Encode};

use primitive_types::U256;
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum SaleAction {
    Send(U256),
    EndSale
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum SaleEvent {
    Bought(U256),
}