#[doc = "Register `miss_cnt` reader"]
pub type R = crate::R<MissCntSpec>;
#[doc = "Register `miss_cnt` writer"]
pub type W = crate::W<MissCntSpec>;
#[doc = "Field `miss_cnt` reader - "]
pub type MissCntR = crate::FieldReader<u32>;
#[doc = "Field `miss_cnt` writer - "]
pub type MissCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn miss_cnt(&self) -> MissCntR {
        MissCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn miss_cnt(&mut self) -> MissCntW<'_, MissCntSpec> {
        MissCntW::new(self, 0)
    }
}
#[doc = "miss_cnt.\n\nYou can [`read`](crate::Reg::read) this register and get [`miss_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miss_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MissCntSpec;
impl crate::RegisterSpec for MissCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miss_cnt::R`](R) reader structure"]
impl crate::Readable for MissCntSpec {}
#[doc = "`write(|w| ..)` method takes [`miss_cnt::W`](W) writer structure"]
impl crate::Writable for MissCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets miss_cnt to value 0"]
impl crate::Resettable for MissCntSpec {}
