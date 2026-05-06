#[doc = "Register `UARTDMA1D4` reader"]
pub type R = crate::R<Uartdma1d4Spec>;
#[doc = "Register `UARTDMA1D4` writer"]
pub type W = crate::W<Uartdma1d4Spec>;
#[doc = "Field `VUART0RXWrPointer` reader - VUART0 RX write pointer"]
pub type Vuart0rxwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:24 - VUART0 RX write pointer"]
    #[inline(always)]
    pub fn vuart0rxwr_pointer(&self) -> Vuart0rxwrPointerR {
        Vuart0rxwrPointerR::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bits 25:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {}
#[doc = "VUART0 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1d4Spec;
impl crate::RegisterSpec for Uartdma1d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1d4::R`](R) reader structure"]
impl crate::Readable for Uartdma1d4Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1d4::W`](W) writer structure"]
impl crate::Writable for Uartdma1d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1D4 to value 0"]
impl crate::Resettable for Uartdma1d4Spec {}
