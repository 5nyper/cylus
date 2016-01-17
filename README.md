# Cylus
######Library for RaspberryPi GPIO

#Example

```rust
#![allow(deprecated)]
extern crate cylus;

use cylus::Cylus;

fn main() {
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
```

#TODO
- [x] get Rust working on raspberrypi
- [x] Read BCM2835 ARM Peripherals manual
- [x] make it work

#Converting into a library

- [x] simple digital GPIO input/output
- [ ] add Pwm functionality
