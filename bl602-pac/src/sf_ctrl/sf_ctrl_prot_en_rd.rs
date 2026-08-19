#[doc = "Register `sf_ctrl_prot_en_rd` reader"]
pub type R = crate::R<SfCtrlProtEnRdSpec>;
#[doc = "Register `sf_ctrl_prot_en_rd` writer"]
pub type W = crate::W<SfCtrlProtEnRdSpec>;
#[doc = "Field `sf_ctrl_prot_en_rd` reader - "]
pub type SfCtrlProtEnRdR = crate::BitReader;
#[doc = "Field `sf_ctrl_prot_en_rd` writer - "]
pub type SfCtrlProtEnRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_ctrl_id0_en_rd` reader - "]
pub type SfCtrlId0EnRdR = crate::BitReader;
#[doc = "Field `sf_ctrl_id0_en_rd` writer - "]
pub type SfCtrlId0EnRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_ctrl_id1_en_rd` reader - "]
pub type SfCtrlId1EnRdR = crate::BitReader;
#[doc = "Field `sf_ctrl_id1_en_rd` writer - "]
pub type SfCtrlId1EnRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_0_trig_wr_lock` reader - "]
pub type SfIf0TrigWrLockR = crate::BitReader;
#[doc = "Field `sf_if_0_trig_wr_lock` writer - "]
pub type SfIf0TrigWrLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_dbg_dis` reader - "]
pub type SfDbgDisR = crate::BitReader;
#[doc = "Field `sf_dbg_dis` writer - "]
pub type SfDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en_rd(&self) -> SfCtrlProtEnRdR {
        SfCtrlProtEnRdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en_rd(&self) -> SfCtrlId0EnRdR {
        SfCtrlId0EnRdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en_rd(&self) -> SfCtrlId1EnRdR {
        SfCtrlId1EnRdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if_0_trig_wr_lock(&self) -> SfIf0TrigWrLockR {
        SfIf0TrigWrLockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_dbg_dis(&self) -> SfDbgDisR {
        SfDbgDisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en_rd(&mut self) -> SfCtrlProtEnRdW<'_, SfCtrlProtEnRdSpec> {
        SfCtrlProtEnRdW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en_rd(&mut self) -> SfCtrlId0EnRdW<'_, SfCtrlProtEnRdSpec> {
        SfCtrlId0EnRdW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en_rd(&mut self) -> SfCtrlId1EnRdW<'_, SfCtrlProtEnRdSpec> {
        SfCtrlId1EnRdW::new(self, 2)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if_0_trig_wr_lock(&mut self) -> SfIf0TrigWrLockW<'_, SfCtrlProtEnRdSpec> {
        SfIf0TrigWrLockW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_dbg_dis(&mut self) -> SfDbgDisW<'_, SfCtrlProtEnRdSpec> {
        SfDbgDisW::new(self, 31)
    }
}
#[doc = "sf_ctrl_prot_en_rd.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_prot_en_rd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_prot_en_rd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfCtrlProtEnRdSpec;
impl crate::RegisterSpec for SfCtrlProtEnRdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_ctrl_prot_en_rd::R`](R) reader structure"]
impl crate::Readable for SfCtrlProtEnRdSpec {}
#[doc = "`write(|w| ..)` method takes [`sf_ctrl_prot_en_rd::W`](W) writer structure"]
impl crate::Writable for SfCtrlProtEnRdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_ctrl_prot_en_rd to value 0"]
impl crate::Resettable for SfCtrlProtEnRdSpec {}
