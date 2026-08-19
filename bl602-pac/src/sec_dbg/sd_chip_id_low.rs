#[doc = "Register `sd_chip_id_low` reader"]
pub type R = crate::R<SdChipIdLowSpec>;
#[doc = "Register `sd_chip_id_low` writer"]
pub type W = crate::W<SdChipIdLowSpec>;
#[doc = "Field `sd_chip_id_low` reader - "]
pub type SdChipIdLowR = crate::FieldReader<u32>;
#[doc = "Field `sd_chip_id_low` writer - "]
pub type SdChipIdLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_chip_id_low(&self) -> SdChipIdLowR {
        SdChipIdLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_chip_id_low(&mut self) -> SdChipIdLowW<'_, SdChipIdLowSpec> {
        SdChipIdLowW::new(self, 0)
    }
}
#[doc = "sd_chip_id_low.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_chip_id_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_chip_id_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdChipIdLowSpec;
impl crate::RegisterSpec for SdChipIdLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_chip_id_low::R`](R) reader structure"]
impl crate::Readable for SdChipIdLowSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_chip_id_low::W`](W) writer structure"]
impl crate::Writable for SdChipIdLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sd_chip_id_low to value 0"]
impl crate::Resettable for SdChipIdLowSpec {}
