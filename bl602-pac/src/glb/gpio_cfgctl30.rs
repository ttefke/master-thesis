#[doc = "Register `GPIO_CFGCTL30` reader"]
pub type R = crate::R<GpioCfgctl30Spec>;
#[doc = "Register `GPIO_CFGCTL30` writer"]
pub type W = crate::W<GpioCfgctl30Spec>;
#[doc = "Field `reg_gpio_0_i` reader - "]
pub type RegGpio0IR = crate::BitReader;
#[doc = "Field `reg_gpio_0_i` writer - "]
pub type RegGpio0IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_1_i` reader - "]
pub type RegGpio1IR = crate::BitReader;
#[doc = "Field `reg_gpio_1_i` writer - "]
pub type RegGpio1IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_2_i` reader - "]
pub type RegGpio2IR = crate::BitReader;
#[doc = "Field `reg_gpio_2_i` writer - "]
pub type RegGpio2IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_3_i` reader - "]
pub type RegGpio3IR = crate::BitReader;
#[doc = "Field `reg_gpio_3_i` writer - "]
pub type RegGpio3IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_4_i` reader - "]
pub type RegGpio4IR = crate::BitReader;
#[doc = "Field `reg_gpio_4_i` writer - "]
pub type RegGpio4IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_5_i` reader - "]
pub type RegGpio5IR = crate::BitReader;
#[doc = "Field `reg_gpio_5_i` writer - "]
pub type RegGpio5IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_6_i` reader - "]
pub type RegGpio6IR = crate::BitReader;
#[doc = "Field `reg_gpio_6_i` writer - "]
pub type RegGpio6IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_7_i` reader - "]
pub type RegGpio7IR = crate::BitReader;
#[doc = "Field `reg_gpio_7_i` writer - "]
pub type RegGpio7IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_8_i` reader - "]
pub type RegGpio8IR = crate::BitReader;
#[doc = "Field `reg_gpio_8_i` writer - "]
pub type RegGpio8IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_9_i` reader - "]
pub type RegGpio9IR = crate::BitReader;
#[doc = "Field `reg_gpio_9_i` writer - "]
pub type RegGpio9IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_10_i` reader - "]
pub type RegGpio10IR = crate::BitReader;
#[doc = "Field `reg_gpio_10_i` writer - "]
pub type RegGpio10IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_11_i` reader - "]
pub type RegGpio11IR = crate::BitReader;
#[doc = "Field `reg_gpio_11_i` writer - "]
pub type RegGpio11IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_12_i` reader - "]
pub type RegGpio12IR = crate::BitReader;
#[doc = "Field `reg_gpio_12_i` writer - "]
pub type RegGpio12IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_13_i` reader - "]
pub type RegGpio13IR = crate::BitReader;
#[doc = "Field `reg_gpio_13_i` writer - "]
pub type RegGpio13IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_14_i` reader - "]
pub type RegGpio14IR = crate::BitReader;
#[doc = "Field `reg_gpio_14_i` writer - "]
pub type RegGpio14IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_15_i` reader - "]
pub type RegGpio15IR = crate::BitReader;
#[doc = "Field `reg_gpio_15_i` writer - "]
pub type RegGpio15IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_16_i` reader - "]
pub type RegGpio16IR = crate::BitReader;
#[doc = "Field `reg_gpio_16_i` writer - "]
pub type RegGpio16IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_17_i` reader - "]
pub type RegGpio17IR = crate::BitReader;
#[doc = "Field `reg_gpio_17_i` writer - "]
pub type RegGpio17IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_18_i` reader - "]
pub type RegGpio18IR = crate::BitReader;
#[doc = "Field `reg_gpio_18_i` writer - "]
pub type RegGpio18IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_19_i` reader - "]
pub type RegGpio19IR = crate::BitReader;
#[doc = "Field `reg_gpio_19_i` writer - "]
pub type RegGpio19IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_20_i` reader - "]
pub type RegGpio20IR = crate::BitReader;
#[doc = "Field `reg_gpio_20_i` writer - "]
pub type RegGpio20IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_21_i` reader - "]
pub type RegGpio21IR = crate::BitReader;
#[doc = "Field `reg_gpio_21_i` writer - "]
pub type RegGpio21IW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_gpio_22_i` reader - "]
pub type RegGpio22IR = crate::BitReader;
#[doc = "Field `reg_gpio_22_i` writer - "]
pub type RegGpio22IW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_i(&self) -> RegGpio0IR {
        RegGpio0IR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_i(&self) -> RegGpio1IR {
        RegGpio1IR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_i(&self) -> RegGpio2IR {
        RegGpio2IR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_i(&self) -> RegGpio3IR {
        RegGpio3IR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_i(&self) -> RegGpio4IR {
        RegGpio4IR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_i(&self) -> RegGpio5IR {
        RegGpio5IR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_i(&self) -> RegGpio6IR {
        RegGpio6IR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_i(&self) -> RegGpio7IR {
        RegGpio7IR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_i(&self) -> RegGpio8IR {
        RegGpio8IR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_i(&self) -> RegGpio9IR {
        RegGpio9IR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_i(&self) -> RegGpio10IR {
        RegGpio10IR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_i(&self) -> RegGpio11IR {
        RegGpio11IR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_i(&self) -> RegGpio12IR {
        RegGpio12IR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_i(&self) -> RegGpio13IR {
        RegGpio13IR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_i(&self) -> RegGpio14IR {
        RegGpio14IR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_i(&self) -> RegGpio15IR {
        RegGpio15IR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_i(&self) -> RegGpio16IR {
        RegGpio16IR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_i(&self) -> RegGpio17IR {
        RegGpio17IR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_i(&self) -> RegGpio18IR {
        RegGpio18IR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_i(&self) -> RegGpio19IR {
        RegGpio19IR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_i(&self) -> RegGpio20IR {
        RegGpio20IR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_i(&self) -> RegGpio21IR {
        RegGpio21IR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_i(&self) -> RegGpio22IR {
        RegGpio22IR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_i(&mut self) -> RegGpio0IW<'_, GpioCfgctl30Spec> {
        RegGpio0IW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_i(&mut self) -> RegGpio1IW<'_, GpioCfgctl30Spec> {
        RegGpio1IW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_i(&mut self) -> RegGpio2IW<'_, GpioCfgctl30Spec> {
        RegGpio2IW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_i(&mut self) -> RegGpio3IW<'_, GpioCfgctl30Spec> {
        RegGpio3IW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_i(&mut self) -> RegGpio4IW<'_, GpioCfgctl30Spec> {
        RegGpio4IW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_i(&mut self) -> RegGpio5IW<'_, GpioCfgctl30Spec> {
        RegGpio5IW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_i(&mut self) -> RegGpio6IW<'_, GpioCfgctl30Spec> {
        RegGpio6IW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_i(&mut self) -> RegGpio7IW<'_, GpioCfgctl30Spec> {
        RegGpio7IW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_i(&mut self) -> RegGpio8IW<'_, GpioCfgctl30Spec> {
        RegGpio8IW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_i(&mut self) -> RegGpio9IW<'_, GpioCfgctl30Spec> {
        RegGpio9IW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_i(&mut self) -> RegGpio10IW<'_, GpioCfgctl30Spec> {
        RegGpio10IW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_i(&mut self) -> RegGpio11IW<'_, GpioCfgctl30Spec> {
        RegGpio11IW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_i(&mut self) -> RegGpio12IW<'_, GpioCfgctl30Spec> {
        RegGpio12IW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_i(&mut self) -> RegGpio13IW<'_, GpioCfgctl30Spec> {
        RegGpio13IW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_i(&mut self) -> RegGpio14IW<'_, GpioCfgctl30Spec> {
        RegGpio14IW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_i(&mut self) -> RegGpio15IW<'_, GpioCfgctl30Spec> {
        RegGpio15IW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_i(&mut self) -> RegGpio16IW<'_, GpioCfgctl30Spec> {
        RegGpio16IW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_i(&mut self) -> RegGpio17IW<'_, GpioCfgctl30Spec> {
        RegGpio17IW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_i(&mut self) -> RegGpio18IW<'_, GpioCfgctl30Spec> {
        RegGpio18IW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_i(&mut self) -> RegGpio19IW<'_, GpioCfgctl30Spec> {
        RegGpio19IW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_i(&mut self) -> RegGpio20IW<'_, GpioCfgctl30Spec> {
        RegGpio20IW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_i(&mut self) -> RegGpio21IW<'_, GpioCfgctl30Spec> {
        RegGpio21IW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_i(&mut self) -> RegGpio22IW<'_, GpioCfgctl30Spec> {
        RegGpio22IW::new(self, 22)
    }
}
#[doc = "GPIO_CFGCTL30.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl30Spec;
impl crate::RegisterSpec for GpioCfgctl30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl30::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl30Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl30::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL30 to value 0"]
impl crate::Resettable for GpioCfgctl30Spec {}
