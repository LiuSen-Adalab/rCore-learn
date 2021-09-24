//! VirtIO guest drivers.

#![no_std]
#![deny(unused_must_use, missing_docs)]
#![allow(clippy::identity_op)]
#![allow(dead_code)]

extern crate log;

mod header;
mod queue;

mod blk;
mod gpu;

pub type Result<T = ()> = core::result::Result<T, Error>;

const PAGE_SIZE: usize = 0x1000;

/// The error type of VirtIO drivers.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// The buffer is too small.
    BufferTooSmall,
    /// The device is not ready.
    NotReady,
    /// The queue is already in use.
    AlreadyUsed,
    /// Invalid parameter.
    InvalidParam,
    /// Failed to alloc DMA memory.
    DmaError,
    /// I/O Error
    IoError,
}
