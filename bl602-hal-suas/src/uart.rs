use crate::{gpio::*, hbn::*};

pub const UART_ID_Type_UART0_ID: UART_ID_Type = 0;
pub const UART_ID_Type_UART1_ID: UART_ID_Type = 1;
pub type UART_ID_Type = u32;

pub const UART_Parity_Type_UART_PARITY_NONE: UART_Parity_Type = 0;
pub const UART_Parity_Type_UART_PARITY_ODD: UART_Parity_Type = 1;
pub const UART_Parity_Type_UART_PARITY_EVEN: UART_Parity_Type = 2;
pub type UART_Parity_Type = u32;

pub const UART_DataBits_Type_UART_DATABITS_5: UART_DataBits_Type = 0;
pub const UART_DataBits_Type_UART_DATABITS_6: UART_DataBits_Type = 1;
pub const UART_DataBits_Type_UART_DATABITS_7: UART_DataBits_Type = 2;
pub const UART_DataBits_Type_UART_DATABITS_8: UART_DataBits_Type = 3;
pub type UART_DataBits_Type = u32;

pub const UART_StopBits_Type_UART_STOPBITS_1: UART_StopBits_Type = 0;
pub const UART_StopBits_Type_UART_STOPBITS_1_5: UART_StopBits_Type = 1;
pub const UART_StopBits_Type_UART_STOPBITS_2: UART_StopBits_Type = 2;
pub type UART_StopBits_Type = u32;

pub const UART_ByteBitInverse_Type_UART_LSB_FIRST: UART_ByteBitInverse_Type = 0;
pub const UART_ByteBitInverse_Type_UART_MSB_FIRST: UART_ByteBitInverse_Type = 1;
pub type UART_ByteBitInverse_Type = u32;

pub const BL_Fun_Type_DISABLE: BL_FUN_Type = 0;
pub const BL_Fun_Type_ENABLE: BL_FUN_Type = 1;
pub type BL_FUN_Type = u32;

pub const GLB_UART_SIG_Type_GLB_UART_SIG_0: GLB_UART_SIG_Type = 0;
pub const GLB_UART_SIG_Type_GLB_UART_SIG_1: GLB_UART_SIG_Type = 1;
pub const GLB_UART_SIG_Type_GLB_UART_SIG_2: GLB_UART_SIG_Type = 2;
pub const GLB_UART_SIG_Type_GLB_UART_SIG_3: GLB_UART_SIG_Type = 3;
pub const GLB_UART_SIG_Type_GLB_UART_SIG_4: GLB_UART_SIG_Type = 4;
pub const GLB_UART_SIG_Type_GLB_UART_SIG_5: GLB_UART_SIG_Type = 5;
pub const GLB_UART_SIG_Type_GLB_UART_SIG_6: GLB_UART_SIG_Type = 6;
pub const GLB_UART_SIG_Type_GLB_UART_SIG_7: GLB_UART_SIG_Type = 7;
pub type GLB_UART_SIG_Type = u32;

pub const GLB_UART_SIG_FUN_Type_GLB_UART_SIG_FUN_UART0_RTS: GLB_UART_SIG_FUN_Type = 0;
pub const GLB_UART_SIG_FUN_Type_GLB_UART_SIG_FUN_UART0_CTS: GLB_UART_SIG_FUN_Type = 1;
pub const GLB_UART_SIG_FUN_Type_GLB_UART_SIG_FUN_UART0_TXD: GLB_UART_SIG_FUN_Type = 2;
pub const GLB_UART_SIG_FUN_Type_GLB_UART_SIG_FUN_UART0_RXD: GLB_UART_SIG_FUN_Type = 3;
pub const GLB_UART_SIG_FUN_Type_GLB_UART_SIG_FUN_UART1_RTS: GLB_UART_SIG_FUN_Type = 4;
pub const GLB_UART_SIG_FUN_Type_GLB_UART_SIG_FUN_UART1_CTS: GLB_UART_SIG_FUN_Type = 5;
pub const GLB_UART_SIG_FUN_Type_GLB_UART_SIG_FUN_UART1_TXD: GLB_UART_SIG_FUN_Type = 6;
pub const GLB_UART_SIG_FUN_Type_GLB_UART_SIG_FUN_UART1_RXD: GLB_UART_SIG_FUN_Type = 7;
pub type GLB_UART_SIG_FUN_Type = u32;

