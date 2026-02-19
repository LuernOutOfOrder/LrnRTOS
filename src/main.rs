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
#[cfg(not(feature = "test"))]
use core::panic::PanicInfo;
use logs::LogLevel;
use mem::mem_kernel_stack_info;

use scheduler::{RUN_QUEUE, RUN_QUEUE_BITMAP};
#[cfg(feature = "idle_task")]
use task::task_idle_task;
use task::{
    TASK_HANDLER, list::task_list_get_task_by_pid, primitives::sleep, task_context_switch,
    task_create,
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
    #[cfg(feature = "idle_task")]
    task_idle_task();
    task_create("High priority", hight_priority_task, 10, 0x1000);
    task_create("Low priority", low_priority_task, 4, 0x1000);
    // High priority task.
    let mut task = task_list_get_task_by_pid(1);
    unsafe { TASK_HANDLER = *task.as_mut().unwrap() };
    unsafe {
        RUN_QUEUE[0][4].push(2);
        RUN_QUEUE_BITMAP[0].set_bit(4);
    };
    task_context_switch(task.unwrap());
    loop {
        log!(LogLevel::Debug, "Main loop.");
        unsafe {
            arch::traps::interrupt::enable_and_halt();
        }
    }
}

fn hight_priority_task() -> ! {
    loop {
        print!("This is a high priority task !!!!!!!!!!\n");
        unsafe { sleep(10) };
    }
}

fn low_priority_task() -> ! {
    loop {
        // print!("Low priority task\n");
    }
}

#[panic_handler]
#[cfg(not(feature = "test"))]
fn panic_handler(panic: &PanicInfo) -> ! {
    kprint_fmt!("PANIC {:?}", panic);
    loop {}
}
