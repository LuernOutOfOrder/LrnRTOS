pub struct Bitmap<T> {
    map: T,
}

impl Bitmap<T> {
    pub const fn new() -> Self {
        Bitmap { map: T }
    }

    /// Set the given bit to 1.
    pub fn set_bit(&mut self, bit: usize) {
        let mask = 1 << bit;
        self.map |= mask;
    }

    /// Clear the given bit. Set it to 0.
    pub fn clear_bit(&mut self, bit: usize) {
        let mask = 0 << bit;
        self.map &= mask;
    }

    /// Iterate over the bitmap and return the heavier bit.
    /// Example: bitmap set to u8 -> map: 01001010. The function will return 6, because the first
    /// bit set to 1, from the highest bit, is the bit 6.
    /// Arguments:
    /// &mut self: call the map initialized. Must be mutable.
    pub fn find_leading_bit(&mut self) -> usize {
        let bits = core::mem::size_of::<usize>() * 8;
        let mut value: usize = 0;
        for i in (0..bits).rev() {
            let bit = (self.map >> i) & 1;
            if bit == 1 {
                value = i;
                break;
            }
        }
        value
    }

    pub fn is_bitmap_zero(&self) -> bool {
        self.map == 0
    }
}
