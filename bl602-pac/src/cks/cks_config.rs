#[doc = "Register `cks_config` reader"]
pub type R = crate::R<CksConfigSpec>;
#[doc = "Register `cks_config` writer"]
pub type W = crate::W<CksConfigSpec>;
#[doc = "Field `cr_cks_clr` reader - "]
pub type CrCksClrR = crate::BitReader;
#[doc = "Field `cr_cks_clr` writer - "]
pub type CrCksClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_cks_byte_swap` reader - "]
pub type CrCksByteSwapR = crate::BitReader;
#[doc = "Field `cr_cks_byte_swap` writer - "]
pub type CrCksByteSwapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_cks_clr(&self) -> CrCksClrR {
        CrCksClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_cks_byte_swap(&self) -> CrCksByteSwapR {
        CrCksByteSwapR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_cks_clr(&mut self) -> CrCksClrW<'_, CksConfigSpec> {
        CrCksClrW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_cks_byte_swap(&mut self) -> CrCksByteSwapW<'_, CksConfigSpec> {
        CrCksByteSwapW::new(self, 1)
    }
}
#[doc = "cks_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`cks_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cks_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CksConfigSpec;
impl crate::RegisterSpec for CksConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cks_config::R`](R) reader structure"]
impl crate::Readable for CksConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cks_config::W`](W) writer structure"]
impl crate::Writable for CksConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cks_config to value 0"]
impl crate::Resettable for CksConfigSpec {}
