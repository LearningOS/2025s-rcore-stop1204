
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]


use ::log::{error, log, warn, Level};
use crate::sbi::*;
use core::env;

mod lang_items;
mod sbi;
mod log;

core::arch::global_asm!(include_str!("entry.asm"));

fn clear_bss(){
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{

        unsafe {
            (a as *mut u8).write_volatile(0)
        }

    });
}

#[no_mangle]
pub fn rust_main(){
    clear_bss();
    log::log_info();

    shutdown();
}
