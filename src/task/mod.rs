use crate::arch::task_context::TaskContext;

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
    pid: u16,
    state: TaskState,
    name: [u8; 16],
}
