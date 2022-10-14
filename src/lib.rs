#![no_std]
#![no_main]

#![allow(non_snake_case)]
#![allow(dead_code)]

// use core::arch::asm;
use core::panic::PanicInfo;

mod uart;

#[no_mangle]
pub extern "C" fn main() -> ! {
    uart::uart_init();
    uart::uart_writeText("Hello world!\n");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
