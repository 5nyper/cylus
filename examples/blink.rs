#![allow(deprecated)]
extern crate cylus;

use cylus::Cylus;

fn main() {
    println!("Starting");
    let gpio = Cylus::new(24);
    unsafe {
        for _ in 1..10 {
          println!("{}", gpio.read());
          gpio.high();
          std::thread::sleep_ms(1000);
          println!("{}", gpio.read());
          gpio.low();
          std::thread::sleep_ms(1000);
        }
    }
}
