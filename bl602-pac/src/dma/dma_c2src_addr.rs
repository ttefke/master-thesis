#[doc = "Register `DMA_C2SrcAddr` reader"]
pub type R = crate::R<DmaC2srcAddrSpec>;
#[doc = "Register `DMA_C2SrcAddr` writer"]
pub type W = crate::W<DmaC2srcAddrSpec>;
#[doc = "Field `SrcAddr` reader - "]
pub type SrcAddrR = crate::FieldReader<u32>;
#[doc = "Field `SrcAddr` writer - "]
pub type SrcAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_addr(&self) -> SrcAddrR {
        SrcAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_addr(&mut self) -> SrcAddrW<'_, DmaC2srcAddrSpec> {
        SrcAddrW::new(self, 0)
    }
}
#[doc = "DMA_C2SrcAddr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c2src_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c2src_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaC2srcAddrSpec;
impl crate::RegisterSpec for DmaC2srcAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_c2src_addr::R`](R) reader structure"]
impl crate::Readable for DmaC2srcAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_c2src_addr::W`](W) writer structure"]
impl crate::Writable for DmaC2srcAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_C2SrcAddr to value 0"]
impl crate::Resettable for DmaC2srcAddrSpec {}
