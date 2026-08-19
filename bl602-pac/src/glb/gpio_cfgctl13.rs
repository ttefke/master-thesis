#[doc = "Register `GPIO_CFGCTL13` reader"]
pub type R = crate::R<GpioCfgctl13Spec>;
#[doc = "Register `GPIO_CFGCTL13` writer"]
pub type W = crate::W<GpioCfgctl13Spec>;
#[doc = "Field `reg_gpio_26_ie` reader - "]
pub type RegGpio26IeR = crate::BitReader;
#[doc = "Field `reg_gpio_26_ie` writer - "]
pub type RegGpio26IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_26_smt` reader - "]
pub type RegGpio26SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_26_smt` writer - "]
pub type RegGpio26SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_26_drv` reader - "]
pub type RegGpio26DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_26_drv` writer - "]
pub type RegGpio26DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_26_pu` reader - "]
pub type RegGpio26PuR = crate::BitReader;
#[doc = "Field `reg_gpio_26_pu` writer - "]
pub type RegGpio26PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_26_pd` reader - "]
pub type RegGpio26PdR = crate::BitReader;
#[doc = "Field `reg_gpio_26_pd` writer - "]
pub type RegGpio26PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_27_ie` reader - "]
pub type RegGpio27IeR = crate::BitReader;
#[doc = "Field `reg_gpio_27_ie` writer - "]
pub type RegGpio27IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_27_smt` reader - "]
pub type RegGpio27SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_27_smt` writer - "]
pub type RegGpio27SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_27_drv` reader - "]
pub type RegGpio27DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_27_drv` writer - "]
pub type RegGpio27DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_27_pu` reader - "]
pub type RegGpio27PuR = crate::BitReader;
#[doc = "Field `reg_gpio_27_pu` writer - "]
pub type RegGpio27PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_27_pd` reader - "]
pub type RegGpio27PdR = crate::BitReader;
#[doc = "Field `reg_gpio_27_pd` writer - "]
pub type RegGpio27PdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_26_ie(&self) -> RegGpio26IeR {
        RegGpio26IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_26_smt(&self) -> RegGpio26SmtR {
        RegGpio26SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_26_drv(&self) -> RegGpio26DrvR {
        RegGpio26DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_26_pu(&self) -> RegGpio26PuR {
        RegGpio26PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_26_pd(&self) -> RegGpio26PdR {
        RegGpio26PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_27_ie(&self) -> RegGpio27IeR {
        RegGpio27IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_27_smt(&self) -> RegGpio27SmtR {
        RegGpio27SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_27_drv(&self) -> RegGpio27DrvR {
        RegGpio27DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_27_pu(&self) -> RegGpio27PuR {
        RegGpio27PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_27_pd(&self) -> RegGpio27PdR {
        RegGpio27PdR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_26_ie(&mut self) -> RegGpio26IeW<'_, GpioCfgctl13Spec> {
        RegGpio26IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_26_smt(&mut self) -> RegGpio26SmtW<'_, GpioCfgctl13Spec> {
        RegGpio26SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_26_drv(&mut self) -> RegGpio26DrvW<'_, GpioCfgctl13Spec> {
        RegGpio26DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_26_pu(&mut self) -> RegGpio26PuW<'_, GpioCfgctl13Spec> {
        RegGpio26PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_26_pd(&mut self) -> RegGpio26PdW<'_, GpioCfgctl13Spec> {
        RegGpio26PdW::new(self, 5)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_27_ie(&mut self) -> RegGpio27IeW<'_, GpioCfgctl13Spec> {
        RegGpio27IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_27_smt(&mut self) -> RegGpio27SmtW<'_, GpioCfgctl13Spec> {
        RegGpio27SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_27_drv(&mut self) -> RegGpio27DrvW<'_, GpioCfgctl13Spec> {
        RegGpio27DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_27_pu(&mut self) -> RegGpio27PuW<'_, GpioCfgctl13Spec> {
        RegGpio27PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_27_pd(&mut self) -> RegGpio27PdW<'_, GpioCfgctl13Spec> {
        RegGpio27PdW::new(self, 21)
    }
}
#[doc = "GPIO_CFGCTL13.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl13Spec;
impl crate::RegisterSpec for GpioCfgctl13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl13::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl13Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl13::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL13 to value 0"]
impl crate::Resettable for GpioCfgctl13Spec {}
