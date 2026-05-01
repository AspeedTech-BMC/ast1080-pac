#[doc = "Register `UHCI084` reader"]
pub type R = crate::R<Uhci084Spec>;
#[doc = "Register `UHCI084` writer"]
pub type W = crate::W<Uhci084Spec>;
#[doc = "Field `SOFTimingValue` reader - SOF Timing Value"]
pub type SoftimingValueR = crate::FieldReader;
#[doc = "Field `SOFTimingValue` writer - SOF Timing Value"]
pub type SoftimingValueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:6 - SOF Timing Value"]
    #[inline(always)]
    pub fn softiming_value(&self) -> SoftimingValueR {
        SoftimingValueR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - SOF Timing Value"]
    #[inline(always)]
    pub fn softiming_value(&mut self) -> SoftimingValueW<Uhci084Spec> {
        SoftimingValueW::new(self, 0)
    }
}
#[doc = "Start of Frame Modify Register (SOFMOD)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uhci084Spec;
impl crate::RegisterSpec for Uhci084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci084::R`](R) reader structure"]
impl crate::Readable for Uhci084Spec {}
#[doc = "`write(|w| ..)` method takes [`uhci084::W`](W) writer structure"]
impl crate::Writable for Uhci084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCI084 to value 0x40"]
impl crate::Resettable for Uhci084Spec {
    const RESET_VALUE: u32 = 0x40;
}
