use core::cell::RefCell;

use crate::trap::TrapContext;

use super::{
    manager,
    switch::__switch,
    task::{TaskControlBlock, TaskStatus},
};
use alloc::sync::Arc;
use lazy_static::*;
pub struct Processor {
    inner: RefCell<ProcessInner>,
}

unsafe impl Sync for Processor {}
struct ProcessInner {
    current: Option<Arc<TaskControlBlock>>,
    idle_task_cx_ptr: usize,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            inner: RefCell::new(ProcessInner {
                current: None,
                idle_task_cx_ptr: 0,
            }),
        }
    }

    pub fn run(&self) {
        loop {
            if let Some(task) = manager::fetch_app() {
                let mut inner = task.acquire_inner_lock();
                let next_task_cx_ptr2 = inner.get_task_cx_ptr2();
                inner.task_status = TaskStatus::Running;
                drop(inner);

                self.inner.borrow_mut().current = Some(task);

                unsafe {
                    __switch(self.get_idle_task_cx_ptr2(), next_task_cx_ptr2);
                }
            }
        }
    }

    pub fn take_current(&self) -> Option<Arc<TaskControlBlock>> {
        self.inner.borrow_mut().current.take()
    }

    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.inner
            .borrow()
            .current
            .as_ref()
            .map(|task| Arc::clone(task))
    }

    pub fn get_idle_task_cx_ptr2(&self) -> *const usize {
        &self.inner.borrow().idle_task_cx_ptr as *const usize
    }
}

lazy_static! {
    pub static ref PROCESSOR: Processor = Processor::new();
}

pub fn run_tasks() {
    PROCESSOR.run();
}

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.take_current()
}

pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.current()
}

pub fn current_user_token() -> usize {
    current_task().unwrap().acquire_inner_lock().get_user_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task().unwrap().acquire_inner_lock().get_trap_cx()
}

pub fn schedule(switched_task_ptr2: *const usize) {
    let idle_task_cx_ptr2 = PROCESSOR.get_idle_task_cx_ptr2();
    unsafe {
        __switch(switched_task_ptr2, idle_task_cx_ptr2);
    }
}
