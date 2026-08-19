#[doc = "Register `spi_fifo_rdata` reader"]
pub type R = crate::R<SpiFifoRdataSpec>;
#[doc = "Register `spi_fifo_rdata` writer"]
pub type W = crate::W<SpiFifoRdataSpec>;
#[doc = "Field `spi_fifo_rdata` reader - "]
pub type SpiFifoRdataR = crate::FieldReader<u32>;
#[doc = "Field `spi_fifo_rdata` writer - "]
pub type SpiFifoRdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_fifo_rdata(&self) -> SpiFifoRdataR {
        SpiFifoRdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_fifo_rdata(&mut self) -> SpiFifoRdataW<'_, SpiFifoRdataSpec> {
        SpiFifoRdataW::new(self, 0)
    }
}
#[doc = "spi_fifo_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiFifoRdataSpec;
impl crate::RegisterSpec for SpiFifoRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_fifo_rdata::R`](R) reader structure"]
impl crate::Readable for SpiFifoRdataSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_fifo_rdata::W`](W) writer structure"]
impl crate::Writable for SpiFifoRdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets spi_fifo_rdata to value 0"]
impl crate::Resettable for SpiFifoRdataSpec {}
