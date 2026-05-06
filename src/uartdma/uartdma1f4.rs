#[doc = "Register `UARTDMA1F4` reader"]
pub type R = crate::R<Uartdma1f4Spec>;
#[doc = "Register `UARTDMA1F4` writer"]
pub type W = crate::W<Uartdma1f4Spec>;
#[doc = "Field `VUART1RXWrPointer` reader - VUART1 RX write pointer"]
pub type Vuart1rxwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:24 - VUART1 RX write pointer"]
    #[inline(always)]
    pub fn vuart1rxwr_pointer(&self) -> Vuart1rxwrPointerR {
        Vuart1rxwrPointerR::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bits 25:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {}
#[doc = "VUART1 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1f4Spec;
impl crate::RegisterSpec for Uartdma1f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1f4::R`](R) reader structure"]
impl crate::Readable for Uartdma1f4Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1f4::W`](W) writer structure"]
impl crate::Writable for Uartdma1f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1F4 to value 0"]
impl crate::Resettable for Uartdma1f4Spec {}
