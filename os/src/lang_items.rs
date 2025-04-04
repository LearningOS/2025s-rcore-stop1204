
use core::panic::PanicInfo;
use crate::{print, println};
use crate::sbi::shutdown;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("[kernel] panic at {}:{} {}", location.file(), location.line(),info.message().unwrap());
    } else {
        println!("[kernel] panic at unknown location: {}",info.message().unwrap());
    }
    shutdown();
}
