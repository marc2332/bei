#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(bei_kernel::test_runner::test_runner)]
#![reexport_test_harness_main = "test_bei"]
#![no_std]
#![no_main]

#[cfg(test)]
mod tests;
extern crate alloc;

use alloc::{string::ToString, sync::Arc};
use bei_kernel::{
    allocator,
    drawing::draw_and_paint,
    executor::Executor,
    gdt, interrupts,
    keyboard::detect_keypresses,
    memory::{self, BootInfoFrameAllocator},
    println,
    task::Task,
    windowing::{Window, WindowManager},
};

use bootloader::{entry_point, BootInfo};
use hashbrown::HashMap;
use spin::Mutex;
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    bei_kernel::vga::init();
    gdt::init();
    interrupts::init_idt();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    if allocator::init_heap(&mut mapper, &mut frame_allocator).is_ok() {
        println!("[Success] Started the Heap.")
    } else {
        println!("[Error] Failed starting the Heap.")
    }

    #[cfg(test)]
    test_bei();

    println!("[Success] Started bei.");

    let window_manager = WindowManager::new(
        Some(0),
        HashMap::from([
            (
                0,
                Window::new(0, "BeiOS v0.1.0".to_string(), (250, 200), true, (25, 25)),
            ),
            (
                1,
                Window::new(
                    1,
                    "Another window".to_string(),
                    (280, 175),
                    false,
                    (290, 70),
                ),
            ),
        ]),
    );

    window_manager.draw();

    let window_manager = Arc::new(Mutex::new(window_manager));

    let mut executor = Executor::new();
    executor.spawn(Task::new(detect_keypresses(window_manager)));
    executor.spawn(Task::new(draw_and_paint()));
    executor.run();
}
