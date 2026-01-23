/*
File info: Task list handling. Where task are saved.

Test coverage: All basic implementation and check if the list is correctly updated when adding a new task.

Tested:
- TaskList structure methods.

Not tested:

Reasons:

Tests files:
- 'src/tests/task/list.rs'
*/

use core::cell::UnsafeCell;

use crate::{config::TASK_LIST_MAX_SIZE, log, logs::LogLevel};

use super::Task;

pub struct TaskList {
    list: [UnsafeCell<Option<Task>>; TASK_LIST_MAX_SIZE],
    last_pid: u16,
    size: u8,
}

impl TaskList {
    pub const fn init() -> Self {
        TaskList {
            list: [const { UnsafeCell::new(None) }; TASK_LIST_MAX_SIZE],
            last_pid: 0,
            size: 0,
        }
    }

    fn add_task(&mut self, new_task: Task) {
        // Check possible overflow, abort if self.size == TASK_LIST_MAX_SIZE
        if self.size as usize == TASK_LIST_MAX_SIZE {
            log!(LogLevel::Warn, "Task list is full, ignore adding new task.");
            return;
        }
        // Iter over the list to add new task
        for i in 0..TASK_LIST_MAX_SIZE {
            let task = unsafe { &*self.list[i].get() };
            if task.is_none() {
                // Increment last_pid by 1 to use as new_task.pid
                self.last_pid += 1;
                // Update new_task pid
                let mut update_task = new_task;
                update_task.pid = self.last_pid;
                // Push new task to the list
                unsafe { *self.list[i].get() = Some(update_task) };
                // Increment current list size by 1
                self.size += 1;
                break;
            }
        }
    }

    pub fn get_task(&mut self, pid: u16) -> Option<&mut Task> {
        for i in 0..TASK_LIST_MAX_SIZE {
            let task = unsafe { (*self.list[i].get()).as_mut() };
            if let Some(is_task) = task
                && is_task.pid == pid
            {
                return Some(is_task);
            }
        }
        None
    }
}

pub static mut TASK_LIST: TaskList = TaskList::init();

// Allow private_interfaces because we want don't want this function to handle the task, it's just
// a public API wrapping the TaskList::add_task function
#[allow(private_interfaces)]
/// Add new task to the TASK_LIST static.
pub fn task_list_add_task(new_task: Task) {
    // Allow static mut refs for now, kernel only run in monocore
    #[allow(static_mut_refs)]
    unsafe {
        TASK_LIST.add_task(new_task)
    };
}

pub fn task_list_size() -> u8 {
    unsafe { TASK_LIST.size }
}

pub fn task_list_get_task_by_pid<'a>(pid: u16) -> Option<&'a mut Task> {
    // Allow static mut refs for now, kernel only run in monocore
    #[allow(static_mut_refs)]
    unsafe {
        TASK_LIST.get_task(pid)
    }
}
