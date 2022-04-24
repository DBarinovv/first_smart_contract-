use gstd::Encode;
use gtest::{Program, System};

use crate::{Action, Approvement, BalanceErrors};

const MY_ADDRESS: u64 = 5;

#[test]
fn test_send() {
    let system = System::new();
    system.init_logger();
    let program = Program::current(&system);

    let res = program.send_bytes(MY_ADDRESS, "INIT");
    assert!(res.log().is_empty());

    let value: u64 = 5;
    let res = program.send(MY_ADDRESS, Action::Send(value));
    assert!(res.contains(&(MY_ADDRESS, Approvement::Ok.encode())));
}

#[test]
fn test_withdraw() {
    let system = System::new();
    system.init_logger();
    let program = Program::current(&system);

    let res = program.send_bytes(MY_ADDRESS, "INIT");
    assert!(res.log().is_empty());

    let value: u64 = 5;
    let res = program.send(MY_ADDRESS, Action::Withdraw(value));
    assert!(res.contains(&(MY_ADDRESS, Approvement::Error(BalanceErrors::NotEnoughMoney).encode())));
}

#[test]
fn check_balance() {
    let system = System::new();
    system.init_logger();
    let program = Program::current(&system);

    let res = program.send_bytes(MY_ADDRESS, "INIT");
    assert!(res.log().is_empty());

    let value: u64 = 5;
    let res = program.send(MY_ADDRESS, Action::Send(value)); // 5
    assert!(res.contains(&(MY_ADDRESS, Approvement::Ok.encode())));

    let res = program.send(MY_ADDRESS, Action::CheckBalance);
    assert!(res.contains(&(MY_ADDRESS, (5 as u64).encode())));

    let res = program.send(MY_ADDRESS, Action::Send(value)); // 10
    assert!(res.contains(&(MY_ADDRESS, Approvement::Ok.encode())));

    let res = program.send(MY_ADDRESS, Action::CheckBalance);
    assert!(res.contains(&(MY_ADDRESS, (10 as u64).encode())));

    let res = program.send(MY_ADDRESS, Action::Withdraw(3)); // 7
    assert!(res.contains(&(MY_ADDRESS, Approvement::Ok.encode())));

    let res = program.send(MY_ADDRESS, Action::CheckBalance);
    assert!(res.contains(&(MY_ADDRESS, (7 as u64).encode())));
}