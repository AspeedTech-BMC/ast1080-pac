#[doc = "Register `UARTDMA080` reader"]
pub type R = crate::R<Uartdma080Spec>;
#[doc = "Register `UARTDMA080` writer"]
pub type W = crate::W<Uartdma080Spec>;
#[doc = "Field `UART2TXReadPointer` reader - UART2 TX read pointer"]
pub type Uart2txreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART2 TX read pointer"]
    #[inline(always)]
    pub fn uart2txread_pointer(&self) -> Uart2txreadPointerR {
        Uart2txreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART2 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma080Spec;
impl crate::RegisterSpec for Uartdma080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma080::R`](R) reader structure"]
impl crate::Readable for Uartdma080Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma080::W`](W) writer structure"]
impl crate::Writable for Uartdma080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA080 to value 0"]
impl crate::Resettable for Uartdma080Spec {}
