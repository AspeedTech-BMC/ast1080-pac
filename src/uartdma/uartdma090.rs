#[doc = "Register `UARTDMA090` reader"]
pub type R = crate::R<Uartdma090Spec>;
#[doc = "Register `UARTDMA090` writer"]
pub type W = crate::W<Uartdma090Spec>;
#[doc = "Field `UART2RXReadPointer` reader - UART2 RX read pointer"]
pub type Uart2rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART2RXReadPointer` writer - UART2 RX read pointer"]
pub type Uart2rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART2 RX read pointer"]
    #[inline(always)]
    pub fn uart2rxread_pointer(&self) -> Uart2rxreadPointerR {
        Uart2rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART2 RX read pointer"]
    #[inline(always)]
    pub fn uart2rxread_pointer(&mut self) -> Uart2rxreadPointerW<Uartdma090Spec> {
        Uart2rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART2 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma090Spec;
impl crate::RegisterSpec for Uartdma090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma090::R`](R) reader structure"]
impl crate::Readable for Uartdma090Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma090::W`](W) writer structure"]
impl crate::Writable for Uartdma090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA090 to value 0"]
impl crate::Resettable for Uartdma090Spec {}
