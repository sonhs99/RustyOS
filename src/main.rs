#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(RustyOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use RustyOS::{println, print};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World!{}", "!");

    RustyOS::init();

    let ptr = 0x2031b2 as *mut u32;
    unsafe { let x = *ptr; }
    println!("읽기 성공");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    RustyOS::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    RustyOS::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    RustyOS::test_panic_handler(_info);
}
