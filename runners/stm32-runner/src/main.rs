#![no_std]
#![no_main]

use bsp_stm32::target;
use core::hint::spin_loop;
use core::panic::PanicInfo;
use precision_replay_core::math::I64F64;

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
    target::init_stlink_vcp_usart2();
    emit_replay_result_line();

    loop {
        spin_loop();
    }
}

fn emit_replay_result_line() {
    let lhs = I64F64::from_bits(I64F64::SCALE);
    let rhs = I64F64::from_bits(I64F64::SCALE);
    let result_bits = (lhs + rhs).to_bits() as u128;

    target::write_stlink_vcp_usart2(b"precision-replay mvp-rc1 vector=math-add-001 result_bits=0x");
    write_hex_u128(result_bits);
    target::write_stlink_vcp_usart2(b"\r\n");
}

fn write_hex_u128(value: u128) {
    let mut shift = 124;
    while shift >= 4 {
        target::write_stlink_vcp_usart2(&[hex_digit((value >> shift) as u8)]);
        shift -= 4;
    }
    target::write_stlink_vcp_usart2(&[hex_digit(value as u8)]);
}

fn hex_digit(value: u8) -> u8 {
    match value & 0x0F {
        digit @ 0..=9 => b'0' + digit,
        digit => b'a' + (digit - 10),
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
