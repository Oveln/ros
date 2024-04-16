use core::panic::PanicInfo;

use crate::{println, sbi::shutdown};

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
    shutdown()
}
