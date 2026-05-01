#[doc = "Register `UDMA180` reader"]
pub type R = crate::R<Udma180Spec>;
#[doc = "Register `UDMA180` writer"]
pub type W = crate::W<Udma180Spec>;
#[doc = "Field `UART11TXReadPointer` reader - UART11 TX read pointer"]
pub type Uart11txreadPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART11 TX read pointer"]
    #[inline(always)]
    pub fn uart11txread_pointer(&self) -> Uart11txreadPointerR {
        Uart11txreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART11 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma180::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma180::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma180Spec;
impl crate::RegisterSpec for Udma180Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma180::R`](R) reader structure"]
impl crate::Readable for Udma180Spec {}
#[doc = "`write(|w| ..)` method takes [`udma180::W`](W) writer structure"]
impl crate::Writable for Udma180Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA180 to value 0"]
impl crate::Resettable for Udma180Spec {}
