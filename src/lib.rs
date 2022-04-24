#![no_std]

use gstd::{msg, lock::mutex::Mutex};

#[no_mangle]
pub unsafe extern "C" fn handle() {

    let new_msg = String::from_utf8(msg::load_bytes()).expect("Invalid message");

    if new_msg == "Send" {

    }
    else if new_msg == "Withdraw" {

    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
}

struct Wallet {
    balance: u64,
}

impl Wallet {
    fn new() -> Wallet {
        Wallet {
            balance: 0,
        }
    }

    fn get_money(&mut self, value: u64) {
        let quard = Mutex::new(self.balance);
        quard.lock();
        self.balance += value;
    }

    fn send_money(&mut self, value: u64) {
        let quard = Mutex::new(self.balance);
        quard.lock();
        if self.balance < value {
            msg::reply(b"Error", 0).unwrap();
        }
        else {
            self.balance -= value;
            msg::reply(b"Ok", 0).unwrap();
        }
    }

    fn get_balance(&self) -> u64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use gtest::{Program, System};

    #[test]
    fn test_send() {
        const MY_ADDRESS: u64 = 5;

        let system = System::new();
        system.init_logger();
        let program = Program::current(&system);


    }

    #[test]
    fn test_withdraw() {

    }
}