#![no_std]
#![no_main]

// use core::ptr::{read_volatile, write_volatile};

extern crate alloc;
use alloc::vec;
use embedded_alloc::LlffHeap as Heap;
use panic_halt as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

struct MyCoolFatFSDriver;

const MEM_OFFSET: u32 = 0x4000;

#[derive(Debug)]
struct FatfsError {
    code: i32,
}

impl fatfs::IoError for FatfsError {
    fn is_interrupted(&self) -> bool {
        false
    }

    fn new_unexpected_eof_error() -> Self {
        FatfsError { code: -1 }
    }

    fn new_write_zero_error() -> Self {
        FatfsError { code: -1 }
    }
}

impl fatfs::IoBase for MyCoolFatFSDriver {
    type Error = FatfsError;
}

impl fatfs::Read for MyCoolFatFSDriver {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        Err(FatfsError { code: -1 })
    }
}

impl fatfs::Write for MyCoolFatFSDriver {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Err(FatfsError { code: -1 })
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Err(FatfsError { code: -1 })
    }
}

fn consume(_t: MyCoolFatFSDriver) {
    return;
}

#[riscv_rt::entry]
#[allow(static_mut_refs)]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let _a = 5 + 9;

    let b = Some(MyCoolFatFSDriver {});
    match b {
        Some(x) => consume(x),
        _ => (),
    };

    let vector = vec![1, 2, 3, 4, 5, 6];
    loop {}
}
