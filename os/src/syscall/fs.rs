const FD_STDOUT: usize = 1;
use crate::task;
use crate::mm::translated_byte_buffer;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let token = task::current_user_token();
            let buffers = translated_byte_buffer(token, buf, len);
            for buffer in buffers{
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        },
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}
