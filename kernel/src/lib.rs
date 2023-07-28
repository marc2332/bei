#![feature(abi_x86_interrupt)]
#![no_std]
#![feature(const_mut_refs)]

pub mod allocator;
pub mod fixed_size_block;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod panic;
pub mod serial;
pub mod test_runner;
pub mod vga;
extern crate alloc;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
