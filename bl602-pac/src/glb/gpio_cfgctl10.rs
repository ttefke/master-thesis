#[doc = "Register `GPIO_CFGCTL10` reader"]
pub type R = crate::R<GpioCfgctl10Spec>;
#[doc = "Register `GPIO_CFGCTL10` writer"]
pub type W = crate::W<GpioCfgctl10Spec>;
#[doc = "Field `reg_gpio_20_ie` reader - "]
pub type RegGpio20IeR = crate::BitReader;
#[doc = "Field `reg_gpio_20_ie` writer - "]
pub type RegGpio20IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_20_smt` reader - "]
pub type RegGpio20SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_20_smt` writer - "]
pub type RegGpio20SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_20_drv` reader - "]
pub type RegGpio20DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_20_drv` writer - "]
pub type RegGpio20DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_20_pu` reader - "]
pub type RegGpio20PuR = crate::BitReader;
#[doc = "Field `reg_gpio_20_pu` writer - "]
pub type RegGpio20PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_20_pd` reader - "]
pub type RegGpio20PdR = crate::BitReader;
#[doc = "Field `reg_gpio_20_pd` writer - "]
pub type RegGpio20PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_20_func_sel` reader - "]
pub type RegGpio20FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_20_func_sel` writer - "]
pub type RegGpio20FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_gpio_21_ie` reader - "]
pub type RegGpio21IeR = crate::BitReader;
#[doc = "Field `reg_gpio_21_ie` writer - "]
pub type RegGpio21IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_21_smt` reader - "]
pub type RegGpio21SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_21_smt` writer - "]
pub type RegGpio21SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_21_drv` reader - "]
pub type RegGpio21DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_21_drv` writer - "]
pub type RegGpio21DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_21_pu` reader - "]
pub type RegGpio21PuR = crate::BitReader;
#[doc = "Field `reg_gpio_21_pu` writer - "]
pub type RegGpio21PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_21_pd` reader - "]
pub type RegGpio21PdR = crate::BitReader;
#[doc = "Field `reg_gpio_21_pd` writer - "]
pub type RegGpio21PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_21_func_sel` reader - "]
pub type RegGpio21FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_21_func_sel` writer - "]
pub type RegGpio21FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_20_ie(&self) -> RegGpio20IeR {
        RegGpio20IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_20_smt(&self) -> RegGpio20SmtR {
        RegGpio20SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_20_drv(&self) -> RegGpio20DrvR {
        RegGpio20DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_20_pu(&self) -> RegGpio20PuR {
        RegGpio20PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_20_pd(&self) -> RegGpio20PdR {
        RegGpio20PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_20_func_sel(&self) -> RegGpio20FuncSelR {
        RegGpio20FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_21_ie(&self) -> RegGpio21IeR {
        RegGpio21IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_21_smt(&self) -> RegGpio21SmtR {
        RegGpio21SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_21_drv(&self) -> RegGpio21DrvR {
        RegGpio21DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_21_pu(&self) -> RegGpio21PuR {
        RegGpio21PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_pd(&self) -> RegGpio21PdR {
        RegGpio21PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_21_func_sel(&self) -> RegGpio21FuncSelR {
        RegGpio21FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_20_ie(&mut self) -> RegGpio20IeW<'_, GpioCfgctl10Spec> {
        RegGpio20IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_20_smt(&mut self) -> RegGpio20SmtW<'_, GpioCfgctl10Spec> {
        RegGpio20SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_20_drv(&mut self) -> RegGpio20DrvW<'_, GpioCfgctl10Spec> {
        RegGpio20DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_20_pu(&mut self) -> RegGpio20PuW<'_, GpioCfgctl10Spec> {
        RegGpio20PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_20_pd(&mut self) -> RegGpio20PdW<'_, GpioCfgctl10Spec> {
        RegGpio20PdW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_20_func_sel(&mut self) -> RegGpio20FuncSelW<'_, GpioCfgctl10Spec> {
        RegGpio20FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_21_ie(&mut self) -> RegGpio21IeW<'_, GpioCfgctl10Spec> {
        RegGpio21IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_21_smt(&mut self) -> RegGpio21SmtW<'_, GpioCfgctl10Spec> {
        RegGpio21SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_21_drv(&mut self) -> RegGpio21DrvW<'_, GpioCfgctl10Spec> {
        RegGpio21DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_21_pu(&mut self) -> RegGpio21PuW<'_, GpioCfgctl10Spec> {
        RegGpio21PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_pd(&mut self) -> RegGpio21PdW<'_, GpioCfgctl10Spec> {
        RegGpio21PdW::new(self, 21)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_21_func_sel(&mut self) -> RegGpio21FuncSelW<'_, GpioCfgctl10Spec> {
        RegGpio21FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO_CFGCTL10.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl10Spec;
impl crate::RegisterSpec for GpioCfgctl10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl10::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl10Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl10::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL10 to value 0"]
impl crate::Resettable for GpioCfgctl10Spec {}
