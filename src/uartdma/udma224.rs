#[doc = "Register `UDMA224` reader"]
pub type R = crate::R<Udma224Spec>;
#[doc = "Register `UDMA224` writer"]
pub type W = crate::W<Udma224Spec>;
#[doc = "Field `VUART3TXWrPointer` reader - VUART3 TX write pointer"]
pub type Vuart3txwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART3TXWrPointer` writer - VUART3 TX write pointer"]
pub type Vuart3txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - VUART3 TX write pointer"]
    #[inline(always)]
    pub fn vuart3txwr_pointer(&self) -> Vuart3txwrPointerR {
        Vuart3txwrPointerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART3 TX write pointer"]
    #[inline(always)]
    pub fn vuart3txwr_pointer(&mut self) -> Vuart3txwrPointerW<Udma224Spec> {
        Vuart3txwrPointerW::new(self, 0)
    }
}
#[doc = "VUART3 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma224::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma224::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma224Spec;
impl crate::RegisterSpec for Udma224Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma224::R`](R) reader structure"]
impl crate::Readable for Udma224Spec {}
#[doc = "`write(|w| ..)` method takes [`udma224::W`](W) writer structure"]
impl crate::Writable for Udma224Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA224 to value 0"]
impl crate::Resettable for Udma224Spec {}
