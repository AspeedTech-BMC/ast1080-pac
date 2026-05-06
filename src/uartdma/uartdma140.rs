#[doc = "Register `UARTDMA140` reader"]
pub type R = crate::R<Uartdma140Spec>;
#[doc = "Register `UARTDMA140` writer"]
pub type W = crate::W<Uartdma140Spec>;
#[doc = "Field `UART9TXReadPointer` reader - UART9 TX read pointer"]
pub type Uart9txreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART9 TX read pointer"]
    #[inline(always)]
    pub fn uart9txread_pointer(&self) -> Uart9txreadPointerR {
        Uart9txreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART9 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma140::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma140::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma140Spec;
impl crate::RegisterSpec for Uartdma140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma140::R`](R) reader structure"]
impl crate::Readable for Uartdma140Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma140::W`](W) writer structure"]
impl crate::Writable for Uartdma140Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA140 to value 0"]
impl crate::Resettable for Uartdma140Spec {}
