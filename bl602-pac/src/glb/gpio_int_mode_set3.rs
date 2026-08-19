#[doc = "Register `GPIO_INT_MODE_SET3` reader"]
pub type R = crate::R<GpioIntModeSet3Spec>;
#[doc = "Register `GPIO_INT_MODE_SET3` writer"]
pub type W = crate::W<GpioIntModeSet3Spec>;
#[doc = "Field `reg_gpio_int_mode_set3` reader - "]
pub type RegGpioIntModeSet3R = crate::FieldReader<u32>;
#[doc = "Field `reg_gpio_int_mode_set3` writer - "]
pub type RegGpioIntModeSet3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_gpio_int_mode_set3(&self) -> RegGpioIntModeSet3R {
        RegGpioIntModeSet3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_gpio_int_mode_set3(&mut self) -> RegGpioIntModeSet3W<'_, GpioIntModeSet3Spec> {
        RegGpioIntModeSet3W::new(self, 0)
    }
}
#[doc = "GPIO_INT_MODE_SET3.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_mode_set3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_mode_set3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntModeSet3Spec;
impl crate::RegisterSpec for GpioIntModeSet3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_mode_set3::R`](R) reader structure"]
impl crate::Readable for GpioIntModeSet3Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_mode_set3::W`](W) writer structure"]
impl crate::Writable for GpioIntModeSet3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_MODE_SET3 to value 0"]
impl crate::Resettable for GpioIntModeSet3Spec {}
