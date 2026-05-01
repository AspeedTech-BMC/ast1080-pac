#[doc = "Register `GPIO000` reader"]
pub type R = crate::R<Gpio000Spec>;
#[doc = "Register `GPIO000` writer"]
pub type W = crate::W<Gpio000Spec>;
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
    pub fn debounce_timer_value(&mut self) -> DebounceTimerValueW<Gpio000Spec> {
        DebounceTimerValueW::new(self, 0)
    }
}
#[doc = "Debounce Timer Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio000Spec;
impl crate::RegisterSpec for Gpio000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio000::R`](R) reader structure"]
impl crate::Readable for Gpio000Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio000::W`](W) writer structure"]
impl crate::Writable for Gpio000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO000 to value 0"]
impl crate::Resettable for Gpio000Spec {}
