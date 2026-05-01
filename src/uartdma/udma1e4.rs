#[doc = "Register `UDMA1E4` reader"]
pub type R = crate::R<Udma1e4Spec>;
#[doc = "Register `UDMA1E4` writer"]
pub type W = crate::W<Udma1e4Spec>;
#[doc = "Field `VUART1TXWrPointer` reader - VUART1 TX write pointer"]
pub type Vuart1txwrPointerR = crate::FieldReader<u32>;
#[doc = "Field `VUART1TXWrPointer` writer - VUART1 TX write pointer"]
pub type Vuart1txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - VUART1 TX write pointer"]
    #[inline(always)]
    pub fn vuart1txwr_pointer(&self) -> Vuart1txwrPointerR {
        Vuart1txwrPointerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - VUART1 TX write pointer"]
    #[inline(always)]
    pub fn vuart1txwr_pointer(&mut self) -> Vuart1txwrPointerW<Udma1e4Spec> {
        Vuart1txwrPointerW::new(self, 0)
    }
}
#[doc = "VUART1 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1e4Spec;
impl crate::RegisterSpec for Udma1e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1e4::R`](R) reader structure"]
impl crate::Readable for Udma1e4Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1e4::W`](W) writer structure"]
impl crate::Writable for Udma1e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1E4 to value 0"]
impl crate::Resettable for Udma1e4Spec {}
