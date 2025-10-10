pub trait KernelPrint: core::fmt::Write {
    fn write_str_raw(&mut self, s: &str);
}

pub struct BootWriter {
    pub base_addr: *mut u8,
}

impl core::fmt::Write for BootWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for byte in s.bytes() {
                core::ptr::write_volatile(self.base_addr, byte);
            }
        }
        Ok(())
    }
}
