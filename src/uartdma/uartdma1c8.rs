#[doc = "Register `UARTDMA1C8` reader"]
pub type R = crate::R<Uartdma1c8Spec>;
#[doc = "Register `UARTDMA1C8` writer"]
pub type W = crate::W<Uartdma1c8Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `VUART0TXBufBaseAddr` reader - VUART0 TX buffer base address"]
pub type Vuart0txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `VUART0TXBufBaseAddr` writer - VUART0 TX buffer base address"]
pub type Vuart0txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - VUART0 TX buffer base address"]
    #[inline(always)]
    pub fn vuart0txbuf_base_addr(&self) -> Vuart0txbufBaseAddrR {
        Vuart0txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - VUART0 TX buffer base address"]
    #[inline(always)]
    pub fn vuart0txbuf_base_addr(&mut self) -> Vuart0txbufBaseAddrW<Uartdma1c8Spec> {
        Vuart0txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "VUART0 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1c8Spec;
impl crate::RegisterSpec for Uartdma1c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1c8::R`](R) reader structure"]
impl crate::Readable for Uartdma1c8Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1c8::W`](W) writer structure"]
impl crate::Writable for Uartdma1c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1C8 to value 0"]
impl crate::Resettable for Uartdma1c8Spec {}
