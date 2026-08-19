#[doc = "Register `GPIO_CFGCTL0` reader"]
pub type R = crate::R<GpioCfgctl0Spec>;
#[doc = "Register `GPIO_CFGCTL0` writer"]
pub type W = crate::W<GpioCfgctl0Spec>;
#[doc = "Field `reg_gpio_0_ie` reader - "]
pub type RegGpio0IeR = crate::BitReader;
#[doc = "Field `reg_gpio_0_ie` writer - "]
pub type RegGpio0IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_0_smt` reader - "]
pub type RegGpio0SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_0_smt` writer - "]
pub type RegGpio0SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_0_drv` reader - "]
pub type RegGpio0DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_0_drv` writer - "]
pub type RegGpio0DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_0_pu` reader - "]
pub type RegGpio0PuR = crate::BitReader;
#[doc = "Field `reg_gpio_0_pu` writer - "]
pub type RegGpio0PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_0_pd` reader - "]
pub type RegGpio0PdR = crate::BitReader;
#[doc = "Field `reg_gpio_0_pd` writer - "]
pub type RegGpio0PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_0_func_sel` reader - "]
pub type RegGpio0FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_0_func_sel` writer - "]
pub type RegGpio0FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `real_gpio_0_func_sel` reader - "]
pub type RealGpio0FuncSelR = crate::FieldReader;
#[doc = "Field `real_gpio_0_func_sel` writer - "]
pub type RealGpio0FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_gpio_1_ie` reader - "]
pub type RegGpio1IeR = crate::BitReader;
#[doc = "Field `reg_gpio_1_ie` writer - "]
pub type RegGpio1IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_1_smt` reader - "]
pub type RegGpio1SmtR = crate::BitReader;
#[doc = "Field `reg_gpio_1_smt` writer - "]
pub type RegGpio1SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_1_drv` reader - "]
pub type RegGpio1DrvR = crate::FieldReader;
#[doc = "Field `reg_gpio_1_drv` writer - "]
pub type RegGpio1DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_gpio_1_pu` reader - "]
pub type RegGpio1PuR = crate::BitReader;
#[doc = "Field `reg_gpio_1_pu` writer - "]
pub type RegGpio1PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_1_pd` reader - "]
pub type RegGpio1PdR = crate::BitReader;
#[doc = "Field `reg_gpio_1_pd` writer - "]
pub type RegGpio1PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_1_func_sel` reader - "]
pub type RegGpio1FuncSelR = crate::FieldReader;
#[doc = "Field `reg_gpio_1_func_sel` writer - "]
pub type RegGpio1FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `real_gpio_1_func_sel` reader - "]
pub type RealGpio1FuncSelR = crate::FieldReader;
#[doc = "Field `real_gpio_1_func_sel` writer - "]
pub type RealGpio1FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_ie(&self) -> RegGpio0IeR {
        RegGpio0IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_0_smt(&self) -> RegGpio0SmtR {
        RegGpio0SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_0_drv(&self) -> RegGpio0DrvR {
        RegGpio0DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_0_pu(&self) -> RegGpio0PuR {
        RegGpio0PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_0_pd(&self) -> RegGpio0PdR {
        RegGpio0PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_0_func_sel(&self) -> RegGpio0FuncSelR {
        RegGpio0FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_0_func_sel(&self) -> RealGpio0FuncSelR {
        RealGpio0FuncSelR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_1_ie(&self) -> RegGpio1IeR {
        RegGpio1IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_1_smt(&self) -> RegGpio1SmtR {
        RegGpio1SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_1_drv(&self) -> RegGpio1DrvR {
        RegGpio1DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_1_pu(&self) -> RegGpio1PuR {
        RegGpio1PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_1_pd(&self) -> RegGpio1PdR {
        RegGpio1PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_1_func_sel(&self) -> RegGpio1FuncSelR {
        RegGpio1FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_1_func_sel(&self) -> RealGpio1FuncSelR {
        RealGpio1FuncSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_ie(&mut self) -> RegGpio0IeW<'_, GpioCfgctl0Spec> {
        RegGpio0IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_0_smt(&mut self) -> RegGpio0SmtW<'_, GpioCfgctl0Spec> {
        RegGpio0SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_0_drv(&mut self) -> RegGpio0DrvW<'_, GpioCfgctl0Spec> {
        RegGpio0DrvW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_0_pu(&mut self) -> RegGpio0PuW<'_, GpioCfgctl0Spec> {
        RegGpio0PuW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_0_pd(&mut self) -> RegGpio0PdW<'_, GpioCfgctl0Spec> {
        RegGpio0PdW::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_0_func_sel(&mut self) -> RegGpio0FuncSelW<'_, GpioCfgctl0Spec> {
        RegGpio0FuncSelW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_0_func_sel(&mut self) -> RealGpio0FuncSelW<'_, GpioCfgctl0Spec> {
        RealGpio0FuncSelW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_1_ie(&mut self) -> RegGpio1IeW<'_, GpioCfgctl0Spec> {
        RegGpio1IeW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_1_smt(&mut self) -> RegGpio1SmtW<'_, GpioCfgctl0Spec> {
        RegGpio1SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_1_drv(&mut self) -> RegGpio1DrvW<'_, GpioCfgctl0Spec> {
        RegGpio1DrvW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_1_pu(&mut self) -> RegGpio1PuW<'_, GpioCfgctl0Spec> {
        RegGpio1PuW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_1_pd(&mut self) -> RegGpio1PdW<'_, GpioCfgctl0Spec> {
        RegGpio1PdW::new(self, 21)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_1_func_sel(&mut self) -> RegGpio1FuncSelW<'_, GpioCfgctl0Spec> {
        RegGpio1FuncSelW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_1_func_sel(&mut self) -> RealGpio1FuncSelW<'_, GpioCfgctl0Spec> {
        RealGpio1FuncSelW::new(self, 28)
    }
}
#[doc = "GPIO_CFGCTL0.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl0Spec;
impl crate::RegisterSpec for GpioCfgctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl0::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl0::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL0 to value 0"]
impl crate::Resettable for GpioCfgctl0Spec {}
