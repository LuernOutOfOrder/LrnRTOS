use crate::{
    arch::traps::{enable_interrupts, trap_frame::init_trap_frame},
    config::TICK_SAFETY_DURATION,
    drivers::{cpufreq::CpuFreq, init_subsystems},
    kprint, kprint_fmt,
    ktime::set_ktime_seconds,
    log,
    logs::LogLevel,
    mem::memory_init,
    platform::platform_init,
};

#[unsafe(no_mangle)]
pub fn kernel_early_boot(core: usize, dtb_addr: usize) -> ! {
    kprint_fmt!("Start kernel booting on CPU Core: {}.\n", core);
    kprint!("Initializing platform...\n");
    platform_init(dtb_addr);
    kprint!("Successfully initialized platform.\n");
    kprint!("Initializing all sub-systems...\n");
    init_subsystems();
    log!(LogLevel::Info, "Successfully initialized all sub-system.");
    log!(LogLevel::Info, "LrnRTOS booting...");
    CpuFreq::init();
    log!(LogLevel::Debug, "Initialing trap frame...");
    init_trap_frame();
    log!(LogLevel::Debug, "Successfully initialized trap frame.");
    set_ktime_seconds(TICK_SAFETY_DURATION);
    enable_interrupts();
    log!(
        LogLevel::Info,
        "Initializing memory and starting LrnRTOS..."
    );
    memory_init();
    // Allow empty loop because it will never enter, just to make the fn never return without
    // warning
    #[allow(clippy::empty_loop)]
    loop {}
}
