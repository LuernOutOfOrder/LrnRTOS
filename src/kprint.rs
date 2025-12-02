use core::{cell::UnsafeCell, fmt::Write};

use crate::config::KPRINT_ADDRESS;

/// Structure that is used for debugging purpose, used to print at a given address before devices
/// are initialized
pub struct BootWriter {
    pub base_addr: *mut u8,
}

/// Implement fmt::Write for BootWriter to allow format
impl core::fmt::Write for BootWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            unsafe {
                core::ptr::write_volatile(self.base_addr, b);
            }
        }
        Ok(())
    }
}

/// Global ptr for default kernel console
/// Principally used for debugging when no uart is initialized
pub struct KernelConsole {
    pub console: UnsafeCell<Option<&'static mut dyn Write>>,
}

unsafe impl Sync for KernelConsole {}

impl KernelConsole {
    // New without default needed for const use
    #[allow(clippy::new_without_default)]
    pub const fn new(console: &'static mut dyn Write) -> Self {
        KernelConsole {
            console: UnsafeCell::new(Some(console)),
        }
    }

    pub fn get(&self) -> Option<&'static mut dyn Write> {
        if let Some(console) = unsafe { &mut *self.console.get() } {
            Some(console)
        } else {
            None
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
    let kconsole = KCONSOLE.get();
    if let Some(w) = kconsole {
        let _ = w.write_fmt(args);
    }
}

/// Get kconsole and use write_fmt of Write trait
pub fn write_str(args: &str) {
    let kconsole = KCONSOLE.get();
    if let Some(w) = kconsole {
        let _ = w.write_str(args);
    }
}

static mut EARLY_WRITER: BootWriter = BootWriter {
    base_addr: KPRINT_ADDRESS as *mut u8,
};

#[allow(static_mut_refs)]
pub static KCONSOLE: KernelConsole = KernelConsole::new(unsafe { &mut EARLY_WRITER });
