#[doc = "Register `bmx_err_addr` reader"]
pub type R = crate::R<BmxErrAddrSpec>;
#[doc = "Register `bmx_err_addr` writer"]
pub type W = crate::W<BmxErrAddrSpec>;
#[doc = "Field `bmx_err_addr` reader - "]
pub type BmxErrAddrR = crate::FieldReader<u32>;
#[doc = "Field `bmx_err_addr` writer - "]
pub type BmxErrAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_err_addr(&self) -> BmxErrAddrR {
        BmxErrAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_err_addr(&mut self) -> BmxErrAddrW<'_, BmxErrAddrSpec> {
        BmxErrAddrW::new(self, 0)
    }
}
#[doc = "bmx_err_addr.\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_err_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_err_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxErrAddrSpec;
impl crate::RegisterSpec for BmxErrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_err_addr::R`](R) reader structure"]
impl crate::Readable for BmxErrAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`bmx_err_addr::W`](W) writer structure"]
impl crate::Writable for BmxErrAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets bmx_err_addr to value 0"]
impl crate::Resettable for BmxErrAddrSpec {}
