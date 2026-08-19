#[doc = "Register `DMA_C1Config` reader"]
pub type R = crate::R<DmaC1configSpec>;
#[doc = "Register `DMA_C1Config` writer"]
pub type W = crate::W<DmaC1configSpec>;
#[doc = "Field `E` reader - "]
pub type ER = crate::BitReader;
#[doc = "Field `E` writer - "]
pub type EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SrcPeripheral` reader - "]
pub type SrcPeripheralR = crate::FieldReader;
#[doc = "Field `SrcPeripheral` writer - "]
pub type SrcPeripheralW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DstPeripheral` reader - "]
pub type DstPeripheralR = crate::FieldReader;
#[doc = "Field `DstPeripheral` writer - "]
pub type DstPeripheralW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FlowCntrl` reader - "]
pub type FlowCntrlR = crate::FieldReader;
#[doc = "Field `FlowCntrl` writer - "]
pub type FlowCntrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IE` reader - "]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - "]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITC` reader - "]
pub type ItcR = crate::BitReader;
#[doc = "Field `ITC` writer - "]
pub type ItcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L` reader - "]
pub type LR = crate::BitReader;
#[doc = "Field `L` writer - "]
pub type LW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A` reader - "]
pub type AR = crate::BitReader;
#[doc = "Field `A` writer - "]
pub type AW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H` reader - "]
pub type HR = crate::BitReader;
#[doc = "Field `H` writer - "]
pub type HW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn src_peripheral(&self) -> SrcPeripheralR {
        SrcPeripheralR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn dst_peripheral(&self) -> DstPeripheralR {
        DstPeripheralR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn flow_cntrl(&self) -> FlowCntrlR {
        FlowCntrlR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn itc(&self) -> ItcR {
        ItcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn l(&self) -> LR {
        LR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn a(&self) -> AR {
        AR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn h(&self) -> HR {
        HR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn e(&mut self) -> EW<'_, DmaC1configSpec> {
        EW::new(self, 0)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn src_peripheral(&mut self) -> SrcPeripheralW<'_, DmaC1configSpec> {
        SrcPeripheralW::new(self, 1)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn dst_peripheral(&mut self) -> DstPeripheralW<'_, DmaC1configSpec> {
        DstPeripheralW::new(self, 6)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn flow_cntrl(&mut self) -> FlowCntrlW<'_, DmaC1configSpec> {
        FlowCntrlW::new(self, 11)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, DmaC1configSpec> {
        IeW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn itc(&mut self) -> ItcW<'_, DmaC1configSpec> {
        ItcW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn l(&mut self) -> LW<'_, DmaC1configSpec> {
        LW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn a(&mut self) -> AW<'_, DmaC1configSpec> {
        AW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn h(&mut self) -> HW<'_, DmaC1configSpec> {
        HW::new(self, 18)
    }
}
#[doc = "DMA_C1Config.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_c1config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_c1config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaC1configSpec;
impl crate::RegisterSpec for DmaC1configSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_c1config::R`](R) reader structure"]
impl crate::Readable for DmaC1configSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_c1config::W`](W) writer structure"]
impl crate::Writable for DmaC1configSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_C1Config to value 0"]
impl crate::Resettable for DmaC1configSpec {}
