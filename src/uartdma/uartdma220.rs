#[doc = "Register `UARTDMA220` reader"]
pub type R = crate::R<Uartdma220Spec>;
#[doc = "Register `UARTDMA220` writer"]
pub type W = crate::W<Uartdma220Spec>;
#[doc = "Field `VUART3TXReadPointer` reader - VUART3 TX read pointer"]
pub type Vuart3txreadPointerR = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - VUART3 TX read pointer"]
    #[inline(always)]
    pub fn vuart3txread_pointer(&self) -> Vuart3txreadPointerR {
        Vuart3txreadPointerR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "VUART3 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma220::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma220::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma220Spec;
impl crate::RegisterSpec for Uartdma220Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma220::R`](R) reader structure"]
impl crate::Readable for Uartdma220Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma220::W`](W) writer structure"]
impl crate::Writable for Uartdma220Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA220 to value 0"]
impl crate::Resettable for Uartdma220Spec {}
