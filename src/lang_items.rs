use core::panic::PanicInfo;

use sbi_rt::{Shutdown, SystemFailure};

use crate::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info
        );
    } else {
        println!("[kernel] Panicked: {}", info)
    }
    sbi_rt::system_reset(Shutdown, SystemFailure);
    unreachable!()
}
