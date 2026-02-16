use crate::{
    arch::traps::{enable_interrupts, trap_frame::init_trap_frame},
    config::TICK_SAFETY_DURATION,
    drivers::{cpufreq::CpuFreq, init_subsystems},
    info::KERNEL_VERSION,
    kernel::context::kcontext_init,
    kprint, kprint_fmt,
    ktime::set_ktime_seconds,
    log,
    logs::LogLevel,
    mem::{mem_update_kernel_sp, memory_init},
    platform::platform_init,
};

#[unsafe(no_mangle)]
pub fn kernel_early_boot(core: usize, dtb_addr: usize) -> ! {
    kprint_fmt!("Kernel version: {}\n", KERNEL_VERSION);
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
    log!(LogLevel::Info, "Initializing memory...");
    memory_init();
    log!(LogLevel::Info, "Memory successfully initialized.");
    log!(LogLevel::Info, "Initializing kernel context...");
    kcontext_init();
    log!(LogLevel::Info, "Kernel context successfully initialized.");
    log!(LogLevel::Info, "Starting LrnRTOS...");
    log!(LogLevel::Debug, "Switch to new kernel stack...");
    mem_update_kernel_sp();
    // Allow empty loop because it will never enter, just to make the fn never return without
    // warning
    #[allow(clippy::empty_loop)]
    loop {}
}
