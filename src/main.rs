#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

extern crate alloc;
use alloc::vec;
use embedded_alloc::LlffHeap as Heap;
use embedded_sdmmc::{TimeSource, Timestamp, VolumeManager};
use panic_halt as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

struct HansMMC;

const MEM_OFFSET: u32 = 0x4000;

impl embedded_sdmmc::blockdevice::BlockDevice for HansMMC {
    type Error = i32;

    fn read(
        &self,
        blocks: &mut [embedded_sdmmc::Block],
        start_block_idx: embedded_sdmmc::BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        let start_addr = MEM_OFFSET + start_block_idx.0 * 512;
        let mut i = 0;
        unsafe {
            for block in blocks {
                for test in &mut block.contents {
                    let addr = (start_addr + i) as *const u8;

                    *test = read_volatile(addr);
                    i += 1;
                }
            }
        }

        Ok(())
    }

    fn write(
        &self,
        blocks: &[embedded_sdmmc::Block],
        start_block_idx: embedded_sdmmc::BlockIdx,
    ) -> Result<(), Self::Error> {
        let start_addr = MEM_OFFSET + start_block_idx.0 * 512;
        let mut i = 0;
        unsafe {
            for block in blocks {
                for test in block.contents {
                    let addr = (start_addr + i) as *mut u8;

                    write_volatile(addr, test);
                    i += 1;
                }
            }
        }

        Ok(())
    }

    fn num_blocks(&self) -> Result<embedded_sdmmc::BlockCount, Self::Error> {
        Ok(embedded_sdmmc::BlockCount(1))
    }
}

impl TimeSource for HansMMC {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_day: 0,
            zero_indexed_month: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

fn consume(_t: HansMMC) {
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

    let b = Some(HansMMC {});
    match b {
        Some(x) => consume(x),
        _ => (),
    };

    let _vector = vec![1, 2, 3, 4, 5, 6];

    let mmc = HansMMC {};
    let mmc2 = HansMMC {};
    let test = VolumeManager::new(mmc, mmc2);
    loop {}
}