pub const UART_INT_Type_UART_INT_TX_END: UART_INT_Type = 0;
pub const UART_INT_Type_UART_INT_RX_END: UART_INT_Type = 1;
pub const UART_INT_Type_UART_INT_TX_FIFO_REQ: UART_INT_Type = 2;
pub const UART_INT_Type_UART_INT_RX_FIFO_REQ: UART_INT_Type = 3;
pub const UART_INT_Type_UART_INT_RTO: UART_INT_Type = 4;
pub const UART_INT_Type_UART_INT_PCE: UART_INT_Type = 5;
pub const UART_INT_Type_UART_INT_TX_FER: UART_INT_Type = 6;
pub const UART_INT_Type_UART_INT_RX_FER: UART_INT_Type = 7;
pub const UART_INT_Type_UART_INT_ALL: UART_INT_Type = 8;
pub type UART_INT_Type = u32;

pub const UART_Direction_Type_UART_TX: UART_Direction_Type = 0;
pub const UART_Direction_Type_UART_RX: UART_Direction_Type = 1;
pub const UART_Direction_Type_UART_TXRX: UART_Direction_Type = 2;
pub type UART_Direction_Type = u32;

pub const BL_Mask_Type_UNMASK: BL_Mask_Type = 0;
pub const BL_Mask_Type_MASK: BL_Mask_Type = 1;
pub type BL_Mask_Type = u32;

static mut UART_CLK_INIT: u8 = 0;
const UART_DIV: u32 = 3;

pub struct UART_CFG_Type {
    pub uartClk: u32,
    pub baudRate: u32,
    pub dataBits: UART_DataBits_Type,
    pub stopBits: UART_StopBits_Type,
    pub parity: UART_Parity_Type,
    pub ctsFlowControl: BL_FUN_Type,
    pub rxDeglitch: BL_FUN_Type,
    pub rtsSoftwareControl: BL_FUN_Type,
    pub byteBitInverse: UART_ByteBitInverse_Type,
}

pub struct UART_FifoCfg_Type {
    pub txFifoDmaThreshold: u8,
    pub rxFifoDmaThreshold: u8,
    pub txFifoDmaEnable: BL_FUN_Type,
    pub rxFifoDmaEnable: BL_FUN_Type,
}

pub fn init(id: UART_ID_Type, tx_pin: u8, rx_pin: u8, baudrate: u32) {
    let uartCfg: UART_CFG_Type = UART_CFG_Type {
        uartClk: 160_000_000 / (UART_DIV + 1),
        baudRate: baudrate,
        dataBits: UART_DataBits_Type_UART_DATABITS_8,
        stopBits: UART_StopBits_Type_UART_STOPBITS_1,
        parity: UART_Parity_Type_UART_PARITY_NONE,
        ctsFlowControl: BL_Fun_Type_DISABLE,
        rxDeglitch: BL_Fun_Type_DISABLE,
        rtsSoftwareControl: BL_Fun_Type_DISABLE,
        byteBitInverse: UART_ByteBitInverse_Type_UART_LSB_FIRST,
    };
    let fifoCfg: UART_FifoCfg_Type = UART_FifoCfg_Type {
        txFifoDmaThreshold: 0x10,
        rxFifoDmaThreshold: 0x10,
        txFifoDmaEnable: BL_Fun_Type_DISABLE,
        rxFifoDmaEnable: BL_Fun_Type_DISABLE,
    };

    // Set UART clock
    unsafe {
        if UART_CLK_INIT == 0 {
            low_uart_set_clock(
                BL_Fun_Type_ENABLE,
                HBN_UART_CLK_TYPE_HBN_UART_CLK_160_M,
                UART_DIV.try_into().unwrap(),
            );
            UART_CLK_INIT = 1;
        }
    }

    // GPIO init
    low_uart_gpio_init(id, tx_pin, rx_pin);

    // Disable all interrupts
    low_uart_int_mask(id, UART_INT_Type_UART_INT_ALL, BL_Mask_Type_MASK);

    // Disable UART before configuring
    low_uart_disable(id, UART_Direction_Type_UART_TXRX);

    // Initialize UART
    low_uart_init(id, &uartCfg);

    // Enable free run mode
    low_uart_conf_tx_free_run(id, BL_Fun_Type_ENABLE);

    // FIFO Config
    low_uart_fifo_config(id, &fifoCfg);

    // Enable UART
    low_uart_enable(id, UART_Direction_Type_UART_TXRX);
}

