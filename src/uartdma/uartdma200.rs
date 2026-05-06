#[doc = "Register `UARTDMA200` reader"]
pub type R = crate::R<Uartdma200Spec>;
#[doc = "Register `UARTDMA200` writer"]
pub type W = crate::W<Uartdma200Spec>;
#[doc = "Field `VUART2TXReadPointer` reader - VUART2 TX read pointer"]
pub type Vuart2txreadPointerR = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - VUART2 TX read pointer"]
    #[inline(always)]
    pub fn vuart2txread_pointer(&self) -> Vuart2txreadPointerR {
        Vuart2txreadPointerR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "VUART2 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma200::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma200::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma200Spec;
impl crate::RegisterSpec for Uartdma200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma200::R`](R) reader structure"]
impl crate::Readable for Uartdma200Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma200::W`](W) writer structure"]
impl crate::Writable for Uartdma200Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA200 to value 0"]
impl crate::Resettable for Uartdma200Spec {}
