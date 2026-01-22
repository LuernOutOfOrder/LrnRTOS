/*
File info: Task management. Handle all task logic.

Test coverage: Basic task creation and test memory region.

Tested:
- task_create function.

Not tested:
- All context, fn ptr, state and priority.

Reasons:
- Not really testable yet. Will be easier once there'll be a scheduler.

Tests files:
- 'src/tests/task/mod.rs'
*/

use list::task_list_add_task;

use crate::{arch::task_context::TaskContext, log, logs::LogLevel, mem::mem_task_alloc};

pub mod list;

// Enum representing all state of a task.
#[repr(u8)]
// Allow unused for now because this issue doesn't need to handle all task state
#[allow(unused)]
enum TaskState {
    New,
    Running,
    Ready,
    Waiting,
    Terminated,
}

#[repr(C)]
struct Task {
    // Arch dependant context, don't handle this field in task, only use struct method when
    // interacting with it.
    context: TaskContext,
    // Fn ptr to task entry point, this must never return.
    func: fn() -> !,
    pid: u16,
    name: [u8; 16],
    // Task state, when creating a new task, use the new variant.
    state: TaskState,
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
            context: TaskContext::init(mem_reg.unwrap()),
            func,
            pid: 0,
            name: buf,
            state: TaskState::New,
            priority,
        })
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
        let name = str::from_utf8(&task.name).unwrap();
        log!(LogLevel::Info, "Successfully created task: {name}");
        task_list_add_task(task);
    } else {
        log!(
            LogLevel::Error,
            "Failed to create task: {name}, try reducing task size if possible."
        );
    }
}
