#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(bei_kernel::test_runner::test_runner)]
#![reexport_test_harness_main = "test_bei"]
#![no_std]
#![no_main]

#[cfg(test)]
mod tests;
use bei_kernel::{interrupts, println, vga};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::init();
    interrupts::init_idt();

    #[cfg(test)]
    test_bei();

    println!("[Success] Started bei.");

    #[allow(unreachable_code)]
    #[allow(clippy::empty_loop)]
    loop {}
}
