#![no_std]
#![no_main]

use bl602_hal::{basic_init, platform::*, println, sec::rng::rand, timer::delay_us};
use core::fmt::Write;

#[riscv_rt::entry]
fn main() -> ! {
    basic_init();

    loop {
        println!("New random number: {}", rand());
        delay_us(1_000_000);
    }
}
