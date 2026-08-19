#[doc = "Register `pds_stat` reader"]
pub type R = crate::R<PdsStatSpec>;
#[doc = "Register `pds_stat` writer"]
pub type W = crate::W<PdsStatSpec>;
#[doc = "Field `ro_pds_state` reader - "]
pub type RoPdsStateR = crate::FieldReader;
#[doc = "Field `ro_pds_state` writer - "]
pub type RoPdsStateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ro_pds_rf_state` reader - "]
pub type RoPdsRfStateR = crate::FieldReader;
#[doc = "Field `ro_pds_rf_state` writer - "]
pub type RoPdsRfStateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ro_pds_pll_state` reader - "]
pub type RoPdsPllStateR = crate::FieldReader;
#[doc = "Field `ro_pds_pll_state` writer - "]
pub type RoPdsPllStateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ro_pds_state(&self) -> RoPdsStateR {
        RoPdsStateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ro_pds_rf_state(&self) -> RoPdsRfStateR {
        RoPdsRfStateR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ro_pds_pll_state(&self) -> RoPdsPllStateR {
        RoPdsPllStateR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ro_pds_state(&mut self) -> RoPdsStateW<'_, PdsStatSpec> {
        RoPdsStateW::new(self, 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ro_pds_rf_state(&mut self) -> RoPdsRfStateW<'_, PdsStatSpec> {
        RoPdsRfStateW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ro_pds_pll_state(&mut self) -> RoPdsPllStateW<'_, PdsStatSpec> {
        RoPdsPllStateW::new(self, 16)
    }
}
#[doc = "pds_stat.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsStatSpec;
impl crate::RegisterSpec for PdsStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_stat::R`](R) reader structure"]
impl crate::Readable for PdsStatSpec {}
#[doc = "`write(|w| ..)` method takes [`pds_stat::W`](W) writer structure"]
impl crate::Writable for PdsStatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pds_stat to value 0"]
impl crate::Resettable for PdsStatSpec {}
