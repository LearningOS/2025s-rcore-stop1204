//! Process management syscalls

use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next, get_syscall_counter},
    timer::get_time_us,

};

use core::ptr::{read_volatile,write_volatile};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0


}


// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    // trace!("kernel: sys_trace");
    match _trace_request {
        0 => {
            // 将 _id 视为 *const u8，读取该地址的一个字节
            // syscall(SYSCALL_TRACE, [_trace_request, _id, _data]);
            let ptr = _id as *const u8;
            unsafe { read_volatile(ptr) as isize }


        }
        1 => {
            // 将 _id 视为 *const u8，写入 _data 的最低一個字节
            let ptr = _id as *mut u8;  // Need mutable pointer for writing
            unsafe {
                // Write the lowest byte of _data to the address pointed by ptr
               write_volatile(ptr, _data as u8)
            };

            0 // 返回 0
        }
        2 => {
            //询当前任务调用编号为 id 的系统调用的次数，返回值为这个调用次数。本次调用也计入统计
            // info!("kernel: sys_trace: syscall count");
            get_syscall_counter(_id).try_into().unwrap_or(0) as isize

        }
        _ => {
            -1 // 其他情况返回 -1
        }
    }


}
