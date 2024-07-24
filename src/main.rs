#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
//remove the c zero runtime to run on baremetal target
pub extern "C" fn _start() -> ! {
    println!("Hello {}", 10);
    panic!("I JUST PEEPEEPOEED MYSELF");
    loop {}
}
