#[doc = "Register `sf_ahb2sif_status` reader"]
pub type R = crate::R<SfAhb2sifStatusSpec>;
#[doc = "Register `sf_ahb2sif_status` writer"]
pub type W = crate::W<SfAhb2sifStatusSpec>;
#[doc = "Field `sf_ahb2sif_status` reader - "]
pub type SfAhb2sifStatusR = crate::FieldReader<u32>;
#[doc = "Field `sf_ahb2sif_status` writer - "]
pub type SfAhb2sifStatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_ahb2sif_status(&self) -> SfAhb2sifStatusR {
        SfAhb2sifStatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_ahb2sif_status(&mut self) -> SfAhb2sifStatusW<'_, SfAhb2sifStatusSpec> {
        SfAhb2sifStatusW::new(self, 0)
    }
}
#[doc = "sf_ahb2sif_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ahb2sif_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ahb2sif_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAhb2sifStatusSpec;
impl crate::RegisterSpec for SfAhb2sifStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_ahb2sif_status::R`](R) reader structure"]
impl crate::Readable for SfAhb2sifStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sf_ahb2sif_status::W`](W) writer structure"]
impl crate::Writable for SfAhb2sifStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_ahb2sif_status to value 0"]
impl crate::Resettable for SfAhb2sifStatusSpec {}
