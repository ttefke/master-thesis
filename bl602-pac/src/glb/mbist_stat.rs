#[doc = "Register `MBIST_STAT` reader"]
pub type R = crate::R<MbistStatSpec>;
#[doc = "Register `MBIST_STAT` writer"]
pub type W = crate::W<MbistStatSpec>;
#[doc = "Field `irom_mbist_done` reader - "]
pub type IromMbistDoneR = crate::BitReader;
#[doc = "Field `irom_mbist_done` writer - "]
pub type IromMbistDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hsram_mbist_done` reader - "]
pub type HsramMbistDoneR = crate::BitReader;
#[doc = "Field `hsram_mbist_done` writer - "]
pub type HsramMbistDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tag_mbist_done` reader - "]
pub type TagMbistDoneR = crate::BitReader;
#[doc = "Field `tag_mbist_done` writer - "]
pub type TagMbistDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ocram_mbist_done` reader - "]
pub type OcramMbistDoneR = crate::BitReader;
#[doc = "Field `ocram_mbist_done` writer - "]
pub type OcramMbistDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_mbist_done` reader - "]
pub type WifiMbistDoneR = crate::BitReader;
#[doc = "Field `wifi_mbist_done` writer - "]
pub type WifiMbistDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `irom_mbist_fail` reader - "]
pub type IromMbistFailR = crate::BitReader;
#[doc = "Field `irom_mbist_fail` writer - "]
pub type IromMbistFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hsram_mbist_fail` reader - "]
pub type HsramMbistFailR = crate::BitReader;
#[doc = "Field `hsram_mbist_fail` writer - "]
pub type HsramMbistFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tag_mbist_fail` reader - "]
pub type TagMbistFailR = crate::BitReader;
#[doc = "Field `tag_mbist_fail` writer - "]
pub type TagMbistFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ocram_mbist_fail` reader - "]
pub type OcramMbistFailR = crate::BitReader;
#[doc = "Field `ocram_mbist_fail` writer - "]
pub type OcramMbistFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_mbist_fail` reader - "]
pub type WifiMbistFailR = crate::BitReader;
#[doc = "Field `wifi_mbist_fail` writer - "]
pub type WifiMbistFailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_done(&self) -> IromMbistDoneR {
        IromMbistDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_done(&self) -> HsramMbistDoneR {
        HsramMbistDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_done(&self) -> TagMbistDoneR {
        TagMbistDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_done(&self) -> OcramMbistDoneR {
        OcramMbistDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_done(&self) -> WifiMbistDoneR {
        WifiMbistDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irom_mbist_fail(&self) -> IromMbistFailR {
        IromMbistFailR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hsram_mbist_fail(&self) -> HsramMbistFailR {
        HsramMbistFailR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tag_mbist_fail(&self) -> TagMbistFailR {
        TagMbistFailR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocram_mbist_fail(&self) -> OcramMbistFailR {
        OcramMbistFailR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifi_mbist_fail(&self) -> WifiMbistFailR {
        WifiMbistFailR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_done(&mut self) -> IromMbistDoneW<'_, MbistStatSpec> {
        IromMbistDoneW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_done(&mut self) -> HsramMbistDoneW<'_, MbistStatSpec> {
        HsramMbistDoneW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_done(&mut self) -> TagMbistDoneW<'_, MbistStatSpec> {
        TagMbistDoneW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_done(&mut self) -> OcramMbistDoneW<'_, MbistStatSpec> {
        OcramMbistDoneW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_done(&mut self) -> WifiMbistDoneW<'_, MbistStatSpec> {
        WifiMbistDoneW::new(self, 4)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irom_mbist_fail(&mut self) -> IromMbistFailW<'_, MbistStatSpec> {
        IromMbistFailW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hsram_mbist_fail(&mut self) -> HsramMbistFailW<'_, MbistStatSpec> {
        HsramMbistFailW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tag_mbist_fail(&mut self) -> TagMbistFailW<'_, MbistStatSpec> {
        TagMbistFailW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocram_mbist_fail(&mut self) -> OcramMbistFailW<'_, MbistStatSpec> {
        OcramMbistFailW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifi_mbist_fail(&mut self) -> WifiMbistFailW<'_, MbistStatSpec> {
        WifiMbistFailW::new(self, 20)
    }
}
#[doc = "MBIST_STAT.\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbist_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistStatSpec;
impl crate::RegisterSpec for MbistStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_stat::R`](R) reader structure"]
impl crate::Readable for MbistStatSpec {}
#[doc = "`write(|w| ..)` method takes [`mbist_stat::W`](W) writer structure"]
impl crate::Writable for MbistStatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBIST_STAT to value 0"]
impl crate::Resettable for MbistStatSpec {}
