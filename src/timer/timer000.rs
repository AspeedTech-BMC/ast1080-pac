#[doc = "Register `TIMER000` reader"]
pub type R = crate::R<Timer000Spec>;
#[doc = "Register `TIMER000` writer"]
pub type W = crate::W<Timer000Spec>;
#[doc = "Field `CounterSts` reader - Counter status"]
pub type CounterStsR = crate::FieldReader<u32>;
#[doc = "Field `CounterSts` writer - Counter status"]
pub type CounterStsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter status"]
    #[inline(always)]
    pub fn counter_sts(&self) -> CounterStsR {
        CounterStsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter status"]
    #[inline(always)]
    pub fn counter_sts(&mut self) -> CounterStsW<Timer000Spec> {
        CounterStsW::new(self, 0)
    }
}
#[doc = "Counter Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer000Spec;
impl crate::RegisterSpec for Timer000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer000::R`](R) reader structure"]
impl crate::Readable for Timer000Spec {}
#[doc = "`write(|w| ..)` method takes [`timer000::W`](W) writer structure"]
impl crate::Writable for Timer000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER000 to value 0"]
impl crate::Resettable for Timer000Spec {}
