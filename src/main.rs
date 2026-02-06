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
use primitives::ring_buff::RingBuffer;
use task::{
    list::task_list_get_task_by_pid, primitives::{delay, sleep, r#yield}, task_context_switch, task_create, CURRENT_TASK_PID, TASK_HANDLER
};

// Static buffer to use as a ready queue for task.
pub static mut BUFFER: RingBuffer<u16, 3> = RingBuffer::init();
// Queue containing all blocked task.
pub static mut BLOCKED_QUEUE: RingBuffer<u16, 3> = RingBuffer::init();

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
    task_create("Test sleep", task_fn, 1, 0x200);
    task_create("Other test task", test_fn, 1, 0x200);
    unsafe { CURRENT_TASK_PID = 1 };
    let mut task = task_list_get_task_by_pid(unsafe { CURRENT_TASK_PID });
    unsafe { TASK_HANDLER = *task.as_mut().unwrap() };
    #[allow(static_mut_refs)]
    unsafe {
        BUFFER.push(2);
    }
    task_context_switch(task.unwrap());
    loop {
        log!(LogLevel::Debug, "Main loop uptime.");
        unsafe {
            arch::traps::interrupt::enable_and_halt();
        }
    }
}

fn task_fn() -> ! {
    loop {
        log!(LogLevel::Debug, "Test sleep task function");
        unsafe { r#yield() };
        unsafe { sleep(10) };
    }
}

fn test_fn() -> ! {
    loop {
        log!(LogLevel::Debug, "Always running or ready task");
        unsafe { r#yield() };
        delay(1000);
    }
}

#[panic_handler]
#[cfg(not(feature = "test"))]
fn panic_handler(panic: &PanicInfo) -> ! {
    kprint_fmt!("PANIC {:?}", panic);
    loop {}
}
