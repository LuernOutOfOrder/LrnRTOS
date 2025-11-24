use crate::devices::serials::{KCONSOLE, SERIAL_DEVICES};

/// Get uart set with default console and pass arg to write_fmt function of the driver
pub fn print(arg: core::fmt::Arguments) {
    if let Some(device) = SERIAL_DEVICES.get_default_console() {
        let _ = device.driver.write_fmt(arg);
    } else {
        panic!("Default console not found");
    }
}

/// Macro for easier use of print function and to use format_args macro
#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => {
        $crate::print::print(format_args!($($arg)*))
    };
}

/// Macro for easier use of write_fmt function and to use format_args macro
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        $crate::print::write_fmt(format_args!($($arg)*))
    };
}

/// Get kconsole and use write_fmt of Write trait
pub fn write_fmt(args: core::fmt::Arguments) {
    let kconsole = KCONSOLE.get();
    if let Some(w) = kconsole {
        let _ = w.write_fmt(args);
    }
}
