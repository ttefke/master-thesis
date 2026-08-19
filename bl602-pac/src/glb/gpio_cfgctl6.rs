#[doc = "Register `GPIO_CFGCTL6` reader"]
pub type R = crate::R<GpioCfgctl6Spec>;
#[doc = "Register `GPIO_CFGCTL6` writer"]
pub type W = crate::W<GpioCfgctl6Spec>;
#[doc = "Field `reg_gpio_12_ie` reader - "]
pub type RegGpio12IeR = crate::BitReader;
#[doc = "Field `reg_gpio_12_ie` writer - "]
pub type RegGpio12IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_12_smt` reader - "]
pub type RegGpio12SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_12_smt` writer - "]
pub type RegGpio12SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_12_drv` reader - "]
pub type RegGpio12DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_12_drv` writer - "]
pub type RegGpio12DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_12_pu` reader - "]
pub type RegGpio12PuR = crate::BitReader;
#[doc = "Field `reg_gpio_12_pu` writer - "]
pub type RegGpio12PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_12_pd` reader - "]
pub type RegGpio12PdR = crate::BitReader;
#[doc = "Field `reg_gpio_12_pd` writer - "]
pub type RegGpio12PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_12_func_sel` reader - "]
pub type RegGpio12FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_12_func_sel` writer - "]
pub type RegGpio12FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_gpio_13_ie` reader - "]
pub type RegGpio13IeR = crate::BitReader;
#[doc = "Field `reg_gpio_13_ie` writer - "]
pub type RegGpio13IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_13_smt` reader - "]
pub type RegGpio13SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_13_smt` writer - "]
pub type RegGpio13SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_13_drv` reader - "]
pub type RegGpio13DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_13_drv` writer - "]
pub type RegGpio13DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_13_pu` reader - "]
pub type RegGpio13PuR = crate::BitReader;
#[doc = "Field `reg_gpio_13_pu` writer - "]
pub type RegGpio13PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_13_pd` reader - "]
pub type RegGpio13PdR = crate::BitReader;
#[doc = "Field `reg_gpio_13_pd` writer - "]
pub type RegGpio13PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_13_func_sel` reader - "]
pub type RegGpio13FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_13_func_sel` writer - "]
pub type RegGpio13FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_12_ie(&self) -> RegGpio12IeR {
        RegGpio12IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_12_smt(&self) -> RegGpio12SmtR {
        RegGpio12SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_12_drv(&self) -> RegGpio12DrvR {
        RegGpio12DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_12_pu(&self) -> RegGpio12PuR {
        RegGpio12PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_12_pd(&self) -> RegGpio12PdR {
        RegGpio12PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_12_func_sel(&self) -> RegGpio12FuncSelR {
        RegGpio12FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_13_ie(&self) -> RegGpio13IeR {
        RegGpio13IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_13_smt(&self) -> RegGpio13SmtR {
        RegGpio13SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_13_drv(&self) -> RegGpio13DrvR {
        RegGpio13DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_13_pu(&self) -> RegGpio13PuR {
        RegGpio13PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_13_pd(&self) -> RegGpio13PdR {
        RegGpio13PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_13_func_sel(&self) -> RegGpio13FuncSelR {
        RegGpio13FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_12_ie(&mut self) -> RegGpio12IeW<'_, GpioCfgctl6Spec> {
        RegGpio12IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_12_smt(&mut self) -> RegGpio12SmtW<'_, GpioCfgctl6Spec> {
        RegGpio12SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_12_drv(&mut self) -> RegGpio12DrvW<'_, GpioCfgctl6Spec> {
        RegGpio12DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_12_pu(&mut self) -> RegGpio12PuW<'_, GpioCfgctl6Spec> {
        RegGpio12PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_12_pd(&mut self) -> RegGpio12PdW<'_, GpioCfgctl6Spec> {
        RegGpio12PdW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_12_func_sel(&mut self) -> RegGpio12FuncSelW<'_, GpioCfgctl6Spec> {
        RegGpio12FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_13_ie(&mut self) -> RegGpio13IeW<'_, GpioCfgctl6Spec> {
        RegGpio13IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_13_smt(&mut self) -> RegGpio13SmtW<'_, GpioCfgctl6Spec> {
        RegGpio13SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_13_drv(&mut self) -> RegGpio13DrvW<'_, GpioCfgctl6Spec> {
        RegGpio13DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_13_pu(&mut self) -> RegGpio13PuW<'_, GpioCfgctl6Spec> {
        RegGpio13PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_13_pd(&mut self) -> RegGpio13PdW<'_, GpioCfgctl6Spec> {
        RegGpio13PdW::new(self, 21)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_13_func_sel(&mut self) -> RegGpio13FuncSelW<'_, GpioCfgctl6Spec> {
        RegGpio13FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO_CFGCTL6.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl6Spec;
impl crate::RegisterSpec for GpioCfgctl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl6::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl6Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl6::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL6 to value 0"]
impl crate::Resettable for GpioCfgctl6Spec {}
