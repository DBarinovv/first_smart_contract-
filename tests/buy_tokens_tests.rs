use core::time::Duration;

use gtest::System;
use gstd::Encode;
use ico_io::*;

use ico_contract::constants::*;

mod init_ico;
use init_ico::*;

#[test]
fn buy_tokens() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(2).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));

    let amount: u128 = 5;
    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), amount * START_PRICE);
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));
}

#[test]
fn buy_tokens_after_price_update() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(2).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));

    sys.spend_blocks((TIME_INCREASE_STEP).try_into().unwrap());

    let amount: u128 = 5;
    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), amount * (START_PRICE + PRICE_INCREASE_STEP));
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));

    sys.spend_blocks((TIME_INCREASE_STEP - 1).try_into().unwrap());

    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), amount * (START_PRICE + PRICE_INCREASE_STEP));
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));

    sys.spend_blocks(1);

    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), amount * (START_PRICE + PRICE_INCREASE_STEP * 2));
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));
}

#[test]
#[should_panic]
fn buy_when_no_time_left() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(2).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));

    sys.spend_blocks(3000); // 3 sec

    let amount: u128 = 5;
    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), amount * START_PRICE);
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));
}

#[test]
#[should_panic]
fn wrong_value_sent() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(2).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));

    let amount: u128 = 5;
    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), amount * START_PRICE - 1);
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));
}

#[test]
#[should_panic]
fn wrong_value_after_price_update() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(2).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));

    sys.spend_blocks((TIME_INCREASE_STEP + 1).try_into().unwrap());

    let amount: u128 = 5;
    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), amount * START_PRICE);
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));
}

#[test]
#[should_panic]
fn all_tokens_bought() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(2).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));

    let amount: u128 = TOKENS_CNT;
    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), amount * START_PRICE);
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));

    let res = ico.send_with_value(USER_ID, IcoAction::Buy(1), START_PRICE);
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount: 1 }).encode())));
}

#[test]
#[should_panic]
fn buy_before_start() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let amount: u128 = 5;
    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), amount * START_PRICE);
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));
}

#[test]
#[should_panic]
fn buy_after_end_sale() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(1).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));

    sys.spend_blocks(1001);

    let res = ico.send(OWNER_ID, IcoAction::EndSale);
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleEnded.encode())));

    let amount: u128 = 5;
    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), amount * START_PRICE);
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));
}

#[test]
#[should_panic]
fn buy_too_many_tokens() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(2).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));

    let amount: u128 = TOKENS_CNT + 1;
    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), amount * START_PRICE);
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));
}