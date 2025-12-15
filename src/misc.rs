#[repr(C)]
pub struct RawTraitObject {
    pub data: *const (),
    pub vtable: *const (),
}
