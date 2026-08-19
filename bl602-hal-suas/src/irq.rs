// Inspired from https://github.com/sipeed/bl602-hal/blob/main/src/interrupts.rs

use riscv::register::{mcause, mtvec, mtvec::Mtvec};

pub const BL_IRQ_EXCEPTION_TYPE_T_BL_IRQ_EXCEPTION_TYPE_LOAD_MISALIGN: BlIrqExceptionTypeT = 0;
pub const BL_IRQ_EXCEPTION_TYPE_T_BL_IRQ_EXCEPTION_TYPE_STORE_MISALIGN: BlIrqExceptionTypeT = 1;
pub const BL_IRQ_EXCEPTION_TYPE_T_BL_IRQ_EXCEPTION_TYPE_ACCESS_ILLEGAL: BlIrqExceptionTypeT = 2;
pub const BL_IRQ_EXCEPTION_TYPE_T_BL_IRQ_EXCEPTION_TYPE_ILLEGAL_INSTRUCTION: BlIrqExceptionTypeT =
    3;
pub type BlIrqExceptionTypeT = ::core::ffi::c_uint;

pub const IRQN_TYPE_MSOFT_IRQN: IrqnType = 3;
pub const IRQN_TYPE_MTIME_IRQN: IrqnType = 7;
pub const IRQN_TYPE_MEXT_IRQN: IrqnType = 11;
pub const IRQN_TYPE_CLIC_SOFT_PEND_IRQN: IrqnType = 12;
pub const IRQN_TYPE_BMX_ERR_IRQN: IrqnType = 16;
pub const IRQN_TYPE_BMX_TO_IRQN: IrqnType = 17;
pub const IRQN_TYPE_L1_C_BMX_ERR_IRQN: IrqnType = 18;
pub const IRQN_TYPE_L1_C_BMX_TO_IRQN: IrqnType = 19;
pub const IRQN_TYPE_SEC_BMX_ERR_IRQN: IrqnType = 20;
pub const IRQN_TYPE_RF_TOP_INT0_IRQN: IrqnType = 21;
pub const IRQN_TYPE_RF_TOP_INT1_IRQN: IrqnType = 22;
pub const IRQN_TYPE_SDIO_IRQN: IrqnType = 23;
pub const IRQN_TYPE_DMA_BMX_ERR_IRQN: IrqnType = 24;
pub const IRQN_TYPE_SEC_GMAC_IRQN: IrqnType = 25;
pub const IRQN_TYPE_SEC_CDET_IRQN: IrqnType = 26;
pub const IRQN_TYPE_SEC_PKA_IRQN: IrqnType = 27;
pub const IRQN_TYPE_SEC_TRNG_IRQN: IrqnType = 28;
pub const IRQN_TYPE_SEC_AES_IRQN: IrqnType = 29;
pub const IRQN_TYPE_SEC_SHA_IRQN: IrqnType = 30;
pub const IRQN_TYPE_DMA_ALL_IRQN: IrqnType = 31;
pub const IRQN_TYPE_RESERVED0: IrqnType = 32;
pub const IRQN_TYPE_RESERVED1: IrqnType = 33;
pub const IRQN_TYPE_RESERVED2: IrqnType = 34;
pub const IRQN_TYPE_IRTX_IRQN: IrqnType = 35;
pub const IRQN_TYPE_IRRX_IRQN: IrqnType = 36;
pub const IRQN_TYPE_RESERVED3: IrqnType = 37;
pub const IRQN_TYPE_RESERVED4: IrqnType = 38;
pub const IRQN_TYPE_SF_CTRL_IRQN: IrqnType = 39;
pub const IRQN_TYPE_RESERVED5: IrqnType = 40;
pub const IRQN_TYPE_GPADC_DMA_IRQN: IrqnType = 41;
pub const IRQN_TYPE_EFUSE_IRQN: IrqnType = 42;
pub const IRQN_TYPE_SPI_IRQN: IrqnType = 43;
pub const IRQN_TYPE_RESERVED6: IrqnType = 44;
pub const IRQN_TYPE_UART0_IRQN: IrqnType = 45;
pub const IRQN_TYPE_UART1_IRQN: IrqnType = 46;
pub const IRQN_TYPE_RESERVED7: IrqnType = 47;
pub const IRQN_TYPE_I2C_IRQN: IrqnType = 48;
pub const IRQN_TYPE_RESERVED8: IrqnType = 49;
pub const IRQN_TYPE_PWM_IRQN: IrqnType = 50;
pub const IRQN_TYPE_RESERVED9: IrqnType = 51;
pub const IRQN_TYPE_TIMER_CH0_IRQN: IrqnType = 52;
pub const IRQN_TYPE_TIMER_CH1_IRQN: IrqnType = 53;
pub const IRQN_TYPE_TIMER_WDT_IRQN: IrqnType = 54;
pub const IRQN_TYPE_RESERVED10: IrqnType = 55;
pub const IRQN_TYPE_RESERVED11: IrqnType = 56;
pub const IRQN_TYPE_RESERVED12: IrqnType = 57;
pub const IRQN_TYPE_RESERVED13: IrqnType = 58;
pub const IRQN_TYPE_RESERVED14: IrqnType = 59;
pub const IRQN_TYPE_GPIO_INT0_IRQN: IrqnType = 60;
pub const IRQN_TYPE_RESERVED16: IrqnType = 61;
pub const IRQN_TYPE_RESERVED17: IrqnType = 62;
pub const IRQN_TYPE_RESERVED18: IrqnType = 63;
pub const IRQN_TYPE_RESERVED19: IrqnType = 64;
pub const IRQN_TYPE_RESERVED20: IrqnType = 65;
pub const IRQN_TYPE_PDS_WAKEUP_IRQN: IrqnType = 66;
pub const IRQN_TYPE_HBN_OUT0_IRQN: IrqnType = 67;
pub const IRQN_TYPE_HBN_OUT1_IRQN: IrqnType = 68;
pub const IRQN_TYPE_BOR_IRQN: IrqnType = 69;
pub const IRQN_TYPE_WIFI_IRQN: IrqnType = 70;
pub const IRQN_TYPE_BZ_PHY_IRQN: IrqnType = 71;
pub const IRQN_TYPE_BLE_IRQN: IrqnType = 72;
pub const IRQN_TYPE_MAC_TXRX_TIMER_IRQN: IrqnType = 73;
pub const IRQN_TYPE_MAC_TXRX_MISC_IRQN: IrqnType = 74;
pub const IRQN_TYPE_MAC_RX_TRG_IRQN: IrqnType = 75;
pub const IRQN_TYPE_MAC_TX_TRG_IRQN: IrqnType = 76;
pub const IRQN_TYPE_MAC_GEN_IRQN: IrqnType = 77;
pub const IRQN_TYPE_MAC_PORT_TRG_IRQN: IrqnType = 78;
pub const IRQN_TYPE_WIFI_IPC_PUBLIC_IRQN: IrqnType = 79;
pub const IRQN_TYPE_IRQN_LAST: IrqnType = 80;
pub type IrqnType = u32;

