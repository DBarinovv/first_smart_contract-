#![no_std]

#[cfg(test)]
mod tests;

pub mod messages;
pub use messages::*;

use ico_io::*;

use core::{panic};

use gstd::{prelude::*, exec, msg, ActorId}; 

#[derive(Default)]
struct IcoContract {
    ico_state: IcoState,
    start_price: u128,
    current_price: u128, 
    price_increase_step: u128,
    time_increase_step: u128,
    tokens_sold: u128,
    tokens_goal: u128,
    owner: ActorId, 
    token_id: ActorId,
    token_holders: BTreeMap<ActorId, u128>,
}

static mut ICO_CONTRACT: Option<IcoContract> = None;

impl IcoContract {
    async fn get_tokens(&self) {
        mint_tokens(&self.token_id, self.tokens_goal).await;
        transfer_tokens(&self.token_id, &self.owner, &exec::program_id(), self.tokens_goal).await;
        approve(&self.token_id, &exec::program_id(), self.tokens_goal).await;
    }

    async fn start_ico(&mut self, duration: u64) {
        if msg::source() == self.owner && !self.ico_state.ico_started {
            self.get_tokens().await;

            self.ico_state.ico_started = true;
            self.ico_state.duration = duration;
            self.ico_state.start_time = exec::block_timestamp();

            msg::reply(IcoEvent::SaleStarted, 0).unwrap();
        }
        else {
            panic!(
                "start_contract(): ICO contract on: {}  Owner message: {}",
                self.in_process(),
                msg::source() == self.owner
            );
        }
    }

    pub fn buy_tokens(&mut self, tokens_cnt: u128)  {
        if self.in_process() {
            self.update_price();

            let (res, overflow) = tokens_cnt.overflowing_mul(self.current_price);
            if overflow {
                panic!("Overflowing multiplication")
            }
    
            if msg::value() != res {
                panic!("Wrong amount sent expect {} get {}", res, msg::value())
            }

            if self.tokens_sold + tokens_cnt > self.tokens_goal {
                panic!("Not enough tokens to sell")
            }

            *self.token_holders.entry(msg::source()).or_insert(tokens_cnt) += tokens_cnt;
            self.tokens_sold += tokens_cnt;
            
            msg::reply(IcoEvent::Bought { buyer: msg::source(), amount: tokens_cnt }, 0).unwrap();
        }
        else {
            panic!(
                "buy_tokens(): ICO contract on: {}",
                self.in_process(),
            );
        }
    }

    async fn end_sale(&self) {
        if self.get_balance() == 0 ||
            (!self.in_process() && self.ico_state.ico_started) {

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
        else {
            panic!(
                "end_sale(): ICO contract on: {} tokens_left = {} was started = {}",
                self.in_process(),
                self.get_balance(),
                self.ico_state.ico_started
            );
        }
    }

    fn update_price(&mut self) {
        let step = self.time_increase_step;
        let amount: u128 = (exec::block_timestamp() - self.ico_state.start_time).into();

        if step > amount {
            return
        }

        let mut steps_passed = amount / step;
        if amount % step != 0 {
            steps_passed += 1;
        }

        self.current_price = self.start_price + self.price_increase_step * steps_passed;
    }
    
    fn get_balance(&self) -> u128 {
        self.tokens_goal - self.tokens_sold
    }

    fn in_process(&self) -> bool {
        self.ico_state.ico_started && 
        (self.ico_state.start_time + self.ico_state.duration) > exec::block_timestamp() &&
        (self.get_balance() > 0)
    }
}


#[gstd::async_main]
async unsafe fn main() {
    let action: IcoAction = msg::load().expect("Unable to decode SaleAction");
    let ico: &mut IcoContract = unsafe { ICO_CONTRACT.get_or_insert(IcoContract::default()) };

    match action {
        IcoAction::StartSale(duration) => {
            ico.start_ico(duration).await;
        }
        IcoAction::Buy(value) => {
            ico.buy_tokens(value);
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: IcoInit = msg::load().expect("Unable to decode ICOInit");
    let ico = IcoContract {
        tokens_goal: config.tokens_goal,
        token_id: config.token_id,
        owner: config.owner,
        start_price: config.start_price,
        price_increase_step: config.price_increase_step,
        time_increase_step: config.time_increase_step,
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