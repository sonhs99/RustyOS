#![no_std]
#![no_main]

use core::panic::PanicInfo;
use RustyOS::{exit_qemu, serial_print, serial_println, QEMUExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[OK]");
    exit_qemu(QEMUExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
