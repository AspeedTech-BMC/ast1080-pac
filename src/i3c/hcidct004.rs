#[doc = "Register `HCIDCT004` reader"]
pub type R = crate::R<Hcidct004Spec>;
#[doc = "Register `HCIDCT004` writer"]
pub type W = crate::W<Hcidct004Spec>;
#[doc = "Field `REGTARGETPIDLO` reader - REG_TARGET_PID_LO"]
pub type RegtargetpidloR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - REG_TARGET_PID_LO"]
    #[inline(always)]
    pub fn regtargetpidlo(&self) -> RegtargetpidloR {
        RegtargetpidloR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "TARGET\\_DCT\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`hcidct004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcidct004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcidct004Spec;
impl crate::RegisterSpec for Hcidct004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcidct004::R`](R) reader structure"]
impl crate::Readable for Hcidct004Spec {}
#[doc = "`write(|w| ..)` method takes [`hcidct004::W`](W) writer structure"]
impl crate::Writable for Hcidct004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIDCT004 to value 0"]
impl crate::Resettable for Hcidct004Spec {}
