use core::fmt::Write;
use core::panic;

use crate::println;

/* Available GPIO pins */
pub const GLB_GPIO_PIN_0: GlbGpioPinType = 0;
pub const GLB_GPIO_PIN_1: GlbGpioPinType = 1;
pub const GLB_GPIO_PIN_2: GlbGpioPinType = 2;
pub const GLB_GPIO_PIN_3: GlbGpioPinType = 3;
pub const GLB_GPIO_PIN_4: GlbGpioPinType = 4;
pub const GLB_GPIO_PIN_5: GlbGpioPinType = 5;
pub const GLB_GPIO_PIN_6: GlbGpioPinType = 6;
pub const GLB_GPIO_PIN_7: GlbGpioPinType = 7;
pub const GLB_GPIO_PIN_8: GlbGpioPinType = 8;
pub const GLB_GPIO_PIN_9: GlbGpioPinType = 9;
pub const GLB_GPIO_PIN_10: GlbGpioPinType = 10;
pub const GLB_GPIO_PIN_11: GlbGpioPinType = 11;
pub const GLB_GPIO_PIN_12: GlbGpioPinType = 12;
pub const GLB_GPIO_PIN_13: GlbGpioPinType = 13;
pub const GLB_GPIO_PIN_14: GlbGpioPinType = 14;
pub const GLB_GPIO_PIN_15: GlbGpioPinType = 15;
pub const GLB_GPIO_PIN_16: GlbGpioPinType = 16;
pub const GLB_GPIO_PIN_17: GlbGpioPinType = 17;
pub const GLB_GPIO_PIN_18: GlbGpioPinType = 18;
pub const GLB_GPIO_PIN_19: GlbGpioPinType = 19;
pub const GLB_GPIO_PIN_20: GlbGpioPinType = 20;
pub const GLB_GPIO_PIN_21: GlbGpioPinType = 21;
pub const GLB_GPIO_PIN_22: GlbGpioPinType = 22;
pub const GLB_GPIO_PIN_MAX: GlbGpioPinType = 23;
pub type GlbGpioPinType = u8;

/* Supported GPIO functionalities  */
pub const GLB_GPIO_FUN_SDIO: GlbGpioFunType = 1;
pub const GLB_GPIO_FUN_FLASH: GlbGpioFunType = 2;
pub const GLB_GPIO_FUN_SPI: GlbGpioFunType = 4;
pub const GLB_GPIO_FUN_I2C: GlbGpioFunType = 6;
pub const GLB_GPIO_FUN_UART: GlbGpioFunType = 7;
pub const GLB_GPIO_FUN_PWM: GlbGpioFunType = 8;
pub const GLB_GPIO_FUN_EXT_PA: GlbGpioFunType = 9;
pub const GLB_GPIO_FUN_ANALOG: GlbGpioFunType = 10;
pub const GLB_GPIO_FUN_SWGPIO: GlbGpioFunType = 11;
pub const GLB_GPIO_FUN_JTAG: GlbGpioFunType = 14;
pub type GlbGpioFunType = u8;

/* Supported GPIO modes */
pub const GLB_GPIO_MODE_INPUT: GlbGpioModeType = 0;
pub const GLB_GPIO_MODE_OUTPUT: GlbGpioModeType = 1;
pub const GLB_GPIO_MODE_AF: GlbGpioModeType = 2;
pub type GlbGpioModeType = u8;

/* Pullup/Pulldown */
pub const GLB_GPIO_PULL_UP: GlbGpioPullType = 0;
pub const GLB_GPIO_PULL_DOWN: GlbGpioPullType = 1;
pub const GLB_GPIO_PULL_NONE: GlbGpioPullType = 2;
pub type GlbGpioPullType = u8;

/* LED (active low) */
pub const GPIO_LED_ON: GpioLedStateType = 0;
pub const GPIO_LED_OFF: GpioLedStateType = 1;
pub type GpioLedStateType = u8;

pub const GLB_GPIO_INT_TRIG_TYPE_GLB_GPIO_INT_TRIG_NEG_PULSE: GlbGpioIntTrigType = 0;
pub const GLB_GPIO_INT_TRIG_TYPE_GLB_GPIO_INT_TRIG_POS_PULSE: GlbGpioIntTrigType = 1;
pub const GLB_GPIO_INT_TRIG_TYPE_GLB_GPIO_INT_TRIG_NEG_LEVEL: GlbGpioIntTrigType = 2;
pub const GLB_GPIO_INT_TRIG_TYPE_GLB_GPIO_INT_TRIG_POS_LEVEL: GlbGpioIntTrigType = 3;
pub type GlbGpioIntTrigType = u32;

