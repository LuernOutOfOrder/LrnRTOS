pub fn write_uart(char: u8) {
    unsafe { core::ptr::write_volatile(0x101f1000 as *mut u8, char) }
}
