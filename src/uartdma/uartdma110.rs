#[doc = "Register `UARTDMA110` reader"]
pub type R = crate::R<Uartdma110Spec>;
#[doc = "Register `UARTDMA110` writer"]
pub type W = crate::W<Uartdma110Spec>;
#[doc = "Field `UART7RXReadPointer` reader - UART7 RX read pointer"]
pub type Uart7rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART7RXReadPointer` writer - UART7 RX read pointer"]
pub type Uart7rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART7 RX read pointer"]
    #[inline(always)]
    pub fn uart7rxread_pointer(&self) -> Uart7rxreadPointerR {
        Uart7rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART7 RX read pointer"]
    #[inline(always)]
    pub fn uart7rxread_pointer(&mut self) -> Uart7rxreadPointerW<Uartdma110Spec> {
        Uart7rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART7 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma110Spec;
impl crate::RegisterSpec for Uartdma110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma110::R`](R) reader structure"]
impl crate::Readable for Uartdma110Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma110::W`](W) writer structure"]
impl crate::Writable for Uartdma110Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA110 to value 0"]
impl crate::Resettable for Uartdma110Spec {}
