use crate::{
    arch::traps::{
        interrupt::{
            enable_mie_msie, enable_mie_mtie, mscratch_read, mscratch_set_trap_frame,
            mtvec_read_mode, mtvec_read_trap_entry, mtvec_set_trap_entry,
            mtvec_switch_to_direct_mode, mtvec_switch_to_vectored_mode, read_mie_msie,
            read_mie_mtie, trap_entry,
        },
        trap_frame::{init_trap_frame, KERNEL_TRAP_FRAME},
    },
    tests::{TestBehavior, TestCase, TestSuite, TestSuiteBehavior, TEST_MANAGER},
};

pub fn test_mtvec_set_direct_mode() -> u8 {
    let current_mode = mtvec_read_mode();
    mtvec_switch_to_direct_mode();
    let update_mode = mtvec_read_mode();
    if current_mode != update_mode {
        panic!("mtvec mode should be 0, got: {}", update_mode);
    }
    0
}

pub fn test_mtvec_set_vectored_mode() -> u8 {
    mtvec_switch_to_vectored_mode();
    let mode = mtvec_read_mode();
    if mode != 1 {
        panic!("mtvec mode should be 1, got: {}", mode);
    }
    0
}

pub fn test_mtvec_trap_entry() -> u8 {
    let mtvec_trap_entry = mtvec_read_trap_entry();
    let trap_entry_addr = trap_entry as usize as u32;
    mtvec_set_trap_entry();
    let updated_mtvec_trap_entry = mtvec_read_trap_entry();
    if mtvec_trap_entry != 0 {
        panic!("mtvec should be empty before setting the trap_entry ptr");
    }
    if trap_entry_addr != updated_mtvec_trap_entry {
        panic!("mtvec trap_entry is wrong. It should be using the trap_entry function address");
    }
    0
}

pub fn test_mscratch_trap_frame() -> u8 {
    // Init trap_frame and declare ptr to it
    init_trap_frame();
    #[allow(static_mut_refs)]
    // Ptr to KERNEL_TRAP_FRAME static
    let ptr = unsafe { &mut KERNEL_TRAP_FRAME } as *mut _ as u32;
    let current_mscratch = mscratch_read();
    mscratch_set_trap_frame();
    let update_mscratch = mscratch_read();
    if current_mscratch == update_mscratch {
        panic!("mscratch hasn't been updated to use the KERNEL_TRAP_FRAME structure.")
    }
    if update_mscratch != ptr {
        panic!("mscratch isn't using the KERNEL_TRAP_FRAME structure.")
    }
    0
}

pub fn test_mie_mtie() -> u8 {
    let current_mie_mtie = read_mie_mtie();
    enable_mie_mtie();
    let update_mie_mtie = read_mie_mtie();
    if current_mie_mtie == update_mie_mtie {
        panic!("mie.mtie should have been updated to enable machine timer interrupt");
    }
    0
}

pub fn test_mie_msie() -> u8 {
    let current_mie_msie = read_mie_msie();
    enable_mie_msie();
    let update_mie_msie = read_mie_msie();
    if current_mie_msie == update_mie_msie {
        panic!("mie.msie should have been updated to enable machine software interrupt");
    }
    0
}

// pub fn test_mstatus_mie() {
//     let current_mstatus_mie = read_mstatus_mie();
//     // Set safety tick to not trigger timer interrupt after enabling it.
//     set_ktime_seconds(TICK_SAFETY_DURATION);
//     enable_mstatus_mie();
//     let update_mstatus_mie = read_mstatus_mie();
//     disable_mstatus_mie();
//     if current_mstatus_mie == update_mstatus_mie {
//         panic!("mstatus.mie should have been updated to enable global machine interrupt");
//     }
// }

pub fn interrupt_enabling_test_suite() {
    const INTERRUPT_TEST_SUITE: TestSuite = TestSuite {
        tests: &[
            TestCase::init(
                "RISC-V 32 bits mtvec set direct mode",
                test_mtvec_set_direct_mode,
                TestBehavior::Default,
            ),
            TestCase::init(
                "RISC-V 32 bits mtvec set vectored mode",
                test_mtvec_set_vectored_mode,
                TestBehavior::Default,
            ),
            TestCase::init(
                "RISC-V 32 bits mtvec trap_entry",
                test_mtvec_trap_entry,
                TestBehavior::Default,
            ),
            TestCase::init(
                "RISC-V 32 bits mscratch trap_frame",
                test_mscratch_trap_frame,
                TestBehavior::Default,
            ),
            TestCase::init(
                "RISC-V 32 bits mie.mtie",
                test_mie_mtie,
                TestBehavior::Default,
            ),
            TestCase::init(
                "RISC-V 32 bits mie.msie",
                test_mie_msie,
                TestBehavior::Default,
            ),
            // TestCase {
            //     name: "RISC-V 32 bits mstatus.mie",
            //     func: test_mstatus_mie,
            // },
        ],
        name: "Interruptions enabling",
        tests_nb: 6,
        behavior: TestSuiteBehavior::Default
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&INTERRUPT_TEST_SUITE)
    };
}
