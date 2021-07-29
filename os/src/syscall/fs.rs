use crate::mm::{self, UserBuffer};
use crate::task;
use crate::fs;


pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = task::current_user_token();
    let task = task::current_task().unwrap();
    let inner = task.acquire_inner_lock();

    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        drop(inner);
        file.write(UserBuffer::new(mm::translated_byte_buffer(token, buf, len))) as isize
    } else {
       return  -1;
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = task::current_user_token();
    let task = task::current_task().unwrap();
    let inner = task.acquire_inner_lock();

    if fd >= inner.fd_table.len(){
        return -1;
    }

    if let Some(file) = &inner.fd_table[fd]{
        let file = file.clone();
        drop(inner);
        file.read(UserBuffer::new(mm::translated_byte_buffer(token, buf, len))) as isize
    }else {
        return -1;
    }
}

pub fn sys_close(fd: usize) -> isize{
    let task = task::current_task().unwrap();
    let mut inner = task.acquire_inner_lock();

    if fd >= inner.fd_table.len(){
        return -1;
    }
    if inner.fd_table[fd].is_none(){
        return -1
    }

    inner.fd_table[fd].take();
    0
}

pub fn sys_pipe(pipe: *mut usize) -> isize{
    let task = task::current_task().unwrap();
    let token = task::current_user_token();
    let mut inner = task.acquire_inner_lock();

    let (pipe_read, pipe_write) = fs::make_pipe();
    let read_fd = inner.alloc_fd();
    inner.fd_table[read_fd] = Some(pipe_read);
    let write_fd = inner.alloc_fd();
    inner.fd_table[write_fd] = Some(pipe_write);
    *mm::translated_refmut(token, pipe) = read_fd;
    *mm::translated_refmut(token, unsafe { pipe.add(1) }) = write_fd;
    0
}