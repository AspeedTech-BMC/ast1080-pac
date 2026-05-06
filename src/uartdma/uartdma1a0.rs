#[doc = "Register `UARTDMA1A0` reader"]
pub type R = crate::R<Uartdma1a0Spec>;
#[doc = "Register `UARTDMA1A0` writer"]
pub type W = crate::W<Uartdma1a0Spec>;
#[doc = "Field `UARTBMCTXReadPointer` reader - UART-BMC TX read pointer"]
pub type UartbmctxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART-BMC TX read pointer"]
    #[inline(always)]
    pub fn uartbmctxread_pointer(&self) -> UartbmctxreadPointerR {
        UartbmctxreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART-BMC TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1a0Spec;
impl crate::RegisterSpec for Uartdma1a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1a0::R`](R) reader structure"]
impl crate::Readable for Uartdma1a0Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1a0::W`](W) writer structure"]
impl crate::Writable for Uartdma1a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1A0 to value 0"]
impl crate::Resettable for Uartdma1a0Spec {}
