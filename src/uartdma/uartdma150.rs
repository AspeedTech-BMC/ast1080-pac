#[doc = "Register `UARTDMA150` reader"]
pub type R = crate::R<Uartdma150Spec>;
#[doc = "Register `UARTDMA150` writer"]
pub type W = crate::W<Uartdma150Spec>;
#[doc = "Field `UART9RXReadPointer` reader - UART9 RX read pointer"]
pub type Uart9rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART9RXReadPointer` writer - UART9 RX read pointer"]
pub type Uart9rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART9 RX read pointer"]
    #[inline(always)]
    pub fn uart9rxread_pointer(&self) -> Uart9rxreadPointerR {
        Uart9rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART9 RX read pointer"]
    #[inline(always)]
    pub fn uart9rxread_pointer(&mut self) -> Uart9rxreadPointerW<Uartdma150Spec> {
        Uart9rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART9 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma150::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma150::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma150Spec;
impl crate::RegisterSpec for Uartdma150Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma150::R`](R) reader structure"]
impl crate::Readable for Uartdma150Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma150::W`](W) writer structure"]
impl crate::Writable for Uartdma150Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA150 to value 0"]
impl crate::Resettable for Uartdma150Spec {}
