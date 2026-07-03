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
    settle_after_usart_init();
    target::init_raw_adc1_pa0();
    emit_replay_result_line();
    emit_raw_adc_witness_stream();
}

fn settle_after_usart_init() {
    for _ in 0..160_000 {
        spin_loop();
    }
}

fn emit_replay_result_line() {
    let lhs = I64F64::from_bits(I64F64::SCALE);
    let rhs = I64F64::from_bits(I64F64::SCALE);
    let result_bits = (lhs + rhs).to_bits() as u128;

    target::write_stlink_vcp_usart2(b"\r\n");
    target::write_stlink_vcp_usart2(
        b"precision-replay v0.1.0-rc1 vector=math-add-001 result_bits=0x",
    );
    write_hex_u128(result_bits);
    target::write_stlink_vcp_usart2(b"\r\n");
}

fn emit_raw_adc_witness_stream() -> ! {
    let mut sample_index = 0u32;

    loop {
        let raw_adc = target::read_raw_adc1_in0_blocking();

        target::write_stlink_vcp_usart2(
            b"precision-replay v0.1.0-rc1 witness=raw-adc sample_index=",
        );
        write_dec_u32(sample_index);
        target::write_stlink_vcp_usart2(b" raw_adc=0x");
        write_hex_u16(raw_adc);
        target::write_stlink_vcp_usart2(b" timing_claim=best_effort_polling_uart_stream\r\n");

        sample_index = sample_index.wrapping_add(1);
    }
}

fn write_hex_u128(value: u128) {
    let mut shift = 124;
    while shift >= 4 {
        target::write_stlink_vcp_usart2(&[hex_digit((value >> shift) as u8)]);
        shift -= 4;
    }
    target::write_stlink_vcp_usart2(&[hex_digit(value as u8)]);
}

fn write_hex_u16(value: u16) {
    let mut shift = 12;
    while shift >= 4 {
        target::write_stlink_vcp_usart2(&[hex_digit((value >> shift) as u8)]);
        shift -= 4;
    }
    target::write_stlink_vcp_usart2(&[hex_digit(value as u8)]);
}

fn write_dec_u32(mut value: u32) {
    let mut digits = [0u8; 10];
    let mut len = 0;

    loop {
        digits[len] = b'0' + (value % 10) as u8;
        len += 1;
        value /= 10;

        if value == 0 {
            break;
        }
    }

    while len > 0 {
        len -= 1;
        target::write_stlink_vcp_usart2(&[digits[len]]);
    }
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
