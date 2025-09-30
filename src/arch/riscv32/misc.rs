pub fn write_uart(char: u8) {
    unsafe { core::ptr::write_volatile(0x10000000 as *mut u8, char) }
}
