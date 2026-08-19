#[doc = "Register `DMA_C0SrcAddr` reader"]
pub type R = crate::R<DmaC0srcAddrSpec>;
#[doc = "Register `DMA_C0SrcAddr` writer"]
pub type W = crate::W<DmaC0srcAddrSpec>;
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
    pub fn src_addr(&mut self) -> SrcAddrW<'_, DmaC0srcAddrSpec> {
        SrcAddrW::new(self, 0)
    }
}
#[doc = "DMA_C0SrcAddr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c0src_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c0src_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaC0srcAddrSpec;
impl crate::RegisterSpec for DmaC0srcAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_c0src_addr::R`](R) reader structure"]
impl crate::Readable for DmaC0srcAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_c0src_addr::W`](W) writer structure"]
impl crate::Writable for DmaC0srcAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_C0SrcAddr to value 0"]
impl crate::Resettable for DmaC0srcAddrSpec {}
