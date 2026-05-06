#[doc = "Register `UARTDMA178` reader"]
pub type R = crate::R<Uartdma178Spec>;
#[doc = "Register `UARTDMA178` writer"]
pub type W = crate::W<Uartdma178Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART10RXBufBaseAddr` reader - UART10 RX buffer base address"]
pub type Uart10rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART10RXBufBaseAddr` writer - UART10 RX buffer base address"]
pub type Uart10rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART10 RX buffer base address"]
    #[inline(always)]
    pub fn uart10rxbuf_base_addr(&self) -> Uart10rxbufBaseAddrR {
        Uart10rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART10 RX buffer base address"]
    #[inline(always)]
    pub fn uart10rxbuf_base_addr(&mut self) -> Uart10rxbufBaseAddrW<Uartdma178Spec> {
        Uart10rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART10 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma178::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma178::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma178Spec;
impl crate::RegisterSpec for Uartdma178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma178::R`](R) reader structure"]
impl crate::Readable for Uartdma178Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma178::W`](W) writer structure"]
impl crate::Writable for Uartdma178Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA178 to value 0"]
impl crate::Resettable for Uartdma178Spec {}