unsafe extern "C" {
    unsafe fn RandomNumberGenerator_ISR();
    unsafe fn Gpio_ISR();
}

pub const CLIC_HART0_ADDR: u32 = 0x0280_0000;
pub const CLIC_INTIP: u32 = 0x000;
pub const CLIC_INTIE: u32 = 0x400;
pub const INT_OFFSET: u32 = 16;
const INTERRUPTS_LEN: usize = 24;

#[unsafe(no_mangle)]
pub fn _setup_interrupts() {
    unsafe extern "C" {
        pub unsafe fn _start_trap_hal();
    }

    // New interrupt handler
    let new_base = _start_trap_hal as usize;

    unsafe {
        // Disable interrupts
        riscv::interrupt::disable();

        // Update interrupt handler
        mtvec::write(Mtvec::from_bits(new_base + 2));

        // Disable all interrupts
        let enabled = core::slice::from_raw_parts_mut(
            (CLIC_HART0_ADDR + CLIC_INTIE) as *mut u32,
            INTERRUPTS_LEN,
        );
        enabled.iter_mut().for_each(|v| *v = 0);

        // Clear all pending interrupts
        let pending = core::slice::from_raw_parts_mut(
            (CLIC_HART0_ADDR + CLIC_INTIP) as *mut u32,
            INTERRUPTS_LEN,
        );
        pending.iter_mut().for_each(|v| *v = 0);

        // Enable interrupts again
        riscv::interrupt::enable();
    }
}

// Start trap handler
#[unsafe(link_section = ".trap.rust")]
#[unsafe(export_name = "_start_trap_rust_hal")]
pub unsafe extern "C" fn start_trap_rust_hal(trap_frame: *mut TrapFrame) {
    unsafe extern "C" {
        pub unsafe fn _start_trap_rust(trap_frame: *const TrapFrame);
    }

    let cause = mcause::read();
    if cause.is_exception() {
        unsafe {
            _start_trap_rust(trap_frame);
        }
    } else {
        let code = cause.code();
        if code < INT_OFFSET as usize {
            unsafe {
                _start_trap_rust(trap_frame);
            }
        } else {
            let interrupt_nr = (code & 0xFF) as u32;
            let interrupt = Interrupt::from(interrupt_nr);

            unsafe {
                match interrupt {
                    Interrupt::Unknown => _start_trap_rust(trap_frame),
                    Interrupt::RNG => RandomNumberGenerator_ISR(),
                    Interrupt::GPIO => Gpio_ISR(),
                };
            }
        }
    }
}

// Enable interrupt
pub fn enable(interrupt: Interrupt) {
    let irq = interrupt.to_irq();
    let ptr = (CLIC_HART0_ADDR + CLIC_INTIE + irq) as *mut u8;
    unsafe {
        ptr.write_volatile(1);
    }
}

// Disable interrupt
pub fn disable(interrupt: Interrupt) {
    let irq = interrupt.to_irq();
    let ptr = (CLIC_HART0_ADDR + CLIC_INTIE + irq) as *mut u8;
    unsafe {
        ptr.write_volatile(0);
    }
}

// Clear interrupt
pub fn clear(interrupt: Interrupt) {
    let irq = interrupt.to_irq();
    let ptr = (CLIC_HART0_ADDR + CLIC_INTIP + irq) as *mut u8;
    unsafe {
        ptr.write_volatile(0);
    }
}

// Available interrupts
pub enum Interrupt {
    Unknown,
    RNG,
    GPIO,
}

impl Interrupt {
    fn to_irq(&self) -> u32 {
        match &self {
            Interrupt::Unknown => panic!("Unknown interrupt has no IRQ number"),
            Interrupt::RNG => IRQN_TYPE_SEC_TRNG_IRQN,
            Interrupt::GPIO => IRQN_TYPE_GPIO_INT0_IRQN,
        }
    }

    fn from(irq: u32) -> Interrupt {
        match irq {
            IRQN_TYPE_SEC_TRNG_IRQN => Interrupt::RNG,
            IRQN_TYPE_GPIO_INT0_IRQN => Interrupt::GPIO,
            _ => Interrupt::Unknown,
        }
    }
}

// Trap frame definition
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct TrapFrame {
    pub ra: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub gp: usize,
    pub tp: usize,
    pub sp: usize,
}
