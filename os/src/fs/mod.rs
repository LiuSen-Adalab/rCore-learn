mod pipe;
mod stdio;
mod inode;

use crate::mm::UserBuffer;
pub use stdio::{Stdout, Stdin};
pub use pipe::{Pipe, make_pipe};
pub use inode::{OSInode, open_file, OpenFlags, list_apps};

pub trait File: Send + Sync {
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
}