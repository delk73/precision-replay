#![no_std]
#![no_main]

use core::hint::spin_loop;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {
        spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        spin_loop();
    }
}
