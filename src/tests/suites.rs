use super::{
    arch::{
        task::task_context::task_context_test_suite,
        traps::{
            handler::trap_handler_test_suite, interrupt::interrupt_enabling_test_suite,
            trap_frame::trap_frame_test_suite,
        },
    },
    drivers::{
        cpu_intc::subsystem::cpu_intc_subsystem_test_suite,
        serials::{ns16550a::ns16550_test_suite, subsystem::serial_subsystem_test_suite},
        timer::subsystem::timer_subsystem_test_suite,
    },
    ktime::ktime_test_suite,
    mem::memory_test_suite,
    platform::platform_test_suite,
    primitives::ring_buff::ring_buff_primitive_test_suite,
    task::{list::task_list_test_suite, task_test_suite},
};

// Call all test suite function to auto register all suites in test manager.
// Don't change the order
pub fn test_suites() {
    platform_test_suite();
    serial_subsystem_test_suite();
    ring_buff_primitive_test_suite();
    timer_subsystem_test_suite();
    cpu_intc_subsystem_test_suite();
    ktime_test_suite();
    ns16550_test_suite();
    trap_frame_test_suite();
    interrupt_enabling_test_suite();
    trap_handler_test_suite();
    memory_test_suite();
    task_list_test_suite();
    task_test_suite();
    task_context_test_suite();
}
