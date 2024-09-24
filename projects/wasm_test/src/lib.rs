#![no_std]

#[no_mangle]
pub extern "C" fn add(a: u64, b: u64) -> u64 {
    a + b
}

//pack for number in one  u16,u16,u16,u16 -> u64
#[no_mangle]
pub extern "C" fn pack(a: u16, b: u16, c: u16, d: u16) -> u64 {
    let mut result: u64 = 0;
    result |= a as u64;
    result |= (b as u64) << 16;
    result |= (c as u64) << 32;
    result |= (d as u64) << 48;
    result
}
//function 



#[panic_handler]fn handle_panic(_: &core::panic::PanicInfo) -> ! {unreachable!()}