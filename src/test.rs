use crate::kprint_fmt;

#[cfg(feature = "test")]
#[macro_export]
macro_rules! test_kprint {
    ($msg:expr) => {
        $crate::test::test_kprint($msg)
    };
}

pub fn test_kprint(s: &str) {
    kprint_fmt!("\x1b[32;1m[TEST PASSED]\x1b[0m {}\n", s);
}

#[panic_handler]
#[cfg(feature = "test")]
pub fn test_panic(s: &core::panic::PanicInfo) -> ! {
    kprint_fmt!("\x1b[31;1m[TEST FAILED]\x1b[0m {:?}", s);
    loop {}
}
