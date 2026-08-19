#[doc = "Register `sts_urx_abr_prd` reader"]
pub type R = crate::R<StsUrxAbrPrdSpec>;
#[doc = "Register `sts_urx_abr_prd` writer"]
pub type W = crate::W<StsUrxAbrPrdSpec>;
#[doc = "Field `sts_urx_abr_prd_start` reader - "]
pub type StsUrxAbrPrdStartR = crate::FieldReader<u16>;
#[doc = "Field `sts_urx_abr_prd_start` writer - "]
pub type StsUrxAbrPrdStartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `sts_urx_abr_prd_0x55` reader - "]
pub type StsUrxAbrPrd0x55R = crate::FieldReader<u16>;
#[doc = "Field `sts_urx_abr_prd_0x55` writer - "]
pub type StsUrxAbrPrd0x55W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_start(&self) -> StsUrxAbrPrdStartR {
        StsUrxAbrPrdStartR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_0x55(&self) -> StsUrxAbrPrd0x55R {
        StsUrxAbrPrd0x55R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_start(&mut self) -> StsUrxAbrPrdStartW<'_, StsUrxAbrPrdSpec> {
        StsUrxAbrPrdStartW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_0x55(&mut self) -> StsUrxAbrPrd0x55W<'_, StsUrxAbrPrdSpec> {
        StsUrxAbrPrd0x55W::new(self, 16)
    }
}
#[doc = "sts_urx_abr_prd.\n\nYou can [`read`](crate::Reg::read) this register and get [`sts_urx_abr_prd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts_urx_abr_prd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsUrxAbrPrdSpec;
impl crate::RegisterSpec for StsUrxAbrPrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts_urx_abr_prd::R`](R) reader structure"]
impl crate::Readable for StsUrxAbrPrdSpec {}
#[doc = "`write(|w| ..)` method takes [`sts_urx_abr_prd::W`](W) writer structure"]
impl crate::Writable for StsUrxAbrPrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sts_urx_abr_prd to value 0"]
impl crate::Resettable for StsUrxAbrPrdSpec {}
