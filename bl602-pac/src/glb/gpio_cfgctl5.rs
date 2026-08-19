#[doc = "Register `GPIO_CFGCTL5` reader"]
pub type R = crate::R<GpioCfgctl5Spec>;
#[doc = "Register `GPIO_CFGCTL5` writer"]
pub type W = crate::W<GpioCfgctl5Spec>;
#[doc = "Field `reg_gpio_10_ie` reader - "]
pub type RegGpio10IeR = crate::BitReader;
#[doc = "Field `reg_gpio_10_ie` writer - "]
pub type RegGpio10IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_10_smt` reader - "]
pub type RegGpio10SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_10_smt` writer - "]
pub type RegGpio10SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_10_drv` reader - "]
pub type RegGpio10DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_10_drv` writer - "]
pub type RegGpio10DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_10_pu` reader - "]
pub type RegGpio10PuR = crate::BitReader;
#[doc = "Field `reg_gpio_10_pu` writer - "]
pub type RegGpio10PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_10_pd` reader - "]
pub type RegGpio10PdR = crate::BitReader;
#[doc = "Field `reg_gpio_10_pd` writer - "]
pub type RegGpio10PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_10_func_sel` reader - "]
pub type RegGpio10FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_10_func_sel` writer - "]
pub type RegGpio10FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_gpio_11_ie` reader - "]
pub type RegGpio11IeR = crate::BitReader;
#[doc = "Field `reg_gpio_11_ie` writer - "]
pub type RegGpio11IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_11_smt` reader - "]
pub type RegGpio11SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_11_smt` writer - "]
pub type RegGpio11SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_11_drv` reader - "]
pub type RegGpio11DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_11_drv` writer - "]
pub type RegGpio11DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_11_pu` reader - "]
pub type RegGpio11PuR = crate::BitReader;
#[doc = "Field `reg_gpio_11_pu` writer - "]
pub type RegGpio11PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_11_pd` reader - "]
pub type RegGpio11PdR = crate::BitReader;
#[doc = "Field `reg_gpio_11_pd` writer - "]
pub type RegGpio11PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_11_func_sel` reader - "]
pub type RegGpio11FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_11_func_sel` writer - "]
pub type RegGpio11FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_10_ie(&self) -> RegGpio10IeR {
        RegGpio10IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_10_smt(&self) -> RegGpio10SmtR {
        RegGpio10SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_10_drv(&self) -> RegGpio10DrvR {
        RegGpio10DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_10_pu(&self) -> RegGpio10PuR {
        RegGpio10PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_10_pd(&self) -> RegGpio10PdR {
        RegGpio10PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_10_func_sel(&self) -> RegGpio10FuncSelR {
        RegGpio10FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_11_ie(&self) -> RegGpio11IeR {
        RegGpio11IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_11_smt(&self) -> RegGpio11SmtR {
        RegGpio11SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_11_drv(&self) -> RegGpio11DrvR {
        RegGpio11DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_11_pu(&self) -> RegGpio11PuR {
        RegGpio11PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_11_pd(&self) -> RegGpio11PdR {
        RegGpio11PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_11_func_sel(&self) -> RegGpio11FuncSelR {
        RegGpio11FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_10_ie(&mut self) -> RegGpio10IeW<'_, GpioCfgctl5Spec> {
        RegGpio10IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_10_smt(&mut self) -> RegGpio10SmtW<'_, GpioCfgctl5Spec> {
        RegGpio10SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_10_drv(&mut self) -> RegGpio10DrvW<'_, GpioCfgctl5Spec> {
        RegGpio10DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_10_pu(&mut self) -> RegGpio10PuW<'_, GpioCfgctl5Spec> {
        RegGpio10PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_10_pd(&mut self) -> RegGpio10PdW<'_, GpioCfgctl5Spec> {
        RegGpio10PdW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_10_func_sel(&mut self) -> RegGpio10FuncSelW<'_, GpioCfgctl5Spec> {
        RegGpio10FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_11_ie(&mut self) -> RegGpio11IeW<'_, GpioCfgctl5Spec> {
        RegGpio11IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_11_smt(&mut self) -> RegGpio11SmtW<'_, GpioCfgctl5Spec> {
        RegGpio11SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_11_drv(&mut self) -> RegGpio11DrvW<'_, GpioCfgctl5Spec> {
        RegGpio11DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_11_pu(&mut self) -> RegGpio11PuW<'_, GpioCfgctl5Spec> {
        RegGpio11PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_11_pd(&mut self) -> RegGpio11PdW<'_, GpioCfgctl5Spec> {
        RegGpio11PdW::new(self, 21)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_11_func_sel(&mut self) -> RegGpio11FuncSelW<'_, GpioCfgctl5Spec> {
        RegGpio11FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO_CFGCTL5.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl5Spec;
impl crate::RegisterSpec for GpioCfgctl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl5::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl5Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl5::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL5 to value 0"]
impl crate::Resettable for GpioCfgctl5Spec {}
