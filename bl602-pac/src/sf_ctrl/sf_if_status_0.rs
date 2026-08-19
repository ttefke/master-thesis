#[doc = "Register `sf_if_status_0` reader"]
pub type R = crate::R<SfIfStatus0Spec>;
#[doc = "Register `sf_if_status_0` writer"]
pub type W = crate::W<SfIfStatus0Spec>;
#[doc = "Field `sf_if_status_0` reader - "]
pub type SfIfStatus0R = crate::FieldReader<u32>;
#[doc = "Field `sf_if_status_0` writer - "]
pub type SfIfStatus0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_status_0(&self) -> SfIfStatus0R {
        SfIfStatus0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_status_0(&mut self) -> SfIfStatus0W<'_, SfIfStatus0Spec> {
        SfIfStatus0W::new(self, 0)
    }
}
#[doc = "sf_if_status_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_status_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_status_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfStatus0Spec;
impl crate::RegisterSpec for SfIfStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_status_0::R`](R) reader structure"]
impl crate::Readable for SfIfStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_status_0::W`](W) writer structure"]
impl crate::Writable for SfIfStatus0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_if_status_0 to value 0"]
impl crate::Resettable for SfIfStatus0Spec {}
