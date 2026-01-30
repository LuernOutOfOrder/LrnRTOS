pub struct AlignedStack<const N: usize> {
    pub buf: [u8; N],
}

impl<const N: usize> AlignedStack<N> {
    pub const fn new() -> Self {
        AlignedStack { buf: [0u8; N] }
    }
}
