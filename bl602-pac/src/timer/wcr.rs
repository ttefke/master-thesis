#[doc = "Register `WCR` reader"]
pub type R = crate::R<WcrSpec>;
#[doc = "Register `WCR` writer"]
pub type W = crate::W<WcrSpec>;
#[doc = "Field `wcr` reader - "]
pub type WcrR = crate::BitReader;
#[doc = "Field `wcr` writer - "]
pub type WcrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wcr(&self) -> WcrR {
        WcrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wcr(&mut self) -> WcrW<'_, WcrSpec> {
        WcrW::new(self, 0)
    }
}
#[doc = "WCR.\n\nYou can [`read`](crate::Reg::read) this register and get [`wcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WcrSpec;
impl crate::RegisterSpec for WcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wcr::R`](R) reader structure"]
impl crate::Readable for WcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wcr::W`](W) writer structure"]
impl crate::Writable for WcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WCR to value 0"]
impl crate::Resettable for WcrSpec {}
