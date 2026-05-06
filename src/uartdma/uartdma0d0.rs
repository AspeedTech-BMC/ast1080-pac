#[doc = "Register `UARTDMA0D0` reader"]
pub type R = crate::R<Uartdma0d0Spec>;
#[doc = "Register `UARTDMA0D0` writer"]
pub type W = crate::W<Uartdma0d0Spec>;
#[doc = "Field `UART5RXReadPointer` reader - UART5 RX read pointer"]
pub type Uart5rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART5RXReadPointer` writer - UART5 RX read pointer"]
pub type Uart5rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART5 RX read pointer"]
    #[inline(always)]
    pub fn uart5rxread_pointer(&self) -> Uart5rxreadPointerR {
        Uart5rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART5 RX read pointer"]
    #[inline(always)]
    pub fn uart5rxread_pointer(&mut self) -> Uart5rxreadPointerW<Uartdma0d0Spec> {
        Uart5rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART5 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma0d0Spec;
impl crate::RegisterSpec for Uartdma0d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma0d0::R`](R) reader structure"]
impl crate::Readable for Uartdma0d0Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma0d0::W`](W) writer structure"]
impl crate::Writable for Uartdma0d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA0D0 to value 0"]
impl crate::Resettable for Uartdma0d0Spec {}
