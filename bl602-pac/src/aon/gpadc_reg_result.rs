#[doc = "Register `gpadc_reg_result` reader"]
pub type R = crate::R<GpadcRegResultSpec>;
#[doc = "Register `gpadc_reg_result` writer"]
pub type W = crate::W<GpadcRegResultSpec>;
#[doc = "Field `gpadc_data_out` reader - "]
pub type GpadcDataOutR = crate::FieldReader<u32>;
#[doc = "Field `gpadc_data_out` writer - "]
pub type GpadcDataOutW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_data_out(&self) -> GpadcDataOutR {
        GpadcDataOutR::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_data_out(&mut self) -> GpadcDataOutW<'_, GpadcRegResultSpec> {
        GpadcDataOutW::new(self, 0)
    }
}
#[doc = "gpadc_reg_result.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegResultSpec;
impl crate::RegisterSpec for GpadcRegResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_result::R`](R) reader structure"]
impl crate::Readable for GpadcRegResultSpec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_result::W`](W) writer structure"]
impl crate::Writable for GpadcRegResultSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gpadc_reg_result to value 0"]
impl crate::Resettable for GpadcRegResultSpec {}
