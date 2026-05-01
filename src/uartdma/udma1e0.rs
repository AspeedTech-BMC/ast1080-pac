#[doc = "Register `UDMA1E0` reader"]
pub type R = crate::R<Udma1e0Spec>;
#[doc = "Register `UDMA1E0` writer"]
pub type W = crate::W<Udma1e0Spec>;
#[doc = "Field `VUART1TXReadPointer` reader - VUART1 TX read pointer"]
pub type Vuart1txreadPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - VUART1 TX read pointer"]
    #[inline(always)]
    pub fn vuart1txread_pointer(&self) -> Vuart1txreadPointerR {
        Vuart1txreadPointerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "VUART1 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1e0Spec;
impl crate::RegisterSpec for Udma1e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1e0::R`](R) reader structure"]
impl crate::Readable for Udma1e0Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1e0::W`](W) writer structure"]
impl crate::Writable for Udma1e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1E0 to value 0"]
impl crate::Resettable for Udma1e0Spec {}
