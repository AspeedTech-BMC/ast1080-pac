#[doc = "Register `UARTDMA138` reader"]
pub type R = crate::R<Uartdma138Spec>;
#[doc = "Register `UARTDMA138` writer"]
pub type W = crate::W<Uartdma138Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART8RXBufBaseAddr` reader - UART8 RX buffer base address"]
pub type Uart8rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART8RXBufBaseAddr` writer - UART8 RX buffer base address"]
pub type Uart8rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART8 RX buffer base address"]
    #[inline(always)]
    pub fn uart8rxbuf_base_addr(&self) -> Uart8rxbufBaseAddrR {
        Uart8rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART8 RX buffer base address"]
    #[inline(always)]
    pub fn uart8rxbuf_base_addr(&mut self) -> Uart8rxbufBaseAddrW<Uartdma138Spec> {
        Uart8rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART8 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma138::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma138::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma138Spec;
impl crate::RegisterSpec for Uartdma138Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma138::R`](R) reader structure"]
impl crate::Readable for Uartdma138Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma138::W`](W) writer structure"]
impl crate::Writable for Uartdma138Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA138 to value 0"]
impl crate::Resettable for Uartdma138Spec {}
