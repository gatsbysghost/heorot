#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(heorot::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use heorot::println;

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    #[cfg(test)]
    test_main();
    loop {}
}

/// This gets called in the event of a panic
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

/// This is the panic handler we call when we're testing
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    heorot::test_panic_handler(info)
}
