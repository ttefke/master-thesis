#[doc = "Register `GPIO_CFGCTL11` reader"]
pub type R = crate::R<GpioCfgctl11Spec>;
#[doc = "Register `GPIO_CFGCTL11` writer"]
pub type W = crate::W<GpioCfgctl11Spec>;
#[doc = "Field `reg_gpio_22_ie` reader - "]
pub type RegGpio22IeR = crate::BitReader;
#[doc = "Field `reg_gpio_22_ie` writer - "]
pub type RegGpio22IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_22_smt` reader - "]
pub type RegGpio22SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_22_smt` writer - "]
pub type RegGpio22SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_22_drv` reader - "]
pub type RegGpio22DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_22_drv` writer - "]
pub type RegGpio22DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_22_pu` reader - "]
pub type RegGpio22PuR = crate::BitReader;
#[doc = "Field `reg_gpio_22_pu` writer - "]
pub type RegGpio22PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_22_pd` reader - "]
pub type RegGpio22PdR = crate::BitReader;
#[doc = "Field `reg_gpio_22_pd` writer - "]
pub type RegGpio22PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_22_func_sel` reader - "]
pub type RegGpio22FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_22_func_sel` writer - "]
pub type RegGpio22FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_gpio_23_ie` reader - "]
pub type RegGpio23IeR = crate::BitReader;
#[doc = "Field `reg_gpio_23_ie` writer - "]
pub type RegGpio23IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_23_smt` reader - "]
pub type RegGpio23SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_23_smt` writer - "]
pub type RegGpio23SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_23_drv` reader - "]
pub type RegGpio23DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_23_drv` writer - "]
pub type RegGpio23DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_23_pu` reader - "]
pub type RegGpio23PuR = crate::BitReader;
#[doc = "Field `reg_gpio_23_pu` writer - "]
pub type RegGpio23PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_23_pd` reader - "]
pub type RegGpio23PdR = crate::BitReader;
#[doc = "Field `reg_gpio_23_pd` writer - "]
pub type RegGpio23PdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_22_ie(&self) -> RegGpio22IeR {
        RegGpio22IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_22_smt(&self) -> RegGpio22SmtR {
        RegGpio22SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_22_drv(&self) -> RegGpio22DrvR {
        RegGpio22DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_22_pu(&self) -> RegGpio22PuR {
        RegGpio22PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_22_pd(&self) -> RegGpio22PdR {
        RegGpio22PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_22_func_sel(&self) -> RegGpio22FuncSelR {
        RegGpio22FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_23_ie(&self) -> RegGpio23IeR {
        RegGpio23IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_23_smt(&self) -> RegGpio23SmtR {
        RegGpio23SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_23_drv(&self) -> RegGpio23DrvR {
        RegGpio23DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_23_pu(&self) -> RegGpio23PuR {
        RegGpio23PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_23_pd(&self) -> RegGpio23PdR {
        RegGpio23PdR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_22_ie(&mut self) -> RegGpio22IeW<'_, GpioCfgctl11Spec> {
        RegGpio22IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_22_smt(&mut self) -> RegGpio22SmtW<'_, GpioCfgctl11Spec> {
        RegGpio22SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_22_drv(&mut self) -> RegGpio22DrvW<'_, GpioCfgctl11Spec> {
        RegGpio22DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_22_pu(&mut self) -> RegGpio22PuW<'_, GpioCfgctl11Spec> {
        RegGpio22PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_22_pd(&mut self) -> RegGpio22PdW<'_, GpioCfgctl11Spec> {
        RegGpio22PdW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_22_func_sel(&mut self) -> RegGpio22FuncSelW<'_, GpioCfgctl11Spec> {
        RegGpio22FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_23_ie(&mut self) -> RegGpio23IeW<'_, GpioCfgctl11Spec> {
        RegGpio23IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_23_smt(&mut self) -> RegGpio23SmtW<'_, GpioCfgctl11Spec> {
        RegGpio23SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_23_drv(&mut self) -> RegGpio23DrvW<'_, GpioCfgctl11Spec> {
        RegGpio23DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_23_pu(&mut self) -> RegGpio23PuW<'_, GpioCfgctl11Spec> {
        RegGpio23PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_23_pd(&mut self) -> RegGpio23PdW<'_, GpioCfgctl11Spec> {
        RegGpio23PdW::new(self, 21)
    }
}
#[doc = "GPIO_CFGCTL11.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl11Spec;
impl crate::RegisterSpec for GpioCfgctl11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl11::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl11Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl11::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL11 to value 0"]
impl crate::Resettable for GpioCfgctl11Spec {}
