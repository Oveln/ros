#![no_std]
#![no_main]

use sbi_rt::{NoReason, Shutdown};

mod console;
mod lang_items;

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() {
    clear_bss();
    println!("Hello World!");
    sbi_rt::system_reset(Shutdown, NoReason);
}
