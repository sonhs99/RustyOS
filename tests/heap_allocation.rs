#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(RustyOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;
    use RustyOS::allocator;
    use RustyOS::memory::{self, BootInfoFrameAllocator};

    RustyOS::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();
    loop {}
}

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(32);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 32);
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_box() {
    use RustyOS::allocator::HEAP_SIZE;
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn many_box_long_lived() {
    use RustyOS::allocator::HEAP_SIZE;
    let first = Box::new(0);
    for i in 1..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*first, 0);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    RustyOS::test_panic_handler(info)
}
