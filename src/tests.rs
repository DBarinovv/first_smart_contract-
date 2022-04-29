#[test]
fn test_init() {
    let system = System::new();
    system.init_logger();
    let program = Program::current(&system);

    let res = program.send_bytes(MY_ADDRESS, "INIT");
    assert!(res.log().is_empty());

    
}