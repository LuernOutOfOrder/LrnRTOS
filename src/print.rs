use crate::drivers::serials::SERIAL_DEVICES;

/// Get uart set with default console and pass arg to write_fmt function of the driver
pub fn print(arg: core::fmt::Arguments) {
    if let Some(device) = SERIAL_DEVICES.get_default_console() {
        let _ = device.write_fmt(arg);
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
