/*
File info: AlignedStack primitive type.

Test coverage: ...

Tested:

Not tested:

Reasons: Not even really implemented so there's no need to test something that doesn't even consider finish

Tests files:

References:
*/

#[repr(align(16))]
pub struct AlignedStack16<const N: usize> {
    pub buf: [u8; N],
}

impl<const N: usize> AlignedStack16<N> {
    // Don't bother with this warning
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        AlignedStack16 { buf: [0u8; N] }
    }
}
