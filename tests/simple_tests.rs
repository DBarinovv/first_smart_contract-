use core::time::Duration;

use gtest::System;
use gstd::Encode;
use ico_io::*;

use ico_contract::constants::*;

mod init_ico;
use init_ico::*;

#[test]
fn test_init() {
    let sys = System::new();
    init(&sys);
}

#[test]
fn start_ico() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(20).as_millis() as u64;
    let res = ico.send(OWNER_ID, IcoAction::StartSale(duration));
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleStarted(duration).encode())));
}

