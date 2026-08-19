#[doc = "Register `hit_cnt_lsb` reader"]
pub type R = crate::R<HitCntLsbSpec>;
#[doc = "Register `hit_cnt_lsb` writer"]
pub type W = crate::W<HitCntLsbSpec>;
#[doc = "Field `hit_cnt_lsb` reader - "]
pub type HitCntLsbR = crate::FieldReader<u32>;
#[doc = "Field `hit_cnt_lsb` writer - "]
pub type HitCntLsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hit_cnt_lsb(&self) -> HitCntLsbR {
        HitCntLsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hit_cnt_lsb(&mut self) -> HitCntLsbW<'_, HitCntLsbSpec> {
        HitCntLsbW::new(self, 0)
    }
}
#[doc = "hit_cnt_lsb.\n\nYou can [`read`](crate::Reg::read) this register and get [`hit_cnt_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hit_cnt_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HitCntLsbSpec;
impl crate::RegisterSpec for HitCntLsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hit_cnt_lsb::R`](R) reader structure"]
impl crate::Readable for HitCntLsbSpec {}
#[doc = "`write(|w| ..)` method takes [`hit_cnt_lsb::W`](W) writer structure"]
impl crate::Writable for HitCntLsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets hit_cnt_lsb to value 0"]
impl crate::Resettable for HitCntLsbSpec {}
