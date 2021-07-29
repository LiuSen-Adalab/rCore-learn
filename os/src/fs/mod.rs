mod pipe;
mod stdio;

use crate::mm::UserBuffer;
pub use stdio::{Stdout, Stdin};
pub use pipe::{Pipe, make_pipe};


pub trait File: Send + Sync {
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
}