#![no_std]

#[cfg(test)]
mod tests;

pub mod messages;
pub use messages::*;

use gstd::{exec, prelude::*, msg, ActorId}; 
use sale_io::*;

// use primitive_types::U256;

#[derive(Default)]
struct TokenSale {
    price: u128, 
    owner: ActorId,
    tokens_sold: u128, 
    token_id: ActorId,
    token_decimals: u32,
    helper_send_id: ActorId,
}

static mut TOKEN_SALE: Option<TokenSale> = None;

impl TokenSale {
    // pub fn new(price: u128, token_id: ActorId, token_decimals: u32) -> TokenSale {
    //     TokenSale {
    //         price,
    //         owner: msg::source(),
    //         tokens_sold: 0, 
    //         token_id,
    //         token_decimals,
    //     }
    // }

    pub async fn add_tokens(&self) {
        let tokens_cnt = balance(&self.token_id, &self.helper_send_id).await;
        transfer_tokens(
            &self.token_id,
            &self.helper_send_id,
            &exec::program_id(),
            tokens_cnt,
        ).await;

        msg::reply(SaleEvent::AddedTokens(tokens_cnt), 0).unwrap();
    }

    pub async fn buy_tokens(&mut self, tokens_cnt: u128)  {
        let (res, overflow) = tokens_cnt.overflowing_mul(self.price);
        if overflow {
            panic!("Overflowing multiplication")
        }

        if msg::value() != res {
            panic!("Wrong amount sent")
        }

        let x: u128 = 10;
        let (scaled, overflow) = tokens_cnt.overflowing_mul((x).pow(self.token_decimals));
        if overflow {
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


#[gstd::async_main]
async unsafe fn main() {
    let action: SaleAction = msg::load().expect("Unable to decode SaleAction");
    let tk_sale: &mut TokenSale = unsafe { TOKEN_SALE.get_or_insert(TokenSale::default()) };

    match action {
        SaleAction::AddTokens => {
            tk_sale.add_tokens().await;
        }
        SaleAction::Buy(value) => {
            tk_sale.buy_tokens(value).await;
        }
        SaleAction::EndSale => {
            tk_sale.end_sale().await
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: SaleInit = msg::load().expect("Unable to decode SaleInit");
    let tk_sale = TokenSale {
        price: config.price,
        owner: msg::source(),
        tokens_sold: 0,
        token_id: config.token_id,
        token_decimals: config.token_decimals,
        helper_send_id: config.helper_send_id,
    };

    TOKEN_SALE = Some(tk_sale);
}

gstd::metadata! {
    title: "Sale_contract",
    init:
        input: SaleInit,
    handle:
        input: SaleAction,
        output: SaleEvent,
}