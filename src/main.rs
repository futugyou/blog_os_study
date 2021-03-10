#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os_study::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use blog_os_study::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
// use x86_64::structures::paging::PageTable;
// use x86_64::structures::paging::{Page, Translate};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    //use blog_os_study::memory::active_level_4_table;
    use blog_os_study::allocator; // new import
    use blog_os_study::memory;
    use x86_64::VirtAddr;

    println!("hahahaha Hello world!!");
    blog_os_study::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    //let mut frame_allocator = memory::EmptyFrameAllocator;
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    // let page = Page::containing_address(VirtAddr::new(0));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // let addresses = [
    //     0xb8000,
    //     0x201008,
    //     0x0100_0020_1a10,
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     // let phys = unsafe { translate_addr(virt, phys_mem_offset) };
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("   L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    #[cfg(test)]
    test_main();

    blog_os_study::hlt_loop()
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
