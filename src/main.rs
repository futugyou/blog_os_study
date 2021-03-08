#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os_study::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os_study::println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os_study::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os_study::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    //vga_buffer::print_someting();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello world 2333").unwrap();
    // write!(vga_buffer::WRITER.lock(),", this si {} and {}",42,10./3.0).unwrap();
    println!();
    println!("hahahaha {} 233", "OJBK!");

    blog_os_study::init();

    //double fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // }

    // break interrupt
    //x86_64::instructions::interrupts::int3();

    // stack_overflow
    // #[allow(unconditional_recursion)]
    // fn stack_overflow() {
    //     stack_overflow();
    // }
    //stack_overflow();

    #[cfg(test)]
    test_main();

    println!("it did not crash");
    //panic!("this si panic!");
    // loop {
    //     use blog_os_study::print;
    //     for _ in 0..10000 {}
    //     print!("-");
    // }
    blog_os_study::hlt_loop()
}
