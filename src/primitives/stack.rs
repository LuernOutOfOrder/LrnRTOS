pub struct AlignedStack<const N: usize> {
    pub buf: [u8; N],
}

impl<const N: usize> AlignedStack<N> {
    // Don't bother with this warning
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        AlignedStack { buf: [0u8; N] }
    }
}
