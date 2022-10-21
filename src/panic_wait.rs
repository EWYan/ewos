// panic handler

use core::panic::PanicInfo;
use crate::bsp::rpi4::{self, uart::uart_write_text};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        uart_write_text("panic here\r\n");
    }
}
