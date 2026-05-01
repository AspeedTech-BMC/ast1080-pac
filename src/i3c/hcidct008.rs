#[doc = "Register `HCIDCT008` reader"]
pub type R = crate::R<Hcidct008Spec>;
#[doc = "Register `HCIDCT008` writer"]
pub type W = crate::W<Hcidct008Spec>;
#[doc = "Field `REGTARGETDCR` reader - REG_TARGET_DCR"]
pub type RegtargetdcrR = crate::FieldReader;
#[doc = "Field `REGTARGETBCR` reader - REG_TARGET_BCR"]
pub type RegtargetbcrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - REG_TARGET_DCR"]
    #[inline(always)]
    pub fn regtargetdcr(&self) -> RegtargetdcrR {
        RegtargetdcrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_TARGET_BCR"]
    #[inline(always)]
    pub fn regtargetbcr(&self) -> RegtargetbcrR {
        RegtargetbcrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {}
#[doc = "TARGET\\_DCT\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`hcidct008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcidct008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcidct008Spec;
impl crate::RegisterSpec for Hcidct008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcidct008::R`](R) reader structure"]
impl crate::Readable for Hcidct008Spec {}
#[doc = "`write(|w| ..)` method takes [`hcidct008::W`](W) writer structure"]
impl crate::Writable for Hcidct008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIDCT008 to value 0"]
impl crate::Resettable for Hcidct008Spec {}
