#[doc = "Register `GPIO_CFGCTL14` reader"]
pub type R = crate::R<GpioCfgctl14Spec>;
#[doc = "Register `GPIO_CFGCTL14` writer"]
pub type W = crate::W<GpioCfgctl14Spec>;
#[doc = "Field `reg_gpio_28_ie` reader - "]
pub type RegGpio28IeR = crate::BitReader;
#[doc = "Field `reg_gpio_28_ie` writer - "]
pub type RegGpio28IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_28_smt` reader - "]
pub type RegGpio28SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_28_smt` writer - "]
pub type RegGpio28SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_28_drv` reader - "]
pub type RegGpio28DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_28_drv` writer - "]
pub type RegGpio28DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_28_pu` reader - "]
pub type RegGpio28PuR = crate::BitReader;
#[doc = "Field `reg_gpio_28_pu` writer - "]
pub type RegGpio28PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_28_pd` reader - "]
pub type RegGpio28PdR = crate::BitReader;
#[doc = "Field `reg_gpio_28_pd` writer - "]
pub type RegGpio28PdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_28_ie(&self) -> RegGpio28IeR {
        RegGpio28IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_28_smt(&self) -> RegGpio28SmtR {
        RegGpio28SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_28_drv(&self) -> RegGpio28DrvR {
        RegGpio28DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_28_pu(&self) -> RegGpio28PuR {
        RegGpio28PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_28_pd(&self) -> RegGpio28PdR {
        RegGpio28PdR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_28_ie(&mut self) -> RegGpio28IeW<'_, GpioCfgctl14Spec> {
        RegGpio28IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_28_smt(&mut self) -> RegGpio28SmtW<'_, GpioCfgctl14Spec> {
        RegGpio28SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_28_drv(&mut self) -> RegGpio28DrvW<'_, GpioCfgctl14Spec> {
        RegGpio28DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_28_pu(&mut self) -> RegGpio28PuW<'_, GpioCfgctl14Spec> {
        RegGpio28PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_28_pd(&mut self) -> RegGpio28PdW<'_, GpioCfgctl14Spec> {
        RegGpio28PdW::new(self, 5)
    }
}
#[doc = "GPIO_CFGCTL14.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl14Spec;
impl crate::RegisterSpec for GpioCfgctl14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl14::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl14Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl14::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL14 to value 0"]
impl crate::Resettable for GpioCfgctl14Spec {}
