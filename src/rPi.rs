use std::mem;
use std::ptr;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;

const BCM2708_PERI_BASE: u32 = 0x20000000;
const GPIO_BASE: u32 = BCM2708_PERI_BASE + 0x200000;
const O_SYNC: u32 = 1052672;

enum Void {} // void type

struct bcm2835_peripheral {
    addr_p: u32,
    mem_fd: i32,
    map: *mut Void,
    addr: *mut i32
}


fn main() {
    unsafe {let gpio = bcm2835_peripheral { addr_p: GPIO_BASE, mem_fd: 0, map: mem::uninitialized(), addr: ptr::null_mut()};}
}

fn map_peripheral(foo: bcm2835_peripheral) {
    let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .mode(O_SYNC)
                    .open("/dev/mem");
    match file {
        Ok(file) => println!("Success"),
        Err(e) => println!("Could not open /dev/mem, are you root?")
    }
    //Finish when you have time
}
