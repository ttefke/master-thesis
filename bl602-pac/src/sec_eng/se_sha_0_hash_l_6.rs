#[doc = "Register `se_sha_0_hash_l_6` reader"]
pub type R = crate::R<SeSha0HashL6Spec>;
#[doc = "Register `se_sha_0_hash_l_6` writer"]
pub type W = crate::W<SeSha0HashL6Spec>;
#[doc = "Field `se_sha_0_hash_l_6` reader - "]
pub type SeSha0HashL6R = crate::FieldReader<u32>;
#[doc = "Field `se_sha_0_hash_l_6` writer - "]
pub type SeSha0HashL6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_l_6(&self) -> SeSha0HashL6R {
        SeSha0HashL6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_l_6(&mut self) -> SeSha0HashL6W<'_, SeSha0HashL6Spec> {
        SeSha0HashL6W::new(self, 0)
    }
}
#[doc = "se_sha_0_hash_l_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_hash_l_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0HashL6Spec;
impl crate::RegisterSpec for SeSha0HashL6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_hash_l_6::R`](R) reader structure"]
impl crate::Readable for SeSha0HashL6Spec {}
#[doc = "`write(|w| ..)` method takes [`se_sha_0_hash_l_6::W`](W) writer structure"]
impl crate::Writable for SeSha0HashL6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_sha_0_hash_l_6 to value 0"]
impl crate::Resettable for SeSha0HashL6Spec {}
