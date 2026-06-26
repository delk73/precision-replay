#![no_std]
#![no_main]

use core::hint::spin_loop;
use core::panic::PanicInfo;

#[link_section = ".vector_table.reset_vector"]
#[used]
static RESET_VECTOR: extern "C" fn() -> ! = reset_handler;

#[link_section = ".vector_table.exceptions"]
#[used]
static EXCEPTION_VECTORS: [extern "C" fn() -> !; 14] = [default_handler; 14];

#[no_mangle]
pub extern "C" fn reset_handler() -> ! {
    firmware_main()
}

fn firmware_main() -> ! {
    loop {
        spin_loop();
    }
}

#[no_mangle]
pub extern "C" fn default_handler() -> ! {
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
