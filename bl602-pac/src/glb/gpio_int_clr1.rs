#[doc = "Register `GPIO_INT_CLR1` reader"]
pub type R = crate::R<GpioIntClr1Spec>;
#[doc = "Register `GPIO_INT_CLR1` writer"]
pub type W = crate::W<GpioIntClr1Spec>;
#[doc = "Field `reg_gpio_int_clr1` reader - "]
pub type RegGpioIntClr1R = crate::FieldReader<u32>;
#[doc = "Field `reg_gpio_int_clr1` writer - "]
pub type RegGpioIntClr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_gpio_int_clr1(&self) -> RegGpioIntClr1R {
        RegGpioIntClr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_gpio_int_clr1(&mut self) -> RegGpioIntClr1W<'_, GpioIntClr1Spec> {
        RegGpioIntClr1W::new(self, 0)
    }
}
#[doc = "GPIO_INT_CLR1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_clr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_clr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntClr1Spec;
impl crate::RegisterSpec for GpioIntClr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_clr1::R`](R) reader structure"]
impl crate::Readable for GpioIntClr1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_clr1::W`](W) writer structure"]
impl crate::Writable for GpioIntClr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_CLR1 to value 0"]
impl crate::Resettable for GpioIntClr1Spec {}