pub const GLB_GPIO_INT_CONTROL_TYPE_GLB_GPIO_INT_CONTROL_SYNC: GlbGpioIntControlType = 0;
pub const GLB_GPIO_INT_CONTROL_TYPE_GLB_GPIO_INT_CONTROL_ASYNC: GlbGpioIntControlType = 1;
pub type GlbGpioIntControlType = u32;

pub const GLB_GPIO_INT_UNMASK: GlbGpioIntMaskType = 0;
pub const GLB_GPIO_INT_MASK: GlbGpioIntMaskType = 1;
pub type GlbGpioIntMaskType = u8;

// Data type for GPIO pin
pub struct GPIO {
    pub pin: GlbGpioPinType,
    pub fun: GlbGpioFunType,
    pub mode: GlbGpioModeType,
    pub pull_type: GlbGpioPullType,
    pub drive: u8,
    pub smt_ctrl: u8,
    pub initialized: bool,
}

// Data type for GPIO pin, used by serial communication protocols such as UART
pub struct GpioExtern {
    pub pin: GlbGpioPinType,
    pub fun: GlbGpioFunType,
    pub mode: GlbGpioModeType,
    pub pull_type: GlbGpioPullType,
    pub drive: u8,
    pub smt_ctrl: u8,
}

// Data type for GPIO interrupt handlers
#[derive(Clone, Copy)]
pub struct GpioInterruptContext {
    pub ctrl_mode: GlbGpioIntControlType,
    pub trg_mode: GlbGpioIntTrigType,
    pub handler: fn(),
}

// Interrupt handlers
fn default_handler() {
    println!("No specific interrupt handler specified");
}

static mut GPIO_INTERRUPT_HANDLERS: [fn(); GLB_GPIO_PIN_MAX as usize] =
    [default_handler; GLB_GPIO_PIN_MAX as usize];

// Macros used to configure GPIO pins
// Set output: channel, value to set, number of the pin to write to
macro_rules! gpio_output_set {
    ($channel: expr, $value: expr,$($pin: expr), *) => {
        $(
            paste::paste! {
              $channel.gpio_cfgctl32().modify(|_r, w| w.[<reg_gpio_ $pin _o>]().bit($value == 1))
            }
        )*
    };
}

// Get input: channel, number of the pin to read
macro_rules! gpio_input_get {
    ($channel: expr, $($pin: expr), *) => {
        $(
            paste::paste! {
                $channel.gpio_cfgctl30().read().[<reg_gpio_ $pin _i>]().bit() as u8
            }
        )*
    };
}

// Initialize pin: self, channel, (number of the pin to configure, number of the register to configure)
macro_rules! gpio_init {
    ($self: expr, $channel: expr, $(($pin:expr, $reg: literal)), *) => {
        $(
            paste::paste! {
                $channel.[<gpio_cfgctl $reg>]().modify(|_r, w| {
                    /* 1. Set gpio function */
                    unsafe {
                        w.[<reg_gpio_ $pin _func_sel>]().bits($self.fun);
                    }

                    /* 2. Set mode */
                    if $self.mode == GLB_GPIO_MODE_OUTPUT {
                        w.[<reg_gpio_ $pin _ie>]().clear_bit();
                        $channel.gpio_cfgctl34().modify(|_r, w| w.[<reg_gpio_ $pin _oe>]().set_bit());
                    } else {
                        w.[<reg_gpio_ $pin _ie>]().set_bit();
                        $channel.gpio_cfgctl34().modify(|_r, w| w.[<reg_gpio_ $pin _oe>]().clear_bit());
                    }

                    /* 3. Set pull type */
                    // Clear first
                    w.[<reg_gpio_ $pin _pu>]().clear_bit();
                    w.[<reg_gpio_ $pin _pd>]().clear_bit();

                    // Set pull
                    if $self.pull_type == GLB_GPIO_PULL_UP {
                        w.[<reg_gpio_ $pin _pu>]().set_bit();
                    } else if $self.pull_type == GLB_GPIO_PULL_DOWN {
                        w.[<reg_gpio_ $pin _pd>]().set_bit();
                    }

                    /* 4. Set drive */
                    unsafe {
                            w.[<reg_gpio_ $pin _drv>]().bits($self.drive);
                    }

                    /* 5. Set SMT control */
                    if $self.smt_ctrl == 0 {
                        w.[<reg_gpio_ $pin _smt>]().clear_bit()
                    } else {
                        w.[<reg_gpio_ $pin _smt>]().set_bit()
                    }
                })
            }
        )*
    };
}

