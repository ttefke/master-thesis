#[doc = "Register `TCR3` reader"]
pub type R = crate::R<Tcr3Spec>;
#[doc = "Register `TCR3` writer"]
pub type W = crate::W<Tcr3Spec>;
#[doc = "Field `tcr3_counter` reader - "]
pub type Tcr3CounterR = crate::FieldReader<u32>;
#[doc = "Field `tcr3_counter` writer - "]
pub type Tcr3CounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr3_counter(&self) -> Tcr3CounterR {
        Tcr3CounterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr3_counter(&mut self) -> Tcr3CounterW<'_, Tcr3Spec> {
        Tcr3CounterW::new(self, 0)
    }
}
#[doc = "TCR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcr3Spec;
impl crate::RegisterSpec for Tcr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr3::R`](R) reader structure"]
impl crate::Readable for Tcr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tcr3::W`](W) writer structure"]
impl crate::Writable for Tcr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCR3 to value 0"]
impl crate::Resettable for Tcr3Spec {}
