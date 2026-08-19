#[doc = "Register `se_gmac_0_status` reader"]
pub type R = crate::R<SeGmac0StatusSpec>;
#[doc = "Register `se_gmac_0_status` writer"]
pub type W = crate::W<SeGmac0StatusSpec>;
#[doc = "Field `se_gmac_0_status` reader - "]
pub type SeGmac0StatusR = crate::FieldReader<u32>;
#[doc = "Field `se_gmac_0_status` writer - "]
pub type SeGmac0StatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_gmac_0_status(&self) -> SeGmac0StatusR {
        SeGmac0StatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_gmac_0_status(&mut self) -> SeGmac0StatusW<'_, SeGmac0StatusSpec> {
        SeGmac0StatusW::new(self, 0)
    }
}
#[doc = "se_gmac_0_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_gmac_0_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeGmac0StatusSpec;
impl crate::RegisterSpec for SeGmac0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_gmac_0_status::R`](R) reader structure"]
impl crate::Readable for SeGmac0StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`se_gmac_0_status::W`](W) writer structure"]
impl crate::Writable for SeGmac0StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_gmac_0_status to value 0"]
impl crate::Resettable for SeGmac0StatusSpec {}