impl GPIO {
    pub fn new(pin: GlbGpioPinType) -> GPIO {
        let cfg = GPIO {
            pin,
            fun: GLB_GPIO_FUN_SWGPIO,
            mode: GLB_GPIO_MODE_OUTPUT,
            pull_type: GLB_GPIO_PULL_NONE,
            drive: 0,
            smt_ctrl: 1,
            initialized: false,
        };

        // TODO: might calling self.init() is necessary already here with some defaults
        // if serial I/O protocols are implemented

        // Return GPIO config
        cfg
    }

    pub fn from(ext_conf: GpioExtern) -> GPIO {
        let mut cfg = GPIO {
            pin: ext_conf.pin,
            fun: ext_conf.fun,
            mode: ext_conf.mode,
            pull_type: ext_conf.pull_type,
            drive: ext_conf.drive,
            smt_ctrl: ext_conf.smt_ctrl,
            initialized: false,
        };

        cfg.init();
        cfg
    }

    fn check_initialized(&self) {
        if !self.initialized {
            panic!("GPIO pin is not initialized yet");
        }
    }

    pub fn output_set(&self, val: u8) {
        /* Check if the pin is initialized */
        self.check_initialized();

        /* Check if the pin is configured as output */
        if self.mode != GLB_GPIO_MODE_OUTPUT {
            panic!("GPIO pin is not configured as output");
        }

        /* Update pin value */
        let channel: &pac::glb::RegisterBlock = unsafe { &*pac::Glb::ptr() };

        match self.pin {
            GLB_GPIO_PIN_0 => gpio_output_set!(channel, val, 0),
            GLB_GPIO_PIN_1 => gpio_output_set!(channel, val, 1),
            GLB_GPIO_PIN_2 => gpio_output_set!(channel, val, 2),
            GLB_GPIO_PIN_3 => gpio_output_set!(channel, val, 3),
            GLB_GPIO_PIN_4 => gpio_output_set!(channel, val, 4),
            GLB_GPIO_PIN_5 => gpio_output_set!(channel, val, 5),
            GLB_GPIO_PIN_6 => gpio_output_set!(channel, val, 6),
            GLB_GPIO_PIN_7 => gpio_output_set!(channel, val, 7),
            GLB_GPIO_PIN_8 => gpio_output_set!(channel, val, 8),
            GLB_GPIO_PIN_9 => gpio_output_set!(channel, val, 9),
            GLB_GPIO_PIN_10 => gpio_output_set!(channel, val, 10),
            GLB_GPIO_PIN_11 => gpio_output_set!(channel, val, 11),
            GLB_GPIO_PIN_12 => gpio_output_set!(channel, val, 12),
            GLB_GPIO_PIN_13 => gpio_output_set!(channel, val, 13),
            GLB_GPIO_PIN_14 => gpio_output_set!(channel, val, 14),
            GLB_GPIO_PIN_15 => gpio_output_set!(channel, val, 15),
            GLB_GPIO_PIN_16 => gpio_output_set!(channel, val, 16),
            GLB_GPIO_PIN_17 => gpio_output_set!(channel, val, 17),
            GLB_GPIO_PIN_18 => gpio_output_set!(channel, val, 18),
            GLB_GPIO_PIN_19 => gpio_output_set!(channel, val, 19),
            GLB_GPIO_PIN_20 => gpio_output_set!(channel, val, 20),
            GLB_GPIO_PIN_21 => gpio_output_set!(channel, val, 21),
            GLB_GPIO_PIN_22 => gpio_output_set!(channel, val, 22),
            _ => panic!("Invalid GPIO port selected"),
        };
    }

