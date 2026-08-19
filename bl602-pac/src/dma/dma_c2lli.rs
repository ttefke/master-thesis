#[doc = "Register `DMA_C2LLI` reader"]
pub type R = crate::R<DmaC2lliSpec>;
#[doc = "Register `DMA_C2LLI` writer"]
pub type W = crate::W<DmaC2lliSpec>;
#[doc = "Field `LLI` reader - "]
pub type LliR = crate::FieldReader<u32>;
#[doc = "Field `LLI` writer - "]
pub type LliW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn lli(&self) -> LliR {
        LliR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn lli(&mut self) -> LliW<'_, DmaC2lliSpec> {
        LliW::new(self, 2)
    }
}
#[doc = "DMA_C2LLI.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c2lli::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c2lli::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaC2lliSpec;
impl crate::RegisterSpec for DmaC2lliSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_c2lli::R`](R) reader structure"]
impl crate::Readable for DmaC2lliSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_c2lli::W`](W) writer structure"]
impl crate::Writable for DmaC2lliSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_C2LLI to value 0"]
impl crate::Resettable for DmaC2lliSpec {}
