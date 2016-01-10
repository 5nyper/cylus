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
    mem_fd: std::fs::File,
    map: mmap::MemoryMap,
    addr: *mut u8
}

impl Drop for Bcm2835Peripheral {
    fn drop(&mut self) {
        println!("Unmapped Peripheral {:?}", self.map.data())
    }
}

fn main() {
    let mut gpio = Bcm2835Peripheral { addr_p: &GPIO_BASE, mem_fd: OpenOptions::new().create(true).open("temp.txt").unwrap(), map: MemoryMap::new(1024, &[]).unwrap(), addr: ptr::null_mut()};
    map_peripheral(&mut gpio);
    unmap_peripheral(gpio);
}

fn map_peripheral(ref mut foo: &mut Bcm2835Peripheral) {
    foo.mem_fd = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .mode(O_SYNC)
                    .open("/dev/mem")
                    .expect("unable to open /dev/mem, Are you root?");
    
    let map_opts = &[
        MapOption::MapNonStandardFlags(MAP_SHARED),
        MapOption::MapReadable,
        MapOption::MapWritable,
       // MapOption::MapAddr(foo.addr_p),
        MapOption::MapFd(foo.mem_fd.as_raw_fd())
    ];
    
   let mmap = match MemoryMap::new(BLOCK_SIZE, map_opts) {
     Ok(mmap) => mmap,
     Err(e) => panic!("ERR: {}", e)
    };
    foo.map = mmap;
    foo.addr = foo.map.data();
}

fn unmap_peripheral(foo: Bcm2835Peripheral) {
    drop(foo);
}

fn in_gpio(foo: &Bcm2835Peripheral, y: isize) {
  unsafe {
      let k = &foo.addr.offset(y/10); 
      **k &= !(7<<(((y)%10)*3));
  }
}

fn out_gpio(foo: &Bcm2835Peripheral, y:isize) {
  unsafe {
      let k = &foo.addr.offset(y/10); 
      **k |= (7<<(((y)%10)*3));
  }
}

fn set_gpio_alt(foo: &Bcm2835Peripheral, y:isize, a:u8) {
    unsafe {
        let k = &foo.addr.offset(y/10);
        **k |= match a {
            a if a<=3 => a+4,
            4         => 3,
            _         => 2,
        } << ((y % 10) * 3);
    }
}

fn set_gpio(foo: &Bcm2835Peripheral, val: *mut u8) {
    foo.addr.offset(7) = val;                            //Invalid left-hand side assignment
}
