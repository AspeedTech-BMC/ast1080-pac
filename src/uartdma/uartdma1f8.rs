#[doc = "Register `UARTDMA1F8` reader"]
pub type R = crate::R<Uartdma1f8Spec>;
#[doc = "Register `UARTDMA1F8` writer"]
pub type W = crate::W<Uartdma1f8Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `VUART1RXBufBaseAddr` reader - VUART1 RX buffer base address"]
pub type Vuart1rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `VUART1RXBufBaseAddr` writer - VUART1 RX buffer base address"]
pub type Vuart1rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - VUART1 RX buffer base address"]
    #[inline(always)]
    pub fn vuart1rxbuf_base_addr(&self) -> Vuart1rxbufBaseAddrR {
        Vuart1rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - VUART1 RX buffer base address"]
    #[inline(always)]
    pub fn vuart1rxbuf_base_addr(&mut self) -> Vuart1rxbufBaseAddrW<Uartdma1f8Spec> {
        Vuart1rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "VUART1 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1f8Spec;
impl crate::RegisterSpec for Uartdma1f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1f8::R`](R) reader structure"]
impl crate::Readable for Uartdma1f8Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1f8::W`](W) writer structure"]
impl crate::Writable for Uartdma1f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1F8 to value 0"]
impl crate::Resettable for Uartdma1f8Spec {}
