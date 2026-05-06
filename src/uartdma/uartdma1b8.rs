#[doc = "Register `UARTDMA1B8` reader"]
pub type R = crate::R<Uartdma1b8Spec>;
#[doc = "Register `UARTDMA1B8` writer"]
pub type W = crate::W<Uartdma1b8Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UARTBMCRXBufBaseAddr` reader - UART-BMC RX buffer base address"]
pub type UartbmcrxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UARTBMCRXBufBaseAddr` writer - UART-BMC RX buffer base address"]
pub type UartbmcrxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART-BMC RX buffer base address"]
    #[inline(always)]
    pub fn uartbmcrxbuf_base_addr(&self) -> UartbmcrxbufBaseAddrR {
        UartbmcrxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART-BMC RX buffer base address"]
    #[inline(always)]
    pub fn uartbmcrxbuf_base_addr(&mut self) -> UartbmcrxbufBaseAddrW<Uartdma1b8Spec> {
        UartbmcrxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART-BMC RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1b8Spec;
impl crate::RegisterSpec for Uartdma1b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1b8::R`](R) reader structure"]
impl crate::Readable for Uartdma1b8Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1b8::W`](W) writer structure"]
impl crate::Writable for Uartdma1b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1B8 to value 0"]
impl crate::Resettable for Uartdma1b8Spec {}
