#[doc = "Register `I3CPHYCTRLREG09C` reader"]
pub type R = crate::R<I3cphyctrlreg09cSpec>;
#[doc = "Register `I3CPHYCTRLREG09C` writer"]
pub type W = crate::W<I3cphyctrlreg09cSpec>;
#[doc = "Field `REGI3CSPECIALPATTERNSCLGOHIGHCNT` reader - REG_I3C_SPECIAL_PATTERN_SCL_GO_HIGH_CNT"]
pub type Regi3cspecialpatternsclgohighcntR = crate::FieldReader;
#[doc = "Field `REGI3CSPECIALPATTERNSCLGOHIGHCNT` writer - REG_I3C_SPECIAL_PATTERN_SCL_GO_HIGH_CNT"]
pub type Regi3cspecialpatternsclgohighcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGI3CSPECIALPATTERNSDALCNT` reader - REG_I3C_SPECIAL_PATTERN_SDA_LCNT"]
pub type Regi3cspecialpatternsdalcntR = crate::FieldReader;
#[doc = "Field `REGI3CSPECIALPATTERNSDALCNT` writer - REG_I3C_SPECIAL_PATTERN_SDA_LCNT"]
pub type Regi3cspecialpatternsdalcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGI3CSPECIALPATTERNSDAHCNT` reader - REG_I3C_SPECIAL_PATTERN_SDA_HCNT"]
pub type Regi3cspecialpatternsdahcntR = crate::FieldReader;
#[doc = "Field `REGI3CSPECIALPATTERNSDAHCNT` writer - REG_I3C_SPECIAL_PATTERN_SDA_HCNT"]
pub type Regi3cspecialpatternsdahcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGHDREXTWITHOUTSTOP` reader - REG_HDR_EXT_WITHOUT_STOP"]
pub type ReghdrextwithoutstopR = crate::BitReader;
#[doc = "Field `REGHDREXTWITHOUTSTOP` writer - REG_HDR_EXT_WITHOUT_STOP"]
pub type ReghdrextwithoutstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTGRSTWITHOUTSTARTSTOP` reader - REG_TG_RST_WITHOUT_START_STOP"]
pub type RegtgrstwithoutstartstopR = crate::BitReader;
#[doc = "Field `REGTGRSTWITHOUTSTARTSTOP` writer - REG_TG_RST_WITHOUT_START_STOP"]
pub type RegtgrstwithoutstartstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSDASTUCKHOLDSCLHRST` reader - REG_SDA_STUCK_HOLD_SCLH_RST"]
pub type RegsdastuckholdsclhrstR = crate::BitReader;
#[doc = "Field `REGSDASTUCKHOLDSCLHRST` writer - REG_SDA_STUCK_HOLD_SCLH_RST"]
pub type RegsdastuckholdsclhrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSDASTUCKPATAUTOHOLD` reader - REG_SDA_STUCK_PAT_AUTO_HOLD"]
pub type RegsdastuckpatautoholdR = crate::BitReader;
#[doc = "Field `REGSDASTUCKPATAUTOHOLD` writer - REG_SDA_STUCK_PAT_AUTO_HOLD"]
pub type RegsdastuckpatautoholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGI3CSDASTUCKPATAUTOTBITHIGH` reader - REG_I3C_SDA_STUCK_PAT_AUTO_TBIT_HIGH"]
pub type Regi3csdastuckpatautotbithighR = crate::BitReader;
#[doc = "Field `REGI3CSDASTUCKPATAUTOTBITHIGH` writer - REG_I3C_SDA_STUCK_PAT_AUTO_TBIT_HIGH"]
pub type Regi3csdastuckpatautotbithighW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_I3C_SPECIAL_PATTERN_SCL_GO_HIGH_CNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternsclgohighcnt(&self) -> Regi3cspecialpatternsclgohighcntR {
        Regi3cspecialpatternsclgohighcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_I3C_SPECIAL_PATTERN_SDA_LCNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternsdalcnt(&self) -> Regi3cspecialpatternsdalcntR {
        Regi3cspecialpatternsdalcntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_I3C_SPECIAL_PATTERN_SDA_HCNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternsdahcnt(&self) -> Regi3cspecialpatternsdahcntR {
        Regi3cspecialpatternsdahcntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - REG_HDR_EXT_WITHOUT_STOP"]
    #[inline(always)]
    pub fn reghdrextwithoutstop(&self) -> ReghdrextwithoutstopR {
        ReghdrextwithoutstopR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - REG_TG_RST_WITHOUT_START_STOP"]
    #[inline(always)]
    pub fn regtgrstwithoutstartstop(&self) -> RegtgrstwithoutstartstopR {
        RegtgrstwithoutstartstopR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - REG_SDA_STUCK_HOLD_SCLH_RST"]
    #[inline(always)]
    pub fn regsdastuckholdsclhrst(&self) -> RegsdastuckholdsclhrstR {
        RegsdastuckholdsclhrstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - REG_SDA_STUCK_PAT_AUTO_HOLD"]
    #[inline(always)]
    pub fn regsdastuckpatautohold(&self) -> RegsdastuckpatautoholdR {
        RegsdastuckpatautoholdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - REG_I3C_SDA_STUCK_PAT_AUTO_TBIT_HIGH"]
    #[inline(always)]
    pub fn regi3csdastuckpatautotbithigh(&self) -> Regi3csdastuckpatautotbithighR {
        Regi3csdastuckpatautotbithighR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_I3C_SPECIAL_PATTERN_SCL_GO_HIGH_CNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternsclgohighcnt(
        &mut self,
    ) -> Regi3cspecialpatternsclgohighcntW<I3cphyctrlreg09cSpec> {
        Regi3cspecialpatternsclgohighcntW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_I3C_SPECIAL_PATTERN_SDA_LCNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternsdalcnt(
        &mut self,
    ) -> Regi3cspecialpatternsdalcntW<I3cphyctrlreg09cSpec> {
        Regi3cspecialpatternsdalcntW::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_I3C_SPECIAL_PATTERN_SDA_HCNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternsdahcnt(
        &mut self,
    ) -> Regi3cspecialpatternsdahcntW<I3cphyctrlreg09cSpec> {
        Regi3cspecialpatternsdahcntW::new(self, 16)
    }
    #[doc = "Bit 24 - REG_HDR_EXT_WITHOUT_STOP"]
    #[inline(always)]
    pub fn reghdrextwithoutstop(&mut self) -> ReghdrextwithoutstopW<I3cphyctrlreg09cSpec> {
        ReghdrextwithoutstopW::new(self, 24)
    }
    #[doc = "Bit 25 - REG_TG_RST_WITHOUT_START_STOP"]
    #[inline(always)]
    pub fn regtgrstwithoutstartstop(&mut self) -> RegtgrstwithoutstartstopW<I3cphyctrlreg09cSpec> {
        RegtgrstwithoutstartstopW::new(self, 25)
    }
    #[doc = "Bit 26 - REG_SDA_STUCK_HOLD_SCLH_RST"]
    #[inline(always)]
    pub fn regsdastuckholdsclhrst(&mut self) -> RegsdastuckholdsclhrstW<I3cphyctrlreg09cSpec> {
        RegsdastuckholdsclhrstW::new(self, 26)
    }
    #[doc = "Bit 27 - REG_SDA_STUCK_PAT_AUTO_HOLD"]
    #[inline(always)]
    pub fn regsdastuckpatautohold(&mut self) -> RegsdastuckpatautoholdW<I3cphyctrlreg09cSpec> {
        RegsdastuckpatautoholdW::new(self, 27)
    }
    #[doc = "Bit 28 - REG_I3C_SDA_STUCK_PAT_AUTO_TBIT_HIGH"]
    #[inline(always)]
    pub fn regi3csdastuckpatautotbithigh(
        &mut self,
    ) -> Regi3csdastuckpatautotbithighW<I3cphyctrlreg09cSpec> {
        Regi3csdastuckpatautotbithighW::new(self, 28)
    }
}
#[doc = "SPECIAL\\_PATTERN\\_SET\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg09c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg09c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg09cSpec;
impl crate::RegisterSpec for I3cphyctrlreg09cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg09c::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg09cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg09c::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg09cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG09C to value 0x0809_0909"]
impl crate::Resettable for I3cphyctrlreg09cSpec {
    const RESET_VALUE: u32 = 0x0809_0909;
}
