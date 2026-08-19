#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dma_int_status: DmaIntStatus,
    dma_int_tcstatus: DmaIntTcstatus,
    dma_int_tcclear: DmaIntTcclear,
    dma_int_error_status: DmaIntErrorStatus,
    dma_int_err_clr: DmaIntErrClr,
    dma_raw_int_tcstatus: DmaRawIntTcstatus,
    dma_raw_int_error_status: DmaRawIntErrorStatus,
    dma_enbld_chns: DmaEnbldChns,
    dma_soft_breq: DmaSoftBreq,
    dma_soft_sreq: DmaSoftSreq,
    dma_soft_lbreq: DmaSoftLbreq,
    dma_soft_lsreq: DmaSoftLsreq,
    dma_top_config: DmaTopConfig,
    dma_sync: DmaSync,
    _reserved14: [u8; 0xc8],
    dma_c0src_addr: DmaC0srcAddr,
    dma_c0dst_addr: DmaC0dstAddr,
    dma_c0lli: DmaC0lli,
    dma_c0control: DmaC0control,
    dma_c0config: DmaC0config,
    _reserved19: [u8; 0xec],
    dma_c1src_addr: DmaC1srcAddr,
    dma_c1dst_addr: DmaC1dstAddr,
    dma_c1lli: DmaC1lli,
    dma_c1control: DmaC1control,
    dma_c1config: DmaC1config,
    _reserved24: [u8; 0xec],
    dma_c2src_addr: DmaC2srcAddr,
    dma_c2dst_addr: DmaC2dstAddr,
    dma_c2lli: DmaC2lli,
    dma_c2control: DmaC2control,
    dma_c2config: DmaC2config,
    _reserved29: [u8; 0xec],
    dma_c3src_addr: DmaC3srcAddr,
    dma_c3dst_addr: DmaC3dstAddr,
    dma_c3lli: DmaC3lli,
    dma_c3control: DmaC3control,
    dma_c3config: DmaC3config,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA_IntStatus."]
    #[inline(always)]
    pub const fn dma_int_status(&self) -> &DmaIntStatus {
        &self.dma_int_status
    }
    #[doc = "0x04 - DMA_IntTCStatus."]
    #[inline(always)]
    pub const fn dma_int_tcstatus(&self) -> &DmaIntTcstatus {
        &self.dma_int_tcstatus
    }
    #[doc = "0x08 - DMA_IntTCClear."]
    #[inline(always)]
    pub const fn dma_int_tcclear(&self) -> &DmaIntTcclear {
        &self.dma_int_tcclear
    }
    #[doc = "0x0c - DMA_IntErrorStatus."]
    #[inline(always)]
    pub const fn dma_int_error_status(&self) -> &DmaIntErrorStatus {
        &self.dma_int_error_status
    }
    #[doc = "0x10 - DMA_IntErrClr."]
    #[inline(always)]
    pub const fn dma_int_err_clr(&self) -> &DmaIntErrClr {
        &self.dma_int_err_clr
    }
    #[doc = "0x14 - DMA_RawIntTCStatus."]
    #[inline(always)]
    pub const fn dma_raw_int_tcstatus(&self) -> &DmaRawIntTcstatus {
        &self.dma_raw_int_tcstatus
    }
    #[doc = "0x18 - DMA_RawIntErrorStatus."]
    #[inline(always)]
    pub const fn dma_raw_int_error_status(&self) -> &DmaRawIntErrorStatus {
        &self.dma_raw_int_error_status
    }
    #[doc = "0x1c - DMA_EnbldChns."]
    #[inline(always)]
    pub const fn dma_enbld_chns(&self) -> &DmaEnbldChns {
        &self.dma_enbld_chns
    }
    #[doc = "0x20 - DMA_SoftBReq."]
    #[inline(always)]
    pub const fn dma_soft_breq(&self) -> &DmaSoftBreq {
        &self.dma_soft_breq
    }
    #[doc = "0x24 - DMA_SoftSReq."]
    #[inline(always)]
    pub const fn dma_soft_sreq(&self) -> &DmaSoftSreq {
        &self.dma_soft_sreq
    }
    #[doc = "0x28 - DMA_SoftLBReq."]
    #[inline(always)]
    pub const fn dma_soft_lbreq(&self) -> &DmaSoftLbreq {
        &self.dma_soft_lbreq
    }
    #[doc = "0x2c - DMA_SoftLSReq."]
    #[inline(always)]
    pub const fn dma_soft_lsreq(&self) -> &DmaSoftLsreq {
        &self.dma_soft_lsreq
    }
    #[doc = "0x30 - DMA_Top_Config."]
    #[inline(always)]
    pub const fn dma_top_config(&self) -> &DmaTopConfig {
        &self.dma_top_config
    }
    #[doc = "0x34 - DMA_Sync."]
    #[inline(always)]
    pub const fn dma_sync(&self) -> &DmaSync {
        &self.dma_sync
    }
    #[doc = "0x100 - DMA_C0SrcAddr."]
    #[inline(always)]
    pub const fn dma_c0src_addr(&self) -> &DmaC0srcAddr {
        &self.dma_c0src_addr
    }
    #[doc = "0x104 - DMA_C0DstAddr."]
    #[inline(always)]
    pub const fn dma_c0dst_addr(&self) -> &DmaC0dstAddr {
        &self.dma_c0dst_addr
    }
    #[doc = "0x108 - DMA_C0LLI."]
    #[inline(always)]
    pub const fn dma_c0lli(&self) -> &DmaC0lli {
        &self.dma_c0lli
    }
    #[doc = "0x10c - DMA_C0Control."]
    #[inline(always)]
    pub const fn dma_c0control(&self) -> &DmaC0control {
        &self.dma_c0control
    }
    #[doc = "0x110 - DMA_C0Config."]
    #[inline(always)]
    pub const fn dma_c0config(&self) -> &DmaC0config {
        &self.dma_c0config
    }
    #[doc = "0x200 - DMA_C1SrcAddr."]
    #[inline(always)]
    pub const fn dma_c1src_addr(&self) -> &DmaC1srcAddr {
        &self.dma_c1src_addr
    }
    #[doc = "0x204 - DMA_C1DstAddr."]
    #[inline(always)]
    pub const fn dma_c1dst_addr(&self) -> &DmaC1dstAddr {
        &self.dma_c1dst_addr
    }
    #[doc = "0x208 - DMA_C1LLI."]
    #[inline(always)]
    pub const fn dma_c1lli(&self) -> &DmaC1lli {
        &self.dma_c1lli
    }
    #[doc = "0x20c - DMA_C1Control."]
    #[inline(always)]
    pub const fn dma_c1control(&self) -> &DmaC1control {
        &self.dma_c1control
    }
    #[doc = "0x210 - DMA_C1Config."]
    #[inline(always)]
    pub const fn dma_c1config(&self) -> &DmaC1config {
        &self.dma_c1config
    }
    #[doc = "0x300 - DMA_C2SrcAddr."]
    #[inline(always)]
    pub const fn dma_c2src_addr(&self) -> &DmaC2srcAddr {
        &self.dma_c2src_addr
    }
    #[doc = "0x304 - DMA_C2DstAddr."]
    #[inline(always)]
    pub const fn dma_c2dst_addr(&self) -> &DmaC2dstAddr {
        &self.dma_c2dst_addr
    }
    #[doc = "0x308 - DMA_C2LLI."]
    #[inline(always)]
    pub const fn dma_c2lli(&self) -> &DmaC2lli {
        &self.dma_c2lli
    }
    #[doc = "0x30c - DMA_C2Control."]
    #[inline(always)]
    pub const fn dma_c2control(&self) -> &DmaC2control {
        &self.dma_c2control
    }
    #[doc = "0x310 - DMA_C2Config."]
    #[inline(always)]
    pub const fn dma_c2config(&self) -> &DmaC2config {
        &self.dma_c2config
    }
    #[doc = "0x400 - DMA_C3SrcAddr."]
    #[inline(always)]
    pub const fn dma_c3src_addr(&self) -> &DmaC3srcAddr {
        &self.dma_c3src_addr
    }
    #[doc = "0x404 - DMA_C3DstAddr."]
    #[inline(always)]
    pub const fn dma_c3dst_addr(&self) -> &DmaC3dstAddr {
        &self.dma_c3dst_addr
    }
    #[doc = "0x408 - DMA_C3LLI."]
    #[inline(always)]
    pub const fn dma_c3lli(&self) -> &DmaC3lli {
        &self.dma_c3lli
    }
    #[doc = "0x40c - DMA_C3Control."]
    #[inline(always)]
    pub const fn dma_c3control(&self) -> &DmaC3control {
        &self.dma_c3control
    }
    #[doc = "0x410 - DMA_C3Config."]
    #[inline(always)]
    pub const fn dma_c3config(&self) -> &DmaC3config {
        &self.dma_c3config
    }
}
#[doc = "DMA_IntStatus (rw) register accessor: DMA_IntStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_status`] module"]
#[doc(alias = "DMA_IntStatus")]
pub type DmaIntStatus = crate::Reg<dma_int_status::DmaIntStatusSpec>;
#[doc = "DMA_IntStatus."]
pub mod dma_int_status;
#[doc = "DMA_IntTCStatus (rw) register accessor: DMA_IntTCStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_tcstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_tcstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_tcstatus`] module"]
#[doc(alias = "DMA_IntTCStatus")]
pub type DmaIntTcstatus = crate::Reg<dma_int_tcstatus::DmaIntTcstatusSpec>;
#[doc = "DMA_IntTCStatus."]
pub mod dma_int_tcstatus;
#[doc = "DMA_IntTCClear (rw) register accessor: DMA_IntTCClear.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_tcclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_tcclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_tcclear`] module"]
#[doc(alias = "DMA_IntTCClear")]
pub type DmaIntTcclear = crate::Reg<dma_int_tcclear::DmaIntTcclearSpec>;
#[doc = "DMA_IntTCClear."]
pub mod dma_int_tcclear;
#[doc = "DMA_IntErrorStatus (rw) register accessor: DMA_IntErrorStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_error_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_error_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_error_status`] module"]
#[doc(alias = "DMA_IntErrorStatus")]
pub type DmaIntErrorStatus = crate::Reg<dma_int_error_status::DmaIntErrorStatusSpec>;
#[doc = "DMA_IntErrorStatus."]
pub mod dma_int_error_status;
#[doc = "DMA_IntErrClr (rw) register accessor: DMA_IntErrClr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_err_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_err_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_err_clr`] module"]
#[doc(alias = "DMA_IntErrClr")]
pub type DmaIntErrClr = crate::Reg<dma_int_err_clr::DmaIntErrClrSpec>;
#[doc = "DMA_IntErrClr."]
pub mod dma_int_err_clr;
#[doc = "DMA_RawIntTCStatus (rw) register accessor: DMA_RawIntTCStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_raw_int_tcstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_raw_int_tcstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_raw_int_tcstatus`] module"]
#[doc(alias = "DMA_RawIntTCStatus")]
pub type DmaRawIntTcstatus = crate::Reg<dma_raw_int_tcstatus::DmaRawIntTcstatusSpec>;
#[doc = "DMA_RawIntTCStatus."]
pub mod dma_raw_int_tcstatus;
#[doc = "DMA_RawIntErrorStatus (rw) register accessor: DMA_RawIntErrorStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_raw_int_error_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_raw_int_error_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_raw_int_error_status`] module"]
#[doc(alias = "DMA_RawIntErrorStatus")]
pub type DmaRawIntErrorStatus = crate::Reg<dma_raw_int_error_status::DmaRawIntErrorStatusSpec>;
#[doc = "DMA_RawIntErrorStatus."]
pub mod dma_raw_int_error_status;
#[doc = "DMA_EnbldChns (rw) register accessor: DMA_EnbldChns.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_enbld_chns::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_enbld_chns::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_enbld_chns`] module"]
#[doc(alias = "DMA_EnbldChns")]
pub type DmaEnbldChns = crate::Reg<dma_enbld_chns::DmaEnbldChnsSpec>;
#[doc = "DMA_EnbldChns."]
pub mod dma_enbld_chns;
#[doc = "DMA_SoftBReq (rw) register accessor: DMA_SoftBReq.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_breq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_breq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_soft_breq`] module"]
#[doc(alias = "DMA_SoftBReq")]
pub type DmaSoftBreq = crate::Reg<dma_soft_breq::DmaSoftBreqSpec>;
#[doc = "DMA_SoftBReq."]
pub mod dma_soft_breq;
#[doc = "DMA_SoftSReq (rw) register accessor: DMA_SoftSReq.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_sreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_sreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_soft_sreq`] module"]
#[doc(alias = "DMA_SoftSReq")]
pub type DmaSoftSreq = crate::Reg<dma_soft_sreq::DmaSoftSreqSpec>;
#[doc = "DMA_SoftSReq."]
pub mod dma_soft_sreq;
#[doc = "DMA_SoftLBReq (rw) register accessor: DMA_SoftLBReq.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_lbreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_lbreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_soft_lbreq`] module"]
#[doc(alias = "DMA_SoftLBReq")]
pub type DmaSoftLbreq = crate::Reg<dma_soft_lbreq::DmaSoftLbreqSpec>;
#[doc = "DMA_SoftLBReq."]
pub mod dma_soft_lbreq;
#[doc = "DMA_SoftLSReq (rw) register accessor: DMA_SoftLSReq.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_lsreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_lsreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_soft_lsreq`] module"]
#[doc(alias = "DMA_SoftLSReq")]
pub type DmaSoftLsreq = crate::Reg<dma_soft_lsreq::DmaSoftLsreqSpec>;
#[doc = "DMA_SoftLSReq."]
pub mod dma_soft_lsreq;
#[doc = "DMA_Top_Config (rw) register accessor: DMA_Top_Config.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_top_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_top_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_top_config`] module"]
#[doc(alias = "DMA_Top_Config")]
pub type DmaTopConfig = crate::Reg<dma_top_config::DmaTopConfigSpec>;
#[doc = "DMA_Top_Config."]
pub mod dma_top_config;
#[doc = "DMA_Sync (rw) register accessor: DMA_Sync.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_sync`] module"]
#[doc(alias = "DMA_Sync")]
pub type DmaSync = crate::Reg<dma_sync::DmaSyncSpec>;
#[doc = "DMA_Sync."]
pub mod dma_sync;
#[doc = "DMA_C0SrcAddr (rw) register accessor: DMA_C0SrcAddr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c0src_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c0src_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c0src_addr`] module"]
#[doc(alias = "DMA_C0SrcAddr")]
pub type DmaC0srcAddr = crate::Reg<dma_c0src_addr::DmaC0srcAddrSpec>;
#[doc = "DMA_C0SrcAddr."]
pub mod dma_c0src_addr;
#[doc = "DMA_C0DstAddr (rw) register accessor: DMA_C0DstAddr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c0dst_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c0dst_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c0dst_addr`] module"]
#[doc(alias = "DMA_C0DstAddr")]
pub type DmaC0dstAddr = crate::Reg<dma_c0dst_addr::DmaC0dstAddrSpec>;
#[doc = "DMA_C0DstAddr."]
pub mod dma_c0dst_addr;
#[doc = "DMA_C0LLI (rw) register accessor: DMA_C0LLI.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c0lli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c0lli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c0lli`] module"]
#[doc(alias = "DMA_C0LLI")]
pub type DmaC0lli = crate::Reg<dma_c0lli::DmaC0lliSpec>;
#[doc = "DMA_C0LLI."]
pub mod dma_c0lli;
#[doc = "DMA_C0Control (rw) register accessor: DMA_C0Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c0control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c0control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c0control`] module"]
#[doc(alias = "DMA_C0Control")]
pub type DmaC0control = crate::Reg<dma_c0control::DmaC0controlSpec>;
#[doc = "DMA_C0Control."]
pub mod dma_c0control;
#[doc = "DMA_C0Config (rw) register accessor: DMA_C0Config.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c0config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c0config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c0config`] module"]
#[doc(alias = "DMA_C0Config")]
pub type DmaC0config = crate::Reg<dma_c0config::DmaC0configSpec>;
#[doc = "DMA_C0Config."]
pub mod dma_c0config;
#[doc = "DMA_C1SrcAddr (rw) register accessor: DMA_C1SrcAddr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c1src_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c1src_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c1src_addr`] module"]
#[doc(alias = "DMA_C1SrcAddr")]
pub type DmaC1srcAddr = crate::Reg<dma_c1src_addr::DmaC1srcAddrSpec>;
#[doc = "DMA_C1SrcAddr."]
pub mod dma_c1src_addr;
#[doc = "DMA_C1DstAddr (rw) register accessor: DMA_C1DstAddr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c1dst_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c1dst_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c1dst_addr`] module"]
#[doc(alias = "DMA_C1DstAddr")]
pub type DmaC1dstAddr = crate::Reg<dma_c1dst_addr::DmaC1dstAddrSpec>;
#[doc = "DMA_C1DstAddr."]
pub mod dma_c1dst_addr;
#[doc = "DMA_C1LLI (rw) register accessor: DMA_C1LLI.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c1lli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c1lli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c1lli`] module"]
#[doc(alias = "DMA_C1LLI")]
pub type DmaC1lli = crate::Reg<dma_c1lli::DmaC1lliSpec>;
#[doc = "DMA_C1LLI."]
pub mod dma_c1lli;
#[doc = "DMA_C1Control (rw) register accessor: DMA_C1Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c1control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c1control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c1control`] module"]
#[doc(alias = "DMA_C1Control")]
pub type DmaC1control = crate::Reg<dma_c1control::DmaC1controlSpec>;
#[doc = "DMA_C1Control."]
pub mod dma_c1control;
#[doc = "DMA_C1Config (rw) register accessor: DMA_C1Config.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c1config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c1config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c1config`] module"]
#[doc(alias = "DMA_C1Config")]
pub type DmaC1config = crate::Reg<dma_c1config::DmaC1configSpec>;
#[doc = "DMA_C1Config."]
pub mod dma_c1config;
#[doc = "DMA_C2SrcAddr (rw) register accessor: DMA_C2SrcAddr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c2src_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c2src_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c2src_addr`] module"]
#[doc(alias = "DMA_C2SrcAddr")]
pub type DmaC2srcAddr = crate::Reg<dma_c2src_addr::DmaC2srcAddrSpec>;
#[doc = "DMA_C2SrcAddr."]
pub mod dma_c2src_addr;
#[doc = "DMA_C2DstAddr (rw) register accessor: DMA_C2DstAddr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c2dst_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c2dst_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c2dst_addr`] module"]
#[doc(alias = "DMA_C2DstAddr")]
pub type DmaC2dstAddr = crate::Reg<dma_c2dst_addr::DmaC2dstAddrSpec>;
#[doc = "DMA_C2DstAddr."]
pub mod dma_c2dst_addr;
#[doc = "DMA_C2LLI (rw) register accessor: DMA_C2LLI.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c2lli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c2lli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c2lli`] module"]
#[doc(alias = "DMA_C2LLI")]
pub type DmaC2lli = crate::Reg<dma_c2lli::DmaC2lliSpec>;
#[doc = "DMA_C2LLI."]
pub mod dma_c2lli;
#[doc = "DMA_C2Control (rw) register accessor: DMA_C2Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c2control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c2control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c2control`] module"]
#[doc(alias = "DMA_C2Control")]
pub type DmaC2control = crate::Reg<dma_c2control::DmaC2controlSpec>;
#[doc = "DMA_C2Control."]
pub mod dma_c2control;
#[doc = "DMA_C2Config (rw) register accessor: DMA_C2Config.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c2config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c2config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c2config`] module"]
#[doc(alias = "DMA_C2Config")]
pub type DmaC2config = crate::Reg<dma_c2config::DmaC2configSpec>;
#[doc = "DMA_C2Config."]
pub mod dma_c2config;
#[doc = "DMA_C3SrcAddr (rw) register accessor: DMA_C3SrcAddr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c3src_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c3src_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c3src_addr`] module"]
#[doc(alias = "DMA_C3SrcAddr")]
pub type DmaC3srcAddr = crate::Reg<dma_c3src_addr::DmaC3srcAddrSpec>;
#[doc = "DMA_C3SrcAddr."]
pub mod dma_c3src_addr;
#[doc = "DMA_C3DstAddr (rw) register accessor: DMA_C3DstAddr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c3dst_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c3dst_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c3dst_addr`] module"]
#[doc(alias = "DMA_C3DstAddr")]
pub type DmaC3dstAddr = crate::Reg<dma_c3dst_addr::DmaC3dstAddrSpec>;
#[doc = "DMA_C3DstAddr."]
pub mod dma_c3dst_addr;
#[doc = "DMA_C3LLI (rw) register accessor: DMA_C3LLI.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c3lli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c3lli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c3lli`] module"]
#[doc(alias = "DMA_C3LLI")]
pub type DmaC3lli = crate::Reg<dma_c3lli::DmaC3lliSpec>;
#[doc = "DMA_C3LLI."]
pub mod dma_c3lli;
#[doc = "DMA_C3Control (rw) register accessor: DMA_C3Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c3control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c3control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c3control`] module"]
#[doc(alias = "DMA_C3Control")]
pub type DmaC3control = crate::Reg<dma_c3control::DmaC3controlSpec>;
#[doc = "DMA_C3Control."]
pub mod dma_c3control;
#[doc = "DMA_C3Config (rw) register accessor: DMA_C3Config.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c3config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c3config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_c3config`] module"]
#[doc(alias = "DMA_C3Config")]
pub type DmaC3config = crate::Reg<dma_c3config::DmaC3configSpec>;
#[doc = "DMA_C3Config."]
pub mod dma_c3config;
