#![no_std]
#![no_main]
#![warn(clippy::all)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

// Config module
pub mod config;

// Arch specific module
pub mod arch;

// Drivers module
pub mod drivers;

// Device init
pub mod devices_info;
pub mod platform;

// Logging modules
pub mod kprint;
pub mod logs;
pub mod print;

// Module for kernel time
pub mod ktime;

// Memory management module
pub mod mem;

// Misc mod
pub mod misc;

// Early boot module
pub mod boot;

// Kernel information
pub mod info;

// Task mod
pub mod task;

// Primitive type mod
pub mod primitives;

// Scheduler module
pub mod scheduler;

// Test module
#[cfg(feature = "test")]
pub mod tests;

// Use from modules
use arch::task::r#yield;
#[cfg(not(feature = "test"))]
use core::panic::PanicInfo;
use logs::LogLevel;
use mem::mem_kernel_stack_info;

use task::{
    CURRENT_TASK_PID, TASK_HANDLER, list::task_list_get_task_by_pid, task_context_switch,
    task_create, task_idle_task,
};

#[unsafe(no_mangle)]
unsafe extern "C" fn main() -> ! {
    log!(LogLevel::Debug, "Successfully switch to new kernel stack.");
    let kernel_stack = mem_kernel_stack_info();
    log!(
        LogLevel::Debug,
        "Kernel new stack: stack-top: {:#x}\tstack-bottom: {:#x}",
        kernel_stack.top,
        kernel_stack.bottom
    );
    log!(LogLevel::Info, "LrnRTOS started!");
    task_idle_task();
    task_create("Test task", test_task, 1, 0x200);
    #[allow(static_mut_refs)]
    unsafe {
        CURRENT_TASK_PID = 2
    };
    let mut task = task_list_get_task_by_pid(unsafe { CURRENT_TASK_PID });
    unsafe { TASK_HANDLER = *task.as_mut().unwrap() };
    task_context_switch(task.unwrap());
    loop {
        log!(LogLevel::Debug, "Main loop uptime.");
        unsafe {
            arch::traps::interrupt::enable_and_halt();
        }
    }
}

fn test_task() -> ! {
    loop {
        log!(LogLevel::Debug, "Test task, only yield.");
        unsafe { r#yield() };
    }
}

#[panic_handler]
#[cfg(not(feature = "test"))]
fn panic_handler(panic: &PanicInfo) -> ! {
    kprint_fmt!("PANIC {:?}", panic);
    loop {}
}
