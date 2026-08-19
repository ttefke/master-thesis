#[doc = "Register `HBN_IRQ_STAT` reader"]
pub type R = crate::R<HbnIrqStatSpec>;
#[doc = "Register `HBN_IRQ_STAT` writer"]
pub type W = crate::W<HbnIrqStatSpec>;
#[doc = "Field `irq_stat` reader - "]
pub type IrqStatR = crate::FieldReader<u32>;
#[doc = "Field `irq_stat` writer - "]
pub type IrqStatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irq_stat(&self) -> IrqStatR {
        IrqStatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irq_stat(&mut self) -> IrqStatW<'_, HbnIrqStatSpec> {
        IrqStatW::new(self, 0)
    }
}
#[doc = "HBN_IRQ_STAT.\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_irq_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_irq_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnIrqStatSpec;
impl crate::RegisterSpec for HbnIrqStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_irq_stat::R`](R) reader structure"]
impl crate::Readable for HbnIrqStatSpec {}
#[doc = "`write(|w| ..)` method takes [`hbn_irq_stat::W`](W) writer structure"]
impl crate::Writable for HbnIrqStatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_IRQ_STAT to value 0"]
impl crate::Resettable for HbnIrqStatSpec {}
