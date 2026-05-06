#[doc = "Register `UARTDMA1C4` reader"]
pub type R = crate::R<Uartdma1c4Spec>;
#[doc = "Register `UARTDMA1C4` writer"]
pub type W = crate::W<Uartdma1c4Spec>;
#[doc = "Field `VUART0TXWrPointer` reader - VUART0 TX write pointer"]
pub type Vuart0txwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART0TXWrPointer` writer - VUART0 TX write pointer"]
pub type Vuart0txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - VUART0 TX write pointer"]
    #[inline(always)]
    pub fn vuart0txwr_pointer(&self) -> Vuart0txwrPointerR {
        Vuart0txwrPointerR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART0 TX write pointer"]
    #[inline(always)]
    pub fn vuart0txwr_pointer(&mut self) -> Vuart0txwrPointerW<Uartdma1c4Spec> {
        Vuart0txwrPointerW::new(self, 0)
    }
}
#[doc = "VUART0 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1c4Spec;
impl crate::RegisterSpec for Uartdma1c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1c4::R`](R) reader structure"]
impl crate::Readable for Uartdma1c4Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1c4::W`](W) writer structure"]
impl crate::Writable for Uartdma1c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1C4 to value 0"]
impl crate::Resettable for Uartdma1c4Spec {}
