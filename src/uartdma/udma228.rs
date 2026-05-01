#[doc = "Register `UDMA228` reader"]
pub type R = crate::R<Udma228Spec>;
#[doc = "Register `UDMA228` writer"]
pub type W = crate::W<Udma228Spec>;
#[doc = "Field `VUART3TXBufBaseAddr` reader - VUART3 TX buffer base address"]
pub type Vuart3txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `VUART3TXBufBaseAddr` writer - VUART3 TX buffer base address"]
pub type Vuart3txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - VUART3 TX buffer base address"]
    #[inline(always)]
    pub fn vuart3txbuf_base_addr(&self) -> Vuart3txbufBaseAddrR {
        Vuart3txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - VUART3 TX buffer base address"]
    #[inline(always)]
    pub fn vuart3txbuf_base_addr(&mut self) -> Vuart3txbufBaseAddrW<Udma228Spec> {
        Vuart3txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "VUART3 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma228::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma228::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma228Spec;
impl crate::RegisterSpec for Udma228Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma228::R`](R) reader structure"]
impl crate::Readable for Udma228Spec {}
#[doc = "`write(|w| ..)` method takes [`udma228::W`](W) writer structure"]
impl crate::Writable for Udma228Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA228 to value 0"]
impl crate::Resettable for Udma228Spec {}
