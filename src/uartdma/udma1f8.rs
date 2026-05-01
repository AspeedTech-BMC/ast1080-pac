#[doc = "Register `UDMA1F8` reader"]
pub type R = crate::R<Udma1f8Spec>;
#[doc = "Register `UDMA1F8` writer"]
pub type W = crate::W<Udma1f8Spec>;
#[doc = "Field `VUART1RXBufBaseAddr` reader - VUART1 RX buffer base address"]
pub type Vuart1rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `VUART1RXBufBaseAddr` writer - VUART1 RX buffer base address"]
pub type Vuart1rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - VUART1 RX buffer base address"]
    #[inline(always)]
    pub fn vuart1rxbuf_base_addr(&self) -> Vuart1rxbufBaseAddrR {
        Vuart1rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - VUART1 RX buffer base address"]
    #[inline(always)]
    pub fn vuart1rxbuf_base_addr(&mut self) -> Vuart1rxbufBaseAddrW<Udma1f8Spec> {
        Vuart1rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "VUART1 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1f8Spec;
impl crate::RegisterSpec for Udma1f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1f8::R`](R) reader structure"]
impl crate::Readable for Udma1f8Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1f8::W`](W) writer structure"]
impl crate::Writable for Udma1f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1F8 to value 0"]
impl crate::Resettable for Udma1f8Spec {}
