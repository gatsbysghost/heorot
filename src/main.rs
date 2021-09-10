#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This gets called in the event of a panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
