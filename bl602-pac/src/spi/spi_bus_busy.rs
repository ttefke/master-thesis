#[doc = "Register `spi_bus_busy` reader"]
pub type R = crate::R<SpiBusBusySpec>;
#[doc = "Register `spi_bus_busy` writer"]
pub type W = crate::W<SpiBusBusySpec>;
#[doc = "Field `sts_spi_bus_busy` reader - "]
pub type StsSpiBusBusyR = crate::BitReader;
#[doc = "Field `sts_spi_bus_busy` writer - "]
pub type StsSpiBusBusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_spi_bus_busy(&self) -> StsSpiBusBusyR {
        StsSpiBusBusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_spi_bus_busy(&mut self) -> StsSpiBusBusyW<'_, SpiBusBusySpec> {
        StsSpiBusBusyW::new(self, 0)
    }
}
#[doc = "spi_bus_busy.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_bus_busy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_bus_busy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiBusBusySpec;
impl crate::RegisterSpec for SpiBusBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_bus_busy::R`](R) reader structure"]
impl crate::Readable for SpiBusBusySpec {}
#[doc = "`write(|w| ..)` method takes [`spi_bus_busy::W`](W) writer structure"]
impl crate::Writable for SpiBusBusySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets spi_bus_busy to value 0"]
impl crate::Resettable for SpiBusBusySpec {}
