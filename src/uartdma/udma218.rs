#[doc = "Register `UDMA218` reader"]
pub type R = crate::R<Udma218Spec>;
#[doc = "Register `UDMA218` writer"]
pub type W = crate::W<Udma218Spec>;
#[doc = "Field `VUART2RXBufBaseAddr` reader - VUART2 RX buffer base address"]
pub type Vuart2rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `VUART2RXBufBaseAddr` writer - VUART2 RX buffer base address"]
pub type Vuart2rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - VUART2 RX buffer base address"]
    #[inline(always)]
    pub fn vuart2rxbuf_base_addr(&self) -> Vuart2rxbufBaseAddrR {
        Vuart2rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - VUART2 RX buffer base address"]
    #[inline(always)]
    pub fn vuart2rxbuf_base_addr(&mut self) -> Vuart2rxbufBaseAddrW<Udma218Spec> {
        Vuart2rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "VUART2 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma218::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma218::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma218Spec;
impl crate::RegisterSpec for Udma218Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma218::R`](R) reader structure"]
impl crate::Readable for Udma218Spec {}
#[doc = "`write(|w| ..)` method takes [`udma218::W`](W) writer structure"]
impl crate::Writable for Udma218Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA218 to value 0"]
impl crate::Resettable for Udma218Spec {}
