#[doc = "Register `UDMA1A4` reader"]
pub type R = crate::R<Udma1a4Spec>;
#[doc = "Register `UDMA1A4` writer"]
pub type W = crate::W<Udma1a4Spec>;
#[doc = "Field `UARTBMCTXWrPointer` reader - UART-BMC TX write pointer"]
pub type UartbmctxwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UARTBMCTXWrPointer` writer - UART-BMC TX write pointer"]
pub type UartbmctxwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART-BMC TX write pointer"]
    #[inline(always)]
    pub fn uartbmctxwr_pointer(&self) -> UartbmctxwrPointerR {
        UartbmctxwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART-BMC TX write pointer"]
    #[inline(always)]
    pub fn uartbmctxwr_pointer(&mut self) -> UartbmctxwrPointerW<Udma1a4Spec> {
        UartbmctxwrPointerW::new(self, 0)
    }
}
#[doc = "UART-BMC TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1a4Spec;
impl crate::RegisterSpec for Udma1a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1a4::R`](R) reader structure"]
impl crate::Readable for Udma1a4Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1a4::W`](W) writer structure"]
impl crate::Writable for Udma1a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1A4 to value 0"]
impl crate::Resettable for Udma1a4Spec {}
