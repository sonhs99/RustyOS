#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use RustyOS::{exit_qemu, serial_print, serial_println, QEMUExitCode};

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(RustyOS::gdt::DOUBLE_FAULT_IST_INDEX)
        };
        idt
    };
}

extern "x86-interrupt" fn test_double_fault_handler(
    stackframe: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[Ok]");
    exit_qemu(QEMUExitCode::Success);
    loop {}
}

fn init_test_idt() {
    TEST_IDT.load();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    RustyOS::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    RustyOS::test_panic_handler(info)
}

fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}
