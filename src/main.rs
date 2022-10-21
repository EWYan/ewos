#![feature(asm_const)]
#![feature(format_args_nl)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_wait;
mod print;
use crate::bsp::rpi4::*;
// main for rust code
pub fn main() -> ! {
    uart::uart_init();
    uart::uart_write_text("Hello world!\n\r");
    println!("see nothing !!");
    loop {}
}
