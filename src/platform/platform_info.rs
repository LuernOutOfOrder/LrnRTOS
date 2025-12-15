pub struct PlatformInfo {
    pub flags: u8,
}

impl PlatformInfo {
    pub const fn init() -> Self {
        PlatformInfo { flags: 0 }
    }

    pub fn set_mode_fdt(&mut self) {
        let mask_mode: u8 = 1 << 0;
        let update_flag = mask_mode | self.flags;
        self.flags = update_flag | mask_mode;
    }

    pub fn read_mode(&self) -> bool {
        let mask_mode: u8 = 1 << 0;
        (self.flags & mask_mode) != 0
    }
}
