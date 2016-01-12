extern crate mmap;
extern crate core;

use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use mmap::{MemoryMap,MapOption};
use std::os::unix::prelude::AsRawFd;
use self::core::intrinsics::{volatile_load, volatile_store};

const BCM2708_PERI_BASE: u32 = 0x20000000;
pub const GPIO_BASE: u8 = (BCM2708_PERI_BASE + 0x200000) as u8;
const O_SYNC: u32 = 1052672;
const MAP_SHARED: i32 = 0x0001;
const BLOCK_SIZE: usize = (4 * 1024);

#[allow(dead_code)]
pub struct Bcm2835Peripheral {
    pub addr_p: *const u8,
    pub mem_fd: ::std::fs::File,
    pub map: ::mmap::MemoryMap,
    pub addr: *mut u32
}

impl Bcm2835Peripheral {
    pub fn map_peripheral( & mut self) {
        self.mem_fd = OpenOptions::new()
            .read(true)
            .write(true)
            .mode(O_SYNC)
            .open("/dev/mem")
            .expect("unable to open /dev/mem, Are you root?");

        let map_opts = & [
            MapOption::MapNonStandardFlags(MAP_SHARED),
            MapOption::MapReadable,
            MapOption::MapWritable,
            // MapOption::MapAddr(self.addr_p),
            MapOption::MapFd(self.mem_fd.as_raw_fd())
        ];

        let mmap = match MemoryMap::new(BLOCK_SIZE, map_opts) {
            Ok(mmap) => mmap,
            Err(e) => panic!("ERR: {}", e)
        };
        self.map = mmap;
        self.addr = self.map.data() as *mut u32;
    }

    pub fn unmap_peripheral(self) {
        drop(self);
    }

    pub unsafe fn in_gpio(&self, y: isize) {
        let mut k = volatile_load(self.addr.offset(y / 10)); 
        k &= !(7 << (((y) % 10) * 3));
        volatile_store(self.addr.offset(y / 10), k) 
    }

    pub unsafe fn out_gpio(&self, y: isize) {
        let mut k = volatile_load(self.addr.offset(y / 10)); 
        k |= (7 << (((y) % 10) * 3));
        volatile_store(self.addr.offset(y / 10), k) 
    }

    pub unsafe fn set_gpio_alt(&self, y: isize, a: u32) {
        let mut k = volatile_load(self.addr.offset(y / 10));  
        k |= match a {
            a if a <= 3 => a + 4,
            4 => 3,
            _ => 2,
        } << ((y % 10) * 3);
        volatile_store(self.addr.offset(y / 10), k) 
    }

    pub unsafe fn set_gpio(&self, val: u32) { 
        volatile_store(self.addr.offset(7), val);
    }
    pub unsafe fn clear_gpio(&self, val: u32) { 
        volatile_store(self.addr.offset(10), val);
    }
}

impl Drop for Bcm2835Peripheral {
    fn drop(&mut self) {
        println!("Unmapped Peripheral {:?}", self.map.data())
    }
}
