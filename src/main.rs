#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os_study::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use blog_os_study::{
    println,
    task::{executor::Executor, keyboard, Task},
};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os_study::allocator;
    use blog_os_study::memory;
    use x86_64::VirtAddr;

    println!("hahahaha Hello world!!");
    blog_os_study::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses())); // new
    executor.run();

    #[cfg(test)]
    test_main();
    // println!("this is ok!");
    // blog_os_study::hlt_loop()
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

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

// #[no_mangle]
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
//     // let vga_buffer = 0xb8000 as *mut u8;
//     // for (i, &byte) in HELLO.iter().enumerate() {
//     //     unsafe {
//     //         *vga_buffer.offset(i as isize * 2) = byte;
//     //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
//     //     }
//     // }

//     //vga_buffer::print_someting();

//     // use core::fmt::Write;
//     // vga_buffer::WRITER.lock().write_str("Hello world 2333").unwrap();
//     // write!(vga_buffer::WRITER.lock(),", this si {} and {}",42,10./3.0).unwrap();
//     println!();
//     println!("hahahaha {} 233", "OJBK!");

//     blog_os_study::init();

//     use x86_64::registers::control::Cr3;
//     let (level_4_page_table, _) = Cr3::read();
//     println!(
//         "level 4 page table at; {:?}",
//         level_4_page_table.start_address()
//     );

//     // double fault
//     // unsafe {
//     //     *(0xdeadbeef as *mut u64) = 42;
//     // }

//     // break interrupt
//     // x86_64::instructions::interrupts::int3();

//     // stack_overflow
//     // #[allow(unconditional_recursion)]
//     // fn stack_overflow() {
//     //     stack_overflow();
//     // }
//     // stack_overflow();

//     // page fault
//     // let ptr = 0xdeadbeef as *mut u32;
//     // Note : the actual address might be different for you.
//     // use the address that your page fault handler reports
//     let ptr = 0x203bd5 as *mut u32;
//     unsafe {
//         let _x = *ptr;
//     }
//     println!("read ok");
//     unsafe {
//         *ptr = 42;
//     }

//     #[cfg(test)]
//     test_main();

//     println!("it did not crash");
//     // panic!("this si panic!");
//     // loop {
//     //     use blog_os_study::print;
//     //     for _ in 0..10000 {}
//     //     print!("-");
//     // }
//     blog_os_study::hlt_loop()
// }
