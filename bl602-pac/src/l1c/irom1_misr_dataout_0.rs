#[doc = "Register `irom1_misr_dataout_0` reader"]
pub type R = crate::R<Irom1MisrDataout0Spec>;
#[doc = "Register `irom1_misr_dataout_0` writer"]
pub type W = crate::W<Irom1MisrDataout0Spec>;
#[doc = "Field `irom1_misr_dataout_0` reader - "]
pub type Irom1MisrDataout0R = crate::FieldReader<u32>;
#[doc = "Field `irom1_misr_dataout_0` writer - "]
pub type Irom1MisrDataout0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irom1_misr_dataout_0(&self) -> Irom1MisrDataout0R {
        Irom1MisrDataout0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irom1_misr_dataout_0(&mut self) -> Irom1MisrDataout0W<'_, Irom1MisrDataout0Spec> {
        Irom1MisrDataout0W::new(self, 0)
    }
}
#[doc = "irom1_misr_dataout_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`irom1_misr_dataout_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irom1_misr_dataout_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Irom1MisrDataout0Spec;
impl crate::RegisterSpec for Irom1MisrDataout0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irom1_misr_dataout_0::R`](R) reader structure"]
impl crate::Readable for Irom1MisrDataout0Spec {}
#[doc = "`write(|w| ..)` method takes [`irom1_misr_dataout_0::W`](W) writer structure"]
impl crate::Writable for Irom1MisrDataout0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irom1_misr_dataout_0 to value 0"]
impl crate::Resettable for Irom1MisrDataout0Spec {}
