use ft_io::*;
use gtest::{Program, System};
use gstd::String;
use ico_io::*;

use ico_contract::constants::*;

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

pub fn init(sys: &System) {
    sys.init_logger();

    init_fungible_token(&sys);
    init_ico(&sys);
}