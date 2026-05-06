#[doc = "Register `UARTDMA074` reader"]
pub type R = crate::R<Uartdma074Spec>;
#[doc = "Register `UARTDMA074` writer"]
pub type W = crate::W<Uartdma074Spec>;
#[doc = "Field `UART1RXWrPointer` reader - UART1 RX write pointer"]
pub type Uart1rxwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:16 - UART1 RX write pointer"]
    #[inline(always)]
    pub fn uart1rxwr_pointer(&self) -> Uart1rxwrPointerR {
        Uart1rxwrPointerR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 17:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {}
#[doc = "UART1 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma074Spec;
impl crate::RegisterSpec for Uartdma074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma074::R`](R) reader structure"]
impl crate::Readable for Uartdma074Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma074::W`](W) writer structure"]
impl crate::Writable for Uartdma074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA074 to value 0"]
impl crate::Resettable for Uartdma074Spec {}
