mod context;
mod manager;
mod pid;
mod process;
mod switch;
mod task;

use crate::fs::{open_file, OpenFlags};
use alloc::sync::Arc;
use lazy_static::*;
use task::TaskControlBlock;

pub use context::TaskContext;
pub use process::{
    current_task, current_trap_cx, current_user_token, run_tasks, schedule, take_current_task,
};
pub use manager::add_task;
pub use pid::{pid_alloc};

use self::task::TaskStatus;


pub fn suspend_current_and_run_next() {
    let current_task = take_current_task().unwrap();
    let mut current_inner = current_task.acquire_inner_lock();
    let task_cx_ptr2 = current_inner.get_task_cx_ptr2();
    current_inner.task_status = TaskStatus::Ready;
    drop(current_inner);
    add_task(current_task);

    schedule(task_cx_ptr2);
}

pub fn exit_current_and_run_next(exit_code: i32) {
    let current_task  = take_current_task().unwrap();
    let mut current_inner = current_task.acquire_inner_lock();

    current_inner.task_status = TaskStatus::Zombie;
    current_inner.exit_code = exit_code;

    {
        let mut initproc_inner = INITPROC.acquire_inner_lock();
        for child in current_inner.children.iter(){
            child.acquire_inner_lock().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }

    current_inner.children.clear();
    current_inner.memory_set.recycle_data_pages();

    drop(current_inner);
    drop(current_task);

    let _unused: usize = 0;
    schedule(&_unused as *const _);
}

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new({
        let inode = open_file("initproc", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice())
    });
}

pub fn add_initproc() {
    manager::add_task(INITPROC.clone());
}
