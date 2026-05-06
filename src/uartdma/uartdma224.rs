#[doc = "Register `UARTDMA224` reader"]
pub type R = crate::R<Uartdma224Spec>;
#[doc = "Register `UARTDMA224` writer"]
pub type W = crate::W<Uartdma224Spec>;
#[doc = "Field `VUART3TXWrPointer` reader - VUART3 TX write pointer"]
pub type Vuart3txwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART3TXWrPointer` writer - VUART3 TX write pointer"]
pub type Vuart3txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - VUART3 TX write pointer"]
    #[inline(always)]
    pub fn vuart3txwr_pointer(&self) -> Vuart3txwrPointerR {
        Vuart3txwrPointerR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART3 TX write pointer"]
    #[inline(always)]
    pub fn vuart3txwr_pointer(&mut self) -> Vuart3txwrPointerW<Uartdma224Spec> {
        Vuart3txwrPointerW::new(self, 0)
    }
}
#[doc = "VUART3 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma224::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma224::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma224Spec;
impl crate::RegisterSpec for Uartdma224Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma224::R`](R) reader structure"]
impl crate::Readable for Uartdma224Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma224::W`](W) writer structure"]
impl crate::Writable for Uartdma224Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA224 to value 0"]
impl crate::Resettable for Uartdma224Spec {}
