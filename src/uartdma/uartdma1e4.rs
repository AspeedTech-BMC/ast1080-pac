#[doc = "Register `UARTDMA1E4` reader"]
pub type R = crate::R<Uartdma1e4Spec>;
#[doc = "Register `UARTDMA1E4` writer"]
pub type W = crate::W<Uartdma1e4Spec>;
#[doc = "Field `VUART1TXWrPointer` reader - VUART1 TX write pointer"]
pub type Vuart1txwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART1TXWrPointer` writer - VUART1 TX write pointer"]
pub type Vuart1txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - VUART1 TX write pointer"]
    #[inline(always)]
    pub fn vuart1txwr_pointer(&self) -> Vuart1txwrPointerR {
        Vuart1txwrPointerR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART1 TX write pointer"]
    #[inline(always)]
    pub fn vuart1txwr_pointer(&mut self) -> Vuart1txwrPointerW<Uartdma1e4Spec> {
        Vuart1txwrPointerW::new(self, 0)
    }
}
#[doc = "VUART1 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1e4Spec;
impl crate::RegisterSpec for Uartdma1e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1e4::R`](R) reader structure"]
impl crate::Readable for Uartdma1e4Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1e4::W`](W) writer structure"]
impl crate::Writable for Uartdma1e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1E4 to value 0"]
impl crate::Resettable for Uartdma1e4Spec {}
