#[doc = "Register `TCVSYN2` reader"]
pub type R = crate::R<Tcvsyn2Spec>;
#[doc = "Register `TCVSYN2` writer"]
pub type W = crate::W<Tcvsyn2Spec>;
#[doc = "Field `tcvsyn2` reader - "]
pub type Tcvsyn2R = crate::FieldReader<u32>;
#[doc = "Field `tcvsyn2` writer - "]
pub type Tcvsyn2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn2(&self) -> Tcvsyn2R {
        Tcvsyn2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn2(&mut self) -> Tcvsyn2W<'_, Tcvsyn2Spec> {
        Tcvsyn2W::new(self, 0)
    }
}
#[doc = "TCVSYN2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvsyn2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcvsyn2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcvsyn2Spec;
impl crate::RegisterSpec for Tcvsyn2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcvsyn2::R`](R) reader structure"]
impl crate::Readable for Tcvsyn2Spec {}
#[doc = "`write(|w| ..)` method takes [`tcvsyn2::W`](W) writer structure"]
impl crate::Writable for Tcvsyn2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCVSYN2 to value 0"]
impl crate::Resettable for Tcvsyn2Spec {}
