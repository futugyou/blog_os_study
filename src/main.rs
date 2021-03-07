#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


mod vga_buffer;
mod serial;
use core::panic::PanicInfo;
 

#[cfg(test)]
fn test_runner(tests:&[&dyn Fn()]){
    serial_println!("Running {} tests",tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}
//static HELLO: &[u8] = b"Hello world!";

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n",info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
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
    println!("hahahaha {} 233","OJBK!");
    #[cfg(test)]
    test_main();
    //panic!("this si panic!");
    loop {}
}


#[derive(Debug,Clone,Copy,PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    Success=0x10,
    Failed=0x11,
}

pub fn exit_qemu(exit_code
    :QemuExitCode){
    use x86_64::instructions::port::Port;
    unsafe{
        let mut port =Port::new(0xf4);
        port.write(exit_code as u32);
    }
}