#[doc = "Register `UARTDMA238` reader"]
pub type R = crate::R<Uartdma238Spec>;
#[doc = "Register `UARTDMA238` writer"]
pub type W = crate::W<Uartdma238Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `VUART3RXBufBaseAddr` reader - VUART3 RX buffer base address"]
pub type Vuart3rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `VUART3RXBufBaseAddr` writer - VUART3 RX buffer base address"]
pub type Vuart3rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - VUART3 RX buffer base address"]
    #[inline(always)]
    pub fn vuart3rxbuf_base_addr(&self) -> Vuart3rxbufBaseAddrR {
        Vuart3rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - VUART3 RX buffer base address"]
    #[inline(always)]
    pub fn vuart3rxbuf_base_addr(&mut self) -> Vuart3rxbufBaseAddrW<Uartdma238Spec> {
        Vuart3rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "VUART3 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma238::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma238::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma238Spec;
impl crate::RegisterSpec for Uartdma238Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma238::R`](R) reader structure"]
impl crate::Readable for Uartdma238Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma238::W`](W) writer structure"]
impl crate::Writable for Uartdma238Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA238 to value 0"]
impl crate::Resettable for Uartdma238Spec {}
