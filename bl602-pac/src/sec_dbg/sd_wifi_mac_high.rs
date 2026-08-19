#[doc = "Register `sd_wifi_mac_high` reader"]
pub type R = crate::R<SdWifiMacHighSpec>;
#[doc = "Register `sd_wifi_mac_high` writer"]
pub type W = crate::W<SdWifiMacHighSpec>;
#[doc = "Field `sd_wifi_mac_high` reader - "]
pub type SdWifiMacHighR = crate::FieldReader<u32>;
#[doc = "Field `sd_wifi_mac_high` writer - "]
pub type SdWifiMacHighW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_wifi_mac_high(&self) -> SdWifiMacHighR {
        SdWifiMacHighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_wifi_mac_high(&mut self) -> SdWifiMacHighW<'_, SdWifiMacHighSpec> {
        SdWifiMacHighW::new(self, 0)
    }
}
#[doc = "sd_wifi_mac_high.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_wifi_mac_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_wifi_mac_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdWifiMacHighSpec;
impl crate::RegisterSpec for SdWifiMacHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_wifi_mac_high::R`](R) reader structure"]
impl crate::Readable for SdWifiMacHighSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_wifi_mac_high::W`](W) writer structure"]
impl crate::Writable for SdWifiMacHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sd_wifi_mac_high to value 0"]
impl crate::Resettable for SdWifiMacHighSpec {}
