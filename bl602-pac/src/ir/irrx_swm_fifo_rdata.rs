#[doc = "Register `irrx_swm_fifo_rdata` reader"]
pub type R = crate::R<IrrxSwmFifoRdataSpec>;
#[doc = "Register `irrx_swm_fifo_rdata` writer"]
pub type W = crate::W<IrrxSwmFifoRdataSpec>;
#[doc = "Field `rx_fifo_rdata` reader - "]
pub type RxFifoRdataR = crate::FieldReader<u16>;
#[doc = "Field `rx_fifo_rdata` writer - "]
pub type RxFifoRdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_fifo_rdata(&self) -> RxFifoRdataR {
        RxFifoRdataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_fifo_rdata(&mut self) -> RxFifoRdataW<'_, IrrxSwmFifoRdataSpec> {
        RxFifoRdataW::new(self, 0)
    }
}
#[doc = "irrx_swm_fifo_rdata.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_swm_fifo_rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_swm_fifo_rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxSwmFifoRdataSpec;
impl crate::RegisterSpec for IrrxSwmFifoRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_swm_fifo_rdata::R`](R) reader structure"]
impl crate::Readable for IrrxSwmFifoRdataSpec {}
#[doc = "`write(|w| ..)` method takes [`irrx_swm_fifo_rdata::W`](W) writer structure"]
impl crate::Writable for IrrxSwmFifoRdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irrx_swm_fifo_rdata to value 0"]
impl crate::Resettable for IrrxSwmFifoRdataSpec {}
