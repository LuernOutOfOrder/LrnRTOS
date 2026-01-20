use crate::{arch::task_context::TaskContext, log, logs::LogLevel, mem::mem_task_alloc};

#[repr(u8)]
enum TaskState {
    New,
    Running,
    Ready,
    Waiting,
    Terminated,
}

#[repr(C)]
struct Task {
    context: TaskContext,
    func: fn() -> !,
    pid: u16,
    name: [u8; 16],
    state: TaskState,
    priority: u8,
}

impl Task {
    /// Create a new task.
    /// name: name of the task as bytes buff.
    /// state: next state of the task.
    /// func: function pointer to the task entry point, the function must never return.
    /// priority: the task priority, highest priority will be executed first and prioritized by the
    /// scheduler.
    /// size: the task size asked for RAM allocation.
    fn init(name: &str, func: fn() -> !, priority: u8, size: usize) -> Option<Self> {
        let name_b = name.as_bytes();
        let mut buf = [0u8; 16];
        let len = core::cmp::min(name_b.len(), 16 - 1);
        buf[..len].copy_from_slice(&name_b[..len]);
        let mem_reg = mem_task_alloc(size);
        if mem_reg.is_none() {
            log!(
                LogLevel::Error,
                "The size asked to create the task: {name} couldn't be allocate, abort task creation."
            );
            return None;
        }
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

pub fn task_create(name: &str, func: fn() -> !, priority: u8, size: usize) {
    let new_task = Task::init(name, func, priority, size);
    if let Some(task) = new_task {
        let name = str::from_utf8(&task.name).unwrap();
        log!(LogLevel::Info, "Successfully created task: {name}");
    }
}
