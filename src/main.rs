#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(RustyOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::{mem::size_of, panic::PanicInfo};
use lazy_static::lazy_static;
use RustyOS::{println, scheduler::TCB, task::keyboard};

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

async fn example_task1() {
    // loop {
    //     println!("This is 'example_task1'");
    //     for _ in 0..100000 {}
    // }
    println!("This is 'example_task1'");
}

async fn example_task2() {
    loop {
        println!("This is 'example_task2'");
        for _ in 0..100000 {}
    }
}

lazy_static! {
    static ref tasks: spin::Mutex<[TCB; 2]> = spin::Mutex::new([TCB::new(), TCB::new()]);
}

static stack: [u64; 1024] = [0; 1024];

fn example_thread() {
    let mut idx: i32 = 0;
    loop {
        let mut task_guard = tasks.lock();
        println!(
            "[{}] This message is from example_thread. Press any key to switch",
            idx
        );
        idx += 1;
        unsafe {
            task_guard[1].context.save();
            task_guard[0].context.load();
        }
    }
}

fn create_task() {
    let mut idx: i32 = {
        let mut task_guard = tasks.lock();
        task_guard[1].init(
            0,
            01,
            example_thread as u64,
            &stack as *const [u64; 1024] as u64,
            8 * 1024,
        );
        0
    };
    loop {
        let mut task_guard = tasks.lock();
        println!(
            "[{}] This message is from main_thread. Press any key to switch",
            idx
        );
        idx += 1;
        unsafe {
            task_guard[0].context.save();
            task_guard[1].context.load();
        }
    }
}

#[no_mangle]
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;
    use RustyOS::allocator;
    use RustyOS::memory;
    use RustyOS::task::{executor::Executor, Task};

    println!("Hello, World{}", '!');
    RustyOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    create_task();

    // let mut executor = Executor::new();
    // executor.spawn(Task::new(example_task1()));
    // executor.spawn(Task::new(example_task2()));
    // executor.spawn(Task::new(keyboard::print_keypresses()));
    // executor.run();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    RustyOS::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
