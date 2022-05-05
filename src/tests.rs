use ft_io::*;
use gtest::{Program, System};
use gstd::{String, Encode};
use sale_io::*;

const TOKEN_ID: u64 = 1;
const TK_SALE_ID: u64 = 2;
const HELPER_ID: u64 = 3;
const MY_ID: u64 = 100001;

fn init_fungible_token(sys: &System) {
    let ft = Program::from_file(
        &sys,
        "fungible-token/target/wasm32-unknown-unknown/release/fungible_token.wasm",
    );

    let res = ft.send(
        MY_ID,
        InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
        },
    );

    assert!(res.log().is_empty());

    mint_tokens(&ft, 100000);
}

fn mint_tokens(ft: &Program<'_>, tokens_cnt: u128) {
    let res = ft.send(HELPER_ID, Action::Mint(tokens_cnt));
    assert!(!res.main_failed());

    let res = ft.send(
        HELPER_ID,
        Action::Approve {
            to: TK_SALE_ID.into(),
            amount: tokens_cnt,
        },  
    );
    assert!(!res.main_failed());
}

fn init_tk_sale(sys: &System) {
    sys.init_logger();
    let tk_sale = Program::current(&sys);

    let res = tk_sale.send(
        MY_ID,
        SaleInit {
            price: 1000,
            token_id: TOKEN_ID.into(),
            token_decimals: 0,
            helper_send_id: HELPER_ID.into(),
        },
    );
    assert!(res.log().is_empty());
}   

#[test]
fn test_init() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_tk_sale(&sys);

    sys.init_logger();
    let _ft = sys.get_program(1);
    let tk_sale = sys.get_program(2);

    let res = tk_sale.send(MY_ID, SaleAction::AddTokens);
    assert!(res.contains(&(MY_ID, (SaleEvent::AddedTokens(100000)).encode())));
}

#[test]
fn test_buy() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_tk_sale(&sys);

    sys.init_logger();
    let _ft = sys.get_program(1);
    let tk_sale = sys.get_program(2);

    tk_sale.send(MY_ID, SaleAction::AddTokens);

    let value: u128 = 5;
    let res = tk_sale.send_with_value(MY_ID, SaleAction::Buy(value), 5000);
    assert!(res.contains(&(MY_ID, (SaleEvent::Bought { buyer: MY_ID.into(), amount: value }).encode())));
}