#[doc = "Register `gpdac_dma_wdata` reader"]
pub type R = crate::R<GpdacDmaWdataSpec>;
#[doc = "Register `gpdac_dma_wdata` writer"]
pub type W = crate::W<GpdacDmaWdataSpec>;
#[doc = "Field `gpdac_dma_wdata` reader - "]
pub type GpdacDmaWdataR = crate::FieldReader<u32>;
#[doc = "Field `gpdac_dma_wdata` writer - "]
pub type GpdacDmaWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpdac_dma_wdata(&self) -> GpdacDmaWdataR {
        GpdacDmaWdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpdac_dma_wdata(&mut self) -> GpdacDmaWdataW<'_, GpdacDmaWdataSpec> {
        GpdacDmaWdataW::new(self, 0)
    }
}
#[doc = "gpdac_dma_wdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_dma_wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_dma_wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdacDmaWdataSpec;
impl crate::RegisterSpec for GpdacDmaWdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_dma_wdata::R`](R) reader structure"]
impl crate::Readable for GpdacDmaWdataSpec {}
#[doc = "`write(|w| ..)` method takes [`gpdac_dma_wdata::W`](W) writer structure"]
impl crate::Writable for GpdacDmaWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gpdac_dma_wdata to value 0"]
impl crate::Resettable for GpdacDmaWdataSpec {}
