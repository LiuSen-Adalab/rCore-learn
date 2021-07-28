use alloc::sync::Arc;

use crate::loader::get_app_by_name;
use crate::mm;
use crate::task;
use crate::timer;

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    task::exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    task::suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    timer::get_time_ms() as isize
}

pub fn sys_getpid() -> isize {
    task::current_task().unwrap().pid.0 as isize
}

pub fn sys_fork() -> isize {
    let current_task = task::current_task().unwrap();
    let new_task = current_task.fork();

    let new_trap_cx = new_task.acquire_inner_lock().get_trap_cx();
    new_trap_cx.x[10] = 0;

    task::add_task(new_task.clone());

    new_task.pid.0 as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = task::current_user_token();
    let path = mm::translated_str(token, path);
    if let Some(data) = get_app_by_name(path.as_str()) {
        let task = task::current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = task::current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if inner
        .children
        .iter()
        .find(|tcb| pid == -1 || tcb.get_pid() == pid as usize)
        .is_none()
    {
        return -1;
    }

    let pair = inner.children.iter().enumerate().find(|(_, tcb)| {
        tcb.acquire_inner_lock().is_zombie() && (pid == -1 || pid as usize == tcb.get_pid())
    });

    if let Some((idx, _)) = pair{
        let child = inner.children.remove(idx);
        assert_eq!(Arc::strong_count(&child), 1);

        let exit_code = child.acquire_inner_lock().exit_code;
        *mm::translated_refmut(inner.memory_set.token(), exit_code_ptr) = exit_code;

        child.get_pid() as isize
    }else {
        -2
    }
}
