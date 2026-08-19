#[doc = "Register `WVR` reader"]
pub type R = crate::R<WvrSpec>;
#[doc = "Register `WVR` writer"]
pub type W = crate::W<WvrSpec>;
#[doc = "Field `wvr` reader - "]
pub type WvrR = crate::FieldReader<u16>;
#[doc = "Field `wvr` writer - "]
pub type WvrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wvr(&self) -> WvrR {
        WvrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wvr(&mut self) -> WvrW<'_, WvrSpec> {
        WvrW::new(self, 0)
    }
}
#[doc = "WVR.\n\nYou can [`read`](crate::Reg::read) this register and get [`wvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WvrSpec;
impl crate::RegisterSpec for WvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wvr::R`](R) reader structure"]
impl crate::Readable for WvrSpec {}
#[doc = "`write(|w| ..)` method takes [`wvr::W`](W) writer structure"]
impl crate::Writable for WvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WVR to value 0"]
impl crate::Resettable for WvrSpec {}
