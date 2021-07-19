use core::usize;

use crate::config::*;
use crate::task::TaskContext;
use crate::trap::TrapContext;
use core::slice;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}
impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    fn push_context(
        &self,
        trap_context: TrapContext,
        task_context: TaskContext,
    ) -> &'static mut TaskContext {
        unsafe {
            let trap_ptr =
                (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
            let task_ptr =
                (trap_ptr as usize - core::mem::size_of::<TaskContext>()) as *mut TaskContext;

            *trap_ptr = trap_context;
            *task_ptr = task_context;

            task_ptr.as_mut().unwrap()
        }
    }
}

// static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
//     data: [0; KERNEL_STACK_SIZE],
// }; MAX_APP_NUM];

// static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
//     data: [0; USER_STACK_SIZE],
// }; MAX_APP_NUM];

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [
    KernelStack { data: [0; KERNEL_STACK_SIZE], };
    MAX_APP_NUM
];

static USER_STACK: [UserStack; MAX_APP_NUM] = [
    UserStack { data: [0; USER_STACK_SIZE], };
    MAX_APP_NUM
];

pub fn load_app() {
    extern "C" {
        fn _num_app();
    }

    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };

    unsafe {
        llvm_asm!("fence.i" :::: "volatile");
    }

    for i in 0..num_app {
        let base_i = get_base_i(i);

        let src = unsafe {
            slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
        };

        let dst = unsafe { slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };

        dst.copy_from_slice(src);
    }
}

/************************************************************************
pub
***********************************************************************/

pub fn get_base_i(appid: usize) -> usize {
    APP_BASE_ADDRESS + appid * APP_SIZE_LIMIT
}

pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn init_app_cx(appid: usize) -> &'static TaskContext {
    KERNEL_STACK[appid].push_context(
        TrapContext::app_init_context(get_base_i(appid), USER_STACK[appid].get_sp()),
        TaskContext::goto_restore(),
    )
}
