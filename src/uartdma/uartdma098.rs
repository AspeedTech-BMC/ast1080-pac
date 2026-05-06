#[doc = "Register `UARTDMA098` reader"]
pub type R = crate::R<Uartdma098Spec>;
#[doc = "Register `UARTDMA098` writer"]
pub type W = crate::W<Uartdma098Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART2RXBufBaseAddr` reader - UART2 RX buffer base address"]
pub type Uart2rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART2RXBufBaseAddr` writer - UART2 RX buffer base address"]
pub type Uart2rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART2 RX buffer base address"]
    #[inline(always)]
    pub fn uart2rxbuf_base_addr(&self) -> Uart2rxbufBaseAddrR {
        Uart2rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART2 RX buffer base address"]
    #[inline(always)]
    pub fn uart2rxbuf_base_addr(&mut self) -> Uart2rxbufBaseAddrW<Uartdma098Spec> {
        Uart2rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART2 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma098Spec;
impl crate::RegisterSpec for Uartdma098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma098::R`](R) reader structure"]
impl crate::Readable for Uartdma098Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma098::W`](W) writer structure"]
impl crate::Writable for Uartdma098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA098 to value 0"]
impl crate::Resettable for Uartdma098Spec {}
