#[doc = "Register `gpadc_dma_rdata` reader"]
pub type R = crate::R<GpadcDmaRdataSpec>;
#[doc = "Register `gpadc_dma_rdata` writer"]
pub type W = crate::W<GpadcDmaRdataSpec>;
#[doc = "Field `gpadc_dma_rdata` reader - "]
pub type GpadcDmaRdataR = crate::FieldReader<u32>;
#[doc = "Field `gpadc_dma_rdata` writer - "]
pub type GpadcDmaRdataW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `rsvd_31_26` reader - "]
pub type Rsvd31_26R = crate::FieldReader;
#[doc = "Field `rsvd_31_26` writer - "]
pub type Rsvd31_26W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_dma_rdata(&self) -> GpadcDmaRdataR {
        GpadcDmaRdataR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rsvd_31_26(&self) -> Rsvd31_26R {
        Rsvd31_26R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_dma_rdata(&mut self) -> GpadcDmaRdataW<'_, GpadcDmaRdataSpec> {
        GpadcDmaRdataW::new(self, 0)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rsvd_31_26(&mut self) -> Rsvd31_26W<'_, GpadcDmaRdataSpec> {
        Rsvd31_26W::new(self, 26)
    }
}
#[doc = "gpadc_dma_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_dma_rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_dma_rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcDmaRdataSpec;
impl crate::RegisterSpec for GpadcDmaRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_dma_rdata::R`](R) reader structure"]
impl crate::Readable for GpadcDmaRdataSpec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_dma_rdata::W`](W) writer structure"]
impl crate::Writable for GpadcDmaRdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gpadc_dma_rdata to value 0"]
impl crate::Resettable for GpadcDmaRdataSpec {}
