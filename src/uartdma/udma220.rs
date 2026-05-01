#[doc = "Register `UDMA220` reader"]
pub type R = crate::R<Udma220Spec>;
#[doc = "Register `UDMA220` writer"]
pub type W = crate::W<Udma220Spec>;
#[doc = "Field `VUART3TXReadPointer` reader - VUART3 TX read pointer"]
pub type Vuart3txreadPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - VUART3 TX read pointer"]
    #[inline(always)]
    pub fn vuart3txread_pointer(&self) -> Vuart3txreadPointerR {
        Vuart3txreadPointerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "VUART3 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma220::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma220::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma220Spec;
impl crate::RegisterSpec for Udma220Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma220::R`](R) reader structure"]
impl crate::Readable for Udma220Spec {}
#[doc = "`write(|w| ..)` method takes [`udma220::W`](W) writer structure"]
impl crate::Writable for Udma220Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA220 to value 0"]
impl crate::Resettable for Udma220Spec {}
