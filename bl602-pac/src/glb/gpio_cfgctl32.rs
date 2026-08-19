#[doc = "Register `GPIO_CFGCTL32` reader"]
pub type R = crate::R<GpioCfgctl32Spec>;
#[doc = "Register `GPIO_CFGCTL32` writer"]
pub type W = crate::W<GpioCfgctl32Spec>;
#[doc = "Field `reg_gpio_0_o` reader - "]
pub type RegGpio0OR = crate::BitReader;
#[doc = "Field `reg_gpio_0_o` writer - "]
pub type RegGpio0OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_1_o` reader - "]
pub type RegGpio1OR = crate::BitReader;
#[doc = "Field `reg_gpio_1_o` writer - "]
pub type RegGpio1OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_2_o` reader - "]
pub type RegGpio2OR = crate::BitReader;
#[doc = "Field `reg_gpio_2_o` writer - "]
pub type RegGpio2OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_3_o` reader - "]
pub type RegGpio3OR = crate::BitReader;
#[doc = "Field `reg_gpio_3_o` writer - "]
pub type RegGpio3OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_4_o` reader - "]
pub type RegGpio4OR = crate::BitReader;
#[doc = "Field `reg_gpio_4_o` writer - "]
pub type RegGpio4OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_5_o` reader - "]
pub type RegGpio5OR = crate::BitReader;
#[doc = "Field `reg_gpio_5_o` writer - "]
pub type RegGpio5OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_6_o` reader - "]
pub type RegGpio6OR = crate::BitReader;
#[doc = "Field `reg_gpio_6_o` writer - "]
pub type RegGpio6OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_7_o` reader - "]
pub type RegGpio7OR = crate::BitReader;
#[doc = "Field `reg_gpio_7_o` writer - "]
pub type RegGpio7OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_8_o` reader - "]
pub type RegGpio8OR = crate::BitReader;
#[doc = "Field `reg_gpio_8_o` writer - "]
pub type RegGpio8OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_9_o` reader - "]
pub type RegGpio9OR = crate::BitReader;
#[doc = "Field `reg_gpio_9_o` writer - "]
pub type RegGpio9OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_10_o` reader - "]
pub type RegGpio10OR = crate::BitReader;
#[doc = "Field `reg_gpio_10_o` writer - "]
pub type RegGpio10OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_11_o` reader - "]
pub type RegGpio11OR = crate::BitReader;
#[doc = "Field `reg_gpio_11_o` writer - "]
pub type RegGpio11OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_12_o` reader - "]
pub type RegGpio12OR = crate::BitReader;
#[doc = "Field `reg_gpio_12_o` writer - "]
pub type RegGpio12OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_13_o` reader - "]
pub type RegGpio13OR = crate::BitReader;
#[doc = "Field `reg_gpio_13_o` writer - "]
pub type RegGpio13OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_14_o` reader - "]
pub type RegGpio14OR = crate::BitReader;
#[doc = "Field `reg_gpio_14_o` writer - "]
pub type RegGpio14OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_15_o` reader - "]
pub type RegGpio15OR = crate::BitReader;
#[doc = "Field `reg_gpio_15_o` writer - "]
pub type RegGpio15OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_16_o` reader - "]
pub type RegGpio16OR = crate::BitReader;
#[doc = "Field `reg_gpio_16_o` writer - "]
pub type RegGpio16OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_17_o` reader - "]
pub type RegGpio17OR = crate::BitReader;
#[doc = "Field `reg_gpio_17_o` writer - "]
pub type RegGpio17OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_18_o` reader - "]
pub type RegGpio18OR = crate::BitReader;
#[doc = "Field `reg_gpio_18_o` writer - "]
pub type RegGpio18OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_19_o` reader - "]
pub type RegGpio19OR = crate::BitReader;
#[doc = "Field `reg_gpio_19_o` writer - "]
pub type RegGpio19OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_20_o` reader - "]
pub type RegGpio20OR = crate::BitReader;
#[doc = "Field `reg_gpio_20_o` writer - "]
pub type RegGpio20OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_21_o` reader - "]
pub type RegGpio21OR = crate::BitReader;
#[doc = "Field `reg_gpio_21_o` writer - "]
pub type RegGpio21OW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_22_o` reader - "]
pub type RegGpio22OR = crate::BitReader;
#[doc = "Field `reg_gpio_22_o` writer - "]
pub type RegGpio22OW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_o(&self) -> RegGpio0OR {
        RegGpio0OR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_o(&self) -> RegGpio1OR {
        RegGpio1OR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_o(&self) -> RegGpio2OR {
        RegGpio2OR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_o(&self) -> RegGpio3OR {
        RegGpio3OR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_o(&self) -> RegGpio4OR {
        RegGpio4OR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_o(&self) -> RegGpio5OR {
        RegGpio5OR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_o(&self) -> RegGpio6OR {
        RegGpio6OR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_o(&self) -> RegGpio7OR {
        RegGpio7OR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_o(&self) -> RegGpio8OR {
        RegGpio8OR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_o(&self) -> RegGpio9OR {
        RegGpio9OR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_o(&self) -> RegGpio10OR {
        RegGpio10OR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_o(&self) -> RegGpio11OR {
        RegGpio11OR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_o(&self) -> RegGpio12OR {
        RegGpio12OR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_o(&self) -> RegGpio13OR {
        RegGpio13OR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_o(&self) -> RegGpio14OR {
        RegGpio14OR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_o(&self) -> RegGpio15OR {
        RegGpio15OR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_o(&self) -> RegGpio16OR {
        RegGpio16OR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_o(&self) -> RegGpio17OR {
        RegGpio17OR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_o(&self) -> RegGpio18OR {
        RegGpio18OR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_o(&self) -> RegGpio19OR {
        RegGpio19OR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_o(&self) -> RegGpio20OR {
        RegGpio20OR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_o(&self) -> RegGpio21OR {
        RegGpio21OR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_o(&self) -> RegGpio22OR {
        RegGpio22OR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_o(&mut self) -> RegGpio0OW<'_, GpioCfgctl32Spec> {
        RegGpio0OW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_o(&mut self) -> RegGpio1OW<'_, GpioCfgctl32Spec> {
        RegGpio1OW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_o(&mut self) -> RegGpio2OW<'_, GpioCfgctl32Spec> {
        RegGpio2OW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_o(&mut self) -> RegGpio3OW<'_, GpioCfgctl32Spec> {
        RegGpio3OW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_o(&mut self) -> RegGpio4OW<'_, GpioCfgctl32Spec> {
        RegGpio4OW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_o(&mut self) -> RegGpio5OW<'_, GpioCfgctl32Spec> {
        RegGpio5OW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_o(&mut self) -> RegGpio6OW<'_, GpioCfgctl32Spec> {
        RegGpio6OW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_o(&mut self) -> RegGpio7OW<'_, GpioCfgctl32Spec> {
        RegGpio7OW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_o(&mut self) -> RegGpio8OW<'_, GpioCfgctl32Spec> {
        RegGpio8OW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_o(&mut self) -> RegGpio9OW<'_, GpioCfgctl32Spec> {
        RegGpio9OW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_o(&mut self) -> RegGpio10OW<'_, GpioCfgctl32Spec> {
        RegGpio10OW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_o(&mut self) -> RegGpio11OW<'_, GpioCfgctl32Spec> {
        RegGpio11OW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_o(&mut self) -> RegGpio12OW<'_, GpioCfgctl32Spec> {
        RegGpio12OW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_o(&mut self) -> RegGpio13OW<'_, GpioCfgctl32Spec> {
        RegGpio13OW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_o(&mut self) -> RegGpio14OW<'_, GpioCfgctl32Spec> {
        RegGpio14OW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_o(&mut self) -> RegGpio15OW<'_, GpioCfgctl32Spec> {
        RegGpio15OW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_o(&mut self) -> RegGpio16OW<'_, GpioCfgctl32Spec> {
        RegGpio16OW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_o(&mut self) -> RegGpio17OW<'_, GpioCfgctl32Spec> {
        RegGpio17OW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_o(&mut self) -> RegGpio18OW<'_, GpioCfgctl32Spec> {
        RegGpio18OW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_o(&mut self) -> RegGpio19OW<'_, GpioCfgctl32Spec> {
        RegGpio19OW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_o(&mut self) -> RegGpio20OW<'_, GpioCfgctl32Spec> {
        RegGpio20OW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_o(&mut self) -> RegGpio21OW<'_, GpioCfgctl32Spec> {
        RegGpio21OW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_o(&mut self) -> RegGpio22OW<'_, GpioCfgctl32Spec> {
        RegGpio22OW::new(self, 22)
    }
}
#[doc = "GPIO_CFGCTL32.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl32Spec;
impl crate::RegisterSpec for GpioCfgctl32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl32::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl32Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl32::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL32 to value 0"]
impl crate::Resettable for GpioCfgctl32Spec {}
