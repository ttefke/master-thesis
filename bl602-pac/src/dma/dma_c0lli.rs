#[doc = "Register `DMA_C0LLI` reader"]
pub type R = crate::R<DmaC0lliSpec>;
#[doc = "Register `DMA_C0LLI` writer"]
pub type W = crate::W<DmaC0lliSpec>;
#[doc = "Field `LLI` reader - "]
pub type LliR = crate::FieldReader<u32>;
#[doc = "Field `LLI` writer - "]
pub type LliW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lli(&self) -> LliR {
        LliR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lli(&mut self) -> LliW<'_, DmaC0lliSpec> {
        LliW::new(self, 0)
    }
}
#[doc = "DMA_C0LLI.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c0lli::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c0lli::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaC0lliSpec;
impl crate::RegisterSpec for DmaC0lliSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_c0lli::R`](R) reader structure"]
impl crate::Readable for DmaC0lliSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_c0lli::W`](W) writer structure"]
impl crate::Writable for DmaC0lliSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_C0LLI to value 0"]
impl crate::Resettable for DmaC0lliSpec {}
