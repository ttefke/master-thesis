#[doc = "Register `se_sha_0_status` reader"]
pub type R = crate::R<SeSha0StatusSpec>;
#[doc = "Register `se_sha_0_status` writer"]
pub type W = crate::W<SeSha0StatusSpec>;
#[doc = "Field `se_sha_0_status` reader - "]
pub type SeSha0StatusR = crate::FieldReader<u32>;
#[doc = "Field `se_sha_0_status` writer - "]
pub type SeSha0StatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_status(&self) -> SeSha0StatusR {
        SeSha0StatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_status(&mut self) -> SeSha0StatusW<'_, SeSha0StatusSpec> {
        SeSha0StatusW::new(self, 0)
    }
}
#[doc = "se_sha_0_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0StatusSpec;
impl crate::RegisterSpec for SeSha0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_status::R`](R) reader structure"]
impl crate::Readable for SeSha0StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`se_sha_0_status::W`](W) writer structure"]
impl crate::Writable for SeSha0StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_sha_0_status to value 0"]
impl crate::Resettable for SeSha0StatusSpec {}
