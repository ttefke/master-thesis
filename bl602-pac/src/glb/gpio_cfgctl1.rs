#[doc = "Register `GPIO_CFGCTL1` reader"]
pub type R = crate::R<GpioCfgctl1Spec>;
#[doc = "Register `GPIO_CFGCTL1` writer"]
pub type W = crate::W<GpioCfgctl1Spec>;
#[doc = "Field `reg_gpio_2_ie` reader - "]
pub type RegGpio2IeR = crate::BitReader;
#[doc = "Field `reg_gpio_2_ie` writer - "]
pub type RegGpio2IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_2_smt` reader - "]
pub type RegGpio2SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_2_smt` writer - "]
pub type RegGpio2SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_2_drv` reader - "]
pub type RegGpio2DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_2_drv` writer - "]
pub type RegGpio2DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_2_pu` reader - "]
pub type RegGpio2PuR = crate::BitReader;
#[doc = "Field `reg_gpio_2_pu` writer - "]
pub type RegGpio2PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_2_pd` reader - "]
pub type RegGpio2PdR = crate::BitReader;
#[doc = "Field `reg_gpio_2_pd` writer - "]
pub type RegGpio2PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_2_func_sel` reader - "]
pub type RegGpio2FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_2_func_sel` writer - "]
pub type RegGpio2FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `real_gpio_2_func_sel` reader - "]
pub type RealGpio2FuncSelR = crate::FieldReader;
#[doc = "Field `real_gpio_2_func_sel` writer - "]
pub type RealGpio2FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_gpio_3_ie` reader - "]
pub type RegGpio3IeR = crate::BitReader;
#[doc = "Field `reg_gpio_3_ie` writer - "]
pub type RegGpio3IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_3_smt` reader - "]
pub type RegGpio3SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_3_smt` writer - "]
pub type RegGpio3SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_3_drv` reader - "]
pub type RegGpio3DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_3_drv` writer - "]
pub type RegGpio3DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_3_pu` reader - "]
pub type RegGpio3PuR = crate::BitReader;
#[doc = "Field `reg_gpio_3_pu` writer - "]
pub type RegGpio3PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_3_pd` reader - "]
pub type RegGpio3PdR = crate::BitReader;
#[doc = "Field `reg_gpio_3_pd` writer - "]
pub type RegGpio3PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_3_func_sel` reader - "]
pub type RegGpio3FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_3_func_sel` writer - "]
pub type RegGpio3FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `real_gpio_3_func_sel` reader - "]
pub type RealGpio3FuncSelR = crate::FieldReader;
#[doc = "Field `real_gpio_3_func_sel` writer - "]
pub type RealGpio3FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_2_ie(&self) -> RegGpio2IeR {
        RegGpio2IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_2_smt(&self) -> RegGpio2SmtR {
        RegGpio2SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_2_drv(&self) -> RegGpio2DrvR {
        RegGpio2DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_2_pu(&self) -> RegGpio2PuR {
        RegGpio2PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_2_pd(&self) -> RegGpio2PdR {
        RegGpio2PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_2_func_sel(&self) -> RegGpio2FuncSelR {
        RegGpio2FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_2_func_sel(&self) -> RealGpio2FuncSelR {
        RealGpio2FuncSelR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_3_ie(&self) -> RegGpio3IeR {
        RegGpio3IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_3_smt(&self) -> RegGpio3SmtR {
        RegGpio3SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_3_drv(&self) -> RegGpio3DrvR {
        RegGpio3DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_3_pu(&self) -> RegGpio3PuR {
        RegGpio3PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_3_pd(&self) -> RegGpio3PdR {
        RegGpio3PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_3_func_sel(&self) -> RegGpio3FuncSelR {
        RegGpio3FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_3_func_sel(&self) -> RealGpio3FuncSelR {
        RealGpio3FuncSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_2_ie(&mut self) -> RegGpio2IeW<'_, GpioCfgctl1Spec> {
        RegGpio2IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_2_smt(&mut self) -> RegGpio2SmtW<'_, GpioCfgctl1Spec> {
        RegGpio2SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_2_drv(&mut self) -> RegGpio2DrvW<'_, GpioCfgctl1Spec> {
        RegGpio2DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_2_pu(&mut self) -> RegGpio2PuW<'_, GpioCfgctl1Spec> {
        RegGpio2PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_2_pd(&mut self) -> RegGpio2PdW<'_, GpioCfgctl1Spec> {
        RegGpio2PdW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_2_func_sel(&mut self) -> RegGpio2FuncSelW<'_, GpioCfgctl1Spec> {
        RegGpio2FuncSelW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_2_func_sel(&mut self) -> RealGpio2FuncSelW<'_, GpioCfgctl1Spec> {
        RealGpio2FuncSelW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_3_ie(&mut self) -> RegGpio3IeW<'_, GpioCfgctl1Spec> {
        RegGpio3IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_3_smt(&mut self) -> RegGpio3SmtW<'_, GpioCfgctl1Spec> {
        RegGpio3SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_3_drv(&mut self) -> RegGpio3DrvW<'_, GpioCfgctl1Spec> {
        RegGpio3DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_3_pu(&mut self) -> RegGpio3PuW<'_, GpioCfgctl1Spec> {
        RegGpio3PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_3_pd(&mut self) -> RegGpio3PdW<'_, GpioCfgctl1Spec> {
        RegGpio3PdW::new(self, 21)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_3_func_sel(&mut self) -> RegGpio3FuncSelW<'_, GpioCfgctl1Spec> {
        RegGpio3FuncSelW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_3_func_sel(&mut self) -> RealGpio3FuncSelW<'_, GpioCfgctl1Spec> {
        RealGpio3FuncSelW::new(self, 28)
    }
}
#[doc = "GPIO_CFGCTL1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl1Spec;
impl crate::RegisterSpec for GpioCfgctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl1::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl1::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL1 to value 0"]
impl crate::Resettable for GpioCfgctl1Spec {}
