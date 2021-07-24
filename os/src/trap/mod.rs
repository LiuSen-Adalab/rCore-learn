mod context;

pub use context::TrapContext;

use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::task;
use crate::{syscall::syscall, timer};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

global_asm!(include_str!("trap.S"));

pub fn init() {
    set_kernel_trap_entry();
}

pub fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

pub fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler() -> ! {
    let cx = task::current_trap_cx();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, core dumped.");
            panic!("[kernel] Cannot continue!")
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, core dumped.");
            panic!("[kernel] Cannot continue!")
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            timer::set_next_trigger();
            task::suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    trap_return()
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = task::current_user_token();

    // println!("trap_cx_ptr: {:#x}", trap_cx_ptr);
    // println!("user_satp: {:#x}", user_satp);
    extern "C" {
        fn __alltraps();
        fn __restore();
    }

    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    // println!("restore_va: {:#x}", restore_va);
    unsafe {
        llvm_asm!("fence.i" :::: "volatile");
        llvm_asm!("jr $0" :: "r"(restore_va), "{a0}"(trap_cx_ptr), "{a1}"(user_satp) :: "volatile");
    }
    panic!("Unreachable in back_to_user!");
}
