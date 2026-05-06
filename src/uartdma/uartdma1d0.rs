#[doc = "Register `UARTDMA1D0` reader"]
pub type R = crate::R<Uartdma1d0Spec>;
#[doc = "Register `UARTDMA1D0` writer"]
pub type W = crate::W<Uartdma1d0Spec>;
#[doc = "Field `VUART0RXReadPointer` reader - VUART0 RX read pointer"]
pub type Vuart0rxreadPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART0RXReadPointer` writer - VUART0 RX read pointer"]
pub type Vuart0rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - VUART0 RX read pointer"]
    #[inline(always)]
    pub fn vuart0rxread_pointer(&self) -> Vuart0rxreadPointerR {
        Vuart0rxreadPointerR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART0 RX read pointer"]
    #[inline(always)]
    pub fn vuart0rxread_pointer(&mut self) -> Vuart0rxreadPointerW<Uartdma1d0Spec> {
        Vuart0rxreadPointerW::new(self, 0)
    }
}
#[doc = "VUART0 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1d0Spec;
impl crate::RegisterSpec for Uartdma1d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1d0::R`](R) reader structure"]
impl crate::Readable for Uartdma1d0Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1d0::W`](W) writer structure"]
impl crate::Writable for Uartdma1d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1D0 to value 0"]
impl crate::Resettable for Uartdma1d0Spec {}
