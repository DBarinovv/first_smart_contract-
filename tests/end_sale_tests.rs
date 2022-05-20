use core::time::Duration;

use gtest::System;
use gstd::Encode;
use ico_io::*;

use ico_contract::constants::*;

mod init_ico;
use init_ico::*;

#[test]
fn end_sale_no_time_left() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(1).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));

    sys.spend_blocks(1001);

    let res = ico.send(OWNER_ID, IcoAction::EndSale);
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleEnded.encode())));
}

#[test]
fn end_sale_zero_tokens() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(1).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));

    let amount: u128 = TOKENS_CNT;
    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), amount * START_PRICE);
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));

    let res = ico.send(OWNER_ID, IcoAction::EndSale);
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleEnded.encode())));
}

#[test]
#[should_panic]
fn not_owner_end_sale() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(1).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));

    sys.spend_blocks(1001);

    let res = ico.send(USER_ID, IcoAction::EndSale);
    assert!(res.contains(&(USER_ID, IcoEvent::SaleEnded.encode())));
}

#[test]
#[should_panic]
fn end_sale_before_start() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let res = ico.send(OWNER_ID, IcoAction::EndSale);
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleEnded.encode())));
}
