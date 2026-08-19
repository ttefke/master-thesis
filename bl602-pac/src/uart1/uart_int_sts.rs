#[doc = "Register `uart_int_sts` reader"]
pub type R = crate::R<UartIntStsSpec>;
#[doc = "Register `uart_int_sts` writer"]
pub type W = crate::W<UartIntStsSpec>;
#[doc = "Field `utx_end_int` reader - "]
pub type UtxEndIntR = crate::BitReader;
#[doc = "Field `utx_end_int` writer - "]
pub type UtxEndIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `urx_end_int` reader - "]
pub type UrxEndIntR = crate::BitReader;
#[doc = "Field `urx_end_int` writer - "]
pub type UrxEndIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `utx_fifo_int` reader - "]
pub type UtxFifoIntR = crate::BitReader;
#[doc = "Field `utx_fifo_int` writer - "]
pub type UtxFifoIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `urx_fifo_int` reader - "]
pub type UrxFifoIntR = crate::BitReader;
#[doc = "Field `urx_fifo_int` writer - "]
pub type UrxFifoIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `urx_rto_int` reader - "]
pub type UrxRtoIntR = crate::BitReader;
#[doc = "Field `urx_rto_int` writer - "]
pub type UrxRtoIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `urx_pce_int` reader - "]
pub type UrxPceIntR = crate::BitReader;
#[doc = "Field `urx_pce_int` writer - "]
pub type UrxPceIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `utx_fer_int` reader - "]
pub type UtxFerIntR = crate::BitReader;
#[doc = "Field `utx_fer_int` writer - "]
pub type UtxFerIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `urx_fer_int` reader - "]
pub type UrxFerIntR = crate::BitReader;
#[doc = "Field `urx_fer_int` writer - "]
pub type UrxFerIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utx_end_int(&self) -> UtxEndIntR {
        UtxEndIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn urx_end_int(&self) -> UrxEndIntR {
        UrxEndIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn utx_fifo_int(&self) -> UtxFifoIntR {
        UtxFifoIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn urx_fifo_int(&self) -> UrxFifoIntR {
        UrxFifoIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn urx_rto_int(&self) -> UrxRtoIntR {
        UrxRtoIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn urx_pce_int(&self) -> UrxPceIntR {
        UrxPceIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn utx_fer_int(&self) -> UtxFerIntR {
        UtxFerIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn urx_fer_int(&self) -> UrxFerIntR {
        UrxFerIntR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utx_end_int(&mut self) -> UtxEndIntW<'_, UartIntStsSpec> {
        UtxEndIntW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn urx_end_int(&mut self) -> UrxEndIntW<'_, UartIntStsSpec> {
        UrxEndIntW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn utx_fifo_int(&mut self) -> UtxFifoIntW<'_, UartIntStsSpec> {
        UtxFifoIntW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn urx_fifo_int(&mut self) -> UrxFifoIntW<'_, UartIntStsSpec> {
        UrxFifoIntW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn urx_rto_int(&mut self) -> UrxRtoIntW<'_, UartIntStsSpec> {
        UrxRtoIntW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn urx_pce_int(&mut self) -> UrxPceIntW<'_, UartIntStsSpec> {
        UrxPceIntW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn utx_fer_int(&mut self) -> UtxFerIntW<'_, UartIntStsSpec> {
        UtxFerIntW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn urx_fer_int(&mut self) -> UrxFerIntW<'_, UartIntStsSpec> {
        UrxFerIntW::new(self, 7)
    }
}
#[doc = "UART interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIntStsSpec;
impl crate::RegisterSpec for UartIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_int_sts::R`](R) reader structure"]
impl crate::Readable for UartIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_int_sts::W`](W) writer structure"]
impl crate::Writable for UartIntStsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets uart_int_sts to value 0"]
impl crate::Resettable for UartIntStsSpec {}
