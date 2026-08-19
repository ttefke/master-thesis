#[doc = "Register `ef_if_cfg_0` reader"]
pub type R = crate::R<EfIfCfg0Spec>;
#[doc = "Register `ef_if_cfg_0` writer"]
pub type W = crate::W<EfIfCfg0Spec>;
#[doc = "Field `ef_if_sf_aes_mode` reader - "]
pub type EfIfSfAesModeR = crate::FieldReader;
#[doc = "Field `ef_if_sf_aes_mode` writer - "]
pub type EfIfSfAesModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_if_sboot_sign_mode` reader - "]
pub type EfIfSbootSignModeR = crate::FieldReader;
#[doc = "Field `ef_if_sboot_sign_mode` writer - "]
pub type EfIfSbootSignModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_if_sboot_en` reader - "]
pub type EfIfSbootEnR = crate::FieldReader;
#[doc = "Field `ef_if_sboot_en` writer - "]
pub type EfIfSbootEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_if_cpu1_enc_en` reader - "]
pub type EfIfCpu1EncEnR = crate::BitReader;
#[doc = "Field `ef_if_cpu1_enc_en` writer - "]
pub type EfIfCpu1EncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_cpu0_enc_en` reader - "]
pub type EfIfCpu0EncEnR = crate::BitReader;
#[doc = "Field `ef_if_cpu0_enc_en` writer - "]
pub type EfIfCpu0EncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_boot_sel` reader - "]
pub type EfIfBootSelR = crate::FieldReader;
#[doc = "Field `ef_if_boot_sel` writer - "]
pub type EfIfBootSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ef_if_sw_usage_1` reader - "]
pub type EfIfSwUsage1R = crate::FieldReader;
#[doc = "Field `ef_if_sw_usage_1` writer - "]
pub type EfIfSwUsage1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_if_sdu_dis` reader - "]
pub type EfIfSduDisR = crate::BitReader;
#[doc = "Field `ef_if_sdu_dis` writer - "]
pub type EfIfSduDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_ble_dis` reader - "]
pub type EfIfBleDisR = crate::BitReader;
#[doc = "Field `ef_if_ble_dis` writer - "]
pub type EfIfBleDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_wifi_dis` reader - "]
pub type EfIfWifiDisR = crate::BitReader;
#[doc = "Field `ef_if_wifi_dis` writer - "]
pub type EfIfWifiDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_0_key_enc_en` reader - "]
pub type EfIf0KeyEncEnR = crate::BitReader;
#[doc = "Field `ef_if_0_key_enc_en` writer - "]
pub type EfIf0KeyEncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_cam_dis` reader - "]
pub type EfIfCamDisR = crate::BitReader;
#[doc = "Field `ef_if_cam_dis` writer - "]
pub type EfIfCamDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_sf_dis` reader - "]
pub type EfIfSfDisR = crate::BitReader;
#[doc = "Field `ef_if_sf_dis` writer - "]
pub type EfIfSfDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_cpu1_dis` reader - "]
pub type EfIfCpu1DisR = crate::BitReader;
#[doc = "Field `ef_if_cpu1_dis` writer - "]
pub type EfIfCpu1DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_cpu_rst_dbg_dis` reader - "]
pub type EfIfCpuRstDbgDisR = crate::BitReader;
#[doc = "Field `ef_if_cpu_rst_dbg_dis` writer - "]
pub type EfIfCpuRstDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_se_dbg_dis` reader - "]
pub type EfIfSeDbgDisR = crate::BitReader;
#[doc = "Field `ef_if_se_dbg_dis` writer - "]
pub type EfIfSeDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_efuse_dbg_dis` reader - "]
pub type EfIfEfuseDbgDisR = crate::BitReader;
#[doc = "Field `ef_if_efuse_dbg_dis` writer - "]
pub type EfIfEfuseDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_dbg_jtag_1_dis` reader - "]
pub type EfIfDbgJtag1DisR = crate::FieldReader;
#[doc = "Field `ef_if_dbg_jtag_1_dis` writer - "]
pub type EfIfDbgJtag1DisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_if_dbg_jtag_0_dis` reader - "]
pub type EfIfDbgJtag0DisR = crate::FieldReader;
#[doc = "Field `ef_if_dbg_jtag_0_dis` writer - "]
pub type EfIfDbgJtag0DisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ef_if_dbg_mode` reader - "]
pub type EfIfDbgModeR = crate::FieldReader;
#[doc = "Field `ef_if_dbg_mode` writer - "]
pub type EfIfDbgModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_if_sf_aes_mode(&self) -> EfIfSfAesModeR {
        EfIfSfAesModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ef_if_sboot_sign_mode(&self) -> EfIfSbootSignModeR {
        EfIfSbootSignModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_if_sboot_en(&self) -> EfIfSbootEnR {
        EfIfSbootEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_cpu1_enc_en(&self) -> EfIfCpu1EncEnR {
        EfIfCpu1EncEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_if_cpu0_enc_en(&self) -> EfIfCpu0EncEnR {
        EfIfCpu0EncEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ef_if_boot_sel(&self) -> EfIfBootSelR {
        EfIfBootSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_if_sw_usage_1(&self) -> EfIfSwUsage1R {
        EfIfSwUsage1R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_sdu_dis(&self) -> EfIfSduDisR {
        EfIfSduDisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_ble_dis(&self) -> EfIfBleDisR {
        EfIfBleDisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_wifi_dis(&self) -> EfIfWifiDisR {
        EfIfWifiDisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_if_0_key_enc_en(&self) -> EfIf0KeyEncEnR {
        EfIf0KeyEncEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_cam_dis(&self) -> EfIfCamDisR {
        EfIfCamDisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_sf_dis(&self) -> EfIfSfDisR {
        EfIfSfDisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_if_cpu1_dis(&self) -> EfIfCpu1DisR {
        EfIfCpu1DisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_cpu_rst_dbg_dis(&self) -> EfIfCpuRstDbgDisR {
        EfIfCpuRstDbgDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_se_dbg_dis(&self) -> EfIfSeDbgDisR {
        EfIfSeDbgDisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_if_efuse_dbg_dis(&self) -> EfIfEfuseDbgDisR {
        EfIfEfuseDbgDisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_1_dis(&self) -> EfIfDbgJtag1DisR {
        EfIfDbgJtag1DisR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_0_dis(&self) -> EfIfDbgJtag0DisR {
        EfIfDbgJtag0DisR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_if_dbg_mode(&self) -> EfIfDbgModeR {
        EfIfDbgModeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_if_sf_aes_mode(&mut self) -> EfIfSfAesModeW<'_, EfIfCfg0Spec> {
        EfIfSfAesModeW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ef_if_sboot_sign_mode(&mut self) -> EfIfSbootSignModeW<'_, EfIfCfg0Spec> {
        EfIfSbootSignModeW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_if_sboot_en(&mut self) -> EfIfSbootEnW<'_, EfIfCfg0Spec> {
        EfIfSbootEnW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_cpu1_enc_en(&mut self) -> EfIfCpu1EncEnW<'_, EfIfCfg0Spec> {
        EfIfCpu1EncEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_if_cpu0_enc_en(&mut self) -> EfIfCpu0EncEnW<'_, EfIfCfg0Spec> {
        EfIfCpu0EncEnW::new(self, 7)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ef_if_boot_sel(&mut self) -> EfIfBootSelW<'_, EfIfCfg0Spec> {
        EfIfBootSelW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_if_sw_usage_1(&mut self) -> EfIfSwUsage1W<'_, EfIfCfg0Spec> {
        EfIfSwUsage1W::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_sdu_dis(&mut self) -> EfIfSduDisW<'_, EfIfCfg0Spec> {
        EfIfSduDisW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_ble_dis(&mut self) -> EfIfBleDisW<'_, EfIfCfg0Spec> {
        EfIfBleDisW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_wifi_dis(&mut self) -> EfIfWifiDisW<'_, EfIfCfg0Spec> {
        EfIfWifiDisW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_if_0_key_enc_en(&mut self) -> EfIf0KeyEncEnW<'_, EfIfCfg0Spec> {
        EfIf0KeyEncEnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_cam_dis(&mut self) -> EfIfCamDisW<'_, EfIfCfg0Spec> {
        EfIfCamDisW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_sf_dis(&mut self) -> EfIfSfDisW<'_, EfIfCfg0Spec> {
        EfIfSfDisW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_if_cpu1_dis(&mut self) -> EfIfCpu1DisW<'_, EfIfCfg0Spec> {
        EfIfCpu1DisW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_cpu_rst_dbg_dis(&mut self) -> EfIfCpuRstDbgDisW<'_, EfIfCfg0Spec> {
        EfIfCpuRstDbgDisW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_se_dbg_dis(&mut self) -> EfIfSeDbgDisW<'_, EfIfCfg0Spec> {
        EfIfSeDbgDisW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_if_efuse_dbg_dis(&mut self) -> EfIfEfuseDbgDisW<'_, EfIfCfg0Spec> {
        EfIfEfuseDbgDisW::new(self, 23)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_1_dis(&mut self) -> EfIfDbgJtag1DisW<'_, EfIfCfg0Spec> {
        EfIfDbgJtag1DisW::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_0_dis(&mut self) -> EfIfDbgJtag0DisW<'_, EfIfCfg0Spec> {
        EfIfDbgJtag0DisW::new(self, 26)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_if_dbg_mode(&mut self) -> EfIfDbgModeW<'_, EfIfCfg0Spec> {
        EfIfDbgModeW::new(self, 28)
    }
}
#[doc = "ef_if_cfg_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_cfg_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_cfg_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfIfCfg0Spec;
impl crate::RegisterSpec for EfIfCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_if_cfg_0::R`](R) reader structure"]
impl crate::Readable for EfIfCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_if_cfg_0::W`](W) writer structure"]
impl crate::Writable for EfIfCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_if_cfg_0 to value 0"]
impl crate::Resettable for EfIfCfg0Spec {}
