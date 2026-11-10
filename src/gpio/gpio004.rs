#[doc = "Register `GPIO004` reader"]
pub type R = crate::R<Gpio004Spec>;
#[doc = "Register `GPIO004` writer"]
pub type W = crate::W<Gpio004Spec>;
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
    pub fn debounce_timer_value(&mut self) -> DebounceTimerValueW<Gpio004Spec> {
        DebounceTimerValueW::new(self, 0)
    }
}
#[doc = "Debounce Timer Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio004Spec;
impl crate::RegisterSpec for Gpio004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio004::R`](R) reader structure"]
impl crate::Readable for Gpio004Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio004::W`](W) writer structure"]
impl crate::Writable for Gpio004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO004 to value 0"]
impl crate::Resettable for Gpio004Spec {}
