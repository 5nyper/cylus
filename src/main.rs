#![feature(core_intrinsics)]
#![allow(deprecated)]
extern crate mmap;

use std::ptr;
use std::fs::OpenOptions;
use mmap::MemoryMap;

#[path="pi.rs"]
mod pi;

fn main() {
    let mut gpio = pi::Bcm2835Peripheral { 
                    addr_p: &pi::GPIO_BASE, 
                    mem_fd: OpenOptions::new().create(true).open("temp.txt").unwrap(), 
                    map: MemoryMap::new(1024, &[]).unwrap(), 
                    addr: ptr::null_mut()
    };
    gpio.map_peripheral();
    unsafe {
        gpio.in_gpio(4);
        gpio.out_gpio(4);

        loop {
          gpio.set_gpio(1u32 << 4);
          std::thread::sleep_ms(1000);

          gpio.clear_gpio(1u32 << 4);
          std::thread::sleep_ms(1000);
        }
    }
}
