#[doc = "Register `UDMA204` reader"]
pub type R = crate::R<Udma204Spec>;
#[doc = "Register `UDMA204` writer"]
pub type W = crate::W<Udma204Spec>;
#[doc = "Field `VUART2TXWrPointer` reader - VUART2 TX write pointer"]
pub type Vuart2txwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART2TXWrPointer` writer - VUART2 TX write pointer"]
pub type Vuart2txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - VUART2 TX write pointer"]
    #[inline(always)]
    pub fn vuart2txwr_pointer(&self) -> Vuart2txwrPointerR {
        Vuart2txwrPointerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART2 TX write pointer"]
    #[inline(always)]
    pub fn vuart2txwr_pointer(&mut self) -> Vuart2txwrPointerW<Udma204Spec> {
        Vuart2txwrPointerW::new(self, 0)
    }
}
#[doc = "VUART2 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma204::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma204::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma204Spec;
impl crate::RegisterSpec for Udma204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma204::R`](R) reader structure"]
impl crate::Readable for Udma204Spec {}
#[doc = "`write(|w| ..)` method takes [`udma204::W`](W) writer structure"]
impl crate::Writable for Udma204Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA204 to value 0"]
impl crate::Resettable for Udma204Spec {}
