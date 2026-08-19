#[doc = "Register `l1c_bmx_err_addr` reader"]
pub type R = crate::R<L1cBmxErrAddrSpec>;
#[doc = "Register `l1c_bmx_err_addr` writer"]
pub type W = crate::W<L1cBmxErrAddrSpec>;
#[doc = "Field `l1c_bmx_err_addr` reader - "]
pub type L1cBmxErrAddrR = crate::FieldReader<u32>;
#[doc = "Field `l1c_bmx_err_addr` writer - "]
pub type L1cBmxErrAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr(&self) -> L1cBmxErrAddrR {
        L1cBmxErrAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr(&mut self) -> L1cBmxErrAddrW<'_, L1cBmxErrAddrSpec> {
        L1cBmxErrAddrW::new(self, 0)
    }
}
#[doc = "l1c_bmx_err_addr.\n\nYou can [`read`](crate::Reg::read) this register and get [`l1c_bmx_err_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1c_bmx_err_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1cBmxErrAddrSpec;
impl crate::RegisterSpec for L1cBmxErrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1c_bmx_err_addr::R`](R) reader structure"]
impl crate::Readable for L1cBmxErrAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`l1c_bmx_err_addr::W`](W) writer structure"]
impl crate::Writable for L1cBmxErrAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets l1c_bmx_err_addr to value 0"]
impl crate::Resettable for L1cBmxErrAddrSpec {}
