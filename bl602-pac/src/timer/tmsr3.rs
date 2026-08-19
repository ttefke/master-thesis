#[doc = "Register `TMSR3` reader"]
pub type R = crate::R<Tmsr3Spec>;
#[doc = "Register `TMSR3` writer"]
pub type W = crate::W<Tmsr3Spec>;
#[doc = "Field `tmsr_0` reader - "]
pub type Tmsr0R = crate::BitReader;
#[doc = "Field `tmsr_0` writer - "]
pub type Tmsr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tmsr_1` reader - "]
pub type Tmsr1R = crate::BitReader;
#[doc = "Field `tmsr_1` writer - "]
pub type Tmsr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tmsr_2` reader - "]
pub type Tmsr2R = crate::BitReader;
#[doc = "Field `tmsr_2` writer - "]
pub type Tmsr2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmsr_0(&self) -> Tmsr0R {
        Tmsr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmsr_1(&self) -> Tmsr1R {
        Tmsr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tmsr_2(&self) -> Tmsr2R {
        Tmsr2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmsr_0(&mut self) -> Tmsr0W<'_, Tmsr3Spec> {
        Tmsr0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmsr_1(&mut self) -> Tmsr1W<'_, Tmsr3Spec> {
        Tmsr1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tmsr_2(&mut self) -> Tmsr2W<'_, Tmsr3Spec> {
        Tmsr2W::new(self, 2)
    }
}
#[doc = "TMSR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmsr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmsr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmsr3Spec;
impl crate::RegisterSpec for Tmsr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmsr3::R`](R) reader structure"]
impl crate::Readable for Tmsr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tmsr3::W`](W) writer structure"]
impl crate::Writable for Tmsr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMSR3 to value 0"]
impl crate::Resettable for Tmsr3Spec {}
