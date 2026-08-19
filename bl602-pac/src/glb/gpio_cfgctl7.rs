#[doc = "Register `GPIO_CFGCTL7` reader"]
pub type R = crate::R<GpioCfgctl7Spec>;
#[doc = "Register `GPIO_CFGCTL7` writer"]
pub type W = crate::W<GpioCfgctl7Spec>;
#[doc = "Field `reg_gpio_14_ie` reader - "]
pub type RegGpio14IeR = crate::BitReader;
#[doc = "Field `reg_gpio_14_ie` writer - "]
pub type RegGpio14IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_14_smt` reader - "]
pub type RegGpio14SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_14_smt` writer - "]
pub type RegGpio14SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_14_drv` reader - "]
pub type RegGpio14DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_14_drv` writer - "]
pub type RegGpio14DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_14_pu` reader - "]
pub type RegGpio14PuR = crate::BitReader;
#[doc = "Field `reg_gpio_14_pu` writer - "]
pub type RegGpio14PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_14_pd` reader - "]
pub type RegGpio14PdR = crate::BitReader;
#[doc = "Field `reg_gpio_14_pd` writer - "]
pub type RegGpio14PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_14_func_sel` reader - "]
pub type RegGpio14FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_14_func_sel` writer - "]
pub type RegGpio14FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_gpio_15_ie` reader - "]
pub type RegGpio15IeR = crate::BitReader;
#[doc = "Field `reg_gpio_15_ie` writer - "]
pub type RegGpio15IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_15_smt` reader - "]
pub type RegGpio15SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_15_smt` writer - "]
pub type RegGpio15SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_15_drv` reader - "]
pub type RegGpio15DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_15_drv` writer - "]
pub type RegGpio15DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_15_pu` reader - "]
pub type RegGpio15PuR = crate::BitReader;
#[doc = "Field `reg_gpio_15_pu` writer - "]
pub type RegGpio15PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_15_pd` reader - "]
pub type RegGpio15PdR = crate::BitReader;
#[doc = "Field `reg_gpio_15_pd` writer - "]
pub type RegGpio15PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_15_func_sel` reader - "]
pub type RegGpio15FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_15_func_sel` writer - "]
pub type RegGpio15FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_14_ie(&self) -> RegGpio14IeR {
        RegGpio14IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_14_smt(&self) -> RegGpio14SmtR {
        RegGpio14SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_14_drv(&self) -> RegGpio14DrvR {
        RegGpio14DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_14_pu(&self) -> RegGpio14PuR {
        RegGpio14PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_14_pd(&self) -> RegGpio14PdR {
        RegGpio14PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_14_func_sel(&self) -> RegGpio14FuncSelR {
        RegGpio14FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_15_ie(&self) -> RegGpio15IeR {
        RegGpio15IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_15_smt(&self) -> RegGpio15SmtR {
        RegGpio15SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_15_drv(&self) -> RegGpio15DrvR {
        RegGpio15DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_15_pu(&self) -> RegGpio15PuR {
        RegGpio15PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_15_pd(&self) -> RegGpio15PdR {
        RegGpio15PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_15_func_sel(&self) -> RegGpio15FuncSelR {
        RegGpio15FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_14_ie(&mut self) -> RegGpio14IeW<'_, GpioCfgctl7Spec> {
        RegGpio14IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_14_smt(&mut self) -> RegGpio14SmtW<'_, GpioCfgctl7Spec> {
        RegGpio14SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_14_drv(&mut self) -> RegGpio14DrvW<'_, GpioCfgctl7Spec> {
        RegGpio14DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_14_pu(&mut self) -> RegGpio14PuW<'_, GpioCfgctl7Spec> {
        RegGpio14PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_14_pd(&mut self) -> RegGpio14PdW<'_, GpioCfgctl7Spec> {
        RegGpio14PdW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_14_func_sel(&mut self) -> RegGpio14FuncSelW<'_, GpioCfgctl7Spec> {
        RegGpio14FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_15_ie(&mut self) -> RegGpio15IeW<'_, GpioCfgctl7Spec> {
        RegGpio15IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_15_smt(&mut self) -> RegGpio15SmtW<'_, GpioCfgctl7Spec> {
        RegGpio15SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_15_drv(&mut self) -> RegGpio15DrvW<'_, GpioCfgctl7Spec> {
        RegGpio15DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_15_pu(&mut self) -> RegGpio15PuW<'_, GpioCfgctl7Spec> {
        RegGpio15PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_15_pd(&mut self) -> RegGpio15PdW<'_, GpioCfgctl7Spec> {
        RegGpio15PdW::new(self, 21)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_15_func_sel(&mut self) -> RegGpio15FuncSelW<'_, GpioCfgctl7Spec> {
        RegGpio15FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO_CFGCTL7.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl7Spec;
impl crate::RegisterSpec for GpioCfgctl7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl7::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl7Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl7::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL7 to value 0"]
impl crate::Resettable for GpioCfgctl7Spec {}
