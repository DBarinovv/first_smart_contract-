use core::time::Duration;

use ft_io::*;
use gtest::{Program, System};
use gstd::{String, Encode};
use ico_io::*;

use ico_contract::constants::*;

mod init_ico;
use init_ico::*;

#[test]
#[should_panic]
fn not_owner_start_ico() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(20).as_millis() as u64;
    let res = ico.send(USER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(USER_ID, IcoEvent::SaleStarted(duration).encode())));
}

#[test]
#[should_panic]
fn second_start_ico() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(20).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));

    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));
}

#[test]
#[should_panic]
fn not_minting_tokens() {
    let sys = System::new();

    let ft = Program::from_file(
        &sys,
        "fungible-token/target/wasm32-unknown-unknown/release/fungible_token.wasm",
    );

    let res = ft.send(
        OWNER_ID,
        InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
        },
    );

    assert!(res.log().is_empty());

    let ico = Program::current(&sys);

    let res = ico.send(
        OWNER_ID,
        IcoInit { 
            tokens_goal: TOKENS_CNT, 
            token_id: TOKEN_ID.into(), 
            owner: OWNER_ID.into(), 
            start_price: 1000, 
            price_increase_step: 100, 
            time_increase_step: 5000, 
        },
    );
    assert!(res.log().is_empty());

    let duration = Duration::from_secs(20).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));
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
    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), 5000);
    assert!(res.contains(&(USER_ID, (IcoEvent::Bought { buyer: USER_ID.into(), amount }).encode())));
}