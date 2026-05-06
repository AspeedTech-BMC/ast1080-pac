#[doc = "Register `UARTDMA204` reader"]
pub type R = crate::R<Uartdma204Spec>;
#[doc = "Register `UARTDMA204` writer"]
pub type W = crate::W<Uartdma204Spec>;
#[doc = "Field `VUART2TXWrPointer` reader - VUART2 TX write pointer"]
pub type Vuart2txwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART2TXWrPointer` writer - VUART2 TX write pointer"]
pub type Vuart2txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - VUART2 TX write pointer"]
    #[inline(always)]
    pub fn vuart2txwr_pointer(&self) -> Vuart2txwrPointerR {
        Vuart2txwrPointerR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART2 TX write pointer"]
    #[inline(always)]
    pub fn vuart2txwr_pointer(&mut self) -> Vuart2txwrPointerW<Uartdma204Spec> {
        Vuart2txwrPointerW::new(self, 0)
    }
}
#[doc = "VUART2 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma204::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma204::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma204Spec;
impl crate::RegisterSpec for Uartdma204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma204::R`](R) reader structure"]
impl crate::Readable for Uartdma204Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma204::W`](W) writer structure"]
impl crate::Writable for Uartdma204Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA204 to value 0"]
impl crate::Resettable for Uartdma204Spec {}
