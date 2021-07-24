mod heap_allocator;
mod frame_allocator;
mod address;
pub mod page_table;
pub mod memory_set;


pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum, VPNRange, StepByOne};
pub use memory_set::{MemorySet, KERNEL_SPACE, MapPermission, MapArea};
pub use page_table::{PageTableEntry,PageTable, translated_byte_buffer, PTEFlags};
pub use frame_allocator::{FrameTracker, frame_alloc};

pub fn init(){
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}