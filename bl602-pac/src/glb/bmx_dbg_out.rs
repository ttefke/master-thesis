#[doc = "Register `bmx_dbg_out` reader"]
pub type R = crate::R<BmxDbgOutSpec>;
#[doc = "Register `bmx_dbg_out` writer"]
pub type W = crate::W<BmxDbgOutSpec>;
#[doc = "Field `bmx_dbg_out` reader - "]
pub type BmxDbgOutR = crate::FieldReader<u32>;
#[doc = "Field `bmx_dbg_out` writer - "]
pub type BmxDbgOutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_dbg_out(&self) -> BmxDbgOutR {
        BmxDbgOutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_dbg_out(&mut self) -> BmxDbgOutW<'_, BmxDbgOutSpec> {
        BmxDbgOutW::new(self, 0)
    }
}
#[doc = "bmx_dbg_out.\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_dbg_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_dbg_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxDbgOutSpec;
impl crate::RegisterSpec for BmxDbgOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_dbg_out::R`](R) reader structure"]
impl crate::Readable for BmxDbgOutSpec {}
#[doc = "`write(|w| ..)` method takes [`bmx_dbg_out::W`](W) writer structure"]
impl crate::Writable for BmxDbgOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets bmx_dbg_out to value 0"]
impl crate::Resettable for BmxDbgOutSpec {}
