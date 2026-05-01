#[doc = "Register `I3CPHYCTRLREG0A8` reader"]
pub type R = crate::R<I3cphyctrlreg0a8Spec>;
#[doc = "Register `I3CPHYCTRLREG0A8` writer"]
pub type W = crate::W<I3cphyctrlreg0a8Spec>;
#[doc = "Field `REGI3CSPECIALPATTERNSCLTOGGLESDAPAT` reader - REG_I3C_SPECIAL_PATTERN_SCL_TOGGLE_SDA_PAT"]
pub type Regi3cspecialpatternscltogglesdapatR = crate::FieldReader<u32>;
#[doc = "Field `REGI3CSPECIALPATTERNSCLTOGGLESDAPAT` writer - REG_I3C_SPECIAL_PATTERN_SCL_TOGGLE_SDA_PAT"]
pub type Regi3cspecialpatternscltogglesdapatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_I3C_SPECIAL_PATTERN_SCL_TOGGLE_SDA_PAT"]
    #[inline(always)]
    pub fn regi3cspecialpatternscltogglesdapat(&self) -> Regi3cspecialpatternscltogglesdapatR {
        Regi3cspecialpatternscltogglesdapatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_I3C_SPECIAL_PATTERN_SCL_TOGGLE_SDA_PAT"]
    #[inline(always)]
    pub fn regi3cspecialpatternscltogglesdapat(
        &mut self,
    ) -> Regi3cspecialpatternscltogglesdapatW<I3cphyctrlreg0a8Spec> {
        Regi3cspecialpatternscltogglesdapatW::new(self, 0)
    }
}
#[doc = "SPECIAL\\_PATTERN\\_SCL\\_TOGGLE\\_PAT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0a8Spec;
impl crate::RegisterSpec for I3cphyctrlreg0a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0a8::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0a8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0a8::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0A8 to value 0xffff_ffff"]
impl crate::Resettable for I3cphyctrlreg0a8Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
