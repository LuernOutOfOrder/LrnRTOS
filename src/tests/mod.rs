#![cfg(feature = "test")]
use crate::{kprint_fmt, platform::test::test_platform_get_device_info_static};

#[macro_export]
macro_rules! test_kprint {
    ($msg:expr) => {
        $crate::tests::test_kprint($msg)
    };
}

pub fn test_kprint(s: &str) {
    kprint_fmt!("\x1b[32;1m[TEST PASSED]\x1b[0m {}\n", s);
}

#[macro_export]
macro_rules! test_info_kprint {
    ($msg:expr) => {
        $crate::tests::test_info_kprint($msg)
    };
}

pub fn test_info_kprint(s: &str) {
    kprint_fmt!("\x1b[33;1m[TEST INFO]\x1b[0m {}\n", s);
}

#[panic_handler]
pub fn test_panic(s: &core::panic::PanicInfo) -> ! {
    kprint_fmt!("\x1b[31;1m[TEST FAILED]\x1b[0m {:?}", s);
    loop {}
}

#[unsafe(no_mangle)]
pub fn test_kernel_early_boot(core: usize, dtb_addr: usize) -> ! {
    use crate::platform::test::{test_platform_get_device_info_fdt, test_platform_init};
    
    test_info_kprint!("Starting kernel in test mode.");
    if core != 0 {
        panic!("Booting on wrong CPU core");
    }
    test_kprint!("Successfully start kernel booting on CPU Core: 0.");
    test_platform_init(dtb_addr);
    test_platform_get_device_info_fdt();
    test_platform_get_device_info_static();
    #[allow(clippy::empty_loop)]
    loop {}
}
