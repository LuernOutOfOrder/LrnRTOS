#![no_std]
#![no_main]
#![feature(naked_functions_rustic_abi)]

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

// Static buffer to use as a ready queue for task.
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
    loop {
        log!(LogLevel::Debug, "Main loop uptime.");
        unsafe {
            arch::traps::interrupt::enable_and_halt();
        }
    }
}

#[panic_handler]
#[cfg(not(feature = "test"))]
fn panic_handler(panic: &PanicInfo) -> ! {
    kprint_fmt!("PANIC {:?}", panic);
    loop {}
}
