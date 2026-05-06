#[doc = "Register `UARTDMA230` reader"]
pub type R = crate::R<Uartdma230Spec>;
#[doc = "Register `UARTDMA230` writer"]
pub type W = crate::W<Uartdma230Spec>;
#[doc = "Field `VUART3RXReadPointer` reader - VUART3 RX read pointer"]
pub type Vuart3rxreadPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART3RXReadPointer` writer - VUART3 RX read pointer"]
pub type Vuart3rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - VUART3 RX read pointer"]
    #[inline(always)]
    pub fn vuart3rxread_pointer(&self) -> Vuart3rxreadPointerR {
        Vuart3rxreadPointerR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART3 RX read pointer"]
    #[inline(always)]
    pub fn vuart3rxread_pointer(&mut self) -> Vuart3rxreadPointerW<Uartdma230Spec> {
        Vuart3rxreadPointerW::new(self, 0)
    }
}
#[doc = "VUART3 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma230::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma230::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma230Spec;
impl crate::RegisterSpec for Uartdma230Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma230::R`](R) reader structure"]
impl crate::Readable for Uartdma230Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma230::W`](W) writer structure"]
impl crate::Writable for Uartdma230Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA230 to value 0"]
impl crate::Resettable for Uartdma230Spec {}
