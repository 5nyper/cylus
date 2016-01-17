//volatile_store is the same as `*ptr = value;` (except that the optimiser won't touch it)
#![allow(dead_code)]
extern crate mmap;
extern crate core;

use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use mmap::{MemoryMap, MapOption};
use std::os::unix::prelude::AsRawFd;
use self::core::intrinsics::{volatile_load, volatile_store};

const BCM2708_PERI_BASE: usize = 0x3F000000;
const GPIO_BASE: usize = BCM2708_PERI_BASE + 0x200000;
const O_SYNC: u32 = 1052672;
const MAP_SHARED: i32 = 0x0001;
const BLOCK_SIZE: usize = 4 * 1024;

pub struct Bcm2835Peripheral {
    pub addr_p: *const usize,
    pub mem_fd: ::std::fs::File,
    pub map: ::mmap::MemoryMap,
    pub addr: *mut usize
}

impl Bcm2835Peripheral {
    pub fn new() -> Bcm2835Peripheral {
        let mem_file = OpenOptions::new()
            .read(true)
            .write(true)
            .mode(O_SYNC)
            .open("/dev/mem")
            .expect("unable to open /dev/mem, Are you root?");

        let map_opts = &[
            MapOption::MapNonStandardFlags(MAP_SHARED),
            MapOption::MapReadable,
            MapOption::MapWritable,
            MapOption::MapOffset(GPIO_BASE),
            MapOption::MapFd(mem_file.as_raw_fd())
        ];

        let mmap = match MemoryMap::new(BLOCK_SIZE, map_opts) {
            Ok(mmap) => mmap,
            Err(e) => panic!("ERR: {}", e)
        };
      
        Bcm2835Peripheral {
            addr_p: &GPIO_BASE,
            mem_fd: mem_file,
            addr: mmap.data() as *mut usize,  //switch order to avoid error of moved value `mmap`
            map: mmap,
        }
    }

    pub unsafe fn out(&self, y: isize) {
        let addr = self.addr.offset(y/10);
        let mut a = volatile_load(addr); 
        a &= !(7 << (((y) % 10) * 3));
        a |= 1 << (((y) % 10) * 3);
        volatile_store(addr, a) 
    }

    pub unsafe fn set_alt(&self, y: isize, a: usize) {
        let addr = self.addr.offset(y/10);
        let mut k = volatile_load(addr);
        k |= match a {
            a if a <= 3 => a + 4,
            4 => 3,
            _ => 2,
        } << ((y % 10) * 3);
        volatile_store(addr, k) 
    }

    pub unsafe fn set(&self, val: usize) { 
        volatile_store(self.addr.offset(7), val);
    }
    pub unsafe fn clear(&self, val: usize) { 
        volatile_store(self.addr.offset(10), val);
    }
    pub unsafe fn read(&self, y: isize) -> usize {
        let addr = self.addr.offset(13);
        let mut k = volatile_load(addr);
        k &= 1 << y;
        volatile_store(addr, k);
        return k;
    }
}

impl Drop for Bcm2835Peripheral {
    fn drop(&mut self) {
        println!("Unmapped Peripheral {:?}", self.map.data())
    }
}
