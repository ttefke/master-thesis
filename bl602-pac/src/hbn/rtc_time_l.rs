#[doc = "Register `RTC_TIME_L` reader"]
pub type R = crate::R<RtcTimeLSpec>;
#[doc = "Register `RTC_TIME_L` writer"]
pub type W = crate::W<RtcTimeLSpec>;
#[doc = "Field `rtc_time_latch_l` reader - "]
pub type RtcTimeLatchLR = crate::FieldReader<u32>;
#[doc = "Field `rtc_time_latch_l` writer - "]
pub type RtcTimeLatchLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_time_latch_l(&self) -> RtcTimeLatchLR {
        RtcTimeLatchLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_time_latch_l(&mut self) -> RtcTimeLatchLW<'_, RtcTimeLSpec> {
        RtcTimeLatchLW::new(self, 0)
    }
}
#[doc = "RTC_TIME_L.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_time_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_time_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcTimeLSpec;
impl crate::RegisterSpec for RtcTimeLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_time_l::R`](R) reader structure"]
impl crate::Readable for RtcTimeLSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_time_l::W`](W) writer structure"]
impl crate::Writable for RtcTimeLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_TIME_L to value 0"]
impl crate::Resettable for RtcTimeLSpec {}
