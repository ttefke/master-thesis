#[doc = "Register `GPIO_INT_STAT1` reader"]
pub type R = crate::R<GpioIntStat1Spec>;
#[doc = "Register `GPIO_INT_STAT1` writer"]
pub type W = crate::W<GpioIntStat1Spec>;
#[doc = "Field `gpio_int_stat1` reader - "]
pub type GpioIntStat1R = crate::FieldReader<u32>;
#[doc = "Field `gpio_int_stat1` writer - "]
pub type GpioIntStat1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_int_stat1(&self) -> GpioIntStat1R {
        GpioIntStat1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_int_stat1(&mut self) -> GpioIntStat1W<'_, GpioIntStat1Spec> {
        GpioIntStat1W::new(self, 0)
    }
}
#[doc = "GPIO_INT_STAT1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_stat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_stat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntStat1Spec;
impl crate::RegisterSpec for GpioIntStat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_stat1::R`](R) reader structure"]
impl crate::Readable for GpioIntStat1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_stat1::W`](W) writer structure"]
impl crate::Writable for GpioIntStat1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_STAT1 to value 0"]
impl crate::Resettable for GpioIntStat1Spec {}
