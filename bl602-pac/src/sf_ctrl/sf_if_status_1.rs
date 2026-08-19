#[doc = "Register `sf_if_status_1` reader"]
pub type R = crate::R<SfIfStatus1Spec>;
#[doc = "Register `sf_if_status_1` writer"]
pub type W = crate::W<SfIfStatus1Spec>;
#[doc = "Field `sf_if_status_1` reader - "]
pub type SfIfStatus1R = crate::FieldReader<u32>;
#[doc = "Field `sf_if_status_1` writer - "]
pub type SfIfStatus1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_status_1(&self) -> SfIfStatus1R {
        SfIfStatus1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_status_1(&mut self) -> SfIfStatus1W<'_, SfIfStatus1Spec> {
        SfIfStatus1W::new(self, 0)
    }
}
#[doc = "sf_if_status_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_status_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_status_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfStatus1Spec;
impl crate::RegisterSpec for SfIfStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_status_1::R`](R) reader structure"]
impl crate::Readable for SfIfStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_status_1::W`](W) writer structure"]
impl crate::Writable for SfIfStatus1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_if_status_1 to value 0"]
impl crate::Resettable for SfIfStatus1Spec {}
