#[doc = "Register `GPIO_INT_MODE_SET1` reader"]
pub type R = crate::R<GpioIntModeSet1Spec>;
#[doc = "Register `GPIO_INT_MODE_SET1` writer"]
pub type W = crate::W<GpioIntModeSet1Spec>;
#[doc = "Field `reg_gpio_int_mode_set1` reader - "]
pub type RegGpioIntModeSet1R = crate::FieldReader<u32>;
#[doc = "Field `reg_gpio_int_mode_set1` writer - "]
pub type RegGpioIntModeSet1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_gpio_int_mode_set1(&self) -> RegGpioIntModeSet1R {
        RegGpioIntModeSet1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_gpio_int_mode_set1(&mut self) -> RegGpioIntModeSet1W<'_, GpioIntModeSet1Spec> {
        RegGpioIntModeSet1W::new(self, 0)
    }
}
#[doc = "GPIO_INT_MODE_SET1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_mode_set1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_mode_set1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntModeSet1Spec;
impl crate::RegisterSpec for GpioIntModeSet1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_mode_set1::R`](R) reader structure"]
impl crate::Readable for GpioIntModeSet1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_mode_set1::W`](W) writer structure"]
impl crate::Writable for GpioIntModeSet1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_MODE_SET1 to value 0"]
impl crate::Resettable for GpioIntModeSet1Spec {}
