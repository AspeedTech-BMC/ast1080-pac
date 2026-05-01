#[doc = "Register `I3CPHYCTRLREG0A0` reader"]
pub type R = crate::R<I3cphyctrlreg0a0Spec>;
#[doc = "Register `I3CPHYCTRLREG0A0` writer"]
pub type W = crate::W<I3cphyctrlreg0a0Spec>;
#[doc = "Field `REGI3CSPECIALPATTERNSWSEL` reader - REG_I3C_SPECIAL_PATTERN_SW_SEL"]
pub type Regi3cspecialpatternswselR = crate::FieldReader;
#[doc = "Field `REGI3CSPECIALPATTERNSWSEL` writer - REG_I3C_SPECIAL_PATTERN_SW_SEL"]
pub type Regi3cspecialpatternswselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REGI3CSPECIALPATTERNSWSPEED` reader - REG_I3C_SPECIAL_PATTERN_SW_SPEED"]
pub type Regi3cspecialpatternswspeedR = crate::FieldReader;
#[doc = "Field `REGI3CSPECIALPATTERNSWSPEED` writer - REG_I3C_SPECIAL_PATTERN_SW_SPEED"]
pub type Regi3cspecialpatternswspeedW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REGHDREXTSTOPRECALC` reader - REG_HDR_EXT_STOP_RECALC"]
pub type ReghdrextstoprecalcR = crate::BitReader;
#[doc = "Field `REGHDREXTSTOPRECALC` writer - REG_HDR_EXT_STOP_RECALC"]
pub type ReghdrextstoprecalcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTGRSTSDARECALC` reader - REG_TG_RST_SDA_RECALC"]
pub type RegtgrstsdarecalcR = crate::BitReader;
#[doc = "Field `REGTGRSTSDARECALC` writer - REG_TG_RST_SDA_RECALC"]
pub type RegtgrstsdarecalcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTGRSTSRRECALC` reader - REG_TG_RST_SR_RECALC"]
pub type RegtgrstsrrecalcR = crate::BitReader;
#[doc = "Field `REGTGRSTSRRECALC` writer - REG_TG_RST_SR_RECALC"]
pub type RegtgrstsrrecalcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTGRSTPRECALC` reader - REG_TG_RST_P_RECALC"]
pub type RegtgrstprecalcR = crate::BitReader;
#[doc = "Field `REGTGRSTPRECALC` writer - REG_TG_RST_P_RECALC"]
pub type RegtgrstprecalcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGI3CSPECIALPATTERNSWREQCLR` reader - REG_I3C_SPECIAL_PATTERN_SW_REQ_CLR"]
pub type Regi3cspecialpatternswreqclrR = crate::BitReader;
#[doc = "Field `REGI3CSPECIALPATTERNSWREQ` reader - REG_I3C_SPECIAL_PATTERN_SW_REQ"]
pub type Regi3cspecialpatternswreqR = crate::BitReader;
#[doc = "Field `REGI3CSPECIALPATTERNSWREQ` writer - REG_I3C_SPECIAL_PATTERN_SW_REQ"]
pub type Regi3cspecialpatternswreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - REG_I3C_SPECIAL_PATTERN_SW_SEL"]
    #[inline(always)]
    pub fn regi3cspecialpatternswsel(&self) -> Regi3cspecialpatternswselR {
        Regi3cspecialpatternswselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - REG_I3C_SPECIAL_PATTERN_SW_SPEED"]
    #[inline(always)]
    pub fn regi3cspecialpatternswspeed(&self) -> Regi3cspecialpatternswspeedR {
        Regi3cspecialpatternswspeedR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - REG_HDR_EXT_STOP_RECALC"]
    #[inline(always)]
    pub fn reghdrextstoprecalc(&self) -> ReghdrextstoprecalcR {
        ReghdrextstoprecalcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REG_TG_RST_SDA_RECALC"]
    #[inline(always)]
    pub fn regtgrstsdarecalc(&self) -> RegtgrstsdarecalcR {
        RegtgrstsdarecalcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_TG_RST_SR_RECALC"]
    #[inline(always)]
    pub fn regtgrstsrrecalc(&self) -> RegtgrstsrrecalcR {
        RegtgrstsrrecalcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REG_TG_RST_P_RECALC"]
    #[inline(always)]
    pub fn regtgrstprecalc(&self) -> RegtgrstprecalcR {
        RegtgrstprecalcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 30 - REG_I3C_SPECIAL_PATTERN_SW_REQ_CLR"]
    #[inline(always)]
    pub fn regi3cspecialpatternswreqclr(&self) -> Regi3cspecialpatternswreqclrR {
        Regi3cspecialpatternswreqclrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - REG_I3C_SPECIAL_PATTERN_SW_REQ"]
    #[inline(always)]
    pub fn regi3cspecialpatternswreq(&self) -> Regi3cspecialpatternswreqR {
        Regi3cspecialpatternswreqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - REG_I3C_SPECIAL_PATTERN_SW_SEL"]
    #[inline(always)]
    pub fn regi3cspecialpatternswsel(
        &mut self,
    ) -> Regi3cspecialpatternswselW<I3cphyctrlreg0a0Spec> {
        Regi3cspecialpatternswselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - REG_I3C_SPECIAL_PATTERN_SW_SPEED"]
    #[inline(always)]
    pub fn regi3cspecialpatternswspeed(
        &mut self,
    ) -> Regi3cspecialpatternswspeedW<I3cphyctrlreg0a0Spec> {
        Regi3cspecialpatternswspeedW::new(self, 4)
    }
    #[doc = "Bit 8 - REG_HDR_EXT_STOP_RECALC"]
    #[inline(always)]
    pub fn reghdrextstoprecalc(&mut self) -> ReghdrextstoprecalcW<I3cphyctrlreg0a0Spec> {
        ReghdrextstoprecalcW::new(self, 8)
    }
    #[doc = "Bit 9 - REG_TG_RST_SDA_RECALC"]
    #[inline(always)]
    pub fn regtgrstsdarecalc(&mut self) -> RegtgrstsdarecalcW<I3cphyctrlreg0a0Spec> {
        RegtgrstsdarecalcW::new(self, 9)
    }
    #[doc = "Bit 10 - REG_TG_RST_SR_RECALC"]
    #[inline(always)]
    pub fn regtgrstsrrecalc(&mut self) -> RegtgrstsrrecalcW<I3cphyctrlreg0a0Spec> {
        RegtgrstsrrecalcW::new(self, 10)
    }
    #[doc = "Bit 11 - REG_TG_RST_P_RECALC"]
    #[inline(always)]
    pub fn regtgrstprecalc(&mut self) -> RegtgrstprecalcW<I3cphyctrlreg0a0Spec> {
        RegtgrstprecalcW::new(self, 11)
    }
    #[doc = "Bit 31 - REG_I3C_SPECIAL_PATTERN_SW_REQ"]
    #[inline(always)]
    pub fn regi3cspecialpatternswreq(
        &mut self,
    ) -> Regi3cspecialpatternswreqW<I3cphyctrlreg0a0Spec> {
        Regi3cspecialpatternswreqW::new(self, 31)
    }
}
#[doc = "SPECIAL\\_PATTERN\\_SW\\_OPT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0a0Spec;
impl crate::RegisterSpec for I3cphyctrlreg0a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0a0::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0a0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0a0::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0A0 to value 0x20"]
impl crate::Resettable for I3cphyctrlreg0a0Spec {
    const RESET_VALUE: u32 = 0x20;
}
