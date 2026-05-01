#[doc = "Register `UDMA210` reader"]
pub type R = crate::R<Udma210Spec>;
#[doc = "Register `UDMA210` writer"]
pub type W = crate::W<Udma210Spec>;
#[doc = "Field `VUART2RXReadPointer` reader - VUART2 RX read pointer"]
pub type Vuart2rxreadPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART2RXReadPointer` writer - VUART2 RX read pointer"]
pub type Vuart2rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - VUART2 RX read pointer"]
    #[inline(always)]
    pub fn vuart2rxread_pointer(&self) -> Vuart2rxreadPointerR {
        Vuart2rxreadPointerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART2 RX read pointer"]
    #[inline(always)]
    pub fn vuart2rxread_pointer(&mut self) -> Vuart2rxreadPointerW<Udma210Spec> {
        Vuart2rxreadPointerW::new(self, 0)
    }
}
#[doc = "VUART2 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma210::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma210::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma210Spec;
impl crate::RegisterSpec for Udma210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma210::R`](R) reader structure"]
impl crate::Readable for Udma210Spec {}
#[doc = "`write(|w| ..)` method takes [`udma210::W`](W) writer structure"]
impl crate::Writable for Udma210Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA210 to value 0"]
impl crate::Resettable for Udma210Spec {}
