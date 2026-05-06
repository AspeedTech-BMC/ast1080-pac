#[doc = "Register `UARTDMA120` reader"]
pub type R = crate::R<Uartdma120Spec>;
#[doc = "Register `UARTDMA120` writer"]
pub type W = crate::W<Uartdma120Spec>;
#[doc = "Field `UART8TXReadPointer` reader - UART8 TX read pointer"]
pub type Uart8txreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART8 TX read pointer"]
    #[inline(always)]
    pub fn uart8txread_pointer(&self) -> Uart8txreadPointerR {
        Uart8txreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART8 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma120::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma120::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma120Spec;
impl crate::RegisterSpec for Uartdma120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma120::R`](R) reader structure"]
impl crate::Readable for Uartdma120Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma120::W`](W) writer structure"]
impl crate::Writable for Uartdma120Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA120 to value 0"]
impl crate::Resettable for Uartdma120Spec {}
