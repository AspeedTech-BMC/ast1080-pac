#[doc = "Register `UARTDMA234` reader"]
pub type R = crate::R<Uartdma234Spec>;
#[doc = "Register `UARTDMA234` writer"]
pub type W = crate::W<Uartdma234Spec>;
#[doc = "Field `VUART3RXWrPointer` reader - VUART3 RX write pointer"]
pub type Vuart3rxwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:24 - VUART3 RX write pointer"]
    #[inline(always)]
    pub fn vuart3rxwr_pointer(&self) -> Vuart3rxwrPointerR {
        Vuart3rxwrPointerR::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bits 25:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {}
#[doc = "VUART3 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma234::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma234::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma234Spec;
impl crate::RegisterSpec for Uartdma234Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma234::R`](R) reader structure"]
impl crate::Readable for Uartdma234Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma234::W`](W) writer structure"]
impl crate::Writable for Uartdma234Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA234 to value 0"]
impl crate::Resettable for Uartdma234Spec {}
