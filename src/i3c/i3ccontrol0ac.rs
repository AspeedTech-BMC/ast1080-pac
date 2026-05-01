#[doc = "Register `I3CCONTROL0AC` reader"]
pub type R = crate::R<I3ccontrol0acSpec>;
#[doc = "Register `I3CCONTROL0AC` writer"]
pub type W = crate::W<I3ccontrol0acSpec>;
#[doc = "Field `REGSLVTRANSSTATE` reader - REG_SLV_TRANS_STATE"]
pub type RegslvtransstateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - REG_SLV_TRANS_STATE"]
    #[inline(always)]
    pub fn regslvtransstate(&self) -> RegslvtransstateR {
        RegslvtransstateR::new((self.bits & 0x3f) as u8)
    }
}
impl W {}
#[doc = "I3C\\_SLV\\_CTL\\_0AC\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0acSpec;
impl crate::RegisterSpec for I3ccontrol0acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0ac::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0acSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0ac::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0AC to value 0"]
impl crate::Resettable for I3ccontrol0acSpec {}
