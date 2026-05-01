#[doc = "Register `I3CPHYCTRLREG0A4` reader"]
pub type R = crate::R<I3cphyctrlreg0a4Spec>;
#[doc = "Register `I3CPHYCTRLREG0A4` writer"]
pub type W = crate::W<I3cphyctrlreg0a4Spec>;
#[doc = "Field `REGI3CSPECIALPATTERNSCLTOGGLECNT` reader - REG_I3C_SPECIAL_PATTERN_SCL_TOGGLE_CNT"]
pub type Regi3cspecialpatternscltogglecntR = crate::FieldReader;
#[doc = "Field `REGI3CSPECIALPATTERNSCLTOGGLECNT` writer - REG_I3C_SPECIAL_PATTERN_SCL_TOGGLE_CNT"]
pub type Regi3cspecialpatternscltogglecntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CSPECIALPATTERNSCLTOGGLEPATREPEAT` reader - REG_I3C_SPECIAL_PATTERN_SCL_TOGGLE_PAT_REPEAT"]
pub type Regi3cspecialpatternscltogglepatrepeatR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSPECIALPATTERNSCLTOGGLEPATREPEAT` writer - REG_I3C_SPECIAL_PATTERN_SCL_TOGGLE_PAT_REPEAT"]
pub type Regi3cspecialpatternscltogglepatrepeatW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:4 - REG_I3C_SPECIAL_PATTERN_SCL_TOGGLE_CNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternscltogglecnt(&self) -> Regi3cspecialpatternscltogglecntR {
        Regi3cspecialpatternscltogglecntR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:17 - REG_I3C_SPECIAL_PATTERN_SCL_TOGGLE_PAT_REPEAT"]
    #[inline(always)]
    pub fn regi3cspecialpatternscltogglepatrepeat(
        &self,
    ) -> Regi3cspecialpatternscltogglepatrepeatR {
        Regi3cspecialpatternscltogglepatrepeatR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - REG_I3C_SPECIAL_PATTERN_SCL_TOGGLE_CNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternscltogglecnt(
        &mut self,
    ) -> Regi3cspecialpatternscltogglecntW<I3cphyctrlreg0a4Spec> {
        Regi3cspecialpatternscltogglecntW::new(self, 0)
    }
    #[doc = "Bits 8:17 - REG_I3C_SPECIAL_PATTERN_SCL_TOGGLE_PAT_REPEAT"]
    #[inline(always)]
    pub fn regi3cspecialpatternscltogglepatrepeat(
        &mut self,
    ) -> Regi3cspecialpatternscltogglepatrepeatW<I3cphyctrlreg0a4Spec> {
        Regi3cspecialpatternscltogglepatrepeatW::new(self, 8)
    }
}
#[doc = "SPECIAL\\_PATTERN\\_SCL\\_TOGGLE\\_SET\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0a4Spec;
impl crate::RegisterSpec for I3cphyctrlreg0a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0a4::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0a4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0a4::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0A4 to value 0x03"]
impl crate::Resettable for I3cphyctrlreg0a4Spec {
    const RESET_VALUE: u32 = 0x03;
}
