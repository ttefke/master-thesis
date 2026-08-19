#[doc = "Register `uart_status` reader"]
pub type R = crate::R<UartStatusSpec>;
#[doc = "Register `uart_status` writer"]
pub type W = crate::W<UartStatusSpec>;
#[doc = "Field `sts_utx_bus_busy` reader - "]
pub type StsUtxBusBusyR = crate::BitReader;
#[doc = "Field `sts_utx_bus_busy` writer - "]
pub type StsUtxBusBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_urx_bus_busy` reader - "]
pub type StsUrxBusBusyR = crate::BitReader;
#[doc = "Field `sts_urx_bus_busy` writer - "]
pub type StsUrxBusBusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_utx_bus_busy(&self) -> StsUtxBusBusyR {
        StsUtxBusBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sts_urx_bus_busy(&self) -> StsUrxBusBusyR {
        StsUrxBusBusyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_utx_bus_busy(&mut self) -> StsUtxBusBusyW<'_, UartStatusSpec> {
        StsUtxBusBusyW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sts_urx_bus_busy(&mut self) -> StsUrxBusBusyW<'_, UartStatusSpec> {
        StsUrxBusBusyW::new(self, 1)
    }
}
#[doc = "uart_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartStatusSpec;
impl crate::RegisterSpec for UartStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_status::R`](R) reader structure"]
impl crate::Readable for UartStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_status::W`](W) writer structure"]
impl crate::Writable for UartStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets uart_status to value 0"]
impl crate::Resettable for UartStatusSpec {}
