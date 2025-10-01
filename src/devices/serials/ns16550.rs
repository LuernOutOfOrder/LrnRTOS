use super::UartDriver;

pub struct Ns16550 {
    addr: usize,
}

impl UartDriver for Ns16550 {
    fn init(&self) {
       todo!() 
    }
    fn putchar(&self, char: u8) {
        unsafe { core::ptr::write_volatile(self.addr as *mut u8, char) }
    }
    fn getchar(&self) -> u8 {
        todo!()
    }
}
