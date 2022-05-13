#![no_std]

#[cfg(test)]
mod tests;

pub mod messages;
pub use messages::*;

use ico_io::*;

use core::panic;

use gstd::{prelude::*, exec, msg, ActorId}; 

#[derive(Default)]
struct IcoContract {
    tokens_goal: u128,
    price: u128, 
    price_step: u128,
    tokens_step: u128,
    tokens_sold: u128,
    owner: ActorId, 
    token_id: ActorId,
    was_ended: bool,
    was_stated: bool,
    token_holders: BTreeMap<ActorId, u128>,
}

static mut ICO_CONTRACT: Option<IcoContract> = None;

impl IcoContract {
    async fn update_end(&mut self) {
        if self.tokens_sold >= self.tokens_goal {
            self.end_sale().await;
        }
    }

    pub async fn start_contract(&mut self) {
        if msg::source() == self.owner && !self.was_stated {
            transfer_tokens(
                &self.token_id,
                &exec::program_id(),
                &self.owner,
                self.tokens_goal,
            )
            .await;

            self.was_stated = true;

            self.update_end();
        }
        else {
            panic!(
                "start_contract(): ICO contract on: {}  Owner message: {}",
                self.was_stated,
                msg::source() == self.owner
            );
        }

        msg::reply(IcoEvent::SaleStarted, 0).unwrap();
    }

    pub async fn buy_tokens(&mut self, tokens_cnt: u128)  {
        if self.in_process() {

        }
    }

    async fn end_sale(&mut self) {   
        if !self.was_stated {
            panic!("Can't end sale before it starts")
        } 
        else {
            self.was_ended = true;

            for (id, val) in &self.token_holders {
                transfer_tokens(
                    &self.token_id,
                    &exec::program_id(),
                    id,
                    *val,
                )
                .await;
            }

            msg::reply(IcoEvent::SaleEnded, 0).unwrap();
        }
    }
    
    fn get_balance(&self) -> u128 {
        self.tokens_goal - self.tokens_sold
    }

    fn in_process(&self) -> bool {
        self.was_stated && !self.was_ended
    }
}


#[gstd::async_main]
async unsafe fn main() {
    let action: IcoAction = msg::load().expect("Unable to decode SaleAction");
    let ico: &mut IcoContract = unsafe { ICO_CONTRACT.get_or_insert(IcoContract::default()) };

    match action {
        IcoAction::StartSale => {
            ico.start_contract().await;
        }
        IcoAction::Buy(value) => {
            ico.buy_tokens(value).await;
        }
        IcoAction::EndSale => {
            ico.end_sale().await
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: IcoInit = msg::load().expect("Unable to decode ICOInit");
    let ico = IcoContract {
        tokens_goal: config.tokens_goal,
        price: config.price,
        price_step: config.price_step,
        tokens_step: config.tokens_step,
        owner: config.owner,
        token_id: config.token_id,
        ..IcoContract::default()
    };

    ICO_CONTRACT = Some(ico);
}

gstd::metadata! {
    title: "ICO_contract",
    init:
        input: IcoInit,
    handle:
        input: IcoAction,
        output: IcoEvent,
}