#[doc = "Register `GPIO_CFGCTL3` reader"]
pub type R = crate::R<GpioCfgctl3Spec>;
#[doc = "Register `GPIO_CFGCTL3` writer"]
pub type W = crate::W<GpioCfgctl3Spec>;
#[doc = "Field `reg_gpio_6_ie` reader - "]
pub type RegGpio6IeR = crate::BitReader;
#[doc = "Field `reg_gpio_6_ie` writer - "]
pub type RegGpio6IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_6_smt` reader - "]
pub type RegGpio6SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_6_smt` writer - "]
pub type RegGpio6SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_6_drv` reader - "]
pub type RegGpio6DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_6_drv` writer - "]
pub type RegGpio6DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_6_pu` reader - "]
pub type RegGpio6PuR = crate::BitReader;
#[doc = "Field `reg_gpio_6_pu` writer - "]
pub type RegGpio6PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_6_pd` reader - "]
pub type RegGpio6PdR = crate::BitReader;
#[doc = "Field `reg_gpio_6_pd` writer - "]
pub type RegGpio6PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_6_func_sel` reader - "]
pub type RegGpio6FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_6_func_sel` writer - "]
pub type RegGpio6FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_gpio_7_ie` reader - "]
pub type RegGpio7IeR = crate::BitReader;
#[doc = "Field `reg_gpio_7_ie` writer - "]
pub type RegGpio7IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_7_smt` reader - "]
pub type RegGpio7SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_7_smt` writer - "]
pub type RegGpio7SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_7_drv` reader - "]
pub type RegGpio7DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_7_drv` writer - "]
pub type RegGpio7DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_7_pu` reader - "]
pub type RegGpio7PuR = crate::BitReader;
#[doc = "Field `reg_gpio_7_pu` writer - "]
pub type RegGpio7PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_7_pd` reader - "]
pub type RegGpio7PdR = crate::BitReader;
#[doc = "Field `reg_gpio_7_pd` writer - "]
pub type RegGpio7PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_7_func_sel` reader - "]
pub type RegGpio7FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_7_func_sel` writer - "]
pub type RegGpio7FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_6_ie(&self) -> RegGpio6IeR {
        RegGpio6IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_6_smt(&self) -> RegGpio6SmtR {
        RegGpio6SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_6_drv(&self) -> RegGpio6DrvR {
        RegGpio6DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_6_pu(&self) -> RegGpio6PuR {
        RegGpio6PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_6_pd(&self) -> RegGpio6PdR {
        RegGpio6PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_6_func_sel(&self) -> RegGpio6FuncSelR {
        RegGpio6FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_7_ie(&self) -> RegGpio7IeR {
        RegGpio7IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_7_smt(&self) -> RegGpio7SmtR {
        RegGpio7SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_7_drv(&self) -> RegGpio7DrvR {
        RegGpio7DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_7_pu(&self) -> RegGpio7PuR {
        RegGpio7PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_7_pd(&self) -> RegGpio7PdR {
        RegGpio7PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_7_func_sel(&self) -> RegGpio7FuncSelR {
        RegGpio7FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_6_ie(&mut self) -> RegGpio6IeW<'_, GpioCfgctl3Spec> {
        RegGpio6IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_6_smt(&mut self) -> RegGpio6SmtW<'_, GpioCfgctl3Spec> {
        RegGpio6SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_6_drv(&mut self) -> RegGpio6DrvW<'_, GpioCfgctl3Spec> {
        RegGpio6DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_6_pu(&mut self) -> RegGpio6PuW<'_, GpioCfgctl3Spec> {
        RegGpio6PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_6_pd(&mut self) -> RegGpio6PdW<'_, GpioCfgctl3Spec> {
        RegGpio6PdW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_6_func_sel(&mut self) -> RegGpio6FuncSelW<'_, GpioCfgctl3Spec> {
        RegGpio6FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_7_ie(&mut self) -> RegGpio7IeW<'_, GpioCfgctl3Spec> {
        RegGpio7IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_7_smt(&mut self) -> RegGpio7SmtW<'_, GpioCfgctl3Spec> {
        RegGpio7SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_7_drv(&mut self) -> RegGpio7DrvW<'_, GpioCfgctl3Spec> {
        RegGpio7DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_7_pu(&mut self) -> RegGpio7PuW<'_, GpioCfgctl3Spec> {
        RegGpio7PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_7_pd(&mut self) -> RegGpio7PdW<'_, GpioCfgctl3Spec> {
        RegGpio7PdW::new(self, 21)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_7_func_sel(&mut self) -> RegGpio7FuncSelW<'_, GpioCfgctl3Spec> {
        RegGpio7FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO_CFGCTL3.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl3Spec;
impl crate::RegisterSpec for GpioCfgctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl3::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl3::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL3 to value 0"]
impl crate::Resettable for GpioCfgctl3Spec {}
