extern crate mmap;

use std::ptr;
use std::fs::OpenOptions;
use mmap::{MemoryMap, MapOption};

#[path="pi.rs"]
mod Pi;

fn main() {
    let mut gpio = Pi::Bcm2835Peripheral { 
                    addr_p: &Pi::GPIO_BASE, 
                    mem_fd: OpenOptions::new().create(true).open("temp.txt").unwrap(), 
                    map: MemoryMap::new(1024, &[]).unwrap(), 
                    addr: ptr::null_mut()
    };
    gpio.map_peripheral();
    unsafe {
        gpio.in_gpio(4);
        gpio.out_gpio(4);

        loop {
          gpio.set_gpio(1u8 << 4);
          std::thread::sleep_ms(1000);

          gpio.clear_gpio(1u8 << 4);
          std::thread::sleep_ms(1000);
      }
  }
}
