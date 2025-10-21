use crate::devices::serials::{KCONSOLE, UART_DEVICES};

pub fn print(arg: core::fmt::Arguments) {
    let devices = unsafe { &mut *UART_DEVICES.get() };
    if let Some(uart) = &mut devices[0] {
        uart.driver.write_fmt(arg).unwrap();
    }
}

#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => {
        $crate::print::print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        $crate::print::write_fmt(format_args!($($arg)*));
    });
}

pub fn write_fmt(args: core::fmt::Arguments) {
    let kconsole = unsafe { &mut *KCONSOLE.get() };
    if let Some(w) = kconsole {
        let _ = w.write_fmt(args);
    }
}

