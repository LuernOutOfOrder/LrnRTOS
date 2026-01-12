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
    tests::{TestCase, TEST_MANAGER},
};

pub fn test_mtvec_set_direct_mode() {
    let current_mode = mtvec_read_mode();
    mtvec_switch_to_direct_mode();
    let update_mode = mtvec_read_mode();
    if current_mode != update_mode {
        panic!("mtvec mode should be 0, got: {}", update_mode);
    }
}

pub fn test_mtvec_set_vectored_mode() {
    mtvec_switch_to_vectored_mode();
    let mode = mtvec_read_mode();
    if mode != 1 {
        panic!("mtvec mode should be 1, got: {}", mode);
    }
}

pub fn test_mtvec_trap_entry() {
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
}

pub fn test_mscratch_trap_frame() {
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
}

pub fn test_mie_mtie() {
    let current_mie_mtie = read_mie_mtie();
    enable_mie_mtie();
    let update_mie_mtie = read_mie_mtie();
    if current_mie_mtie == update_mie_mtie {
        panic!("mie.mtie should have been updated to enable machine timer interrupt");
    }
}

pub fn test_mie_msie() {
    let current_mie_msie = read_mie_msie();
    enable_mie_msie();
    let update_mie_msie = read_mie_msie();
    if current_mie_msie == update_mie_msie {
        panic!("mie.msie should have been updated to enable machine software interrupt");
    }
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
//
pub fn interrupt_enabling_test_suite() {
    let interruptions_riscv32_test_suite: &[TestCase] = &[
        TestCase {
            name: "RISC-V 32 bits mtvec set direct mode",
            func: test_mtvec_set_direct_mode,
        },
        TestCase {
            name: "RISC-V 32 bits mtvec set vectored mode",
            func: test_mtvec_set_vectored_mode,
        },
        TestCase {
            name: "RISC-V 32 bits mtvec trap_entry",
            func: test_mtvec_trap_entry,
        },
        TestCase {
            name: "RISC-V 32 bits mscratch trap_frame",
            func: test_mscratch_trap_frame,
        },
        TestCase {
            name: "RISC-V 32 bits mie.mtie",
            func: test_mie_mtie,
        },
        TestCase {
            name: "RISC-V 32 bits mie.msie",
            func: test_mie_msie,
        },
        // TestCase {
        //     name: "RISC-V 32 bits mstatus.mie",
        //     func: test_mstatus_mie,
        // },
    ];
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(interruptions_riscv32_test_suite)
    };
}
