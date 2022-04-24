#![no_std]

#[cfg(test)]
mod tests;

use codec::{Decode, Encode};
use gstd::{msg, ActorId};

const MY_ACTOR_ID: ActorId = ActorId::new([1u8; 32]);

#[derive(Debug)]
struct Wallet {
    balance: u64,
    id: ActorId,
}

impl Default for Wallet {
    fn default() -> Wallet {  
        Wallet { balance: 0, id: MY_ACTOR_ID }
    }
}

static mut MY_WALLET: Option<Wallet> = None;

impl Wallet {
    fn new() -> Wallet {
        Wallet {
            balance: 0,
            id: MY_ACTOR_ID
        }
    }

    fn get_money(&mut self, value: u64) {
        if msg::source() == self.id {
            msg::reply(Approvement::Error(BalanceErrors::SameWallet), 0).unwrap();
            return;
        }

        self.balance += value;

        msg::reply(Approvement::Ok, 0).unwrap();
    }

    fn send_money(&mut self, value: u64) {
        if msg::source() == self.id {
            msg::reply(Approvement::Error(BalanceErrors::SameWallet), 0).unwrap();
            return;
        }

        if self.balance < value {
            msg::reply(Approvement::Error(BalanceErrors::NotEnoughMoney), 0).unwrap();
        }
        else {
            self.balance -= value;
            msg::reply(Approvement::Ok, 0).unwrap();
        }
    }

    fn get_balance(&self) -> u64 {
        msg::reply(self.balance, 0).unwrap();
        self.balance
    }
}

#[derive(Debug, Decode, Encode)]
pub enum Action {
    Send(u64),
    Withdraw(u64),
    CheckBalance,
}

#[derive(Debug, Decode, Encode)]
pub enum Approvement {
    Ok,
    Error(BalanceErrors),
}

#[derive(Debug, Decode, Encode)]
pub enum BalanceErrors {
    NotEnoughMoney,
    SameWallet,
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action = msg::load().expect("Could not load");
    let wal: &mut Wallet = MY_WALLET.get_or_insert(Wallet::default());

    match action {
        Action::Send(value) => {
            wal.get_money(value)
        }
        Action::Withdraw(value) => {
            wal.send_money(value)
        }
        Action::CheckBalance => {
            wal.get_balance();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {}
