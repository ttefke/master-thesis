#[doc = "Register `gpdac_tx_fifo_status` reader"]
pub type R = crate::R<GpdacTxFifoStatusSpec>;
#[doc = "Register `gpdac_tx_fifo_status` writer"]
pub type W = crate::W<GpdacTxFifoStatusSpec>;
#[doc = "Field `tx_fifo_empty` reader - "]
pub type TxFifoEmptyR = crate::BitReader;
#[doc = "Field `tx_fifo_empty` writer - "]
pub type TxFifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_full` reader - "]
pub type TxFifoFullR = crate::BitReader;
#[doc = "Field `tx_fifo_full` writer - "]
pub type TxFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_cs` reader - "]
pub type TxCsR = crate::FieldReader;
#[doc = "Field `tx_cs` writer - "]
pub type TxCsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TxFifoRdPtr` reader - "]
pub type TxFifoRdPtrR = crate::FieldReader;
#[doc = "Field `TxFifoRdPtr` writer - "]
pub type TxFifoRdPtrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TxFifoWrPtr` reader - "]
pub type TxFifoWrPtrR = crate::FieldReader;
#[doc = "Field `TxFifoWrPtr` writer - "]
pub type TxFifoWrPtrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TxFifoEmptyR {
        TxFifoEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> TxFifoFullR {
        TxFifoFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_cs(&self) -> TxCsR {
        TxCsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tx_fifo_rd_ptr(&self) -> TxFifoRdPtrR {
        TxFifoRdPtrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tx_fifo_wr_ptr(&self) -> TxFifoWrPtrR {
        TxFifoWrPtrR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_fifo_empty(&mut self) -> TxFifoEmptyW<'_, GpdacTxFifoStatusSpec> {
        TxFifoEmptyW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_full(&mut self) -> TxFifoFullW<'_, GpdacTxFifoStatusSpec> {
        TxFifoFullW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_cs(&mut self) -> TxCsW<'_, GpdacTxFifoStatusSpec> {
        TxCsW::new(self, 2)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tx_fifo_rd_ptr(&mut self) -> TxFifoRdPtrW<'_, GpdacTxFifoStatusSpec> {
        TxFifoRdPtrW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tx_fifo_wr_ptr(&mut self) -> TxFifoWrPtrW<'_, GpdacTxFifoStatusSpec> {
        TxFifoWrPtrW::new(self, 8)
    }
}
#[doc = "gpdac_tx_fifo_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_tx_fifo_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_tx_fifo_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdacTxFifoStatusSpec;
impl crate::RegisterSpec for GpdacTxFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_tx_fifo_status::R`](R) reader structure"]
impl crate::Readable for GpdacTxFifoStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`gpdac_tx_fifo_status::W`](W) writer structure"]
impl crate::Writable for GpdacTxFifoStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gpdac_tx_fifo_status to value 0"]
impl crate::Resettable for GpdacTxFifoStatusSpec {}
