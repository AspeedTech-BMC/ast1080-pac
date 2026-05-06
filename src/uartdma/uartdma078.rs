#[doc = "Register `UARTDMA078` reader"]
pub type R = crate::R<Uartdma078Spec>;
#[doc = "Register `UARTDMA078` writer"]
pub type W = crate::W<Uartdma078Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART1RXBufBaseAddr` reader - UART1 RX buffer base address"]
pub type Uart1rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART1RXBufBaseAddr` writer - UART1 RX buffer base address"]
pub type Uart1rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART1 RX buffer base address"]
    #[inline(always)]
    pub fn uart1rxbuf_base_addr(&self) -> Uart1rxbufBaseAddrR {
        Uart1rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART1 RX buffer base address"]
    #[inline(always)]
    pub fn uart1rxbuf_base_addr(&mut self) -> Uart1rxbufBaseAddrW<Uartdma078Spec> {
        Uart1rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART1 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma078::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma078::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma078Spec;
impl crate::RegisterSpec for Uartdma078Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma078::R`](R) reader structure"]
impl crate::Readable for Uartdma078Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma078::W`](W) writer structure"]
impl crate::Writable for Uartdma078Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA078 to value 0"]
impl crate::Resettable for Uartdma078Spec {}
