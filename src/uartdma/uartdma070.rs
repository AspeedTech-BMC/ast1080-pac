#[doc = "Register `UARTDMA070` reader"]
pub type R = crate::R<Uartdma070Spec>;
#[doc = "Register `UARTDMA070` writer"]
pub type W = crate::W<Uartdma070Spec>;
#[doc = "Field `UART1RXReadPointer` reader - UART1 RX read pointer"]
pub type Uart1rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART1RXReadPointer` writer - UART1 RX read pointer"]
pub type Uart1rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART1 RX read pointer"]
    #[inline(always)]
    pub fn uart1rxread_pointer(&self) -> Uart1rxreadPointerR {
        Uart1rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART1 RX read pointer"]
    #[inline(always)]
    pub fn uart1rxread_pointer(&mut self) -> Uart1rxreadPointerW<Uartdma070Spec> {
        Uart1rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART1 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma070Spec;
impl crate::RegisterSpec for Uartdma070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma070::R`](R) reader structure"]
impl crate::Readable for Uartdma070Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma070::W`](W) writer structure"]
impl crate::Writable for Uartdma070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA070 to value 0"]
impl crate::Resettable for Uartdma070Spec {}
