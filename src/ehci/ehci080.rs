#[doc = "Register `EHCI080` reader"]
pub type R = crate::R<Ehci080Spec>;
#[doc = "Register `EHCI080` writer"]
pub type W = crate::W<Ehci080Spec>;
#[doc = "Field `FrameLengthTimingValue` reader - Frame Length Timing Value"]
pub type FrameLengthTimingValueR = crate::FieldReader;
#[doc = "Field `FrameLengthTimingValue` writer - Frame Length Timing Value"]
pub type FrameLengthTimingValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - Frame Length Timing Value"]
    #[inline(always)]
    pub fn frame_length_timing_value(&self) -> FrameLengthTimingValueR {
        FrameLengthTimingValueR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frame Length Timing Value"]
    #[inline(always)]
    pub fn frame_length_timing_value(&mut self) -> FrameLengthTimingValueW<Ehci080Spec> {
        FrameLengthTimingValueW::new(self, 0)
    }
}
#[doc = "Frame Length Adjustment Register (FLADJ)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci080Spec;
impl crate::RegisterSpec for Ehci080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci080::R`](R) reader structure"]
impl crate::Readable for Ehci080Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci080::W`](W) writer structure"]
impl crate::Writable for Ehci080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI080 to value 0x20"]
impl crate::Resettable for Ehci080Spec {
    const RESET_VALUE: u32 = 0x20;
}
