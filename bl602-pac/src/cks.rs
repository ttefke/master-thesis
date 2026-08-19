#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cks_config: CksConfig,
    data_in: DataIn,
    cks_out: CksOut,
}
impl RegisterBlock {
    #[doc = "0x00 - cks_config."]
    #[inline(always)]
    pub const fn cks_config(&self) -> &CksConfig {
        &self.cks_config
    }
    #[doc = "0x04 - data_in."]
    #[inline(always)]
    pub const fn data_in(&self) -> &DataIn {
        &self.data_in
    }
    #[doc = "0x08 - cks_out."]
    #[inline(always)]
    pub const fn cks_out(&self) -> &CksOut {
        &self.cks_out
    }
}
#[doc = "cks_config (rw) register accessor: cks_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`cks_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cks_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cks_config`] module"]
#[doc(alias = "cks_config")]
pub type CksConfig = crate::Reg<cks_config::CksConfigSpec>;
#[doc = "cks_config."]
pub mod cks_config;
#[doc = "data_in (rw) register accessor: data_in.\n\nYou can [`read`](crate::Reg::read) this register and get [`data_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_in`] module"]
#[doc(alias = "data_in")]
pub type DataIn = crate::Reg<data_in::DataInSpec>;
#[doc = "data_in."]
pub mod data_in;
#[doc = "cks_out (rw) register accessor: cks_out.\n\nYou can [`read`](crate::Reg::read) this register and get [`cks_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cks_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cks_out`] module"]
#[doc(alias = "cks_out")]
pub type CksOut = crate::Reg<cks_out::CksOutSpec>;
#[doc = "cks_out."]
pub mod cks_out;
