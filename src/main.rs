#![feature(core_intrinsics)]
#![allow(deprecated)]
extern crate mmap;

#[path="pi.rs"]
mod pi;

fn main() {
    println!("Starting...");
    let gpio = pi::Bcm2835Peripheral::new();
    unsafe {
        gpio.out(4);
        for _ in 1..10 {
          println!("{}", gpio.read(4));
          gpio.set(1usize << 4);
          std::thread::sleep_ms(1000);
          println!("{}", gpio.read(4));
          gpio.clear(1usize << 4);
          std::thread::sleep_ms(1000);
        }
    }
}
