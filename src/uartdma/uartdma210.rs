#[doc = "Register `UARTDMA210` reader"]
pub type R = crate::R<Uartdma210Spec>;
#[doc = "Register `UARTDMA210` writer"]
pub type W = crate::W<Uartdma210Spec>;
#[doc = "Field `VUART2RXReadPointer` reader - VUART2 RX read pointer"]
pub type Vuart2rxreadPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART2RXReadPointer` writer - VUART2 RX read pointer"]
pub type Vuart2rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - VUART2 RX read pointer"]
    #[inline(always)]
    pub fn vuart2rxread_pointer(&self) -> Vuart2rxreadPointerR {
        Vuart2rxreadPointerR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART2 RX read pointer"]
    #[inline(always)]
    pub fn vuart2rxread_pointer(&mut self) -> Vuart2rxreadPointerW<Uartdma210Spec> {
        Vuart2rxreadPointerW::new(self, 0)
    }
}
#[doc = "VUART2 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma210::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma210::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma210Spec;
impl crate::RegisterSpec for Uartdma210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma210::R`](R) reader structure"]
impl crate::Readable for Uartdma210Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma210::W`](W) writer structure"]
impl crate::Writable for Uartdma210Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA210 to value 0"]
impl crate::Resettable for Uartdma210Spec {}