pub fn data_send(id: UART_ID_Type, data: u8) {
    let channel: &pac::uart0::RegisterBlock = unsafe { &*pac::Uart0::ptr() };

    // Wait for fifo
    while (low_uart_get_tx_fifo_count(id)) == 0 {}

    channel
        .uart_fifo_wdata()
        .write(|w| unsafe { w.bits(data.into()) });
}

pub fn data_receive(id: UART_ID_Type) -> u8 {
    if id != 0 {
        panic!("Unsupported UART channel selected");
    }
    let mut result = 0;
    let channel: &pac::uart0::RegisterBlock = unsafe { &*pac::Uart0::ptr() };

    /* Get number bits in the FIFO queue */
    let rx_fifo_count = channel.uart_fifo_config_1().read().rx_fifo_cnt().bits();

    /* Read data from receiver register if present */
    if rx_fifo_count > 0 {
        result = channel.uart_fifo_rdata().read().uart_fifo_rdata().bits();
    }

    return result;
}

fn low_uart_set_clock(enable: BL_FUN_Type, clock: HbnUartClkType, div: u8) {
    if div > 7 {
        panic!("Invalid clock divider for UART");
    }

    let channel: &pac::glb::RegisterBlock = unsafe { &*pac::Glb::ptr() };

    /* 1. Disable UART clock */
    channel
        .clk_cfg2()
        .modify(|_r, w| w.uart_clk_en().clear_bit());

    /* 2. Set divider */
    channel
        .clk_cfg2()
        .modify(|_r, w| unsafe { w.uart_clk_div().bits(div) });

    /* 3. Select clock source */
    set_uart_clk_sel(clock);

    /* 4. Enable/disable clock */
    match enable {
        BL_Fun_Type_ENABLE => channel.clk_cfg2().modify(|_r, w| w.uart_clk_en().set_bit()),
        BL_Fun_Type_DISABLE => channel
            .clk_cfg2()
            .modify(|_r, w| w.uart_clk_en().clear_bit()),
        _ => panic!("Invalid UART clock mode"),
    };
}

// GLB_UART_Fun_Sel
fn low_uart_select_signal_function(signal: GLB_UART_SIG_Type, function: GLB_UART_SIG_FUN_Type) {
    let channel: &pac::glb::RegisterBlock = unsafe { &*pac::Glb::ptr() };

    channel.uart_sig_sel_0().modify(|_r, w| match signal {
        GLB_UART_SIG_Type_GLB_UART_SIG_0 => unsafe {
            w.uart_sig_0_sel().bits(function.try_into().unwrap())
        },
        GLB_UART_SIG_Type_GLB_UART_SIG_1 => unsafe {
            w.uart_sig_1_sel().bits(function.try_into().unwrap())
        },
        GLB_UART_SIG_Type_GLB_UART_SIG_2 => unsafe {
            w.uart_sig_2_sel().bits(function.try_into().unwrap())
        },
        GLB_UART_SIG_Type_GLB_UART_SIG_3 => unsafe {
            w.uart_sig_3_sel().bits(function.try_into().unwrap())
        },
        GLB_UART_SIG_Type_GLB_UART_SIG_4 => unsafe {
            w.uart_sig_4_sel().bits(function.try_into().unwrap())
        },
        GLB_UART_SIG_Type_GLB_UART_SIG_5 => unsafe {
            w.uart_sig_5_sel().bits(function.try_into().unwrap())
        },
        GLB_UART_SIG_Type_GLB_UART_SIG_6 => unsafe {
            w.uart_sig_6_sel().bits(function.try_into().unwrap())
        },
        GLB_UART_SIG_Type_GLB_UART_SIG_7 => unsafe {
            w.uart_sig_7_sel().bits(function.try_into().unwrap())
        },
        _ => unreachable!(),
    });
}

