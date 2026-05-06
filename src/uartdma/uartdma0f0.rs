#[doc = "Register `UARTDMA0F0` reader"]
pub type R = crate::R<Uartdma0f0Spec>;
#[doc = "Register `UARTDMA0F0` writer"]
pub type W = crate::W<Uartdma0f0Spec>;
#[doc = "Field `UART6RXReadPointer` reader - UART6 RX read pointer"]
pub type Uart6rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART6RXReadPointer` writer - UART6 RX read pointer"]
pub type Uart6rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART6 RX read pointer"]
    #[inline(always)]
    pub fn uart6rxread_pointer(&self) -> Uart6rxreadPointerR {
        Uart6rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART6 RX read pointer"]
    #[inline(always)]
    pub fn uart6rxread_pointer(&mut self) -> Uart6rxreadPointerW<Uartdma0f0Spec> {
        Uart6rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART6 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma0f0Spec;
impl crate::RegisterSpec for Uartdma0f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma0f0::R`](R) reader structure"]
impl crate::Readable for Uartdma0f0Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma0f0::W`](W) writer structure"]
impl crate::Writable for Uartdma0f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA0F0 to value 0"]
impl crate::Resettable for Uartdma0f0Spec {}
