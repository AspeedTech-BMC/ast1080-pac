#[doc = "Register `UDMA1E8` reader"]
pub type R = crate::R<Udma1e8Spec>;
#[doc = "Register `UDMA1E8` writer"]
pub type W = crate::W<Udma1e8Spec>;
#[doc = "Field `VUART1TXBufBaseAddr` reader - VUART1 TX buffer base address"]
pub type Vuart1txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `VUART1TXBufBaseAddr` writer - VUART1 TX buffer base address"]
pub type Vuart1txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - VUART1 TX buffer base address"]
    #[inline(always)]
    pub fn vuart1txbuf_base_addr(&self) -> Vuart1txbufBaseAddrR {
        Vuart1txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - VUART1 TX buffer base address"]
    #[inline(always)]
    pub fn vuart1txbuf_base_addr(&mut self) -> Vuart1txbufBaseAddrW<Udma1e8Spec> {
        Vuart1txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "VUART1 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1e8Spec;
impl crate::RegisterSpec for Udma1e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1e8::R`](R) reader structure"]
impl crate::Readable for Udma1e8Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1e8::W`](W) writer structure"]
impl crate::Writable for Udma1e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1E8 to value 0"]
impl crate::Resettable for Udma1e8Spec {}
