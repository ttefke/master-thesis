#[doc = "Register `TICR3` reader"]
pub type R = crate::R<Ticr3Spec>;
#[doc = "Register `TICR3` writer"]
pub type W = crate::W<Ticr3Spec>;
#[doc = "Field `tclr_0` reader - "]
pub type Tclr0R = crate::BitReader;
#[doc = "Field `tclr_0` writer - "]
pub type Tclr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tclr_1` reader - "]
pub type Tclr1R = crate::BitReader;
#[doc = "Field `tclr_1` writer - "]
pub type Tclr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tclr_2` reader - "]
pub type Tclr2R = crate::BitReader;
#[doc = "Field `tclr_2` writer - "]
pub type Tclr2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tclr_0(&self) -> Tclr0R {
        Tclr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tclr_1(&self) -> Tclr1R {
        Tclr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tclr_2(&self) -> Tclr2R {
        Tclr2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tclr_0(&mut self) -> Tclr0W<'_, Ticr3Spec> {
        Tclr0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tclr_1(&mut self) -> Tclr1W<'_, Ticr3Spec> {
        Tclr1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tclr_2(&mut self) -> Tclr2W<'_, Ticr3Spec> {
        Tclr2W::new(self, 2)
    }
}
#[doc = "TICR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`ticr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ticr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ticr3Spec;
impl crate::RegisterSpec for Ticr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ticr3::R`](R) reader structure"]
impl crate::Readable for Ticr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ticr3::W`](W) writer structure"]
impl crate::Writable for Ticr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TICR3 to value 0"]
impl crate::Resettable for Ticr3Spec {}
