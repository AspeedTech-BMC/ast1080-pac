#[doc = "Register `UARTDMA190` reader"]
pub type R = crate::R<Uartdma190Spec>;
#[doc = "Register `UARTDMA190` writer"]
pub type W = crate::W<Uartdma190Spec>;
#[doc = "Field `UART11RXReadPointer` reader - UART11 RX read pointer"]
pub type Uart11rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART11RXReadPointer` writer - UART11 RX read pointer"]
pub type Uart11rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART11 RX read pointer"]
    #[inline(always)]
    pub fn uart11rxread_pointer(&self) -> Uart11rxreadPointerR {
        Uart11rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART11 RX read pointer"]
    #[inline(always)]
    pub fn uart11rxread_pointer(&mut self) -> Uart11rxreadPointerW<Uartdma190Spec> {
        Uart11rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART11 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma190::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma190::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma190Spec;
impl crate::RegisterSpec for Uartdma190Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma190::R`](R) reader structure"]
impl crate::Readable for Uartdma190Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma190::W`](W) writer structure"]
impl crate::Writable for Uartdma190Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA190 to value 0"]
impl crate::Resettable for Uartdma190Spec {}
