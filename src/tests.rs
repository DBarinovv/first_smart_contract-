use ft_io::*;
use gtest::{Program, System};
use gstd::{String, Encode};
use ico_io::*;

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
    let res = ft.send(OWNER_ID, Action::Mint(TOKENS_CNT));
    assert!(!res.main_failed());

    let res = ft.send(
        OWNER_ID,
        Action::Approve {
            to: TK_SALE_ID.into(),
            amount: TOKENS_CNT,
        },  
    );
    assert!(!res.main_failed());
}

fn init_ico(sys: &System) {
    sys.init_logger();
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
}   

fn init(sys: &System) {
    init_fungible_token(&sys);
    init_ico(&sys);

    sys.init_logger();
    let _ft = sys.get_program(1);
    let ico = sys.get_program(2);

    let res = ico.send(OWNER_ID, IcoAction::StartSale(20000));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted.encode())));
}

#[test]
fn test_init() {
    let sys = System::new();
    init(&sys);
}

