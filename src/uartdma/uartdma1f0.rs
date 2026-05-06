#[doc = "Register `UARTDMA1F0` reader"]
pub type R = crate::R<Uartdma1f0Spec>;
#[doc = "Register `UARTDMA1F0` writer"]
pub type W = crate::W<Uartdma1f0Spec>;
#[doc = "Field `VUART1RXReadPointer` reader - VUART1 RX read pointer"]
pub type Vuart1rxreadPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART1RXReadPointer` writer - VUART1 RX read pointer"]
pub type Vuart1rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - VUART1 RX read pointer"]
    #[inline(always)]
    pub fn vuart1rxread_pointer(&self) -> Vuart1rxreadPointerR {
        Vuart1rxreadPointerR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART1 RX read pointer"]
    #[inline(always)]
    pub fn vuart1rxread_pointer(&mut self) -> Vuart1rxreadPointerW<Uartdma1f0Spec> {
        Vuart1rxreadPointerW::new(self, 0)
    }
}
#[doc = "VUART1 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1f0Spec;
impl crate::RegisterSpec for Uartdma1f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1f0::R`](R) reader structure"]
impl crate::Readable for Uartdma1f0Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1f0::W`](W) writer structure"]
impl crate::Writable for Uartdma1f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1F0 to value 0"]
impl crate::Resettable for Uartdma1f0Spec {}
