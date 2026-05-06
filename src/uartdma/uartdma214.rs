#[doc = "Register `UARTDMA214` reader"]
pub type R = crate::R<Uartdma214Spec>;
#[doc = "Register `UARTDMA214` writer"]
pub type W = crate::W<Uartdma214Spec>;
#[doc = "Field `VUART2RXWrPointer` reader - VUART2 RX write pointer"]
pub type Vuart2rxwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:24 - VUART2 RX write pointer"]
    #[inline(always)]
    pub fn vuart2rxwr_pointer(&self) -> Vuart2rxwrPointerR {
        Vuart2rxwrPointerR::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bits 25:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {}
#[doc = "VUART2 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma214::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma214::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma214Spec;
impl crate::RegisterSpec for Uartdma214Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma214::R`](R) reader structure"]
impl crate::Readable for Uartdma214Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma214::W`](W) writer structure"]
impl crate::Writable for Uartdma214Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA214 to value 0"]
impl crate::Resettable for Uartdma214Spec {}
