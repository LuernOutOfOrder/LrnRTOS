use crate::devices::serials::UART_DEVICES;

pub fn print(str: &str) {
    for byte in str.bytes() {
        unsafe {
            if let Some(uart) = UART_DEVICES[0] {
                uart.driver.putchar(byte);
            }
        }
    }
}
