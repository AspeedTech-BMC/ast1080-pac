#[doc = "Register `UARTDMA170` reader"]
pub type R = crate::R<Uartdma170Spec>;
#[doc = "Register `UARTDMA170` writer"]
pub type W = crate::W<Uartdma170Spec>;
#[doc = "Field `UART10RXReadPointer` reader - UART10 RX read pointer"]
pub type Uart10rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART10RXReadPointer` writer - UART10 RX read pointer"]
pub type Uart10rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART10 RX read pointer"]
    #[inline(always)]
    pub fn uart10rxread_pointer(&self) -> Uart10rxreadPointerR {
        Uart10rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART10 RX read pointer"]
    #[inline(always)]
    pub fn uart10rxread_pointer(&mut self) -> Uart10rxreadPointerW<Uartdma170Spec> {
        Uart10rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART10 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma170::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma170::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma170Spec;
impl crate::RegisterSpec for Uartdma170Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma170::R`](R) reader structure"]
impl crate::Readable for Uartdma170Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma170::W`](W) writer structure"]
impl crate::Writable for Uartdma170Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA170 to value 0"]
impl crate::Resettable for Uartdma170Spec {}
