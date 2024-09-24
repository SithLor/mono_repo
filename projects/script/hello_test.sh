#[panic_handler]fn handle_panic(_: &core::panic::PanicInfo) -> ! {unreachable!()}
curl -X POST -d '#![no_std]#[no_mangle] pub extern "C" fn add(a:u64,b:u64)-> u64 {a+b} #[panic_handler]fn handle_panic(_: &core::panic::PanicInfo)-> ! {unreachable!()}
' http://localhost:8000/create_function