use ft_io::*;
use gtest::{Program, System};
use gstd::{String, Encode, msg, ActorId};
use sale_io::*;

const MY_ADDRESS: u64 = 5;
const MY_ID: u64 = 100001;

fn init_fungible_token(system: &System) {
    let ft = Program::from_file(
        &system,
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
}

fn mint_tokens(ft: &Program, user: u64) {
    let res = ft.send(user, Action::Mint(10000));
    assert!(!res.main_failed());

    let res = ft.send(
        user,
        Action::Approve {
            to: 2.into(),
            amount: 10000,
        },
    );
    assert!(!res.main_failed());
}

fn init(program: &Program, system: &System) {
    init_fungible_token(&system);

    let res = program.send_bytes(
        MY_ADDRESS, 
        (SaleInit { price: 10, token_id: ActorId::new([2u8; 32]), token_decimals: 0 }).encode()
    );
    assert!(res.log().is_empty());

    mint_tokens(&program, MY_ID);
}

#[test]
fn test_init() {
    let system = System::new();
    system.init_logger();
    let program = Program::current(&system);

    init(&program, &system);
}

#[test]
fn test_buy() {
    let system = System::new();
    system.init_logger();
    let program = Program::current(&system);

    init(&program, &system);

    let value: u128 = 5;
    let res = program.send(MY_ADDRESS, SaleAction::Buy(value));
    // assert!(res.contains(&(MY_ADDRESS, (SaleEvent::Bought { buyer: ActorId::new([0u8; 32]), amount: value }).encode())));
}