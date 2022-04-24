#![no_std]

use gstd::{msg, String};

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let new_msg: String = msg::load().expect("Can't load(");

    if new_msg == "Hello" {
        msg::reply(b"Hello!", 0).unwrap();
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {}


#[cfg(test)]
mod tests {
    use gtest::{Program, System};

    #[test]
    fn test_hello() {
        let system = System::new();
        system.init_logger();
        let program = Program::current(&system);

        let res = program.send_bytes(2, "INIT");
        assert!(res.log().is_empty());

        let res = program.send_bytes(2, "Hello");
        let reply = "Hello!".as_bytes();
        assert!(res.contains(&(10, reply)));       
    }
}