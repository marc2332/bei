#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::init();

    println!("Booting bei.");

    #[allow(unreachable_code)]
    #[allow(clippy::empty_loop)]
    loop {}
}
