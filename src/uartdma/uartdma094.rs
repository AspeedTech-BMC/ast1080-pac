#[doc = "Register `UARTDMA094` reader"]
pub type R = crate::R<Uartdma094Spec>;
#[doc = "Register `UARTDMA094` writer"]
pub type W = crate::W<Uartdma094Spec>;
#[doc = "Field `UART2RXWrPointer` reader - UART2 RX write pointer"]
pub type Uart2rxwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:16 - UART2 RX write pointer"]
    #[inline(always)]
    pub fn uart2rxwr_pointer(&self) -> Uart2rxwrPointerR {
        Uart2rxwrPointerR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 17:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {}
#[doc = "UART2 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma094Spec;
impl crate::RegisterSpec for Uartdma094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma094::R`](R) reader structure"]
impl crate::Readable for Uartdma094Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma094::W`](W) writer structure"]
impl crate::Writable for Uartdma094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA094 to value 0"]
impl crate::Resettable for Uartdma094Spec {}
