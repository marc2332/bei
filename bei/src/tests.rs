use bei_kernel::{print, println};

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[test_case]
fn print() {
    print!("print");
    println!("[ok]");
}

#[test_case]
fn println() {
    print!("println");
    println!("[ok]");
}
