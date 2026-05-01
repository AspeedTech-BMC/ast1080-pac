#[doc = "Register `I3CPHYCTRLREG0AC` reader"]
pub type R = crate::R<I3cphyctrlreg0acSpec>;
#[doc = "Register `I3CPHYCTRLREG0AC` writer"]
pub type W = crate::W<I3cphyctrlreg0acSpec>;
#[doc = "Field `REGI3CSPECIALPATTERNSCLTIELCNT` reader - REG_I3C_SPECIAL_PATTERN_SCL_TIEL_CNT"]
pub type Regi3cspecialpatternscltielcntR = crate::FieldReader<u32>;
#[doc = "Field `REGI3CSPECIALPATTERNSCLTIELCNT` writer - REG_I3C_SPECIAL_PATTERN_SCL_TIEL_CNT"]
pub type Regi3cspecialpatternscltielcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_I3C_SPECIAL_PATTERN_SCL_TIEL_CNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternscltielcnt(&self) -> Regi3cspecialpatternscltielcntR {
        Regi3cspecialpatternscltielcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_I3C_SPECIAL_PATTERN_SCL_TIEL_CNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternscltielcnt(
        &mut self,
    ) -> Regi3cspecialpatternscltielcntW<I3cphyctrlreg0acSpec> {
        Regi3cspecialpatternscltielcntW::new(self, 0)
    }
}
#[doc = "SPECIAL\\_PATTERN\\_SCL\\_TIEL\\_SET\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0acSpec;
impl crate::RegisterSpec for I3cphyctrlreg0acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0ac::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0acSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0ac::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0AC to value 0x001e_8480"]
impl crate::Resettable for I3cphyctrlreg0acSpec {
    const RESET_VALUE: u32 = 0x001e_8480;
}
