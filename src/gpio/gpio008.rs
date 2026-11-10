#[doc = "Register `GPIO008` reader"]
pub type R = crate::R<Gpio008Spec>;
#[doc = "Register `GPIO008` writer"]
pub type W = crate::W<Gpio008Spec>;
#[doc = "Field `DebounceTimerValue` reader - Debounce Timer Value"]
pub type DebounceTimerValueR = crate::FieldReader<u32>;
#[doc = "Field `DebounceTimerValue` writer - Debounce Timer Value"]
pub type DebounceTimerValueW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Debounce Timer Value"]
    #[inline(always)]
    pub fn debounce_timer_value(&self) -> DebounceTimerValueR {
        DebounceTimerValueR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Debounce Timer Value"]
    #[inline(always)]
    pub fn debounce_timer_value(&mut self) -> DebounceTimerValueW<Gpio008Spec> {
        DebounceTimerValueW::new(self, 0)
    }
}
#[doc = "Debounce Timer Setting Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio008Spec;
impl crate::RegisterSpec for Gpio008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio008::R`](R) reader structure"]
impl crate::Readable for Gpio008Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio008::W`](W) writer structure"]
impl crate::Writable for Gpio008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO008 to value 0"]
impl crate::Resettable for Gpio008Spec {}
