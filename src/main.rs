#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    loop {}
}

/// This gets called in the event of a panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
