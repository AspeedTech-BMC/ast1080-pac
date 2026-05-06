#[doc = "Register `UARTDMA228` reader"]
pub type R = crate::R<Uartdma228Spec>;
#[doc = "Register `UARTDMA228` writer"]
pub type W = crate::W<Uartdma228Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `VUART3TXBufBaseAddr` reader - VUART3 TX buffer base address"]
pub type Vuart3txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `VUART3TXBufBaseAddr` writer - VUART3 TX buffer base address"]
pub type Vuart3txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - VUART3 TX buffer base address"]
    #[inline(always)]
    pub fn vuart3txbuf_base_addr(&self) -> Vuart3txbufBaseAddrR {
        Vuart3txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - VUART3 TX buffer base address"]
    #[inline(always)]
    pub fn vuart3txbuf_base_addr(&mut self) -> Vuart3txbufBaseAddrW<Uartdma228Spec> {
        Vuart3txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "VUART3 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma228::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma228::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma228Spec;
impl crate::RegisterSpec for Uartdma228Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma228::R`](R) reader structure"]
impl crate::Readable for Uartdma228Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma228::W`](W) writer structure"]
impl crate::Writable for Uartdma228Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA228 to value 0"]
impl crate::Resettable for Uartdma228Spec {}
