#[doc = "Register `DMA_C0DstAddr` reader"]
pub type R = crate::R<DmaC0dstAddrSpec>;
#[doc = "Register `DMA_C0DstAddr` writer"]
pub type W = crate::W<DmaC0dstAddrSpec>;
#[doc = "Field `DstAddr` reader - "]
pub type DstAddrR = crate::FieldReader<u32>;
#[doc = "Field `DstAddr` writer - "]
pub type DstAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_addr(&self) -> DstAddrR {
        DstAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_addr(&mut self) -> DstAddrW<'_, DmaC0dstAddrSpec> {
        DstAddrW::new(self, 0)
    }
}
#[doc = "DMA_C0DstAddr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c0dst_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c0dst_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaC0dstAddrSpec;
impl crate::RegisterSpec for DmaC0dstAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_c0dst_addr::R`](R) reader structure"]
impl crate::Readable for DmaC0dstAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_c0dst_addr::W`](W) writer structure"]
impl crate::Writable for DmaC0dstAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_C0DstAddr to value 0"]
impl crate::Resettable for DmaC0dstAddrSpec {}
