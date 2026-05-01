#[doc = "Register `UDMA234` reader"]
pub type R = crate::R<Udma234Spec>;
#[doc = "Register `UDMA234` writer"]
pub type W = crate::W<Udma234Spec>;
#[doc = "Field `VUART3RXWrPointer` reader - VUART3 RX write pointer"]
pub type Vuart3rxwrPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - VUART3 RX write pointer"]
    #[inline(always)]
    pub fn vuart3rxwr_pointer(&self) -> Vuart3rxwrPointerR {
        Vuart3rxwrPointerR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {}
#[doc = "VUART3 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma234::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma234::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma234Spec;
impl crate::RegisterSpec for Udma234Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma234::R`](R) reader structure"]
impl crate::Readable for Udma234Spec {}
#[doc = "`write(|w| ..)` method takes [`udma234::W`](W) writer structure"]
impl crate::Writable for Udma234Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA234 to value 0"]
impl crate::Resettable for Udma234Spec {}
