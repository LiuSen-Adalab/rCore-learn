use core::usize;
use crate::trap::trap_return;

#[repr(C)]
pub struct TaskContext {
    ra: usize,
    s: [usize; 12],
}

impl TaskContext {
    // pub fn goto_restore() -> Self {
    //     extern "C" {
    //         fn __restore();
    //     }
    //     TaskContext{
    //         ra: __restore as usize,
    //         s: [0;12],
    //     }
    // }

    pub fn goto_trap_return() -> Self{
        Self{
            ra: trap_return as usize,
            s: [0; 12],
        }
    }
}