#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os_study::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    blog_os_study::hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os_study::test_panic_handler(info)
}

use blog_os_study::{println, serial_print, serial_println};
#[test_case]
fn test_println() {
    serial_print!("testing_println...\n");
    println!("this is test");
    serial_println!("[ok]");
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
