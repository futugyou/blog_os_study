#![no_std]
#![no_main]

use core::panic::PanicInfo;

//static HELLO: &[u8] = b"Hello world!";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
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

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello world 2333").unwrap();
    write!(vga_buffer::WRITER.lock(),", this si {} and {}",42,10./3.0).unwrap();
    println!();
    println!("hahahaha {} 233","OJBK!");
    panic!("this si panic!");
    loop {}
}


mod vga_buffer;