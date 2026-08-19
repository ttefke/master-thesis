#[doc = "Register `sd_chip_id_high` reader"]
pub type R = crate::R<SdChipIdHighSpec>;
#[doc = "Register `sd_chip_id_high` writer"]
pub type W = crate::W<SdChipIdHighSpec>;
#[doc = "Field `sd_chip_id_high` reader - "]
pub type SdChipIdHighR = crate::FieldReader<u32>;
#[doc = "Field `sd_chip_id_high` writer - "]
pub type SdChipIdHighW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_chip_id_high(&self) -> SdChipIdHighR {
        SdChipIdHighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_chip_id_high(&mut self) -> SdChipIdHighW<'_, SdChipIdHighSpec> {
        SdChipIdHighW::new(self, 0)
    }
}
#[doc = "sd_chip_id_high.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_chip_id_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_chip_id_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdChipIdHighSpec;
impl crate::RegisterSpec for SdChipIdHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_chip_id_high::R`](R) reader structure"]
impl crate::Readable for SdChipIdHighSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_chip_id_high::W`](W) writer structure"]
impl crate::Writable for SdChipIdHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sd_chip_id_high to value 0"]
impl crate::Resettable for SdChipIdHighSpec {}
