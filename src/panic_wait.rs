// panic handler
use core::panic::PanicInfo;
use crate::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("panic!");
    loop {
        continue;
    }
}
