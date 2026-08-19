#[doc = "Register `se_trng_0_status` reader"]
pub type R = crate::R<SeTrng0StatusSpec>;
#[doc = "Register `se_trng_0_status` writer"]
pub type W = crate::W<SeTrng0StatusSpec>;
#[doc = "Field `se_trng_0_status` reader - "]
pub type SeTrng0StatusR = crate::FieldReader<u32>;
#[doc = "Field `se_trng_0_status` writer - "]
pub type SeTrng0StatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_status(&self) -> SeTrng0StatusR {
        SeTrng0StatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_status(&mut self) -> SeTrng0StatusW<'_, SeTrng0StatusSpec> {
        SeTrng0StatusW::new(self, 0)
    }
}
#[doc = "se_trng_0_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0StatusSpec;
impl crate::RegisterSpec for SeTrng0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_status::R`](R) reader structure"]
impl crate::Readable for SeTrng0StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`se_trng_0_status::W`](W) writer structure"]
impl crate::Writable for SeTrng0StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_trng_0_status to value 0"]
impl crate::Resettable for SeTrng0StatusSpec {}