    //TODO: Return bool?
    pub fn input_get(&self) -> u8 {
        /* Check if the poin is initialized */
        self.check_initialized();

        /* Check if the pin is configured as input */
        if self.mode != GLB_GPIO_MODE_INPUT {
            panic!("GPIO pin is not configured as input");
        }

        /* Get register */
        let channel: &pac::glb::RegisterBlock = unsafe { &*pac::Glb::ptr() };

        /* Get pin value  */
        match self.pin {
            GLB_GPIO_PIN_0 => gpio_input_get!(channel, 0),
            GLB_GPIO_PIN_1 => gpio_input_get!(channel, 1),
            GLB_GPIO_PIN_2 => gpio_input_get!(channel, 2),
            GLB_GPIO_PIN_3 => gpio_input_get!(channel, 3),
            GLB_GPIO_PIN_4 => gpio_input_get!(channel, 4),
            GLB_GPIO_PIN_5 => gpio_input_get!(channel, 5),
            GLB_GPIO_PIN_6 => gpio_input_get!(channel, 6),
            GLB_GPIO_PIN_7 => gpio_input_get!(channel, 7),
            GLB_GPIO_PIN_8 => gpio_input_get!(channel, 8),
            GLB_GPIO_PIN_9 => gpio_input_get!(channel, 9),
            GLB_GPIO_PIN_10 => gpio_input_get!(channel, 10),
            GLB_GPIO_PIN_11 => gpio_input_get!(channel, 11),
            GLB_GPIO_PIN_12 => gpio_input_get!(channel, 12),
            GLB_GPIO_PIN_13 => gpio_input_get!(channel, 13),
            GLB_GPIO_PIN_14 => gpio_input_get!(channel, 14),
            GLB_GPIO_PIN_15 => gpio_input_get!(channel, 15),
            GLB_GPIO_PIN_16 => gpio_input_get!(channel, 16),
            GLB_GPIO_PIN_17 => gpio_input_get!(channel, 17),
            GLB_GPIO_PIN_18 => gpio_input_get!(channel, 18),
            GLB_GPIO_PIN_19 => gpio_input_get!(channel, 19),
            GLB_GPIO_PIN_20 => gpio_input_get!(channel, 20),
            GLB_GPIO_PIN_21 => gpio_input_get!(channel, 21),
            GLB_GPIO_PIN_22 => gpio_input_get!(channel, 22),
            _ => panic!("Invalid GPIO port selected"),
        }
    }

    pub fn enable_output(&mut self, pull: GlbGpioPullType) {
        self.fun = GLB_GPIO_FUN_SWGPIO;
        self.mode = GLB_GPIO_MODE_OUTPUT;
        self.pull_type = pull;
        self.drive = 0;
        self.smt_ctrl = 1;

        self.init();
    }

    pub fn enable_input(&mut self, pull: GlbGpioPullType) {
        self.fun = GLB_GPIO_FUN_SWGPIO;
        self.mode = GLB_GPIO_MODE_INPUT;
        self.pull_type = pull;
        self.drive = 0;
        self.smt_ctrl = 1;

        self.init();
    }

    fn init(&mut self) {
        let channel: &pac::glb::RegisterBlock = unsafe { &*pac::Glb::ptr() };

        match self.pin {
            GLB_GPIO_PIN_0 => gpio_init!(self, channel, (0, 0)),
            GLB_GPIO_PIN_1 => gpio_init!(self, channel, (1, 0)),
            GLB_GPIO_PIN_2 => gpio_init!(self, channel, (2, 1)),
            GLB_GPIO_PIN_3 => gpio_init!(self, channel, (3, 1)),
            GLB_GPIO_PIN_4 => gpio_init!(self, channel, (4, 2)),
            GLB_GPIO_PIN_5 => gpio_init!(self, channel, (5, 2)),
            GLB_GPIO_PIN_6 => gpio_init!(self, channel, (6, 3)),
            GLB_GPIO_PIN_7 => gpio_init!(self, channel, (7, 3)),
            GLB_GPIO_PIN_8 => gpio_init!(self, channel, (8, 4)),
            GLB_GPIO_PIN_9 => gpio_init!(self, channel, (9, 4)),
            GLB_GPIO_PIN_10 => gpio_init!(self, channel, (10, 5)),
            GLB_GPIO_PIN_11 => gpio_init!(self, channel, (11, 5)),
            GLB_GPIO_PIN_12 => gpio_init!(self, channel, (12, 6)),
            GLB_GPIO_PIN_13 => gpio_init!(self, channel, (13, 6)),
            GLB_GPIO_PIN_14 => gpio_init!(self, channel, (14, 7)),
            GLB_GPIO_PIN_15 => gpio_init!(self, channel, (15, 7)),
            GLB_GPIO_PIN_16 => gpio_init!(self, channel, (16, 8)),
            GLB_GPIO_PIN_17 => gpio_init!(self, channel, (17, 8)),
            GLB_GPIO_PIN_18 => gpio_init!(self, channel, (18, 9)),
            GLB_GPIO_PIN_19 => gpio_init!(self, channel, (19, 9)),
            GLB_GPIO_PIN_20 => gpio_init!(self, channel, (20, 10)),
            GLB_GPIO_PIN_21 => gpio_init!(self, channel, (21, 10)),
            GLB_GPIO_PIN_22 => gpio_init!(self, channel, (22, 11)),
            _ => panic!("Invalid GPIO port selected"),
        };

        self.initialized = true;
    }

