#[doc = "Register `HCIDCT00C` reader"]
pub type R = crate::R<Hcidct00cSpec>;
#[doc = "Register `HCIDCT00C` writer"]
pub type W = crate::W<Hcidct00cSpec>;
#[doc = "Field `REGTARGETDYNAMICADDRESS` reader - REG_TARGET_DYNAMIC_ADDRESS"]
pub type RegtargetdynamicaddressR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - REG_TARGET_DYNAMIC_ADDRESS"]
    #[inline(always)]
    pub fn regtargetdynamicaddress(&self) -> RegtargetdynamicaddressR {
        RegtargetdynamicaddressR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "TARGET\\_DCT\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`hcidct00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcidct00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcidct00cSpec;
impl crate::RegisterSpec for Hcidct00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcidct00c::R`](R) reader structure"]
impl crate::Readable for Hcidct00cSpec {}
#[doc = "`write(|w| ..)` method takes [`hcidct00c::W`](W) writer structure"]
impl crate::Writable for Hcidct00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIDCT00C to value 0"]
impl crate::Resettable for Hcidct00cSpec {}
