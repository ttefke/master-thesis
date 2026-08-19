#[doc = "Register `WSAR` reader"]
pub type R = crate::R<WsarSpec>;
#[doc = "Register `WSAR` writer"]
pub type W = crate::W<WsarSpec>;
#[doc = "Field `wsar` reader - "]
pub type WsarR = crate::FieldReader<u16>;
#[doc = "Field `wsar` writer - "]
pub type WsarW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wsar(&self) -> WsarR {
        WsarR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wsar(&mut self) -> WsarW<'_, WsarSpec> {
        WsarW::new(self, 0)
    }
}
#[doc = "WSAR.\n\nYou can [`read`](crate::Reg::read) this register and get [`wsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WsarSpec;
impl crate::RegisterSpec for WsarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wsar::R`](R) reader structure"]
impl crate::Readable for WsarSpec {}
#[doc = "`write(|w| ..)` method takes [`wsar::W`](W) writer structure"]
impl crate::Writable for WsarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WSAR to value 0"]
impl crate::Resettable for WsarSpec {}
