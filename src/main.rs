#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(RustyOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use RustyOS::{println, print};
use core::panic::PanicInfo;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use RustyOS::allocator;
    use RustyOS::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello, World!{}", "!");
    RustyOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

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
