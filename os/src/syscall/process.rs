use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::fs::{open_file, OpenFlags};
use crate::mm::{self, translated_ref};
use crate::mm::translated_str;
use crate::task::{self, current_task};
use crate::task::current_user_token;
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
    let new_pid = new_task.pid.0;

    let new_trap_cx = new_task.acquire_inner_lock().get_trap_cx();
    new_trap_cx.x[10] = 0;

    task::add_task(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const u8, mut args: *const usize) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    let mut args_vec: Vec<String> = Vec::new();
    loop {
        let arg_str_ptr = *translated_ref(token, args);
        if arg_str_ptr == 0 {
            break;
        }
        args_vec.push(translated_str(token, arg_str_ptr as *const u8));
        unsafe { args = args.add(1); }
    }
    if let Some(app_inode) = open_file(path.as_str(), OpenFlags::RDONLY) {
        let all_data = app_inode.read_all();
        let task = current_task().unwrap();
        let argc = args_vec.len();
        task.exec(all_data.as_slice(), args_vec);
        // return argc because cx.x[10] will be covered with it later
        argc as isize
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
