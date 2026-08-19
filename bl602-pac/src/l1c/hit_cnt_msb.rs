#[doc = "Register `hit_cnt_msb` reader"]
pub type R = crate::R<HitCntMsbSpec>;
#[doc = "Register `hit_cnt_msb` writer"]
pub type W = crate::W<HitCntMsbSpec>;
#[doc = "Field `hit_cnt_msb` reader - "]
pub type HitCntMsbR = crate::FieldReader<u32>;
#[doc = "Field `hit_cnt_msb` writer - "]
pub type HitCntMsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hit_cnt_msb(&self) -> HitCntMsbR {
        HitCntMsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hit_cnt_msb(&mut self) -> HitCntMsbW<'_, HitCntMsbSpec> {
        HitCntMsbW::new(self, 0)
    }
}
#[doc = "hit_cnt_msb.\n\nYou can [`read`](crate::Reg::read) this register and get [`hit_cnt_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hit_cnt_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HitCntMsbSpec;
impl crate::RegisterSpec for HitCntMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hit_cnt_msb::R`](R) reader structure"]
impl crate::Readable for HitCntMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`hit_cnt_msb::W`](W) writer structure"]
impl crate::Writable for HitCntMsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets hit_cnt_msb to value 0"]
impl crate::Resettable for HitCntMsbSpec {}
