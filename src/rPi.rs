#[allow(dead_code)]
#[allow(unused_variables)]
extern crate mmap;

use std::mem;
use std::ptr;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use mmap::{MemoryMap, MapOption};
use std::os::unix::prelude::AsRawFd;

const BCM2708_PERI_BASE: u32 = 0x20000000;
const GPIO_BASE: u32 = BCM2708_PERI_BASE + 0x200000;
const O_SYNC: u32 = 1052672;
const MAP_SHARED: i32 = 0x0001;
enum Void {} // void type

struct Bcm2835Peripheral {
    addr_p: u32,
    mem_fd: i32,
    map: *mut Void,
    addr: *mut i32
}


fn main() {
    unsafe {
        let gpio = Bcm2835Peripheral { addr_p: GPIO_BASE, mem_fd: 0, map: mem::uninitialized(), addr: ptr::null_mut()};
        map_peripheral(gpio);
    }
}

fn map_peripheral(foo: Bcm2835Peripheral) {
    let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .mode(O_SYNC)
                    .open("/dev/mem")
                    .expect("Unable to open /dev/mem/, Are you root?");
    
    let map_opts = &[
        MapOption::MapNonStandardFlags(MAP_SHARED),
        MapOption::MapReadable,
        MapOption::MapWritable,
        MapOption::MapFd(file.as_raw_fd())
    ];
    
}
