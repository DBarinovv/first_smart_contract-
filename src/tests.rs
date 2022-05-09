use ft_io::*;
use gtest::{Program, System};
use gstd::{String, Encode};
use sale_io::*;

use crate::constants::*;

fn init_fungible_token(sys: &System) {
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

    mint_tokens(&ft);
}

fn mint_tokens(ft: &Program<'_>) {
    let res = ft.send(HELPER_ID, Action::Mint(TOKENS_CNT));
    assert!(!res.main_failed());

    let res = ft.send(
        HELPER_ID,
        Action::Approve {
            to: TK_SALE_ID.into(),
            amount: TOKENS_CNT,
        },  
    );
    assert!(!res.main_failed());
}

fn init_tk_sale(sys: &System) {
    sys.init_logger();
    let tk_sale = Program::current(&sys);

    let res = tk_sale.send(
        OWNER_ID,
        SaleInit {
            price: 1000,
            token_id: TOKEN_ID.into(),
            token_decimals: 18,
            helper_send_id: HELPER_ID.into(),
        },
    );
    assert!(res.log().is_empty());
}   

fn init(sys: &System) {
    init_fungible_token(&sys);
    init_tk_sale(&sys);

    sys.init_logger();
    let _ft = sys.get_program(1);
    let tk_sale = sys.get_program(2);

    let res = tk_sale.send(OWNER_ID, SaleAction::AddTokens);
    assert!(res.contains(&(OWNER_ID, (SaleEvent::AddedTokens(TOKENS_CNT)).encode())));
}

#[test]
fn test_init() {
    let sys = System::new();
    init(&sys);
}

#[test]
fn test_buy() {
    let sys = System::new();
    init(&sys);
    let tk_sale = sys.get_program(2);

    let value: u128 = 5;
    let res = tk_sale.send_with_value(USER_ID, SaleAction::Buy(value), value * 1000);
    assert!(res.contains(&(USER_ID, (SaleEvent::Bought { buyer: USER_ID.into(), amount: value }).encode())));
}

#[test]
fn wrong_buy() {
    let sys = System::new();
    init(&sys);
    let tk_sale = sys.get_program(2);

    let value: u128 = 5;
    let res = tk_sale.send_with_value(USER_ID, SaleAction::Buy(value), value * 1000 - 1); // wrong value
    assert!(!res.contains(&(USER_ID, (SaleEvent::Bought { buyer: USER_ID.into(), amount: value }).encode())));
}

#[test]
fn end_sale() {
    let sys = System::new();
    init(&sys);
    let tk_sale = sys.get_program(2);

    let res = tk_sale.send(OWNER_ID, SaleAction::EndSale);
    assert!(res.contains(&(OWNER_ID, (SaleEvent::EndedSale).encode())));

    let value: u128 = 5;
    let res = tk_sale.send_with_value(OWNER_ID, SaleAction::Buy(value), value * 1000); // must panic
    assert!(!res.contains(&(OWNER_ID, (SaleEvent::Bought { buyer: OWNER_ID.into(), amount: value }).encode()))); 
}

#[test]
fn not_owner_end_sale() {
    let sys = System::new();
    init(&sys);
    let tk_sale = sys.get_program(2);

    let res = tk_sale.send(USER_ID, SaleAction::EndSale); // must panic
    assert!(!res.contains(&(USER_ID, (SaleEvent::EndedSale).encode())));
}