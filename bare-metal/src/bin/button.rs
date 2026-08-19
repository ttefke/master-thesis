#![no_std]
#![no_main]

use bl602_hal::{gpio::*, platform::puts, println};
use core::fmt::Write;

/* Functions to be called on interrupts */
fn handler_pin_2() {
    println!("Pin 2 was pressed");
}

fn handler_pin_3() {
    println!("Pin 3 was released");
}

fn handler_pin_12() {
    println!("Pin 12 was pressed");
}

// Interrupt register macro
macro_rules! register_interrupt {
    ($pin: ident, $handler: ident, $pulse: ident) => {
        let mut button = GPIO::new($pin);
        button.enable_input(GLB_GPIO_PULL_DOWN);

        let int_ctx = GpioInterruptContext {
            ctrl_mode: GLB_GPIO_INT_CONTROL_TYPE_GLB_GPIO_INT_CONTROL_SYNC,
            trg_mode: $pulse,
            handler: $handler,
        };
        button.register_interrupt(int_ctx);
    };
}

/* Main function */
#[riscv_rt::entry]
fn main() -> ! {
    // Basic hardware init
    bl602_hal::basic_init();

    // Register interrupt handlers for pins
    register_interrupt!(
        GLB_GPIO_PIN_2,
        handler_pin_2,
        GLB_GPIO_INT_TRIG_TYPE_GLB_GPIO_INT_TRIG_POS_PULSE
    );
    register_interrupt!(
        GLB_GPIO_PIN_3,
        handler_pin_3,
        GLB_GPIO_INT_TRIG_TYPE_GLB_GPIO_INT_TRIG_NEG_PULSE
    );
    register_interrupt!(
        GLB_GPIO_PIN_12,
        handler_pin_12,
        GLB_GPIO_INT_TRIG_TYPE_GLB_GPIO_INT_TRIG_POS_PULSE
    );

    // Loop
    println!("'Touch' any button");
    loop {}
}
