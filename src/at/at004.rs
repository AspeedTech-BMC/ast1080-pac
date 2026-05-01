#[doc = "Register `AT004` reader"]
pub type R = crate::R<At004Spec>;
#[doc = "Register `AT004` writer"]
pub type W = crate::W<At004Spec>;
#[doc = "Field `ATINTSTS` reader - AT_INT_STS"]
pub type AtintstsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - AT_INT_STS"]
    #[inline(always)]
    pub fn atintsts(&self) -> AtintstsR {
        AtintstsR::new(self.bits)
    }
}
impl W {}
#[doc = "Anti-Tamper Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At004Spec;
impl crate::RegisterSpec for At004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at004::R`](R) reader structure"]
impl crate::Readable for At004Spec {}
#[doc = "`write(|w| ..)` method takes [`at004::W`](W) writer structure"]
impl crate::Writable for At004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT004 to value 0"]
impl crate::Resettable for At004Spec {}
