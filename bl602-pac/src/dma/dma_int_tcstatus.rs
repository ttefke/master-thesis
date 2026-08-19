#[doc = "Register `DMA_IntTCStatus` reader"]
pub type R = crate::R<DmaIntTcstatusSpec>;
#[doc = "Register `DMA_IntTCStatus` writer"]
pub type W = crate::W<DmaIntTcstatusSpec>;
#[doc = "Field `IntTCStatus` reader - "]
pub type IntTcstatusR = crate::FieldReader;
#[doc = "Field `IntTCStatus` writer - "]
pub type IntTcstatusW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_tcstatus(&self) -> IntTcstatusR {
        IntTcstatusR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_tcstatus(&mut self) -> IntTcstatusW<'_, DmaIntTcstatusSpec> {
        IntTcstatusW::new(self, 0)
    }
}
#[doc = "DMA_IntTCStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_tcstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_tcstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntTcstatusSpec;
impl crate::RegisterSpec for DmaIntTcstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_tcstatus::R`](R) reader structure"]
impl crate::Readable for DmaIntTcstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_int_tcstatus::W`](W) writer structure"]
impl crate::Writable for DmaIntTcstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_IntTCStatus to value 0"]
impl crate::Resettable for DmaIntTcstatusSpec {}
