use core::fmt;
use core::fmt::Write;


// const SBI_SET_TIMER: usize = 0;
const SBI_SET_TIMER: usize = 0x54494D45;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
// const SBI_SHUTDOWN: usize = 8;
const SBI_SHUTDOWN: usize = 0x53525354;



// 第三章 开始，将 os/src/sbi.rs 文件中的常量
// SBI_SHUTDOWN 的值替换为 const SBI_SHUTDOWN: usize = 0x53525354;
// SBI_SET_TIMER 的值替换为 const SBI_SET_TIMER: usize = 0x54494D45;

struct Stdout;



#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
        "li x16, 0",
        // 在 RiscV 中 ecall 指令用于触发系统调用, 从用户态切换到内核态
        "ecall",
        inlateout("x10") arg0 => ret,
        in("x11" ) arg1,
        in("x12" ) arg2,
        in("x17" ) which,
        );
    }
    ret
}


pub fn shutdown() ->! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}


pub fn console_putchar(c:usize){
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}


pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}