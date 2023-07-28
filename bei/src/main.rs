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
    allocator, gdt, hlt_loop, interrupts,
    memory::{self, BootInfoFrameAllocator},
    println, vga,
};

use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    vga::init();
    gdt::init();
    interrupts::init_idt();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_bei();

    println!("[Success] Started bei.");

    hlt_loop()
}
