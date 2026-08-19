#[doc = "Register `GPIO_CFGCTL9` reader"]
pub type R = crate::R<GpioCfgctl9Spec>;
#[doc = "Register `GPIO_CFGCTL9` writer"]
pub type W = crate::W<GpioCfgctl9Spec>;
#[doc = "Field `reg_gpio_18_ie` reader - "]
pub type RegGpio18IeR = crate::BitReader;
#[doc = "Field `reg_gpio_18_ie` writer - "]
pub type RegGpio18IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_18_smt` reader - "]
pub type RegGpio18SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_18_smt` writer - "]
pub type RegGpio18SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_18_drv` reader - "]
pub type RegGpio18DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_18_drv` writer - "]
pub type RegGpio18DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_18_pu` reader - "]
pub type RegGpio18PuR = crate::BitReader;
#[doc = "Field `reg_gpio_18_pu` writer - "]
pub type RegGpio18PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_18_pd` reader - "]
pub type RegGpio18PdR = crate::BitReader;
#[doc = "Field `reg_gpio_18_pd` writer - "]
pub type RegGpio18PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_18_func_sel` reader - "]
pub type RegGpio18FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_18_func_sel` writer - "]
pub type RegGpio18FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_gpio_19_ie` reader - "]
pub type RegGpio19IeR = crate::BitReader;
#[doc = "Field `reg_gpio_19_ie` writer - "]
pub type RegGpio19IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_19_smt` reader - "]
pub type RegGpio19SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_19_smt` writer - "]
pub type RegGpio19SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_19_drv` reader - "]
pub type RegGpio19DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_19_drv` writer - "]
pub type RegGpio19DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_19_pu` reader - "]
pub type RegGpio19PuR = crate::BitReader;
#[doc = "Field `reg_gpio_19_pu` writer - "]
pub type RegGpio19PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_19_pd` reader - "]
pub type RegGpio19PdR = crate::BitReader;
#[doc = "Field `reg_gpio_19_pd` writer - "]
pub type RegGpio19PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_19_func_sel` reader - "]
pub type RegGpio19FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_19_func_sel` writer - "]
pub type RegGpio19FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_18_ie(&self) -> RegGpio18IeR {
        RegGpio18IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_18_smt(&self) -> RegGpio18SmtR {
        RegGpio18SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_18_drv(&self) -> RegGpio18DrvR {
        RegGpio18DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_18_pu(&self) -> RegGpio18PuR {
        RegGpio18PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_18_pd(&self) -> RegGpio18PdR {
        RegGpio18PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_18_func_sel(&self) -> RegGpio18FuncSelR {
        RegGpio18FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_19_ie(&self) -> RegGpio19IeR {
        RegGpio19IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_19_smt(&self) -> RegGpio19SmtR {
        RegGpio19SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_19_drv(&self) -> RegGpio19DrvR {
        RegGpio19DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_19_pu(&self) -> RegGpio19PuR {
        RegGpio19PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_19_pd(&self) -> RegGpio19PdR {
        RegGpio19PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_19_func_sel(&self) -> RegGpio19FuncSelR {
        RegGpio19FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_18_ie(&mut self) -> RegGpio18IeW<'_, GpioCfgctl9Spec> {
        RegGpio18IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_18_smt(&mut self) -> RegGpio18SmtW<'_, GpioCfgctl9Spec> {
        RegGpio18SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_18_drv(&mut self) -> RegGpio18DrvW<'_, GpioCfgctl9Spec> {
        RegGpio18DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_18_pu(&mut self) -> RegGpio18PuW<'_, GpioCfgctl9Spec> {
        RegGpio18PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_18_pd(&mut self) -> RegGpio18PdW<'_, GpioCfgctl9Spec> {
        RegGpio18PdW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_18_func_sel(&mut self) -> RegGpio18FuncSelW<'_, GpioCfgctl9Spec> {
        RegGpio18FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_19_ie(&mut self) -> RegGpio19IeW<'_, GpioCfgctl9Spec> {
        RegGpio19IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_19_smt(&mut self) -> RegGpio19SmtW<'_, GpioCfgctl9Spec> {
        RegGpio19SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_19_drv(&mut self) -> RegGpio19DrvW<'_, GpioCfgctl9Spec> {
        RegGpio19DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_19_pu(&mut self) -> RegGpio19PuW<'_, GpioCfgctl9Spec> {
        RegGpio19PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_19_pd(&mut self) -> RegGpio19PdW<'_, GpioCfgctl9Spec> {
        RegGpio19PdW::new(self, 21)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_19_func_sel(&mut self) -> RegGpio19FuncSelW<'_, GpioCfgctl9Spec> {
        RegGpio19FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO_CFGCTL9.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl9Spec;
impl crate::RegisterSpec for GpioCfgctl9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl9::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl9Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl9::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL9 to value 0"]
impl crate::Resettable for GpioCfgctl9Spec {}
