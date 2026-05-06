#[doc = "Register `UARTDMA1B0` reader"]
pub type R = crate::R<Uartdma1b0Spec>;
#[doc = "Register `UARTDMA1B0` writer"]
pub type W = crate::W<Uartdma1b0Spec>;
#[doc = "Field `UARTBMCRXReadPointer` reader - UART-BMC RX read pointer"]
pub type UartbmcrxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UARTBMCRXReadPointer` writer - UART-BMC RX read pointer"]
pub type UartbmcrxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART-BMC RX read pointer"]
    #[inline(always)]
    pub fn uartbmcrxread_pointer(&self) -> UartbmcrxreadPointerR {
        UartbmcrxreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART-BMC RX read pointer"]
    #[inline(always)]
    pub fn uartbmcrxread_pointer(&mut self) -> UartbmcrxreadPointerW<Uartdma1b0Spec> {
        UartbmcrxreadPointerW::new(self, 0)
    }
}
#[doc = "UART-BMC RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1b0Spec;
impl crate::RegisterSpec for Uartdma1b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1b0::R`](R) reader structure"]
impl crate::Readable for Uartdma1b0Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1b0::W`](W) writer structure"]
impl crate::Writable for Uartdma1b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1B0 to value 0"]
impl crate::Resettable for Uartdma1b0Spec {}