    // Register interrupt for a pin
    pub fn register_interrupt(&self, node: GpioInterruptContext) {
        /* 1. Mask pin */
        gpio_intmask(self.pin, GLB_GPIO_INT_MASK);

        /* 2. Set interrupt handler */
        unsafe {
            GPIO_INTERRUPT_HANDLERS[self.pin as usize] = node.handler;
        }

        /* 3. Set interrupt mode */
        let channel: &pac::glb::RegisterBlock = unsafe { &*pac::Glb::ptr() };
        if self.pin < GLB_GPIO_PIN_10 {
            channel.gpio_int_mode_set1().modify(|r, w| {
                let mut value = r.reg_gpio_int_mode_set1().bits();
                value = (value & !(0x7 << (3 * self.pin)))
                    | (((node.ctrl_mode << 2) | node.trg_mode) << (3 * self.pin));
                unsafe { w.reg_gpio_int_mode_set1().bits(value) }
            });
        } else if self.pin < GLB_GPIO_PIN_20 {
            channel.gpio_int_mode_set2().modify(|r, w| {
                let tmp_pin = self.pin - GLB_GPIO_PIN_10;
                let mut value = r.reg_gpio_int_mode_set2().bits();
                value = (value & !(0x7 << (3 * tmp_pin)))
                    | (((node.ctrl_mode << 2) | node.trg_mode) << (3 * tmp_pin));
                unsafe { w.reg_gpio_int_mode_set2().bits(value) }
            });
        } else {
            channel.gpio_int_mode_set3().modify(|r, w| {
                let tmp_pin = self.pin - GLB_GPIO_PIN_20;
                let mut value = r.reg_gpio_int_mode_set3().bits();
                value = (value & !(0x7 << (3 * tmp_pin)))
                    | (((node.ctrl_mode << 2) | node.trg_mode) << (3 * tmp_pin));
                unsafe { w.reg_gpio_int_mode_set3().bits(value) }
            });
        }

        /* 4. Unmask pin */
        gpio_intmask(self.pin, GLB_GPIO_INT_UNMASK);

        /* 5. Enable GPIO interrupt */
        crate::irq::enable(crate::irq::Interrupt::GPIO);
    }
}

// Mask or unmask GPIO pin
fn gpio_intmask(pin: GlbGpioPinType, mask: GlbGpioIntMaskType) {
    if pin < GLB_GPIO_PIN_MAX {
        let channel: &pac::glb::RegisterBlock = unsafe { &*pac::Glb::ptr() };
        channel.gpio_int_mask1().modify(|r, w| {
            // Read current value
            let mut value = r.reg_gpio_int_mask1().bits();

            // Mask or unmask pin
            if mask == GLB_GPIO_INT_MASK {
                value = value | (1 << pin);
            } else {
                value = value & !(1 << pin);
            }

            // Write updated value
            unsafe { w.reg_gpio_int_mask1().bits(value) }
        });
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
fn Gpio_ISR() {
    /* 1. Disable and clear GPIO interrupt */
    crate::irq::disable(crate::irq::Interrupt::GPIO);
    crate::irq::clear(crate::irq::Interrupt::GPIO);

    /* 2. Get the GPIO pin that fired the interrupt */
    /* 2.1. Get current state of GPIO interrupt register */
    let channel: &pac::glb::RegisterBlock = unsafe { &*pac::Glb::ptr() };
    let int_state = channel.gpio_int_stat1().read().gpio_int_stat1().bits();

    /* 2.2. Get affected pin */
    let mut affected_pin = 0;
    for i in 0..GLB_GPIO_PIN_MAX {
        // Compute bit state the register must have position if the given pin fired
        let bit_state = 1 << i;

        // Apply mask to check whether that pin fired
        if (bit_state & int_state) != 0 {
            affected_pin = i;

            // We can stop here as we found the interrupt
            break;
        }
    }

    /* 3. Handle interrupt request */
    /* 3.1. Mask pin */
    gpio_intmask(affected_pin, 1);

    /* 3.2. Handle interrupt */
    unsafe {
        let handler = GPIO_INTERRUPT_HANDLERS[affected_pin as usize];
        handler();
    }

    /* 3.3. Unmask pin */
    gpio_intmask(affected_pin, 0);

    /* 4. Re-enable GPIO interrupt */
    crate::irq::enable(crate::irq::Interrupt::GPIO);
}
