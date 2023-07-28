#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(bei_kernel::test_runner::test_runner)]
#![reexport_test_harness_main = "test_bei"]
#![no_std]
#![no_main]

#[cfg(test)]
mod tests;
extern crate alloc;
use bei_kernel::{
    allocator,
    drawing::{add_draw_task, draw_and_paint, DrawTask},
    executor::Executor,
    gdt, interrupts, keyboard,
    memory::{self, BootInfoFrameAllocator},
    println,
    task::Task,
};

use bootloader::{entry_point, BootInfo};
use vga::colors::Color16;
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

    add_draw_task(DrawTask::DrawLine {
        x: (50, 50),
        y: (100, 100),
        color: Color16::Blue,
    });

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.spawn(Task::new(draw_and_paint()));
    executor.run();
}
