#[doc = "Register `UARTDMA0D8` reader"]
pub type R = crate::R<Uartdma0d8Spec>;
#[doc = "Register `UARTDMA0D8` writer"]
pub type W = crate::W<Uartdma0d8Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART5RXBufBaseAddr` reader - UART5 RX buffer base address"]
pub type Uart5rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART5RXBufBaseAddr` writer - UART5 RX buffer base address"]
pub type Uart5rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART5 RX buffer base address"]
    #[inline(always)]
    pub fn uart5rxbuf_base_addr(&self) -> Uart5rxbufBaseAddrR {
        Uart5rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART5 RX buffer base address"]
    #[inline(always)]
    pub fn uart5rxbuf_base_addr(&mut self) -> Uart5rxbufBaseAddrW<Uartdma0d8Spec> {
        Uart5rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART5 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma0d8Spec;
impl crate::RegisterSpec for Uartdma0d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma0d8::R`](R) reader structure"]
impl crate::Readable for Uartdma0d8Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma0d8::W`](W) writer structure"]
impl crate::Writable for Uartdma0d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA0D8 to value 0"]
impl crate::Resettable for Uartdma0d8Spec {}
