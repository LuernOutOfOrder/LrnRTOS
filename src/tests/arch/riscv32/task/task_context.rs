use core::{arch::asm, mem};

use crate::{
    BUFFER,
    arch::{task::task_context::TaskContext, traps::interrupt::halt},
    task::{
        CURRENT_TASK_PID, list::task_list_get_task_by_pid, task_context_switch, task_create,
        r#yield,
    },
    test_failed, test_info,
    tests::{TEST_MANAGER, TestBehavior, TestCase, TestSuite, TestSuiteBehavior},
};

/// Test purpose function to create a task context.
/// DO NOT USE OUTSIDE OF THOSE TESTS
fn test_task_context_entry_ptn_fn() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

pub fn test_task_context_init() -> u8 {
    let task_size: [usize; 2] = [0x8800_0000, 0x8700_0000];
    let task_context: TaskContext = TaskContext::init(task_size, test_task_context_entry_ptn_fn);
    // Check context field that are testable
    // Check GPRs
    for i in 0..32 {
        if task_context.gpr[i] != 0 {
            test_failed!(
                "One of the gpr register has not been initialized at 0. This can lead to UB"
            );
            return 1;
        }
    }
    // Check address_space
    if task_context.address_space[0] != task_size[0] as u32
        || task_context.address_space[1] != task_size[1] as u32
    {
        panic!("Task context has been initialized with wrong address space.");
    }
    // Check pc
    if task_context.pc != test_task_context_entry_ptn_fn as usize as u32 {
        panic!(
            "Task context has been initialized with wrong PC, expect pc to be set to the address of the given function"
        );
    }
    // Check sp
    if task_context.sp != task_size[0] as u32 {
        panic!(
            "Task context has been initialized with wrong SP, expect sp to be set to the hi address of the task address space"
        );
    }
    0
}

pub fn test_task_context_offset() -> u8 {
    let gpr_off = mem::offset_of!(TaskContext, gpr);
    if gpr_off != 0 {
        panic!("Task context gpr offset must be 0, got: {gpr_off}");
    }
    let addr_spc_off = mem::offset_of!(TaskContext, address_space);
    if addr_spc_off != 128 {
        panic!("Task context address_space offset must be 128, got: {addr_spc_off}");
    }
    let pc_off = mem::offset_of!(TaskContext, pc);
    if pc_off != 136 {
        panic!("Task context pc offset must be 136, got: {pc_off}");
    }
    let sp_off = mem::offset_of!(TaskContext, sp);
    if sp_off != 140 {
        panic!("Task context sp offset must be 140, got: {sp_off}");
    }
    let ra_off = mem::offset_of!(TaskContext, ra);
    if ra_off != 144 {
        panic!("Task context ra offset must be 144, got: {sp_off}");
    }
    let flags_off = mem::offset_of!(TaskContext, flags);
    if flags_off != 148 {
        panic!("Task context flags offset must be 144, got: {flags_off}");
    }
    let instruction_reg_off = mem::offset_of!(TaskContext, instruction_register);
    if instruction_reg_off != 151 {
        panic!("Task context instruction_register offset must be 147, got: {instruction_reg_off}");
    };
    0
}

fn test_context_switch_a() -> ! {
    let mut task: usize = 0;
    unsafe { asm!("mv {}, sp", out(reg) task) };
    let mut i: usize = 0;
    print!("\nA DEBUG SP start task: {:#x}", task);
    loop {
        i += 1;
        print!("\nA {i}\n");
        if i >= 28 {
            unsafe {
                halt();
            }
        } else {
            print!("A DEBUG SP before yield: {:#x}\n", task);
            r#yield();
            print!("A DEBUG SP after yield: {:#x}", task);
        }
    }
}

fn test_context_switch_b() -> ! {
    let mut task: usize = 0;
    unsafe { asm!("mv {}, sp", out(reg) task) };
    let mut i: usize = 0;
    print!("\nB DEBUG SP start task: {:#x}", task);
    loop {
        i += 1;
        print!("\nB {i}\n");
        if i >= 27 {
            unsafe {
                halt();
            }
        } else {
            print!("B DEBUG SP before yield: {:#x}\n", task);
            r#yield();
            print!("B DEBUG SP after yield: {:#x}", task);
        }
    }
}

/// Test context switch across 2 tasks
/// It's hard to test context switch, so the dev running this test must intercept any invariants
/// violated.
/// Don't work yet, must make the kernel work on test mode with memory discovery and switch kernel
/// sp ?
#[unsafe(no_mangle)]
pub fn test_task_context_switch() -> u8 {
    // Temporary task creation and retrieving to test context switch.
    task_create("A", test_context_switch_a, 0, 256);
    task_create("B", test_context_switch_b, 0, 256);
    #[allow(static_mut_refs)]
    unsafe {
        BUFFER.push(3)
    };
    unsafe { CURRENT_TASK_PID = 2 };
    let task = task_list_get_task_by_pid(unsafe { CURRENT_TASK_PID });
    test_info!(
        "The next output should be the task A and B, which print alternately A, and B, with a digit. The final output must from A should be 28, and from B, 26"
    );
    task_context_switch(task.unwrap());
    0
}

pub fn task_context_test_suite() {
    const TASK_CONTEXT_TEST_SUITE: TestSuite = TestSuite {
        tests: &[
            TestCase::init(
                "Task context init",
                test_task_context_init,
                TestBehavior::Default,
            ),
            TestCase::init(
                "Task context memory layout offset",
                test_task_context_offset,
                TestBehavior::Default,
            ),
            TestCase::init(
                "Task context switch no invariants violated",
                test_task_context_switch,
                TestBehavior::Default,
            ),
        ],
        name: "RISC-V32 bit task context layout",
        tests_nb: 2,
        behavior: TestSuiteBehavior::Default,
    };
    #[allow(static_mut_refs)]
    unsafe {
        TEST_MANAGER.add_suite(&TASK_CONTEXT_TEST_SUITE)
    };
}
