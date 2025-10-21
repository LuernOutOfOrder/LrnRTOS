pub struct BootWriter {
    pub base_addr: *mut u8,
}

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
