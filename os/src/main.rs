
#![no_std]
#![no_main]

use crate::sbi::{shutdown};
use crate::syscall::{sys_exit,print};


mod lang_items;
mod sbi;
mod syscall;

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


// // no_mangle is used to prevent the Rust compiler from changing the name of the function
// #[no_mangle]
// extern "C" fn _start() {
//     println!("Hello, world!");
//     sys_exit(9);
//     // shutdown();
// }
#[no_mangle]
pub fn rust_main(){
    clear_bss();
    println!("Hello, world!");
    // shutdown();
    println!("Hello, world!~~~~~~~~~~~~");

    sys_exit(0);
}