fn low_uart_gpio_init(id: UART_ID_Type, tx_pin: u8, rx_pin: u8) {
    /* 1. Initialize RX pin */
    let cfg_rx: GpioExtern = GpioExtern {
        pin: rx_pin,
        fun: GLB_GPIO_FUN_UART,
        mode: GLB_GPIO_MODE_AF,
        pull_type: GLB_GPIO_PULL_UP,
        drive: 1,
        smt_ctrl: 1,
    };
    GPIO::from(cfg_rx);

    /* 2. Initialize TX pin */
    let cfg_tx: GpioExtern = GpioExtern {
        pin: tx_pin,
        fun: GLB_GPIO_FUN_UART,
        mode: GLB_GPIO_MODE_AF,
        pull_type: GLB_GPIO_PULL_UP,
        drive: 1,
        smt_ctrl: 1,
    };
    GPIO::from(cfg_tx);

    /* 3. Select UART GPIO function */
    let tx_sigfun: GLB_UART_SIG_FUN_Type;
    let rx_sigfun: GLB_UART_SIG_FUN_Type;
    match id {
        0 => {
            tx_sigfun = GLB_UART_SIG_FUN_Type_GLB_UART_SIG_FUN_UART0_TXD;
            rx_sigfun = GLB_UART_SIG_FUN_Type_GLB_UART_SIG_FUN_UART0_RXD
        }
        1 => {
            tx_sigfun = GLB_UART_SIG_FUN_Type_GLB_UART_SIG_FUN_UART1_TXD;
            rx_sigfun = GLB_UART_SIG_FUN_Type_GLB_UART_SIG_FUN_UART1_RXD
        }
        _ => {
            panic!("Invalid UART channel selected");
        }
    }

    low_uart_select_signal_function((tx_pin % 8).into(), tx_sigfun);
    low_uart_select_signal_function((rx_pin % 8).into(), rx_sigfun);
}

