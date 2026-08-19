#[doc = "Register `GPIO_CFGCTL12` reader"]
pub type R = crate::R<GpioCfgctl12Spec>;
#[doc = "Register `GPIO_CFGCTL12` writer"]
pub type W = crate::W<GpioCfgctl12Spec>;
#[doc = "Field `reg_gpio_24_ie` reader - "]
pub type RegGpio24IeR = crate::BitReader;
#[doc = "Field `reg_gpio_24_ie` writer - "]
pub type RegGpio24IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_24_smt` reader - "]
pub type RegGpio24SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_24_smt` writer - "]
pub type RegGpio24SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_24_drv` reader - "]
pub type RegGpio24DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_24_drv` writer - "]
pub type RegGpio24DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_24_pu` reader - "]
pub type RegGpio24PuR = crate::BitReader;
#[doc = "Field `reg_gpio_24_pu` writer - "]
pub type RegGpio24PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_24_pd` reader - "]
pub type RegGpio24PdR = crate::BitReader;
#[doc = "Field `reg_gpio_24_pd` writer - "]
pub type RegGpio24PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_25_ie` reader - "]
pub type RegGpio25IeR = crate::BitReader;
#[doc = "Field `reg_gpio_25_ie` writer - "]
pub type RegGpio25IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_25_smt` reader - "]
pub type RegGpio25SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_25_smt` writer - "]
pub type RegGpio25SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_25_drv` reader - "]
pub type RegGpio25DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_25_drv` writer - "]
pub type RegGpio25DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_25_pu` reader - "]
pub type RegGpio25PuR = crate::BitReader;
#[doc = "Field `reg_gpio_25_pu` writer - "]
pub type RegGpio25PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_25_pd` reader - "]
pub type RegGpio25PdR = crate::BitReader;
#[doc = "Field `reg_gpio_25_pd` writer - "]
pub type RegGpio25PdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_24_ie(&self) -> RegGpio24IeR {
        RegGpio24IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_24_smt(&self) -> RegGpio24SmtR {
        RegGpio24SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_24_drv(&self) -> RegGpio24DrvR {
        RegGpio24DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_24_pu(&self) -> RegGpio24PuR {
        RegGpio24PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_24_pd(&self) -> RegGpio24PdR {
        RegGpio24PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_25_ie(&self) -> RegGpio25IeR {
        RegGpio25IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_25_smt(&self) -> RegGpio25SmtR {
        RegGpio25SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_25_drv(&self) -> RegGpio25DrvR {
        RegGpio25DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_25_pu(&self) -> RegGpio25PuR {
        RegGpio25PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_25_pd(&self) -> RegGpio25PdR {
        RegGpio25PdR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_24_ie(&mut self) -> RegGpio24IeW<'_, GpioCfgctl12Spec> {
        RegGpio24IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_24_smt(&mut self) -> RegGpio24SmtW<'_, GpioCfgctl12Spec> {
        RegGpio24SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_24_drv(&mut self) -> RegGpio24DrvW<'_, GpioCfgctl12Spec> {
        RegGpio24DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_24_pu(&mut self) -> RegGpio24PuW<'_, GpioCfgctl12Spec> {
        RegGpio24PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_24_pd(&mut self) -> RegGpio24PdW<'_, GpioCfgctl12Spec> {
        RegGpio24PdW::new(self, 5)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_25_ie(&mut self) -> RegGpio25IeW<'_, GpioCfgctl12Spec> {
        RegGpio25IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_25_smt(&mut self) -> RegGpio25SmtW<'_, GpioCfgctl12Spec> {
        RegGpio25SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_25_drv(&mut self) -> RegGpio25DrvW<'_, GpioCfgctl12Spec> {
        RegGpio25DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_25_pu(&mut self) -> RegGpio25PuW<'_, GpioCfgctl12Spec> {
        RegGpio25PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_25_pd(&mut self) -> RegGpio25PdW<'_, GpioCfgctl12Spec> {
        RegGpio25PdW::new(self, 21)
    }
}
#[doc = "GPIO_CFGCTL12.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl12Spec;
impl crate::RegisterSpec for GpioCfgctl12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl12::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl12Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl12::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL12 to value 0"]
impl crate::Resettable for GpioCfgctl12Spec {}
