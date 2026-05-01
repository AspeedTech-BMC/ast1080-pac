#[doc = "Register `WDT014` reader"]
pub type R = crate::R<Wdt014Spec>;
#[doc = "Register `WDT014` writer"]
pub type W = crate::W<Wdt014Spec>;
#[doc = "Field `ClearTimeoutAndINTSts` reader - Clear timeout and interrupt status"]
pub type ClearTimeoutAndIntstsR = crate::BitReader;
#[doc = "Field `ClearTimeoutAndINTSts` writer - Clear timeout and interrupt status"]
pub type ClearTimeoutAndIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ClearTimeoutCounterSts` reader - Clear timeout counter status"]
pub type ClearTimeoutCounterStsR = crate::FieldReader;
#[doc = "Field `ClearTimeoutCounterSts` writer - Clear timeout counter status"]
pub type ClearTimeoutCounterStsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Clear timeout and interrupt status"]
    #[inline(always)]
    pub fn clear_timeout_and_intsts(&self) -> ClearTimeoutAndIntstsR {
        ClearTimeoutAndIntstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Clear timeout counter status"]
    #[inline(always)]
    pub fn clear_timeout_counter_sts(&self) -> ClearTimeoutCounterStsR {
        ClearTimeoutCounterStsR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Clear timeout and interrupt status"]
    #[inline(always)]
    pub fn clear_timeout_and_intsts(&mut self) -> ClearTimeoutAndIntstsW<Wdt014Spec> {
        ClearTimeoutAndIntstsW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Clear timeout counter status"]
    #[inline(always)]
    pub fn clear_timeout_counter_sts(&mut self) -> ClearTimeoutCounterStsW<Wdt014Spec> {
        ClearTimeoutCounterStsW::new(self, 1)
    }
}
#[doc = "WDTn Clear Timeout Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt014Spec;
impl crate::RegisterSpec for Wdt014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt014::R`](R) reader structure"]
impl crate::Readable for Wdt014Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt014::W`](W) writer structure"]
impl crate::Writable for Wdt014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT014 to value 0"]
impl crate::Resettable for Wdt014Spec {}