fn low_uart_init(id: UART_ID_Type, cfg: &UART_CFG_Type) {
    // Only support channel 0 for now
    if id != 0 {
        panic!("Invalid UART channel selected");
    }

    let channel: &pac::uart0::RegisterBlock = unsafe { &*pac::Uart0::ptr() };

    /* 1. Set baud rate */
    /* 1.1. Compute register value */
    let fraction = cfg.uartClk * 10 / cfg.baudRate % 10;
    let mut baudRateDivisor = cfg.uartClk / cfg.baudRate;
    if fraction >= 5 {
        baudRateDivisor += 1;
    }

    /* 1.2. Set register value */
    channel.uart_bit_prd().write(|w| unsafe {
        w.bits(((baudRateDivisor - 1) << 0x10) | ((baudRateDivisor - 1) & 0xFFFF))
    });

    /* 2. Configure TX */
    channel.utx_config().modify(|_r, w| {
        /* 2.1. Configure parity */
        match cfg.parity {
            UART_Parity_Type_UART_PARITY_NONE => {
                w.cr_utx_prt_en().clear_bit();
            }
            UART_Parity_Type_UART_PARITY_ODD => {
                w.cr_utx_prt_en().set_bit();
                w.cr_utx_prt_sel().set_bit();
            }
            UART_Parity_Type_UART_PARITY_EVEN => {
                w.cr_utx_prt_en().set_bit();
                w.cr_utx_prt_sel().clear_bit();
            }
            _ => {
                panic!("Invalid TX parity configuration for UART");
            }
        }

        unsafe {
            /* 2.2. Configure data bits */
            w.cr_utx_bit_cnt_d()
                .bits((cfg.dataBits + 4).try_into().unwrap());
            /* 2.3. Configure TX stop pits */
            w.cr_utx_bit_cnt_p()
                .bits((cfg.stopBits + 1).try_into().unwrap());
        }

        /* 2.4. Configure TX cts flow control function */
        match cfg.ctsFlowControl {
            BL_Fun_Type_ENABLE => w.cr_utx_cts_en().set_bit(),
            BL_Fun_Type_DISABLE => w.cr_utx_cts_en().clear_bit(),
            _ => {
                panic!("Invalid CTS flow control configuration for UART");
            }
        }
    });

    /* 3. Configure RX */
    channel.urx_config().modify(|_r, w| {
        /* 3.1. Configure parity */
        match cfg.parity {
            UART_Parity_Type_UART_PARITY_NONE => {
                w.cr_urx_prt_en().clear_bit();
            }
            UART_Parity_Type_UART_PARITY_ODD => {
                w.cr_urx_prt_en().set_bit();
                w.cr_urx_prt_sel().set_bit();
            }
            UART_Parity_Type_UART_PARITY_EVEN => {
                w.cr_urx_prt_en().set_bit();
                w.cr_urx_prt_sel().clear_bit();
            }
            _ => {
                panic!("Invalid RX parity configuration for UART");
            }
        }

        /* 3.2. Configure data bits */
        unsafe {
            w.cr_urx_bit_cnt_d()
                .bits((cfg.dataBits + 4).try_into().unwrap());
        }

        /* 3.3. Configure RX input de-glitch function */
        match cfg.rxDeglitch {
            BL_Fun_Type_ENABLE => {
                w.cr_urx_deg_en().set_bit();
            }
            BL_Fun_Type_DISABLE => {
                w.cr_urx_deg_en().clear_bit();
            }
            _ => {
                panic!("Invalid RX deglitch configuration for UART");
            }
        }

        /* 3.4 Configure RX RTS output SW control mode */
        match cfg.rtsSoftwareControl {
            BL_Fun_Type_ENABLE => w.cr_urx_rts_sw_mode().set_bit(),
            BL_Fun_Type_DISABLE => w.cr_urx_rts_sw_mode().clear_bit(),
            _ => {
                panic!("Invalid RX RTS output SW control mode configuration for UART");
            }
        }
    });

    /* 4. Configure LSB-first or MSB-first  */
    channel
        .data_config()
        .modify(|_r, w| match cfg.byteBitInverse {
            UART_ByteBitInverse_Type_UART_MSB_FIRST => w.cr_uart_bit_inv().set_bit(),
            UART_ByteBitInverse_Type_UART_LSB_FIRST => w.cr_uart_bit_inv().clear_bit(),
            _ => {
                panic!("Invalid LSB/MSB mode set for UART");
            }
        });
}

fn low_uart_int_mask(id: UART_ID_Type, int_type: UART_INT_Type, mask: BL_Mask_Type) {
    if id != 0 {
        panic!("Unsupported UART channel selected");
    }
    let channel: &pac::uart0::RegisterBlock = unsafe { &*pac::Uart0::ptr() };
    let value: bool = mask == BL_Mask_Type_MASK;

    channel.uart_int_mask().modify(|_r, w| match int_type {
        UART_INT_Type_UART_INT_TX_END => w.cr_utx_end_mask().bit(value),
        UART_INT_Type_UART_INT_RX_END => w.cr_urx_end_mask().bit(value),
        UART_INT_Type_UART_INT_TX_FIFO_REQ => w.cr_utx_fifo_mask().bit(value),
        UART_INT_Type_UART_INT_RX_FIFO_REQ => w.cr_urx_fifo_mask().bit(value),
        UART_INT_Type_UART_INT_RTO => w.cr_urx_rto_mask().bit(value),
        UART_INT_Type_UART_INT_PCE => w.cr_urx_pce_mask().bit(value),
        UART_INT_Type_UART_INT_TX_FER => w.cr_utx_fer_mask().bit(value),
        UART_INT_Type_UART_INT_RX_FER => w.cr_urx_fer_mask().bit(value),
        UART_INT_Type_UART_INT_ALL => {
            w.cr_utx_end_mask().bit(value);
            w.cr_urx_end_mask().bit(value);
            w.cr_utx_fifo_mask().bit(value);
            w.cr_urx_fifo_mask().bit(value);
            w.cr_urx_rto_mask().bit(value);
            w.cr_urx_pce_mask().bit(value);
            w.cr_utx_fer_mask().bit(value);
            w.cr_urx_fer_mask().bit(value)
        }
        _ => unreachable!(),
    });
}

