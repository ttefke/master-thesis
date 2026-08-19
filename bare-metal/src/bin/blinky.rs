#![no_std]
#![no_main]

use bl602_hal::{basic_init, gpio::*};

// Toggle macro
macro_rules! toggle {
    ($handler: ident) => {
        $handler.output_set(GPIO_LED_ON);
        bl602_hal::timer::delay_us(250_000);
        $handler.output_set(GPIO_LED_OFF);
    };
}

// Macro to configure led
macro_rules! configure_led {
    ($handler: ident, $pin: ident) => {
        let mut $handler = GPIO::new($pin);
        $handler.enable_output(GLB_GPIO_PULL_NONE);
        $handler.output_set(GPIO_LED_OFF);
    };
}

#[riscv_rt::entry]
fn main() -> ! {
    // Initialize system
    basic_init();

    // Configure LEDs
    configure_led!(led_red, GLB_GPIO_PIN_17);
    configure_led!(led_green, GLB_GPIO_PIN_14);
    configure_led!(led_blue, GLB_GPIO_PIN_11);

    // Toggle leds
    loop {
        toggle!(led_red);
        toggle!(led_green);
        toggle!(led_blue);
    }
}
