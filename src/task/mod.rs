/*
File info: Task management. Handle all task logic.

Test coverage: Basic task creation, memory region and context switch(save and restore).

Tested:
- task_create function.
- Context save and context restore.

Not tested:
- fn ptr, state and priority.

Reasons:
- Some of the task field or properties or whatever are not fully testable yet, will be easier with a real scheduler. But the main features are tested.

Tests files:
- 'src/tests/task/mod.rs'
*/

use list::task_list_add_task;

use crate::{
    arch::{task::task_context::TaskContext, traps::interrupt::enable_and_halt},
    log,
    logs::LogLevel,
    mem::mem_task_alloc,
};

pub mod list;
pub mod primitives;

// Mutable static to keep track of the current task
// Only relevant on a monocore CPU.
pub static mut CURRENT_TASK_PID: u16 = 0;

// Only mutable ptr to the current task.
// Used as a handler to the current task to be able to save the context rapidly.
#[unsafe(no_mangle)]
pub static mut TASK_HANDLER: *mut Task = core::ptr::null_mut();

// Enum representing all state of a task.
#[repr(u8)]
// Allow unused for now because this issue doesn't need to handle all task state
#[allow(unused)]
#[derive(Copy, Clone, PartialEq)]
pub enum TaskState {
    New,
    Running,
    Ready,
    Waiting,
    Blocked,
    Terminated,
}

#[derive(Copy, Clone)]
pub enum TaskBlockControl {
    AwakeTick(usize),
    None,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Task {
    // Arch dependant context, don't handle this field in task, only use struct method when
    // interacting with it.
    pub context: TaskContext,
    // Task block control, define the reason the task is blocked.
    pub block_control: TaskBlockControl,
    // Fn ptr to task entry point, this must never return.
    pub func: fn() -> !,
    pid: u16,
    name: [u8; 16],
    // Task state, when creating a new task, use the new variant.
    pub state: TaskState,
    // Priority of a task, use an u8, u8 max size represent the higher level of priority.
    priority: u8,
}

impl Task {
    /// Return an Option, if the memory allocator cannot allocate the asked size for the task,
    /// return None, else return the task.
    fn init(name: &str, func: fn() -> !, priority: u8, size: usize) -> Option<Self> {
        // Copy bytes in name str to slice
        let name_b = name.as_bytes();
        let mut buf = [0u8; 16];
        let len = core::cmp::min(name_b.len(), 16 - 1);
        buf[..len].copy_from_slice(&name_b[..len]);
        // Allocate the asked size
        let mem_reg = mem_task_alloc(size);
        // If the allocator cannot allocate the asked size, return None and log error.
        if mem_reg.is_none() {
            log!(
                LogLevel::Error,
                "The size asked to create the task: {name} couldn't be allocate, abort task creation."
            );
            return None;
        }
        // Return new task
        Some(Task {
            // Allow expect use, we check the Option<> before, but if we can't get the memory
            // region behind it, fail-fast, we don't want a task with wrong mem reg or UB.
            #[allow(clippy::expect_used)]
            context: TaskContext::init(
                mem_reg.expect("Error: failed to get the task memory region"),
                func,
            ),
            block_control: TaskBlockControl::None,
            func,
            pid: 0,
            name: buf,
            state: TaskState::New,
            priority,
        })
    }

    /// Trigger context switch for the given task.
    fn context_switch(&self) {
        self.context.context_switch();
        // match self.state {
        //     TaskState::New => {
        //         self.context.context_switch();
        //     }
        //     TaskState::Running => {
        //         self.context.context_switch();
        //     }
        //     TaskState::Ready => {
        //         self.context.context_switch();
        //     }
        //     TaskState::Waiting => todo!(),
        //     TaskState::Terminated => todo!(),
        // }
    }

    fn context_save(&self, ra: usize, sp: usize) {
        self.context.context_save(ra, sp);
    }
}

/// Create a new task. And register it to the task list.
/// name: name of the task as &str.
/// state: next state of the task.
/// func: function pointer to the task entry point, the function must never return.
/// priority: the task priority, highest priority will be executed first and prioritized by the
/// scheduler.
/// size: the task size asked for RAM allocation.
pub fn task_create(name: &str, func: fn() -> !, priority: u8, size: usize) {
    let new_task = Task::init(name, func, priority, size);
    if let Some(task) = new_task {
        // Allow the use of expect to avoid casting non-utf8 char to str.
        #[allow(clippy::expect_used)]
        let name = str::from_utf8(&task.name).expect("Failed to cast bytes buffer to &str.");
        let updated_task_pid = task_list_add_task(task);
        log!(
            LogLevel::Info,
            "Successfully created task: {name} with pid: {}",
            updated_task_pid
        );
    } else {
        log!(
            LogLevel::Error,
            "Failed to create task: {name}, try reducing task size if possible."
        );
    }
}

/// Temporary function to trigger context switch on a given task
pub fn task_context_switch(task: &Task) {
    task.context_switch();
}

pub fn task_context_save(task: &Task, ra: usize, sp: usize) {
    task.context_save(ra, sp);
}

pub fn task_pid(task: &Task) -> u16 {
    task.pid
}

pub fn task_priority(task: &Task) -> u8 {
    task.priority
}

/// Create the idle task
pub fn task_idle_task() {
    let task_name: &str = "Idle task";
    let func: fn() -> ! = idle_task_fn;
    let priority: u8 = 0;
    let size: usize = 0x100;
    task_create(task_name, func, priority, size);
}

fn idle_task_fn() -> ! {
    loop {
        log!(LogLevel::Debug, "Idle task.");
        unsafe { enable_and_halt() };
    }
}
