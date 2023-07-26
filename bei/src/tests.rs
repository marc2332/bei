use bei_kernel::{
    println,
    vga::{BUFFER_HEIGHT, WRITER},
};

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1)
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    assert!(true)
}
