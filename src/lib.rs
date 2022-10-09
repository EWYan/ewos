#![no_std]
#![no_main]
#![allow(non_snake_case)]
use core::arch::asm;
#[allow(dead_code)]
use core::panic::PanicInfo;

const PERIPHERAL_BASE: u32 = 0xFE000000;
const AUX_BASE: u32 = PERIPHERAL_BASE + 0x215000;
const AUX_ENABLES: u32 = AUX_BASE + 4;
const AUX_MU_IO_REG: u32 = AUX_BASE + 64;
const AUX_MU_IER_REG: u32 = AUX_BASE + 68;
const AUX_MU_IIR_REG: u32 = AUX_BASE + 72;
const AUX_MU_LCR_REG: u32 = AUX_BASE + 76;
const AUX_MU_MCR_REG: u32 = AUX_BASE + 80;
const AUX_MU_LSR_REG: u32 = AUX_BASE + 84;
const AUX_MU_CNTL_REG: u32 = AUX_BASE + 96;
const AUX_MU_BAUD_REG: u32 = AUX_BASE + 104;
const AUX_UART_CLOCK: u32 = 500000000;
const UART_MAX_QUEUE: u32 = 16 * 1024;

const GPFSEL0: u32 = PERIPHERAL_BASE + 0x200000;
const GPSET0: u32 = PERIPHERAL_BASE + 0x20001C;
const GPCLR0: u32 = PERIPHERAL_BASE + 0x200028;
const GPPUPPDN0: u32 = PERIPHERAL_BASE + 0x2000E4;

const GPIO_MAX_PIN: u32 = 53;
const GPIO_FUNCTION_ALT5: u32 = 2;

#[no_mangle]
pub extern "C" fn main() -> ! {
    uart_init();
    uart_writeText("Hello world!\n");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn uart_init() {
    //enable UART1
    mmio_write(AUX_BASE, 1);
    mmio_write(AUX_MU_IER_REG, 0);
    mmio_write(AUX_MU_CNTL_REG, 0);
    mmio_write(AUX_MU_LCR_REG, 3);
    mmio_write(AUX_MU_MCR_REG, 0);
    mmio_write(AUX_MU_IER_REG, 0);
    mmio_write(AUX_MU_IIR_REG, 0xC6);
    mmio_write(AUX_MU_BAUD_REG, AUX_UART_CLOCK / (115200 * 8) - 1);
    gpio_useAsAlt5(14);
    gpio_useAsAlt5(15);
    //enable RX/TX
    mmio_write(AUX_MU_CNTL_REG, 3);
}

fn mmio_write(reg: u32, value: u32) {
    unsafe {
        core::ptr::write_volatile(reg as *mut u32, value);
    }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { core::ptr::read_volatile(reg as *mut u32) }
}

fn gpio_useAsAlt5(pin_number: u32) {
    gpio_pull(pin_number, 0);
    gpio_function(pin_number, GPIO_FUNCTION_ALT5);
}

fn gpio_set(pin_number: u32, value: u32) {
    gpio_call(pin_number, value, GPSET0, 1, GPIO_MAX_PIN);
}

fn gpio_clear(pin_number: u32, value: u32) {
    gpio_call(pin_number, value, GPCLR0, 1, GPIO_MAX_PIN);
}

fn gpio_pull(pin_number: u32, value: u32) {
    gpio_call(pin_number, value, GPPUPPDN0, 2, GPIO_MAX_PIN);
}

fn gpio_function(pin_number: u32, value: u32) {
    gpio_call(pin_number, value, GPFSEL0, 3, GPIO_MAX_PIN);
}

fn gpio_call(pin_number: u32, value: u32, base: u32, field_size: u32, field_max: u32) -> u32 {
    let field_mask: u32 = (1 << field_size) - 1;

    if pin_number > field_max {
        0
    } else {
        if value > field_mask {
            0
        } else {
            let num_fields:u32 = 32 / field_size;
            let reg:u32 = base + ((pin_number / num_fields) * 4);
            let shift:u32 = (pin_number % num_fields) * field_size;

            let mut curval: u32 = mmio_read(reg);
            curval &= !(field_mask << shift);
            curval |= value << shift;
            mmio_write(reg, curval);
            0
        }
    }
}

fn uart_isWriteByteReady() -> u32 {
    mmio_read(AUX_MU_LSR_REG) & 0x20
}

fn uart_writeByteBlockingActual(ch: u8) {
    loop {
        let condi = uart_isWriteByteReady();
        if condi != 0 {
            break;
        }
    }
    mmio_write(AUX_MU_IO_REG, ch as u32);
}

fn uart_writeText(buffer: &str) {
    for byte in buffer.bytes() {
        if byte == '\n' as u8 {
            uart_writeByteBlockingActual('\r' as u8);
        } else {
            uart_writeByteBlockingActual(byte);
        }
    }
}
