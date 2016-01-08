extern crate mmap;

use std::ptr;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use mmap::{MemoryMap, MapOption};
use std::os::unix::prelude::AsRawFd;

const BCM2708_PERI_BASE: u32 = 0x20000000;
const GPIO_BASE: u8 = (BCM2708_PERI_BASE + 0x200000) as u8;
const O_SYNC: u32 = 1052672;
const MAP_SHARED: i32 = 0x0001;
const BLOCK_SIZE: usize = (4*1024);

#[allow(dead_code)]
struct Bcm2835Peripheral {
    addr_p: *const u8,
    mem_fd: i32,
    map: mmap::MemoryMap,
    addr: *mut u8
}


fn main() {
    let gpio = Bcm2835Peripheral { addr_p: &GPIO_BASE, mem_fd: 0, map: MemoryMap::new(1024, &[]).unwrap(), addr: ptr::null_mut()};
    map_peripheral(gpio);
}

fn map_peripheral(mut foo: Bcm2835Peripheral) {
    let file = match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .mode(O_SYNC)
                    .open("/dev/mem") {
      Ok(file) => file,
      Err(_e) => panic!("Unable to open /dev/mem, Are you root?")
    };
    
    let map_opts = &[
        MapOption::MapNonStandardFlags(MAP_SHARED),
        MapOption::MapReadable,
        MapOption::MapWritable,
       // MapOption::MapAddr(foo.addr_p),
        MapOption::MapFd(file.as_raw_fd())
    ];
    
   let mmap = match MemoryMap::new(BLOCK_SIZE, map_opts) {
     Ok(mmap) => mmap,
     Err(e) => panic!("ERR: {}", e)
    };
    foo.map = mmap;
    foo.addr = foo.map.data();
    println!("{:?}", foo.map.data());
}
