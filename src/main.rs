#![feature(core_intrinsics)]
#![allow(deprecated)]
extern crate mmap;

#[path="pi.rs"]
mod pi;

fn main() {
    let gpio = pi::Bcm2835Peripheral::new();
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
