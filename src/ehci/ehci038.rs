#[doc = "Register `EHCI038` reader"]
pub type R = crate::R<Ehci038Spec>;
#[doc = "Register `EHCI038` writer"]
pub type W = crate::W<Ehci038Spec>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `LinkPointerLowLPL` reader - Link Pointer Low (LPL)"]
pub type LinkPointerLowLplR = crate::FieldReader<u32>;
#[doc = "Field `LinkPointerLowLPL` writer - Link Pointer Low (LPL)"]
pub type LinkPointerLowLplW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:4 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - Link Pointer Low (LPL)"]
    #[inline(always)]
    pub fn link_pointer_low_lpl(&self) -> LinkPointerLowLplR {
        LinkPointerLowLplR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Link Pointer Low (LPL)"]
    #[inline(always)]
    pub fn link_pointer_low_lpl(&mut self) -> LinkPointerLowLplW<Ehci038Spec> {
        LinkPointerLowLplW::new(self, 5)
    }
}
#[doc = "Current Asynchronous List Address Register (ASYNCLISTADDR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci038Spec;
impl crate::RegisterSpec for Ehci038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci038::R`](R) reader structure"]
impl crate::Readable for Ehci038Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci038::W`](W) writer structure"]
impl crate::Writable for Ehci038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI038 to value 0"]
impl crate::Resettable for Ehci038Spec {}
