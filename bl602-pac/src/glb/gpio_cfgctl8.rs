#[doc = "Register `GPIO_CFGCTL8` reader"]
pub type R = crate::R<GpioCfgctl8Spec>;
#[doc = "Register `GPIO_CFGCTL8` writer"]
pub type W = crate::W<GpioCfgctl8Spec>;
#[doc = "Field `reg_gpio_16_ie` reader - "]
pub type RegGpio16IeR = crate::BitReader;
#[doc = "Field `reg_gpio_16_ie` writer - "]
pub type RegGpio16IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_16_smt` reader - "]
pub type RegGpio16SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_16_smt` writer - "]
pub type RegGpio16SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_16_drv` reader - "]
pub type RegGpio16DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_16_drv` writer - "]
pub type RegGpio16DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_16_pu` reader - "]
pub type RegGpio16PuR = crate::BitReader;
#[doc = "Field `reg_gpio_16_pu` writer - "]
pub type RegGpio16PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_16_pd` reader - "]
pub type RegGpio16PdR = crate::BitReader;
#[doc = "Field `reg_gpio_16_pd` writer - "]
pub type RegGpio16PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_16_func_sel` reader - "]
pub type RegGpio16FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_16_func_sel` writer - "]
pub type RegGpio16FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_gpio_17_ie` reader - "]
pub type RegGpio17IeR = crate::BitReader;
#[doc = "Field `reg_gpio_17_ie` writer - "]
pub type RegGpio17IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_17_smt` reader - "]
pub type RegGpio17SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_17_smt` writer - "]
pub type RegGpio17SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_17_drv` reader - "]
pub type RegGpio17DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_17_drv` writer - "]
pub type RegGpio17DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_17_pu` reader - "]
pub type RegGpio17PuR = crate::BitReader;
#[doc = "Field `reg_gpio_17_pu` writer - "]
pub type RegGpio17PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_17_pd` reader - "]
pub type RegGpio17PdR = crate::BitReader;
#[doc = "Field `reg_gpio_17_pd` writer - "]
pub type RegGpio17PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_17_func_sel` reader - "]
pub type RegGpio17FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_17_func_sel` writer - "]
pub type RegGpio17FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_16_ie(&self) -> RegGpio16IeR {
        RegGpio16IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_16_smt(&self) -> RegGpio16SmtR {
        RegGpio16SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_16_drv(&self) -> RegGpio16DrvR {
        RegGpio16DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_16_pu(&self) -> RegGpio16PuR {
        RegGpio16PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_16_pd(&self) -> RegGpio16PdR {
        RegGpio16PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_16_func_sel(&self) -> RegGpio16FuncSelR {
        RegGpio16FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_17_ie(&self) -> RegGpio17IeR {
        RegGpio17IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_smt(&self) -> RegGpio17SmtR {
        RegGpio17SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_17_drv(&self) -> RegGpio17DrvR {
        RegGpio17DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_17_pu(&self) -> RegGpio17PuR {
        RegGpio17PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_17_pd(&self) -> RegGpio17PdR {
        RegGpio17PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_17_func_sel(&self) -> RegGpio17FuncSelR {
        RegGpio17FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_16_ie(&mut self) -> RegGpio16IeW<'_, GpioCfgctl8Spec> {
        RegGpio16IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_16_smt(&mut self) -> RegGpio16SmtW<'_, GpioCfgctl8Spec> {
        RegGpio16SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_16_drv(&mut self) -> RegGpio16DrvW<'_, GpioCfgctl8Spec> {
        RegGpio16DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_16_pu(&mut self) -> RegGpio16PuW<'_, GpioCfgctl8Spec> {
        RegGpio16PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_16_pd(&mut self) -> RegGpio16PdW<'_, GpioCfgctl8Spec> {
        RegGpio16PdW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_16_func_sel(&mut self) -> RegGpio16FuncSelW<'_, GpioCfgctl8Spec> {
        RegGpio16FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_17_ie(&mut self) -> RegGpio17IeW<'_, GpioCfgctl8Spec> {
        RegGpio17IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_smt(&mut self) -> RegGpio17SmtW<'_, GpioCfgctl8Spec> {
        RegGpio17SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_17_drv(&mut self) -> RegGpio17DrvW<'_, GpioCfgctl8Spec> {
        RegGpio17DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_17_pu(&mut self) -> RegGpio17PuW<'_, GpioCfgctl8Spec> {
        RegGpio17PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_17_pd(&mut self) -> RegGpio17PdW<'_, GpioCfgctl8Spec> {
        RegGpio17PdW::new(self, 21)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_17_func_sel(&mut self) -> RegGpio17FuncSelW<'_, GpioCfgctl8Spec> {
        RegGpio17FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO_CFGCTL8.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl8Spec;
impl crate::RegisterSpec for GpioCfgctl8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl8::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl8::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL8 to value 0"]
impl crate::Resettable for GpioCfgctl8Spec {}
