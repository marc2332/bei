#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(bei_kernel::test_runner::test_runner)]
#![reexport_test_harness_main = "test_bei"]
#![no_std]
#![no_main]

#[cfg(test)]
mod tests;
extern crate alloc;

use alloc::sync::Arc;
use bei_kernel::{
    allocator,
    executor::Executor,
    gdt, interrupts,
    keyboard::detect_keypresses,
    memory::{self, BootInfoFrameAllocator},
    println,
    shell::Shell,
    task::Task,
};

use bootloader::{entry_point, BootInfo};
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

    let shell = Shell::new();
    let shell = Arc::new(Mutex::new(shell));

    let mut executor = Executor::new();
    executor.spawn(Task::new(detect_keypresses(shell)));
    executor.run();
}
