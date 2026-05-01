#[doc = "Register `UDMA238` reader"]
pub type R = crate::R<Udma238Spec>;
#[doc = "Register `UDMA238` writer"]
pub type W = crate::W<Udma238Spec>;
#[doc = "Field `VUART3RXBufBaseAddr` reader - VUART3 RX buffer base address"]
pub type Vuart3rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `VUART3RXBufBaseAddr` writer - VUART3 RX buffer base address"]
pub type Vuart3rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - VUART3 RX buffer base address"]
    #[inline(always)]
    pub fn vuart3rxbuf_base_addr(&self) -> Vuart3rxbufBaseAddrR {
        Vuart3rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - VUART3 RX buffer base address"]
    #[inline(always)]
    pub fn vuart3rxbuf_base_addr(&mut self) -> Vuart3rxbufBaseAddrW<Udma238Spec> {
        Vuart3rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "VUART3 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma238::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma238::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma238Spec;
impl crate::RegisterSpec for Udma238Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma238::R`](R) reader structure"]
impl crate::Readable for Udma238Spec {}
#[doc = "`write(|w| ..)` method takes [`udma238::W`](W) writer structure"]
impl crate::Writable for Udma238Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA238 to value 0"]
impl crate::Resettable for Udma238Spec {}
