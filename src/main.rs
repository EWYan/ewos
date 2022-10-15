#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_wait;

// main for rust code
pub fn main() -> ! {
    bsp::uart::uart_init();
    bsp::uart::uart_writeText("Hello world!\n");
    loop {}
}
