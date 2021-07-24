use super::{PhysAddr, PhysPageNum};
use crate::config::MEMORY_END;
use alloc::vec::Vec;
use core::fmt::{self, Debug, Formatter};
use lazy_static::*;
use spin::Mutex;

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

#[derive(Debug)]
pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        self.current = l.0;
        self.end = r.0;
    }
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    // fn alloc(&mut self) -> Option<PhysPageNum> {
    //     println!("alloc");
    //     if let Some(ppn) = self.recycled.pop(){
    //         Some(ppn.into())
    //     }else {
    //         if self.current == self.end{
    //             None
    //         }else{
    //             self.current += 1;
    //             Some((self.current - 1).into())
    //         }
    //     }
    // }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else {
            if self.current == self.end {
                None
            } else {
                self.current += 1;
                Some((self.current - 1).into())
            }
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        if ppn >= self.current || self.recycled.iter().find(|&v|{
            *v == ppn
        }).is_some(){
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        self.recycled.push(ppn);
    }
}

////////////////////////////////////////////////////
// pub func，全局空间分配器
///////////////////////////////////////////////////
type FramAllocatorImpl = StackFrameAllocator;

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<FramAllocatorImpl> = Mutex::new(FramAllocatorImpl::new());
}

pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR.lock().init(
        PhysAddr::from(ekernel as usize).ceil(),
        PhysAddr::from(MEMORY_END).floor(),
    );
}

pub fn frame_alloc() -> Option<FrameTracker>{
    FRAME_ALLOCATOR.lock().alloc().map(|ppn|{
        FrameTracker::new(ppn)
    })
}

pub fn frame_dealloc(ppn:PhysPageNum) {
    FRAME_ALLOCATOR.lock().dealloc(ppn);
}

/////////////////////////////////////////////////////////
// Frame Tracker
/////////////////////////////////////////////////////////
pub struct FrameTracker{
    pub ppn: PhysPageNum
}

impl FrameTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        // page cleaning
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("FrameTracker: PPN={:#x}", self.ppn.0))
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

