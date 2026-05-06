#[doc = "Register `UARTDMA198` reader"]
pub type R = crate::R<Uartdma198Spec>;
#[doc = "Register `UARTDMA198` writer"]
pub type W = crate::W<Uartdma198Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART11RXBufBaseAddr` reader - UART11 RX buffer base address"]
pub type Uart11rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART11RXBufBaseAddr` writer - UART11 RX buffer base address"]
pub type Uart11rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART11 RX buffer base address"]
    #[inline(always)]
    pub fn uart11rxbuf_base_addr(&self) -> Uart11rxbufBaseAddrR {
        Uart11rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART11 RX buffer base address"]
    #[inline(always)]
    pub fn uart11rxbuf_base_addr(&mut self) -> Uart11rxbufBaseAddrW<Uartdma198Spec> {
        Uart11rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART11 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma198::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma198::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma198Spec;
impl crate::RegisterSpec for Uartdma198Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma198::R`](R) reader structure"]
impl crate::Readable for Uartdma198Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma198::W`](W) writer structure"]
impl crate::Writable for Uartdma198Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA198 to value 0"]
impl crate::Resettable for Uartdma198Spec {}
