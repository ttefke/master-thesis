#[doc = "Register `ef_if_sw_usage_0` reader"]
pub type R = crate::R<EfIfSwUsage0Spec>;
#[doc = "Register `ef_if_sw_usage_0` writer"]
pub type W = crate::W<EfIfSwUsage0Spec>;
#[doc = "Field `ef_if_sw_usage_0` reader - "]
pub type EfIfSwUsage0R = crate::FieldReader<u32>;
#[doc = "Field `ef_if_sw_usage_0` writer - "]
pub type EfIfSwUsage0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_if_sw_usage_0(&self) -> EfIfSwUsage0R {
        EfIfSwUsage0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_if_sw_usage_0(&mut self) -> EfIfSwUsage0W<'_, EfIfSwUsage0Spec> {
        EfIfSwUsage0W::new(self, 0)
    }
}
#[doc = "ef_if_sw_usage_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_sw_usage_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_sw_usage_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfIfSwUsage0Spec;
impl crate::RegisterSpec for EfIfSwUsage0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_if_sw_usage_0::R`](R) reader structure"]
impl crate::Readable for EfIfSwUsage0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_if_sw_usage_0::W`](W) writer structure"]
impl crate::Writable for EfIfSwUsage0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_if_sw_usage_0 to value 0"]
impl crate::Resettable for EfIfSwUsage0Spec {}
