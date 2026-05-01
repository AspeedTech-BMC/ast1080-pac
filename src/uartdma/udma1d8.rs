#[doc = "Register `UDMA1D8` reader"]
pub type R = crate::R<Udma1d8Spec>;
#[doc = "Register `UDMA1D8` writer"]
pub type W = crate::W<Udma1d8Spec>;
#[doc = "Field `VUART0RXBufBaseAddr` reader - VUART0 RX buffer base address"]
pub type Vuart0rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `VUART0RXBufBaseAddr` writer - VUART0 RX buffer base address"]
pub type Vuart0rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - VUART0 RX buffer base address"]
    #[inline(always)]
    pub fn vuart0rxbuf_base_addr(&self) -> Vuart0rxbufBaseAddrR {
        Vuart0rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - VUART0 RX buffer base address"]
    #[inline(always)]
    pub fn vuart0rxbuf_base_addr(&mut self) -> Vuart0rxbufBaseAddrW<Udma1d8Spec> {
        Vuart0rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "VUART0 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1d8Spec;
impl crate::RegisterSpec for Udma1d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1d8::R`](R) reader structure"]
impl crate::Readable for Udma1d8Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1d8::W`](W) writer structure"]
impl crate::Writable for Udma1d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1D8 to value 0"]
impl crate::Resettable for Udma1d8Spec {}
