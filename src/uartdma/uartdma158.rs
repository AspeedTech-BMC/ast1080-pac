#[doc = "Register `UARTDMA158` reader"]
pub type R = crate::R<Uartdma158Spec>;
#[doc = "Register `UARTDMA158` writer"]
pub type W = crate::W<Uartdma158Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART9RXBufBaseAddr` reader - UART9 RX buffer base address"]
pub type Uart9rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART9RXBufBaseAddr` writer - UART9 RX buffer base address"]
pub type Uart9rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART9 RX buffer base address"]
    #[inline(always)]
    pub fn uart9rxbuf_base_addr(&self) -> Uart9rxbufBaseAddrR {
        Uart9rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART9 RX buffer base address"]
    #[inline(always)]
    pub fn uart9rxbuf_base_addr(&mut self) -> Uart9rxbufBaseAddrW<Uartdma158Spec> {
        Uart9rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART9 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma158::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma158::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma158Spec;
impl crate::RegisterSpec for Uartdma158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma158::R`](R) reader structure"]
impl crate::Readable for Uartdma158Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma158::W`](W) writer structure"]
impl crate::Writable for Uartdma158Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA158 to value 0"]
impl crate::Resettable for Uartdma158Spec {}
