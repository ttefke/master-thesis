#![no_std]
use core::fmt::Write;

pub mod gpio;

pub mod hbn;

pub mod irq;

pub mod platform;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod sec;

pub mod sys;

pub mod timer;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod uart;

// Initialize platform
use crate::platform::*;

pub fn basic_init() {
    // Enable FPU
    unsafe extern "C" {
        pub unsafe fn _enable_fpu();
    }

    unsafe {
        _enable_fpu();
    }

    // Early init
    crate::sys::early_init();

    // Initialize UART
    crate::uart::init(0, 16, 7, 2_000_000);
    println!("[BL602] Starting up");

    // System init
    crate::sys::sys_init();

    // Initialize system
    crate::sec::init();

    //crate::dma::bl_dma_init(); // (depends on freertos and blog)
    //crate::rtc::bl_rtc_init();
    //crate::boot2::hal_boot2_init(); // (depends on blog)
    //crate::board::hal_board_cfg(0); // depends on blog and dts

    // Throwaway timer values
    for _i in 0..5 {
        crate::timer::delay_us(50);
    }
}
