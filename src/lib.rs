#![allow(dead_code)]
#![feature(core_intrinsics)]
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

pub struct Cylus {
    addr_p: *const usize,
    mem_fd: ::std::fs::File,
    map: ::mmap::MemoryMap,
    addr: *mut usize,
    pin: u32 
}

impl Cylus {
    pub fn new(x: u32) -> Cylus {
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
      
        let result = Cylus {
            addr_p: &GPIO_BASE,
            mem_fd: mem_file,
            addr: mmap.data() as *mut usize,  //switch order to avoid error of moved value `mmap`
            map: mmap,
            pin: x
        };
        //setting mode for GPIO
        unsafe {
            let addr = result.addr.offset((x/10) as isize);
            let mut a = volatile_load(addr); 
            a &= !(7 << (((x) % 10) * 3));
            a |= 1 << (((x) % 10) * 3);
            volatile_store(addr, a);
        }
        result
    }
    pub fn set_alt(&self, a: usize) {
        let y = self.pin as isize;
        unsafe {
            let addr = self.addr.offset(y/10);
            let mut k = volatile_load(addr);
            k |= match a {
                a if a <= 3 => a + 4,
                4 => 3,
                _ => 2,
            } << ((y % 10) * 3);
            volatile_store(addr, k); 
       }
    }

    pub fn high(&self) { 
        let val = 1usize << self.pin;
        unsafe { volatile_store(self.addr.offset(7), val); }
    }
    pub fn low(&self) { 
        let val = 1usize << self.pin;
        unsafe { volatile_store(self.addr.offset(10), val); }
    }
    pub fn read(&self) -> usize {
        unsafe {
            let addr = self.addr.offset(13);
            let mut k = volatile_load(addr);
            k &= 1 << self.pin as isize;
            volatile_store(addr, k);
            return k
        }
    }
}

impl Drop for Cylus {
    fn drop(&mut self) {
        println!("Unmapped Peripheral {:?}", self.map.data())
    }
}
