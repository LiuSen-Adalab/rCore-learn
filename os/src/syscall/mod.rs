const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize= 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_READ: usize = 63;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_DUP: usize = 24;
const SYSCALL_OPEN: usize = 56;

mod fs;
mod process;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_DUP=> fs::sys_dup(args[0]),
        SYSCALL_READ => fs::sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL_OPEN => fs::sys_open(args[0] as *const u8, args[1] as u32),
        SYSCALL_WRITE => fs::sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => process::sys_exit(args[0] as i32),
        SYSCALL_YIELD => process::sys_yield(),
        SYSCALL_GET_TIME => process::sys_get_time(),
        SYSCALL_GETPID => process::sys_getpid(),
        SYSCALL_FORK => process::sys_fork(),
        SYSCALL_EXEC => process::sys_exec(args[0] as *const u8, args[1] as *const usize),
        SYSCALL_WAITPID => process::sys_waitpid(args[0] as isize, args[1] as *mut i32),
        SYSCALL_CLOSE => fs::sys_close(args[0]),
        SYSCALL_PIPE => fs::sys_pipe(args[0] as *mut usize), 
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

