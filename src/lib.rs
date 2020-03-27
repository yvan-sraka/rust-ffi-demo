extern crate libc;

#[link(name = "hello", kind = "static")]
extern {
    fn say_hello_to_world() -> u32;
}

#[no_mangle]
pub fn dummy() -> u32 {
    return unsafe { say_hello_to_world() };
}
