#[doc = "Register `WDT000` reader"]
pub type R = crate::R<Wdt000Spec>;
#[doc = "Register `WDT000` writer"]
pub type W = crate::W<Wdt000Spec>;
#[doc = "Field `CounterSts` reader - Counter status"]
pub type CounterStsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Counter status"]
    #[inline(always)]
    pub fn counter_sts(&self) -> CounterStsR {
        CounterStsR::new(self.bits)
    }
}
impl W {}
#[doc = "WDTn Counter Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt000Spec;
impl crate::RegisterSpec for Wdt000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt000::R`](R) reader structure"]
impl crate::Readable for Wdt000Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt000::W`](W) writer structure"]
impl crate::Writable for Wdt000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT000 to value 0x014f_b180"]
impl crate::Resettable for Wdt000Spec {
    const RESET_VALUE: u32 = 0x014f_b180;
}
