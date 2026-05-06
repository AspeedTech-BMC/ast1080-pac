#[doc = "Register `UARTDMA040` reader"]
pub type R = crate::R<Uartdma040Spec>;
#[doc = "Register `UARTDMA040` writer"]
pub type W = crate::W<Uartdma040Spec>;
#[doc = "Field `UART0TXReadPointer` reader - UART0 TX read pointer"]
pub type Uart0txreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART0 TX read pointer"]
    #[inline(always)]
    pub fn uart0txread_pointer(&self) -> Uart0txreadPointerR {
        Uart0txreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART0 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma040Spec;
impl crate::RegisterSpec for Uartdma040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma040::R`](R) reader structure"]
impl crate::Readable for Uartdma040Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma040::W`](W) writer structure"]
impl crate::Writable for Uartdma040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA040 to value 0"]
impl crate::Resettable for Uartdma040Spec {}
