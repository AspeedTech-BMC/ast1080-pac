#[doc = "Register `UARTDMA194` reader"]
pub type R = crate::R<Uartdma194Spec>;
#[doc = "Register `UARTDMA194` writer"]
pub type W = crate::W<Uartdma194Spec>;
#[doc = "Field `UART11RXWrPointer` reader - UART11 RX write pointer"]
pub type Uart11rxwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:16 - UART11 RX write pointer"]
    #[inline(always)]
    pub fn uart11rxwr_pointer(&self) -> Uart11rxwrPointerR {
        Uart11rxwrPointerR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 17:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {}
#[doc = "UART11 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma194::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma194::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma194Spec;
impl crate::RegisterSpec for Uartdma194Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma194::R`](R) reader structure"]
impl crate::Readable for Uartdma194Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma194::W`](W) writer structure"]
impl crate::Writable for Uartdma194Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA194 to value 0"]
impl crate::Resettable for Uartdma194Spec {}
