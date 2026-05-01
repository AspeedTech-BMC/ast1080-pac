#[doc = "Register `UDMA200` reader"]
pub type R = crate::R<Udma200Spec>;
#[doc = "Register `UDMA200` writer"]
pub type W = crate::W<Udma200Spec>;
#[doc = "Field `VUART2TXReadPointer` reader - VUART2 TX read pointer"]
pub type Vuart2txreadPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - VUART2 TX read pointer"]
    #[inline(always)]
    pub fn vuart2txread_pointer(&self) -> Vuart2txreadPointerR {
        Vuart2txreadPointerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "VUART2 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma200::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma200::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma200Spec;
impl crate::RegisterSpec for Udma200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma200::R`](R) reader structure"]
impl crate::Readable for Udma200Spec {}
#[doc = "`write(|w| ..)` method takes [`udma200::W`](W) writer structure"]
impl crate::Writable for Udma200Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA200 to value 0"]
impl crate::Resettable for Udma200Spec {}
