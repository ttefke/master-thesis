#[doc = "Register `TCR2` reader"]
pub type R = crate::R<Tcr2Spec>;
#[doc = "Register `TCR2` writer"]
pub type W = crate::W<Tcr2Spec>;
#[doc = "Field `tcr` reader - "]
pub type TcrR = crate::FieldReader<u32>;
#[doc = "Field `tcr` writer - "]
pub type TcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr(&self) -> TcrR {
        TcrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr(&mut self) -> TcrW<'_, Tcr2Spec> {
        TcrW::new(self, 0)
    }
}
#[doc = "TCR2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcr2Spec;
impl crate::RegisterSpec for Tcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr2::R`](R) reader structure"]
impl crate::Readable for Tcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tcr2::W`](W) writer structure"]
impl crate::Writable for Tcr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCR2 to value 0"]
impl crate::Resettable for Tcr2Spec {}
