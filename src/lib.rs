#![no_std]

mod tests;

pub mod messages;
pub use messages::*;

use gstd::{exec, prelude::*, msg, ActorId};
use sale_io::*;

// use primitive_types::U256;

struct TokenSale {
    price: u128, 
    owner: ActorId,
    tokens_sold: u128, 
    token_id: ActorId,
    token_decimals: u32,
}

static mut TOKEN_SALE: Option<TokenSale> = None;
const TOKEN_ID: ActorId = ActorId::new([2u8; 32]);

impl TokenSale {
    pub fn new(price: u128, token_id: ActorId, token_decimals: u32) -> TokenSale {
        TokenSale {
            price,
            owner: msg::source(),
            tokens_sold: 0, 
            token_id,
            token_decimals
        }
    }

    pub async fn buy_tokens(&mut self, tokens_cnt: u128)  {

        let (res, overflow) = tokens_cnt.overflowing_mul(self.price);
        if !overflow {
            panic!("Overflowing multiplication")
        }

        if msg::value() != res {
            panic!("Wrong amount sent")
        }

        let x: u128 = 10;
        let (scaled, overflow) = tokens_cnt.overflowing_mul((x).pow(self.token_decimals));
        if !overflow {
            panic!("Overflowing multiplication")
        }

        let tokens_left = balance(&self.token_id, &exec::program_id()).await;
        if tokens_left < scaled {
            panic!("Not enough tokens")
        }

        msg::reply(SaleEvent::Bought { buyer: msg::source(), amount: tokens_cnt }, 0).unwrap();
        self.tokens_sold += tokens_cnt;

        transfer_tokens(
            &self.token_id,
            &exec::program_id(),
            &msg::source(),
            scaled,
        )
        .await;
    }

    pub async fn end_sale(&self) {
        if msg::source() != self.owner {
            panic!("Wrong owner")
        }

        let tokens_left = balance(&self.token_id, &exec::program_id()).await;

        transfer_tokens(
            &self.token_id,
            &exec::program_id(),
            &self.owner,
            tokens_left,
        )
        .await;

        msg::reply(SaleEvent::EndedSale, 0).unwrap();
    }
}


#[no_mangle]
pub async unsafe extern "C" fn handle() {
    let action = msg::load().expect("Could not load");
    let tk_sale: &mut TokenSale = 
        TOKEN_SALE.get_or_insert(TokenSale::new(10, TOKEN_ID, 0));

    match action {
        SaleAction::Send(value) => {
            tk_sale.buy_tokens(value).await
        }
        SaleAction::EndSale => {
            tk_sale.end_sale().await
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn init() {}

gstd::metadata! {
    title: "Sale_contract",
    handle:
        input: SaleAction,
        output: SaleEvent,
}