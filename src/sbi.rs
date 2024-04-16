fn sbi_call(which: usize, args: (usize, usize, usize)) -> usize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "li x16, 0",
            "ecall",
            inlateout("x10") args.0 => ret,
            in("x11") args.1,
            in("x12") args.2,
            in("x17") which
        );
    }
    ret
}

const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_SHUTDOWN: usize = 0x53525354;

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, (0, 0, 0));
    unreachable!();
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, (c, 0, 0));
}
