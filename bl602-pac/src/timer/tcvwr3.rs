#[doc = "Register `TCVWR3` reader"]
pub type R = crate::R<Tcvwr3Spec>;
#[doc = "Register `TCVWR3` writer"]
pub type W = crate::W<Tcvwr3Spec>;
#[doc = "Field `tcvwr` reader - "]
pub type TcvwrR = crate::FieldReader<u32>;
#[doc = "Field `tcvwr` writer - "]
pub type TcvwrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvwr(&self) -> TcvwrR {
        TcvwrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvwr(&mut self) -> TcvwrW<'_, Tcvwr3Spec> {
        TcvwrW::new(self, 0)
    }
}
#[doc = "TCVWR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvwr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcvwr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcvwr3Spec;
impl crate::RegisterSpec for Tcvwr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcvwr3::R`](R) reader structure"]
impl crate::Readable for Tcvwr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tcvwr3::W`](W) writer structure"]
impl crate::Writable for Tcvwr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCVWR3 to value 0"]
impl crate::Resettable for Tcvwr3Spec {}
