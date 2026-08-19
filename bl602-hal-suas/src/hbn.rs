#[doc = "< Select FCLK as UART clock"]
pub const HBN_UART_CLK_TYPE_HBN_UART_CLK_FCLK: HbnUartClkType = 0;
#[doc = "< Select 160M as UART clock"]
pub const HBN_UART_CLK_TYPE_HBN_UART_CLK_160_M: HbnUartClkType = 1;
pub type HbnUartClkType = u32;

//TODO: Check
pub fn set_uart_clk_sel(clock: HbnUartClkType) {
    let hbn = unsafe { &*pac::Hbn::ptr() };
    hbn.hbn_glb().modify(|_r, w| {
        w.hbn_uart_clk_sel()
            .bit(clock == HBN_UART_CLK_TYPE_HBN_UART_CLK_160_M)
    });
}
