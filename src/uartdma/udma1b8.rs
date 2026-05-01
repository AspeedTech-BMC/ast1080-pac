#[doc = "Register `UDMA1B8` reader"]
pub type R = crate::R<Udma1b8Spec>;
#[doc = "Register `UDMA1B8` writer"]
pub type W = crate::W<Udma1b8Spec>;
#[doc = "Field `UARTBMCRXBufBaseAddr` reader - UART-BMC RX buffer base address"]
pub type UartbmcrxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UARTBMCRXBufBaseAddr` writer - UART-BMC RX buffer base address"]
pub type UartbmcrxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART-BMC RX buffer base address"]
    #[inline(always)]
    pub fn uartbmcrxbuf_base_addr(&self) -> UartbmcrxbufBaseAddrR {
        UartbmcrxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART-BMC RX buffer base address"]
    #[inline(always)]
    pub fn uartbmcrxbuf_base_addr(&mut self) -> UartbmcrxbufBaseAddrW<Udma1b8Spec> {
        UartbmcrxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART-BMC RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1b8Spec;
impl crate::RegisterSpec for Udma1b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1b8::R`](R) reader structure"]
impl crate::Readable for Udma1b8Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1b8::W`](W) writer structure"]
impl crate::Writable for Udma1b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1B8 to value 0"]
impl crate::Resettable for Udma1b8Spec {}
