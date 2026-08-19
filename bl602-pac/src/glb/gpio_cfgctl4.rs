#[doc = "Register `GPIO_CFGCTL4` reader"]
pub type R = crate::R<GpioCfgctl4Spec>;
#[doc = "Register `GPIO_CFGCTL4` writer"]
pub type W = crate::W<GpioCfgctl4Spec>;
#[doc = "Field `reg_gpio_8_ie` reader - "]
pub type RegGpio8IeR = crate::BitReader;
#[doc = "Field `reg_gpio_8_ie` writer - "]
pub type RegGpio8IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_8_smt` reader - "]
pub type RegGpio8SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_8_smt` writer - "]
pub type RegGpio8SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_8_drv` reader - "]
pub type RegGpio8DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_8_drv` writer - "]
pub type RegGpio8DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_8_pu` reader - "]
pub type RegGpio8PuR = crate::BitReader;
#[doc = "Field `reg_gpio_8_pu` writer - "]
pub type RegGpio8PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_8_pd` reader - "]
pub type RegGpio8PdR = crate::BitReader;
#[doc = "Field `reg_gpio_8_pd` writer - "]
pub type RegGpio8PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_8_func_sel` reader - "]
pub type RegGpio8FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_8_func_sel` writer - "]
pub type RegGpio8FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_gpio_9_ie` reader - "]
pub type RegGpio9IeR = crate::BitReader;
#[doc = "Field `reg_gpio_9_ie` writer - "]
pub type RegGpio9IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_9_smt` reader - "]
pub type RegGpio9SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_9_smt` writer - "]
pub type RegGpio9SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_9_drv` reader - "]
pub type RegGpio9DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_9_drv` writer - "]
pub type RegGpio9DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_9_pu` reader - "]
pub type RegGpio9PuR = crate::BitReader;
#[doc = "Field `reg_gpio_9_pu` writer - "]
pub type RegGpio9PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_9_pd` reader - "]
pub type RegGpio9PdR = crate::BitReader;
#[doc = "Field `reg_gpio_9_pd` writer - "]
pub type RegGpio9PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_9_func_sel` reader - "]
pub type RegGpio9FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_9_func_sel` writer - "]
pub type RegGpio9FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_8_ie(&self) -> RegGpio8IeR {
        RegGpio8IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_8_smt(&self) -> RegGpio8SmtR {
        RegGpio8SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_8_drv(&self) -> RegGpio8DrvR {
        RegGpio8DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_8_pu(&self) -> RegGpio8PuR {
        RegGpio8PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_8_pd(&self) -> RegGpio8PdR {
        RegGpio8PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_8_func_sel(&self) -> RegGpio8FuncSelR {
        RegGpio8FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_9_ie(&self) -> RegGpio9IeR {
        RegGpio9IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_9_smt(&self) -> RegGpio9SmtR {
        RegGpio9SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_9_drv(&self) -> RegGpio9DrvR {
        RegGpio9DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_9_pu(&self) -> RegGpio9PuR {
        RegGpio9PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_9_pd(&self) -> RegGpio9PdR {
        RegGpio9PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_9_func_sel(&self) -> RegGpio9FuncSelR {
        RegGpio9FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_8_ie(&mut self) -> RegGpio8IeW<'_, GpioCfgctl4Spec> {
        RegGpio8IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_8_smt(&mut self) -> RegGpio8SmtW<'_, GpioCfgctl4Spec> {
        RegGpio8SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_8_drv(&mut self) -> RegGpio8DrvW<'_, GpioCfgctl4Spec> {
        RegGpio8DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_8_pu(&mut self) -> RegGpio8PuW<'_, GpioCfgctl4Spec> {
        RegGpio8PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_8_pd(&mut self) -> RegGpio8PdW<'_, GpioCfgctl4Spec> {
        RegGpio8PdW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_8_func_sel(&mut self) -> RegGpio8FuncSelW<'_, GpioCfgctl4Spec> {
        RegGpio8FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_9_ie(&mut self) -> RegGpio9IeW<'_, GpioCfgctl4Spec> {
        RegGpio9IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_9_smt(&mut self) -> RegGpio9SmtW<'_, GpioCfgctl4Spec> {
        RegGpio9SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_9_drv(&mut self) -> RegGpio9DrvW<'_, GpioCfgctl4Spec> {
        RegGpio9DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_9_pu(&mut self) -> RegGpio9PuW<'_, GpioCfgctl4Spec> {
        RegGpio9PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_9_pd(&mut self) -> RegGpio9PdW<'_, GpioCfgctl4Spec> {
        RegGpio9PdW::new(self, 21)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_9_func_sel(&mut self) -> RegGpio9FuncSelW<'_, GpioCfgctl4Spec> {
        RegGpio9FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO_CFGCTL4.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl4Spec;
impl crate::RegisterSpec for GpioCfgctl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl4::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl4::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL4 to value 0"]
impl crate::Resettable for GpioCfgctl4Spec {}