fn low_uart_fifo_config(id: UART_ID_Type, cfg: &UART_FifoCfg_Type) {
    // Only support channel 0 for now
    if id != 0 {
        panic!("Invalid UART channel selected");
    }

    let channel: &pac::uart0::RegisterBlock = unsafe { &*pac::Uart0::ptr() };

    /* 1. Configure FiFo */
    channel.uart_fifo_config_1().modify(|_r, w| unsafe {
        w.tx_fifo_th().bits(cfg.txFifoDmaThreshold - 1);
        w.rx_fifo_th().bits(cfg.rxFifoDmaThreshold - 1)
    });

    /* 2. Enable/Disable FiFo DMA */
    channel.uart_fifo_config_0().modify(|_r, w| {
        /* 2.1. TX */
        match cfg.txFifoDmaEnable {
            BL_Fun_Type_ENABLE => {
                w.uart_dma_tx_en().set_bit();
            }
            BL_Fun_Type_DISABLE => {
                w.uart_dma_tx_en().clear_bit();
            }
            _ => {
                panic!("Invalid DMA mode set for UART TX");
            }
        }

        /* 2.2 RX */
        match cfg.rxFifoDmaEnable {
            BL_Fun_Type_ENABLE => w.uart_dma_rx_en().set_bit(),
            BL_Fun_Type_DISABLE => w.uart_dma_rx_en().clear_bit(),
            _ => {
                panic!("Invalid DMA mode set for UART RX");
            }
        }
    });
}

fn low_uart_disable(id: UART_ID_Type, direction: UART_Direction_Type) {
    if id != 0 {
        panic!("Unsupported UART channel selected");
    }

    let channel: &pac::uart0::RegisterBlock = unsafe { &*pac::Uart0::ptr() };

    // Disable TX unit
    if (direction == UART_Direction_Type_UART_TX) || (direction == UART_Direction_Type_UART_TXRX) {
        channel
            .utx_config()
            .modify(|_r, w| w.cr_utx_en().clear_bit());
    }

    // Disable RX unit
    if (direction == UART_Direction_Type_UART_RX) || (direction == UART_Direction_Type_UART_TXRX) {
        channel
            .urx_config()
            .modify(|_r, w| w.cr_urx_en().clear_bit());
    }
}

fn low_uart_enable(id: UART_ID_Type, direction: UART_Direction_Type) {
    if id != 0 {
        panic!("Unsupported UART channel selected");
    }

    let channel: &pac::uart0::RegisterBlock = unsafe { &*pac::Uart0::ptr() };

    // Enable TX unit
    if (direction == UART_Direction_Type_UART_TX) || (direction == UART_Direction_Type_UART_TXRX) {
        channel.utx_config().modify(|_r, w| w.cr_utx_en().set_bit());
    }

    // Enable RX unit
    if (direction == UART_Direction_Type_UART_RX) || (direction == UART_Direction_Type_UART_TXRX) {
        channel.urx_config().modify(|_r, w| w.cr_urx_en().set_bit());
    }
}

fn low_uart_conf_tx_free_run(id: UART_ID_Type, freeRun: BL_FUN_Type) {
    if id != 0 {
        panic!("Unsupported UART channel selected");
    }

    let channel: &pac::uart0::RegisterBlock = unsafe { &*pac::Uart0::ptr() };
    channel.utx_config().modify(|_r, w| match freeRun {
        BL_Fun_Type_ENABLE => w.cr_utx_frm_en().set_bit(),
        BL_Fun_Type_DISABLE => w.cr_utx_frm_en().clear_bit(),
        _ => unreachable!(),
    });
}

fn low_uart_get_tx_fifo_count(id: UART_ID_Type) -> u8 {
    if id != 0 {
        panic!("Unsupported UART channel selected");
    }

    let channel: &pac::uart0::RegisterBlock = unsafe { &*pac::Uart0::ptr() };
    channel.uart_fifo_config_1().read().tx_fifo_cnt().bits()
}
