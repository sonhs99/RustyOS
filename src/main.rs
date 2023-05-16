#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(RustyOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::{PhysAddr, VirtAddr};
use RustyOS::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    RustyOS::test_panic_handler(_info);
}

entry_point!(kernel_main);

#[no_mangle]
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use RustyOS::memory::activate_level_4_table;

    println!("Hello, World{}", '!');
    RustyOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { activate_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    RustyOS::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
