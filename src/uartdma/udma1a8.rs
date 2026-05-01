#[doc = "Register `UDMA1A8` reader"]
pub type R = crate::R<Udma1a8Spec>;
#[doc = "Register `UDMA1A8` writer"]
pub type W = crate::W<Udma1a8Spec>;
#[doc = "Field `UARTBMCTXBufBaseAddr` reader - UART-BMC TX buffer base address"]
pub type UartbmctxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UARTBMCTXBufBaseAddr` writer - UART-BMC TX buffer base address"]
pub type UartbmctxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART-BMC TX buffer base address"]
    #[inline(always)]
    pub fn uartbmctxbuf_base_addr(&self) -> UartbmctxbufBaseAddrR {
        UartbmctxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART-BMC TX buffer base address"]
    #[inline(always)]
    pub fn uartbmctxbuf_base_addr(&mut self) -> UartbmctxbufBaseAddrW<Udma1a8Spec> {
        UartbmctxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART-BMC TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1a8Spec;
impl crate::RegisterSpec for Udma1a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1a8::R`](R) reader structure"]
impl crate::Readable for Udma1a8Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1a8::W`](W) writer structure"]
impl crate::Writable for Udma1a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1A8 to value 0"]
impl crate::Resettable for Udma1a8Spec {}
