use crate::{
    arch::traps::interrupt::{
        mtvec_read_mode, mtvec_read_trap_entry, mtvec_set_trap_entry, mtvec_switch_to_direct_mode,
        mtvec_switch_to_vectored_mode, trap_entry,
    },
    tests::TestCase,
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

pub static INTERRUPTIONS_RISCV32_TEST_SUITE: &[TestCase] = &[
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
];
