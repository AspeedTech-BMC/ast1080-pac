#[doc = "Register `SPIF020` reader"]
pub type R = crate::R<Spif020Spec>;
#[doc = "Register `SPIF020` writer"]
pub type W = crate::W<Spif020Spec>;
#[doc = "Field `WTABLEWLOCK` reader - WTABLE_WLOCK"]
pub type WtablewlockR = crate::FieldReader<u32>;
#[doc = "Field `WTABLEWLOCK` writer - WTABLE_WLOCK"]
pub type WtablewlockW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE_WLOCK"]
    #[inline(always)]
    pub fn wtablewlock(&self) -> WtablewlockR {
        WtablewlockR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE_WLOCK"]
    #[inline(always)]
    pub fn wtablewlock(&mut self) -> WtablewlockW<Spif020Spec> {
        WtablewlockW::new(self, 0)
    }
}
#[doc = "SPIF\\_WLOCKTB\n\nYou can [`read`](crate::Reg::read) this register and get [`spif020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif020Spec;
impl crate::RegisterSpec for Spif020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif020::R`](R) reader structure"]
impl crate::Readable for Spif020Spec {}
#[doc = "`write(|w| ..)` method takes [`spif020::W`](W) writer structure"]
impl crate::Writable for Spif020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF020 to value 0"]
impl crate::Resettable for Spif020Spec {}
