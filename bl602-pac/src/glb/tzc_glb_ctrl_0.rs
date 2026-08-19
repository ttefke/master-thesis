#[doc = "Register `tzc_glb_ctrl_0` reader"]
pub type R = crate::R<TzcGlbCtrl0Spec>;
#[doc = "Register `tzc_glb_ctrl_0` writer"]
pub type W = crate::W<TzcGlbCtrl0Spec>;
#[doc = "Field `tzc_glb_swrst_s00_lock` reader - "]
pub type TzcGlbSwrstS00LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s00_lock` writer - "]
pub type TzcGlbSwrstS00LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_swrst_s01_lock` reader - "]
pub type TzcGlbSwrstS01LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s01_lock` writer - "]
pub type TzcGlbSwrstS01LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_swrst_s30_lock` reader - "]
pub type TzcGlbSwrstS30LockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_s30_lock` writer - "]
pub type TzcGlbSwrstS30LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_ctrl_pwron_rst_lock` reader - "]
pub type TzcGlbCtrlPwronRstLockR = crate::BitReader;
#[doc = "Field `tzc_glb_ctrl_pwron_rst_lock` writer - "]
pub type TzcGlbCtrlPwronRstLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_ctrl_cpu_reset_lock` reader - "]
pub type TzcGlbCtrlCpuResetLockR = crate::BitReader;
#[doc = "Field `tzc_glb_ctrl_cpu_reset_lock` writer - "]
pub type TzcGlbCtrlCpuResetLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_ctrl_sys_reset_lock` reader - "]
pub type TzcGlbCtrlSysResetLockR = crate::BitReader;
#[doc = "Field `tzc_glb_ctrl_sys_reset_lock` writer - "]
pub type TzcGlbCtrlSysResetLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_ctrl_ungated_ap_lock` reader - "]
pub type TzcGlbCtrlUngatedApLockR = crate::BitReader;
#[doc = "Field `tzc_glb_ctrl_ungated_ap_lock` writer - "]
pub type TzcGlbCtrlUngatedApLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_misc_lock` reader - "]
pub type TzcGlbMiscLockR = crate::BitReader;
#[doc = "Field `tzc_glb_misc_lock` writer - "]
pub type TzcGlbMiscLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_sram_lock` reader - "]
pub type TzcGlbSramLockR = crate::BitReader;
#[doc = "Field `tzc_glb_sram_lock` writer - "]
pub type TzcGlbSramLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_l2c_lock` reader - "]
pub type TzcGlbL2cLockR = crate::BitReader;
#[doc = "Field `tzc_glb_l2c_lock` writer - "]
pub type TzcGlbL2cLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_bmx_lock` reader - "]
pub type TzcGlbBmxLockR = crate::BitReader;
#[doc = "Field `tzc_glb_bmx_lock` writer - "]
pub type TzcGlbBmxLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_dbg_lock` reader - "]
pub type TzcGlbDbgLockR = crate::BitReader;
#[doc = "Field `tzc_glb_dbg_lock` writer - "]
pub type TzcGlbDbgLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_mbist_lock` reader - "]
pub type TzcGlbMbistLockR = crate::BitReader;
#[doc = "Field `tzc_glb_mbist_lock` writer - "]
pub type TzcGlbMbistLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_clk_lock` reader - "]
pub type TzcGlbClkLockR = crate::BitReader;
#[doc = "Field `tzc_glb_clk_lock` writer - "]
pub type TzcGlbClkLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s00_lock(&self) -> TzcGlbSwrstS00LockR {
        TzcGlbSwrstS00LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s01_lock(&self) -> TzcGlbSwrstS01LockR {
        TzcGlbSwrstS01LockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s30_lock(&self) -> TzcGlbSwrstS30LockR {
        TzcGlbSwrstS30LockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_pwron_rst_lock(&self) -> TzcGlbCtrlPwronRstLockR {
        TzcGlbCtrlPwronRstLockR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_cpu_reset_lock(&self) -> TzcGlbCtrlCpuResetLockR {
        TzcGlbCtrlCpuResetLockR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_sys_reset_lock(&self) -> TzcGlbCtrlSysResetLockR {
        TzcGlbCtrlSysResetLockR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_ungated_ap_lock(&self) -> TzcGlbCtrlUngatedApLockR {
        TzcGlbCtrlUngatedApLockR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_misc_lock(&self) -> TzcGlbMiscLockR {
        TzcGlbMiscLockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_sram_lock(&self) -> TzcGlbSramLockR {
        TzcGlbSramLockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_l2c_lock(&self) -> TzcGlbL2cLockR {
        TzcGlbL2cLockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_bmx_lock(&self) -> TzcGlbBmxLockR {
        TzcGlbBmxLockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_glb_dbg_lock(&self) -> TzcGlbDbgLockR {
        TzcGlbDbgLockR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_glb_mbist_lock(&self) -> TzcGlbMbistLockR {
        TzcGlbMbistLockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_glb_clk_lock(&self) -> TzcGlbClkLockR {
        TzcGlbClkLockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s00_lock(&mut self) -> TzcGlbSwrstS00LockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbSwrstS00LockW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s01_lock(&mut self) -> TzcGlbSwrstS01LockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbSwrstS01LockW::new(self, 1)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s30_lock(&mut self) -> TzcGlbSwrstS30LockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbSwrstS30LockW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_pwron_rst_lock(&mut self) -> TzcGlbCtrlPwronRstLockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbCtrlPwronRstLockW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_cpu_reset_lock(&mut self) -> TzcGlbCtrlCpuResetLockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbCtrlCpuResetLockW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_sys_reset_lock(&mut self) -> TzcGlbCtrlSysResetLockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbCtrlSysResetLockW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_ungated_ap_lock(
        &mut self,
    ) -> TzcGlbCtrlUngatedApLockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbCtrlUngatedApLockW::new(self, 15)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_misc_lock(&mut self) -> TzcGlbMiscLockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbMiscLockW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_sram_lock(&mut self) -> TzcGlbSramLockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbSramLockW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_l2c_lock(&mut self) -> TzcGlbL2cLockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbL2cLockW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_bmx_lock(&mut self) -> TzcGlbBmxLockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbBmxLockW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_glb_dbg_lock(&mut self) -> TzcGlbDbgLockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbDbgLockW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_glb_mbist_lock(&mut self) -> TzcGlbMbistLockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbMbistLockW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_glb_clk_lock(&mut self) -> TzcGlbClkLockW<'_, TzcGlbCtrl0Spec> {
        TzcGlbClkLockW::new(self, 31)
    }
}
#[doc = "tzc_glb_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_glb_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcGlbCtrl0Spec;
impl crate::RegisterSpec for TzcGlbCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_glb_ctrl_0::R`](R) reader structure"]
impl crate::Readable for TzcGlbCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_glb_ctrl_0::W`](W) writer structure"]
impl crate::Writable for TzcGlbCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_0 to value 0"]
impl crate::Resettable for TzcGlbCtrl0Spec {}
