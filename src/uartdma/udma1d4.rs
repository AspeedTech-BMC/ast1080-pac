#[doc = "Register `UDMA1D4` reader"]
pub type R = crate::R<Udma1d4Spec>;
#[doc = "Register `UDMA1D4` writer"]
pub type W = crate::W<Udma1d4Spec>;
#[doc = "Field `VUART0RXWrPointer` reader - VUART0 RX write pointer"]
pub type Vuart0rxwrPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - VUART0 RX write pointer"]
    #[inline(always)]
    pub fn vuart0rxwr_pointer(&self) -> Vuart0rxwrPointerR {
        Vuart0rxwrPointerR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {}
#[doc = "VUART0 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1d4Spec;
impl crate::RegisterSpec for Udma1d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1d4::R`](R) reader structure"]
impl crate::Readable for Udma1d4Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1d4::W`](W) writer structure"]
impl crate::Writable for Udma1d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1D4 to value 0"]
impl crate::Resettable for Udma1d4Spec {}
