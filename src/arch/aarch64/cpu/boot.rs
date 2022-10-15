use core::arch::global_asm;

// Assembly counterpart to this file.
global_asm!(include_str!("boot.s"));

#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    crate::main()
}
