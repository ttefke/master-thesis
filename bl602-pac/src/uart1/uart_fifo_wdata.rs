#[doc = "Register `uart_fifo_wdata` reader"]
pub type R = crate::R<UartFifoWdataSpec>;
#[doc = "Register `uart_fifo_wdata` writer"]
pub type W = crate::W<UartFifoWdataSpec>;
#[doc = "Field `uart_fifo_wdata` reader - "]
pub type UartFifoWdataR = crate::FieldReader;
#[doc = "Field `uart_fifo_wdata` writer - "]
pub type UartFifoWdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_fifo_wdata(&self) -> UartFifoWdataR {
        UartFifoWdataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_fifo_wdata(&mut self) -> UartFifoWdataW<'_, UartFifoWdataSpec> {
        UartFifoWdataW::new(self, 0)
    }
}
#[doc = "uart_fifo_wdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_fifo_wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_fifo_wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartFifoWdataSpec;
impl crate::RegisterSpec for UartFifoWdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_fifo_wdata::R`](R) reader structure"]
impl crate::Readable for UartFifoWdataSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_fifo_wdata::W`](W) writer structure"]
impl crate::Writable for UartFifoWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets uart_fifo_wdata to value 0"]
impl crate::Resettable for UartFifoWdataSpec {}
