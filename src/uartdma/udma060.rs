#[doc = "Register `UDMA060` reader"]
pub type R = crate::R<Udma060Spec>;
#[doc = "Register `UDMA060` writer"]
pub type W = crate::W<Udma060Spec>;
#[doc = "Field `UART1TXReadPointer` reader - UART1 TX read pointer"]
pub type Uart1txreadPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART1 TX read pointer"]
    #[inline(always)]
    pub fn uart1txread_pointer(&self) -> Uart1txreadPointerR {
        Uart1txreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART1 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma060Spec;
impl crate::RegisterSpec for Udma060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma060::R`](R) reader structure"]
impl crate::Readable for Udma060Spec {}
#[doc = "`write(|w| ..)` method takes [`udma060::W`](W) writer structure"]
impl crate::Writable for Udma060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA060 to value 0"]
impl crate::Resettable for Udma060Spec {}
