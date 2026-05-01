#[doc = "Register `UDMA230` reader"]
pub type R = crate::R<Udma230Spec>;
#[doc = "Register `UDMA230` writer"]
pub type W = crate::W<Udma230Spec>;
#[doc = "Field `VUART3RXReadPointer` reader - VUART3 RX read pointer"]
pub type Vuart3rxreadPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART3RXReadPointer` writer - VUART3 RX read pointer"]
pub type Vuart3rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - VUART3 RX read pointer"]
    #[inline(always)]
    pub fn vuart3rxread_pointer(&self) -> Vuart3rxreadPointerR {
        Vuart3rxreadPointerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART3 RX read pointer"]
    #[inline(always)]
    pub fn vuart3rxread_pointer(&mut self) -> Vuart3rxreadPointerW<Udma230Spec> {
        Vuart3rxreadPointerW::new(self, 0)
    }
}
#[doc = "VUART3 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma230::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma230::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma230Spec;
impl crate::RegisterSpec for Udma230Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma230::R`](R) reader structure"]
impl crate::Readable for Udma230Spec {}
#[doc = "`write(|w| ..)` method takes [`udma230::W`](W) writer structure"]
impl crate::Writable for Udma230Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA230 to value 0"]
impl crate::Resettable for Udma230Spec {}
