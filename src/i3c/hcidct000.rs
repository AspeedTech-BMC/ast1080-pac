#[doc = "Register `HCIDCT000` reader"]
pub type R = crate::R<Hcidct000Spec>;
#[doc = "Register `HCIDCT000` writer"]
pub type W = crate::W<Hcidct000Spec>;
#[doc = "Field `REGTARGETPIDHI` reader - REG_TARGET_PID_HI"]
pub type RegtargetpidhiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TARGET_PID_HI"]
    #[inline(always)]
    pub fn regtargetpidhi(&self) -> RegtargetpidhiR {
        RegtargetpidhiR::new(self.bits)
    }
}
impl W {}
#[doc = "TARGET\\_DCT\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`hcidct000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcidct000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcidct000Spec;
impl crate::RegisterSpec for Hcidct000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcidct000::R`](R) reader structure"]
impl crate::Readable for Hcidct000Spec {}
#[doc = "`write(|w| ..)` method takes [`hcidct000::W`](W) writer structure"]
impl crate::Writable for Hcidct000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIDCT000 to value 0"]
impl crate::Resettable for Hcidct000Spec {}
