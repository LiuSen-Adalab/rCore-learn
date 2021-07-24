mod context;
mod switch;
mod task;

use crate::loader;
use crate::loader::get_num_app;
use crate::trap::TrapContext;
use alloc::vec::Vec;
use core::{cell::RefCell, usize};
use lazy_static::*;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

use self::switch::__switch;
pub struct TaskManager {
    num_app: usize,
    inner: RefCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>,
    current_task: usize,
}

unsafe impl Sync for TaskManager {}


lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks: Vec<TaskControlBlock> = Vec::new();

        for i in 0..num_app {
            tasks.push(TaskControlBlock::new(loader::get_app_data(i), i));
        }

        TaskManager{
            num_app,
            inner: RefCell::new(TaskManagerInner{
                tasks,
                current_task:0,
            })
        }
    };
}

impl TaskManager {
    fn run_first_task(&self) {
        self.inner.borrow_mut().tasks[0].task_status = TaskStatus::Running;
        let ptr_ptr = self.inner.borrow_mut().tasks[0].get_task_cx_ptr2();
        let _unuse: usize = 0;
        unsafe {
            __switch(&_unuse as *const _, ptr_ptr);
        }
    }

    fn mark_current_suspend(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.borrow();
        let current = inner.current_task;

        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;

            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;

            let current_task_cx_ptr2 = inner.tasks[current].get_task_cx_ptr2();
            let next_task_cx_ptr2 = inner.tasks[next].get_task_cx_ptr2();

            core::mem::drop(inner);

            unsafe { __switch(current_task_cx_ptr2, next_task_cx_ptr2) }
        } else {
            panic!("All applications commpleted!");
        }
    }

    fn get_current_token(&self) -> usize {
        let inner = self.inner.borrow();
        let current = inner.current_task;
        inner.tasks[current].get_user_token()
    }


    fn get_current_trap_cx(&self) -> &mut TrapContext {
        let inner = self.inner.borrow();
        let current = inner.current_task;
        inner.tasks[current].get_trap_cx()
    }
}

/*****************************************
pub functions
*****************************************/
pub(crate) fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspend();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn current_user_token() -> usize {
    TASK_MANAGER.get_current_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}
/***********************************************************/

fn mark_current_suspend() {
    TASK_MANAGER.mark_current_suspend();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}
