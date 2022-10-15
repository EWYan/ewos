#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_wait;

use crate::bsp::uart::*;
// main for rust code
pub fn main() -> ! {
    uart_init();
    uart_write_text("Hello world!\n");
    loop {
        let read = uart_read_byte();
        if read != 0x00 {
            uart_write_byte(read);
            uart_write_byte('\n' as u8);
            uart_write_byte('\r' as u8);
        }
    }
}
