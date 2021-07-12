#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

mod console;
mod lang_items;
mod sbi;

global_asm!(include_str!("entry.asm"));

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
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();

    println!("\x1b[31mhello world\x1b[0m");
    println!(".text [{:#x}, {:#x}]", stext as usize, etext as usize);
    println!(".rodata [{:#x}, {:#x}]", srodata as usize, erodata as usize);
    println!(".data [{:#x}, {:#x}]", sdata as usize, edata as usize);
    println!(
        ".boot_stack [{:#x}, {:#x}]",
        boot_stack as usize, boot_stack_top as usize
    );
    println!(".bss [{:#x}, {:#x}]", sbss as usize, ebss as usize);

    panic!("shutdown the machine!")
}
