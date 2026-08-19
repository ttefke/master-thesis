#[doc = "Register `GPIO_CFGCTL2` reader"]
pub type R = crate::R<GpioCfgctl2Spec>;
#[doc = "Register `GPIO_CFGCTL2` writer"]
pub type W = crate::W<GpioCfgctl2Spec>;
#[doc = "Field `reg_gpio_4_ie` reader - "]
pub type RegGpio4IeR = crate::BitReader;
#[doc = "Field `reg_gpio_4_ie` writer - "]
pub type RegGpio4IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_4_smt` reader - "]
pub type RegGpio4SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_4_smt` writer - "]
pub type RegGpio4SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_4_drv` reader - "]
pub type RegGpio4DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_4_drv` writer - "]
pub type RegGpio4DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_4_pu` reader - "]
pub type RegGpio4PuR = crate::BitReader;
#[doc = "Field `reg_gpio_4_pu` writer - "]
pub type RegGpio4PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_4_pd` reader - "]
pub type RegGpio4PdR = crate::BitReader;
#[doc = "Field `reg_gpio_4_pd` writer - "]
pub type RegGpio4PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_4_func_sel` reader - "]
pub type RegGpio4FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_4_func_sel` writer - "]
pub type RegGpio4FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `real_gpio_4_func_sel` reader - "]
pub type RealGpio4FuncSelR = crate::FieldReader;
#[doc = "Field `real_gpio_4_func_sel` writer - "]
pub type RealGpio4FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_gpio_5_ie` reader - "]
pub type RegGpio5IeR = crate::BitReader;
#[doc = "Field `reg_gpio_5_ie` writer - "]
pub type RegGpio5IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_5_smt` reader - "]
pub type RegGpio5SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_5_smt` writer - "]
pub type RegGpio5SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_5_drv` reader - "]
pub type RegGpio5DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_5_drv` writer - "]
pub type RegGpio5DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_5_pu` reader - "]
pub type RegGpio5PuR = crate::BitReader;
#[doc = "Field `reg_gpio_5_pu` writer - "]
pub type RegGpio5PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_5_pd` reader - "]
pub type RegGpio5PdR = crate::BitReader;
#[doc = "Field `reg_gpio_5_pd` writer - "]
pub type RegGpio5PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_5_func_sel` reader - "]
pub type RegGpio5FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_5_func_sel` writer - "]
pub type RegGpio5FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `real_gpio_5_func_sel` reader - "]
pub type RealGpio5FuncSelR = crate::FieldReader;
#[doc = "Field `real_gpio_5_func_sel` writer - "]
pub type RealGpio5FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_4_ie(&self) -> RegGpio4IeR {
        RegGpio4IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_4_smt(&self) -> RegGpio4SmtR {
        RegGpio4SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_4_drv(&self) -> RegGpio4DrvR {
        RegGpio4DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_pu(&self) -> RegGpio4PuR {
        RegGpio4PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_4_pd(&self) -> RegGpio4PdR {
        RegGpio4PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_4_func_sel(&self) -> RegGpio4FuncSelR {
        RegGpio4FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_4_func_sel(&self) -> RealGpio4FuncSelR {
        RealGpio4FuncSelR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_5_ie(&self) -> RegGpio5IeR {
        RegGpio5IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_5_smt(&self) -> RegGpio5SmtR {
        RegGpio5SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_5_drv(&self) -> RegGpio5DrvR {
        RegGpio5DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_5_pu(&self) -> RegGpio5PuR {
        RegGpio5PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_5_pd(&self) -> RegGpio5PdR {
        RegGpio5PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_5_func_sel(&self) -> RegGpio5FuncSelR {
        RegGpio5FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_5_func_sel(&self) -> RealGpio5FuncSelR {
        RealGpio5FuncSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_4_ie(&mut self) -> RegGpio4IeW<'_, GpioCfgctl2Spec> {
        RegGpio4IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_4_smt(&mut self) -> RegGpio4SmtW<'_, GpioCfgctl2Spec> {
        RegGpio4SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_4_drv(&mut self) -> RegGpio4DrvW<'_, GpioCfgctl2Spec> {
        RegGpio4DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_pu(&mut self) -> RegGpio4PuW<'_, GpioCfgctl2Spec> {
        RegGpio4PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_4_pd(&mut self) -> RegGpio4PdW<'_, GpioCfgctl2Spec> {
        RegGpio4PdW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_4_func_sel(&mut self) -> RegGpio4FuncSelW<'_, GpioCfgctl2Spec> {
        RegGpio4FuncSelW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_4_func_sel(&mut self) -> RealGpio4FuncSelW<'_, GpioCfgctl2Spec> {
        RealGpio4FuncSelW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_5_ie(&mut self) -> RegGpio5IeW<'_, GpioCfgctl2Spec> {
        RegGpio5IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_5_smt(&mut self) -> RegGpio5SmtW<'_, GpioCfgctl2Spec> {
        RegGpio5SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_5_drv(&mut self) -> RegGpio5DrvW<'_, GpioCfgctl2Spec> {
        RegGpio5DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_5_pu(&mut self) -> RegGpio5PuW<'_, GpioCfgctl2Spec> {
        RegGpio5PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_5_pd(&mut self) -> RegGpio5PdW<'_, GpioCfgctl2Spec> {
        RegGpio5PdW::new(self, 21)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_5_func_sel(&mut self) -> RegGpio5FuncSelW<'_, GpioCfgctl2Spec> {
        RegGpio5FuncSelW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_5_func_sel(&mut self) -> RealGpio5FuncSelW<'_, GpioCfgctl2Spec> {
        RealGpio5FuncSelW::new(self, 28)
    }
}
#[doc = "GPIO_CFGCTL2.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl2Spec;
impl crate::RegisterSpec for GpioCfgctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl2::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl2::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL2 to value 0"]
impl crate::Resettable for GpioCfgctl2Spec {}
