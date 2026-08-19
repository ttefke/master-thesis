#[doc = "Register `TCVSYN3` reader"]
pub type R = crate::R<Tcvsyn3Spec>;
#[doc = "Register `TCVSYN3` writer"]
pub type W = crate::W<Tcvsyn3Spec>;
#[doc = "Field `tcvsyn3` reader - "]
pub type Tcvsyn3R = crate::FieldReader<u32>;
#[doc = "Field `tcvsyn3` writer - "]
pub type Tcvsyn3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn3(&self) -> Tcvsyn3R {
        Tcvsyn3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn3(&mut self) -> Tcvsyn3W<'_, Tcvsyn3Spec> {
        Tcvsyn3W::new(self, 0)
    }
}
#[doc = "TCVSYN3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvsyn3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcvsyn3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcvsyn3Spec;
impl crate::RegisterSpec for Tcvsyn3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcvsyn3::R`](R) reader structure"]
impl crate::Readable for Tcvsyn3Spec {}
#[doc = "`write(|w| ..)` method takes [`tcvsyn3::W`](W) writer structure"]
impl crate::Writable for Tcvsyn3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCVSYN3 to value 0"]
impl crate::Resettable for Tcvsyn3Spec {}
