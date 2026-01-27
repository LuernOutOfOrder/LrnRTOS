#![no_std]
#![no_main]

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
use primitives::RingBuffer;
use task::{
    CURRENT_TASK_PID, list::task_list_get_task_by_pid, task_context_switch, task_create, r#yield,
};

/// Temporary static mut buffer, used to store and retrieve task.
pub static mut BUFFER: RingBuffer<u16, 3> = RingBuffer::init();

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

    // Temporary task creation and retrieving to test context switch.
    task_create("Testing task", task_fn, 0, 128);
    task_create("Testing task 2", task_2_fn, 0, 128);
    #[allow(static_mut_refs)]
    unsafe {
        BUFFER.push(2)
    };
    unsafe { CURRENT_TASK_PID = 1 };
    let task = task_list_get_task_by_pid(unsafe { CURRENT_TASK_PID });
    task_context_switch(task.unwrap());
    loop {
        log!(LogLevel::Debug, "Main loop uptime.");
        unsafe {
            arch::traps::interrupt::enable_and_halt();
        }
    }
}

// Temp task entry point
fn task_fn() -> ! {
    loop {
        log!(LogLevel::Info, "A");
        r#yield();
    }
}

fn task_2_fn() -> ! {
    loop {
        log!(LogLevel::Info, "B");
        r#yield();
    }
}

#[panic_handler]
#[cfg(not(feature = "test"))]
fn panic_handler(panic: &PanicInfo) -> ! {
    kprint_fmt!("PANIC {:?}", panic);
    loop {}
}
