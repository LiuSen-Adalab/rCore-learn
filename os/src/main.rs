#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod batch;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/** .bss clear */
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/*******************************************************************
_start program entry
******************************************************************/
// #[no_mangle]
// extern "C" fn _start() {
//     println!("hello");
//     shutdown();
// }

#[no_mangle]
pub fn rust_main() {
    clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    batch::init();
    batch::run_next_app();
}

