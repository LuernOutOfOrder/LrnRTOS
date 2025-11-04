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
