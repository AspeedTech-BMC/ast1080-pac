#[doc = "Register `EHCI088` reader"]
pub type R = crate::R<Ehci088Spec>;
#[doc = "Register `EHCI088` writer"]
pub type W = crate::W<Ehci088Spec>;
#[doc = "Field `SOFTxDelayTiming` reader - SOF transmit delay timing"]
pub type SoftxDelayTimingR = crate::FieldReader<u16>;
#[doc = "Field `SOFTxDelayTiming` writer - SOF transmit delay timing"]
pub type SoftxDelayTimingW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PreEOF1Timing` reader - preEOF1 timing"]
pub type PreEof1timingR = crate::FieldReader<u16>;
#[doc = "Field `PreEOF1Timing` writer - preEOF1 timing"]
pub type PreEof1timingW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PreEOF2Timing` reader - preEOF2 timing"]
pub type PreEof2timingR = crate::FieldReader<u16>;
#[doc = "Field `PreEOF2Timing` writer - preEOF2 timing"]
pub type PreEof2timingW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:11 - SOF transmit delay timing"]
    #[inline(always)]
    pub fn softx_delay_timing(&self) -> SoftxDelayTimingR {
        SoftxDelayTimingR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:21 - preEOF1 timing"]
    #[inline(always)]
    pub fn pre_eof1timing(&self) -> PreEof1timingR {
        PreEof1timingR::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31 - preEOF2 timing"]
    #[inline(always)]
    pub fn pre_eof2timing(&self) -> PreEof2timingR {
        PreEof2timingR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - SOF transmit delay timing"]
    #[inline(always)]
    pub fn softx_delay_timing(&mut self) -> SoftxDelayTimingW<Ehci088Spec> {
        SoftxDelayTimingW::new(self, 0)
    }
    #[doc = "Bits 12:21 - preEOF1 timing"]
    #[inline(always)]
    pub fn pre_eof1timing(&mut self) -> PreEof1timingW<Ehci088Spec> {
        PreEof1timingW::new(self, 12)
    }
    #[doc = "Bits 22:31 - preEOF2 timing"]
    #[inline(always)]
    pub fn pre_eof2timing(&mut self) -> PreEof2timingW<Ehci088Spec> {
        PreEof2timingW::new(self, 22)
    }
}
#[doc = "Frame Timing Adjustment\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci088Spec;
impl crate::RegisterSpec for Ehci088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci088::R`](R) reader structure"]
impl crate::Readable for Ehci088Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci088::W`](W) writer structure"]
impl crate::Writable for Ehci088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI088 to value 0x4010_0000"]
impl crate::Resettable for Ehci088Spec {
    const RESET_VALUE: u32 = 0x4010_0000;
}
