#[doc = "Register `GPIO008` reader"]
pub type R = crate::R<Gpio008Spec>;
#[doc = "Register `GPIO008` writer"]
pub type W = crate::W<Gpio008Spec>;
#[doc = "Field `DebounceTimerValue2` reader - Debounce Timer Value"]
pub type DebounceTimerValue2R = crate::FieldReader<u32>;
#[doc = "Field `DebounceTimerValue2` writer - Debounce Timer Value"]
pub type DebounceTimerValue2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - Debounce Timer Value"]
    #[inline(always)]
    pub fn debounce_timer_value2(&self) -> DebounceTimerValue2R {
        DebounceTimerValue2R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Debounce Timer Value"]
    #[inline(always)]
    pub fn debounce_timer_value2(&mut self) -> DebounceTimerValue2W<Gpio008Spec> {
        DebounceTimerValue2W::new(self, 0)
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
