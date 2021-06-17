#![allow(non_snake_case)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(RustyOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use RustyOS::{println,
    task::{
        Task, simple_executor::SimpleExecutor
    }
};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number : {}", number);
}

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

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();
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
