#![no_std]

use gstd::{msg, String};

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let new_msg = String::from_utf8(msg::load_bytes()).expect("Invalid message");

    if new_msg == "Hello" {
        msg::reply(b"Hello!", 0).expect("Can't send(");
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {}

#[cfg(test)]
mod tests {
    use gtest::{Program, System};

    #[test]
    fn test_hello() {
        const MY_ADDRESS: u64 = 2;

        let system = System::new();
        system.init_logger();
        let program = Program::current(&system);

        let res = program.send_bytes(MY_ADDRESS, "INIT");
        assert!(res.log().is_empty());

        let res = program.send_bytes(MY_ADDRESS, "Hello");
        let reply = "Hello!".as_bytes();
        assert!(res.contains(&(MY_ADDRESS, reply)));
    }
}
