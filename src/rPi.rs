const BCM2708_PERI_BASE: u32 = 0x20000000;
const GPIO_BASE: u32 = BCM2708_PERI_BASE + 0x200000;

enum Void { } // void type

struct bcm2835_peripheral {
    addr_p: u64,
    mem_fd: i32,
    map: *mut Void,
    addr: *mut i32
}
fn main() {
    let gpio = //continue later when you have time 
}
