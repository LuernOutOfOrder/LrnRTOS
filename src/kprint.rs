use core::fmt::Write;

use crate::config::KPRINT_ADDRESS;

/// Global ptr for default kernel console
/// Principally used for debugging when no uart is initialized
/// Use global static KPRINT_ADDRESS in config file to make the KernelConsole work
pub struct KernelConsole {
    pub base_addr: *mut u8,
}

/// Implement fmt::Write for BootWriter to allow format
impl core::fmt::Write for KernelConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            unsafe {
                core::ptr::write_volatile(self.base_addr, b);
            }
        }
        Ok(())
    }
}

unsafe impl Sync for KernelConsole {}

impl KernelConsole {
    pub const fn init() -> Self {
        KernelConsole {
            base_addr: KPRINT_ADDRESS as *mut u8,
        }
    }
}

/// Macro for easier use of write_fmt function and to use format_args macro
#[cfg(feature = "kprint")]
#[macro_export]
macro_rules! kprint_fmt {
    ($($arg:tt)*) => {
        $crate::kprint::write_fmt(format_args!($($arg)*))
    };
}

#[cfg(feature = "kprint")]
#[macro_export]
macro_rules! kprint {
    ($msg:expr) => {
        $crate::kprint::write_str($msg)
    };
}

#[cfg(not(feature = "kprint"))]
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {};
}

/// Get kconsole and use write_fmt of Write trait
pub fn write_fmt(args: core::fmt::Arguments) {
    #[allow(static_mut_refs)]
    let kconsole = unsafe { &mut KCONSOLE };
    let _ = kconsole.write_fmt(args);
}

/// Get kconsole and use write_fmt of Write trait
pub fn write_str(args: &str) {
    #[allow(static_mut_refs)]
    let kconsole = unsafe { &mut KCONSOLE };
    let _ = kconsole.write_str(args);
}

#[allow(static_mut_refs)]
pub static mut KCONSOLE: KernelConsole = KernelConsole::init();
