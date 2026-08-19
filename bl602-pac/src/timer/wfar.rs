#[doc = "Register `WFAR` reader"]
pub type R = crate::R<WfarSpec>;
#[doc = "Register `WFAR` writer"]
pub type W = crate::W<WfarSpec>;
#[doc = "Field `wfar` reader - "]
pub type WfarR = crate::FieldReader<u16>;
#[doc = "Field `wfar` writer - "]
pub type WfarW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wfar(&self) -> WfarR {
        WfarR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wfar(&mut self) -> WfarW<'_, WfarSpec> {
        WfarW::new(self, 0)
    }
}
#[doc = "WFAR.\n\nYou can [`read`](crate::Reg::read) this register and get [`wfar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WfarSpec;
impl crate::RegisterSpec for WfarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wfar::R`](R) reader structure"]
impl crate::Readable for WfarSpec {}
#[doc = "`write(|w| ..)` method takes [`wfar::W`](W) writer structure"]
impl crate::Writable for WfarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WFAR to value 0"]
impl crate::Resettable for WfarSpec {}
