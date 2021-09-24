#![no_std]

extern crate alloc;

mod block_dev;
mod layout;
mod efs;
mod bitmap;
mod vfs;
mod block_cache;

pub use block_dev::BlockDevice;
pub use vfs::Inode;
pub use efs::EasyFileSystem;

use block_cache::get_block_cache;
use bitmap::Bitmap;
use layout::*;

pub const BLOCK_SZ: usize = 512;