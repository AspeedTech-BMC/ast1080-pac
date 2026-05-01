#[doc = "Register `UDMA208` reader"]
pub type R = crate::R<Udma208Spec>;
#[doc = "Register `UDMA208` writer"]
pub type W = crate::W<Udma208Spec>;
#[doc = "Field `VUART2TXBufBaseAddr` reader - VUART2 TX buffer base address"]
pub type Vuart2txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `VUART2TXBufBaseAddr` writer - VUART2 TX buffer base address"]
pub type Vuart2txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - VUART2 TX buffer base address"]
    #[inline(always)]
    pub fn vuart2txbuf_base_addr(&self) -> Vuart2txbufBaseAddrR {
        Vuart2txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - VUART2 TX buffer base address"]
    #[inline(always)]
    pub fn vuart2txbuf_base_addr(&mut self) -> Vuart2txbufBaseAddrW<Udma208Spec> {
        Vuart2txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "VUART2 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma208::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma208::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma208Spec;
impl crate::RegisterSpec for Udma208Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma208::R`](R) reader structure"]
impl crate::Readable for Udma208Spec {}
#[doc = "`write(|w| ..)` method takes [`udma208::W`](W) writer structure"]
impl crate::Writable for Udma208Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA208 to value 0"]
impl crate::Resettable for Udma208Spec {}
