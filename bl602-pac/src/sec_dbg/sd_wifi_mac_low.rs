#[doc = "Register `sd_wifi_mac_low` reader"]
pub type R = crate::R<SdWifiMacLowSpec>;
#[doc = "Register `sd_wifi_mac_low` writer"]
pub type W = crate::W<SdWifiMacLowSpec>;
#[doc = "Field `sd_wifi_mac_low` reader - "]
pub type SdWifiMacLowR = crate::FieldReader<u32>;
#[doc = "Field `sd_wifi_mac_low` writer - "]
pub type SdWifiMacLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_wifi_mac_low(&self) -> SdWifiMacLowR {
        SdWifiMacLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_wifi_mac_low(&mut self) -> SdWifiMacLowW<'_, SdWifiMacLowSpec> {
        SdWifiMacLowW::new(self, 0)
    }
}
#[doc = "sd_wifi_mac_low.\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_wifi_mac_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_wifi_mac_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdWifiMacLowSpec;
impl crate::RegisterSpec for SdWifiMacLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_wifi_mac_low::R`](R) reader structure"]
impl crate::Readable for SdWifiMacLowSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_wifi_mac_low::W`](W) writer structure"]
impl crate::Writable for SdWifiMacLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sd_wifi_mac_low to value 0"]
impl crate::Resettable for SdWifiMacLowSpec {}
