#[doc = "Register `DMA_C2Control` reader"]
pub type R = crate::R<DmaC2controlSpec>;
#[doc = "Register `DMA_C2Control` writer"]
pub type W = crate::W<DmaC2controlSpec>;
#[doc = "Field `TransferSize` reader - "]
pub type TransferSizeR = crate::FieldReader<u16>;
#[doc = "Field `TransferSize` writer - "]
pub type TransferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SBSize` reader - "]
pub type SbsizeR = crate::FieldReader;
#[doc = "Field `SBSize` writer - "]
pub type SbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBSize` reader - "]
pub type DbsizeR = crate::FieldReader;
#[doc = "Field `DBSize` writer - "]
pub type DbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SWidth` reader - "]
pub type SwidthR = crate::FieldReader;
#[doc = "Field `SWidth` writer - "]
pub type SwidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DWidth` reader - "]
pub type DwidthR = crate::FieldReader;
#[doc = "Field `DWidth` writer - "]
pub type DwidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SI` reader - "]
pub type SiR = crate::BitReader;
#[doc = "Field `SI` writer - "]
pub type SiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI` reader - "]
pub type DiR = crate::BitReader;
#[doc = "Field `DI` writer - "]
pub type DiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Prot` reader - "]
pub type ProtR = crate::FieldReader;
#[doc = "Field `Prot` writer - "]
pub type ProtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `I` reader - "]
pub type IR = crate::BitReader;
#[doc = "Field `I` writer - "]
pub type IW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn transfer_size(&self) -> TransferSizeR {
        TransferSizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sbsize(&self) -> SbsizeR {
        SbsizeR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn dbsize(&self) -> DbsizeR {
        DbsizeR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn swidth(&self) -> SwidthR {
        SwidthR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dwidth(&self) -> DwidthR {
        DwidthR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn si(&self) -> SiR {
        SiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn di(&self) -> DiR {
        DiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i(&self) -> IR {
        IR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn transfer_size(&mut self) -> TransferSizeW<'_, DmaC2controlSpec> {
        TransferSizeW::new(self, 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sbsize(&mut self) -> SbsizeW<'_, DmaC2controlSpec> {
        SbsizeW::new(self, 12)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn dbsize(&mut self) -> DbsizeW<'_, DmaC2controlSpec> {
        DbsizeW::new(self, 15)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn swidth(&mut self) -> SwidthW<'_, DmaC2controlSpec> {
        SwidthW::new(self, 18)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DwidthW<'_, DmaC2controlSpec> {
        DwidthW::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn si(&mut self) -> SiW<'_, DmaC2controlSpec> {
        SiW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn di(&mut self) -> DiW<'_, DmaC2controlSpec> {
        DiW::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn prot(&mut self) -> ProtW<'_, DmaC2controlSpec> {
        ProtW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i(&mut self) -> IW<'_, DmaC2controlSpec> {
        IW::new(self, 31)
    }
}
#[doc = "DMA_C2Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c2control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c2control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaC2controlSpec;
impl crate::RegisterSpec for DmaC2controlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_c2control::R`](R) reader structure"]
impl crate::Readable for DmaC2controlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_c2control::W`](W) writer structure"]
impl crate::Writable for DmaC2controlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_C2Control to value 0"]
impl crate::Resettable for DmaC2controlSpec {}
