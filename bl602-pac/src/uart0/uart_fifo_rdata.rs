#[doc = "Register `uart_fifo_rdata` reader"]
pub type R = crate::R<UartFifoRdataSpec>;
#[doc = "Register `uart_fifo_rdata` writer"]
pub type W = crate::W<UartFifoRdataSpec>;
#[doc = "Field `uart_fifo_rdata` reader - "]
pub type UartFifoRdataR = crate::FieldReader;
#[doc = "Field `uart_fifo_rdata` writer - "]
pub type UartFifoRdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_fifo_rdata(&self) -> UartFifoRdataR {
        UartFifoRdataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_fifo_rdata(&mut self) -> UartFifoRdataW<'_, UartFifoRdataSpec> {
        UartFifoRdataW::new(self, 0)
    }
}
#[doc = "uart_fifo_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_fifo_rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_fifo_rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartFifoRdataSpec;
impl crate::RegisterSpec for UartFifoRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_fifo_rdata::R`](R) reader structure"]
impl crate::Readable for UartFifoRdataSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_fifo_rdata::W`](W) writer structure"]
impl crate::Writable for UartFifoRdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets uart_fifo_rdata to value 0"]
impl crate::Resettable for UartFifoRdataSpec {}
