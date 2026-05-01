#[doc = "Register `PRIC_IO224` reader"]
pub type R = crate::R<PricIo224Spec>;
#[doc = "Register `PRIC_IO224` writer"]
pub type W = crate::W<PricIo224Spec>;
#[doc = "Field `EnblWrGroup0OfESPIGlobal` reader - Enable Write Group #0 of eSPI Global"]
pub type EnblWrGroup0ofEspiglobalR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfESPIGlobal` writer - Enable Write Group #0 of eSPI Global"]
pub type EnblWrGroup0ofEspiglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfESPIGlobal` reader - Enable Write Group #1 of eSPI Global"]
pub type EnblWrGroup1ofEspiglobalR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfESPIGlobal` writer - Enable Write Group #1 of eSPI Global"]
pub type EnblWrGroup1ofEspiglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfESPIGlobal` reader - Enable Write Group #2 of eSPI Global"]
pub type EnblWrGroup2ofEspiglobalR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfESPIGlobal` writer - Enable Write Group #2 of eSPI Global"]
pub type EnblWrGroup2ofEspiglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfESPIGlobal` reader - Enable Write Group #3 of eSPI Global"]
pub type EnblWrGroup3ofEspiglobalR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfESPIGlobal` writer - Enable Write Group #3 of eSPI Global"]
pub type EnblWrGroup3ofEspiglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfESPIGlobal` reader - Enable Write Group #4 of eSPI Global"]
pub type EnblWrGroup4ofEspiglobalR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfESPIGlobal` writer - Enable Write Group #4 of eSPI Global"]
pub type EnblWrGroup4ofEspiglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfESPIGlobal` reader - Enable Write Group #5 of eSPI Global"]
pub type EnblWrGroup5ofEspiglobalR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfESPIGlobal` writer - Enable Write Group #5 of eSPI Global"]
pub type EnblWrGroup5ofEspiglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1224PRIC1_224\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1224pric12240500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1224pric12240500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1224pric12240500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1224PRIC12240500` reader - Enable Reset Tolerance of PRIC1224PRIC1_224\\[05:00\\]"]
pub type EnblRstToleranceOfPric1224pric12240500R =
    crate::BitReader<EnblRstToleranceOfPric1224pric12240500>;
impl EnblRstToleranceOfPric1224pric12240500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1224pric12240500 {
        match self.bits {
            false => EnblRstToleranceOfPric1224pric12240500::ResetBySrst,
            true => EnblRstToleranceOfPric1224pric12240500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1224pric12240500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1224pric12240500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1224PRIC12240500` writer - Enable Reset Tolerance of PRIC1224PRIC1_224\\[05:00\\]"]
pub type EnblRstToleranceOfPric1224pric12240500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1224pric12240500>;
impl<'a, REG> EnblRstToleranceOfPric1224pric12240500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1224pric12240500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1224pric12240500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1224PRIC12240600` reader - Enable Write Protection of PRIC1224PRIC1_224\\[06:00\\]"]
pub type EnblWrProtOfPric1224pric12240600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1224PRIC12240600` writer - Enable Write Protection of PRIC1224PRIC1_224\\[06:00\\]"]
pub type EnblWrProtOfPric1224pric12240600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfESPIChannel0` reader - Enable Write Group #0 of eSPI Channel 0"]
pub type EnblWrGroup0ofEspichannel0R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfESPIChannel0` writer - Enable Write Group #0 of eSPI Channel 0"]
pub type EnblWrGroup0ofEspichannel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfESPIChannel0` reader - Enable Write Group #1 of eSPI Channel 0"]
pub type EnblWrGroup1ofEspichannel0R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfESPIChannel0` writer - Enable Write Group #1 of eSPI Channel 0"]
pub type EnblWrGroup1ofEspichannel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfESPIChannel0` reader - Enable Write Group #2 of eSPI Channel 0"]
pub type EnblWrGroup2ofEspichannel0R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfESPIChannel0` writer - Enable Write Group #2 of eSPI Channel 0"]
pub type EnblWrGroup2ofEspichannel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfESPIChannel0` reader - Enable Write Group #3 of eSPI Channel 0"]
pub type EnblWrGroup3ofEspichannel0R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfESPIChannel0` writer - Enable Write Group #3 of eSPI Channel 0"]
pub type EnblWrGroup3ofEspichannel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfESPIChannel0` reader - Enable Write Group #4 of eSPI Channel 0"]
pub type EnblWrGroup4ofEspichannel0R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfESPIChannel0` writer - Enable Write Group #4 of eSPI Channel 0"]
pub type EnblWrGroup4ofEspichannel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfESPIChannel0` reader - Enable Write Group #5 of eSPI Channel 0"]
pub type EnblWrGroup5ofEspichannel0R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfESPIChannel0` writer - Enable Write Group #5 of eSPI Channel 0"]
pub type EnblWrGroup5ofEspichannel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1224PRIC1_224\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1224pric12241308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1224pric12241308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1224pric12241308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1224PRIC12241308` reader - Enable Reset Tolerance of PRIC1224PRIC1_224\\[13:08\\]"]
pub type EnblRstToleranceOfPric1224pric12241308R =
    crate::BitReader<EnblRstToleranceOfPric1224pric12241308>;
impl EnblRstToleranceOfPric1224pric12241308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1224pric12241308 {
        match self.bits {
            false => EnblRstToleranceOfPric1224pric12241308::ResetBySrst,
            true => EnblRstToleranceOfPric1224pric12241308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1224pric12241308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1224pric12241308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1224PRIC12241308` writer - Enable Reset Tolerance of PRIC1224PRIC1_224\\[13:08\\]"]
pub type EnblRstToleranceOfPric1224pric12241308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1224pric12241308>;
impl<'a, REG> EnblRstToleranceOfPric1224pric12241308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1224pric12241308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1224pric12241308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1224PRIC12241408` reader - Enable Write Protection of PRIC1224PRIC1_224\\[14:08\\]"]
pub type EnblWrProtOfPric1224pric12241408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1224PRIC12241408` writer - Enable Write Protection of PRIC1224PRIC1_224\\[14:08\\]"]
pub type EnblWrProtOfPric1224pric12241408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfESPIChannel1` reader - Enable Write Group #0 of eSPI Channel 1"]
pub type EnblWrGroup0ofEspichannel1R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfESPIChannel1` writer - Enable Write Group #0 of eSPI Channel 1"]
pub type EnblWrGroup0ofEspichannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfESPIChannel1` reader - Enable Write Group #1 of eSPI Channel 1"]
pub type EnblWrGroup1ofEspichannel1R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfESPIChannel1` writer - Enable Write Group #1 of eSPI Channel 1"]
pub type EnblWrGroup1ofEspichannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfESPIChannel1` reader - Enable Write Group #2 of eSPI Channel 1"]
pub type EnblWrGroup2ofEspichannel1R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfESPIChannel1` writer - Enable Write Group #2 of eSPI Channel 1"]
pub type EnblWrGroup2ofEspichannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfESPIChannel1` reader - Enable Write Group #3 of eSPI Channel 1"]
pub type EnblWrGroup3ofEspichannel1R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfESPIChannel1` writer - Enable Write Group #3 of eSPI Channel 1"]
pub type EnblWrGroup3ofEspichannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfESPIChannel1` reader - Enable Write Group #4 of eSPI Channel 1"]
pub type EnblWrGroup4ofEspichannel1R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfESPIChannel1` writer - Enable Write Group #4 of eSPI Channel 1"]
pub type EnblWrGroup4ofEspichannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfESPIChannel1` reader - Enable Write Group #5 of eSPI Channel 1"]
pub type EnblWrGroup5ofEspichannel1R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfESPIChannel1` writer - Enable Write Group #5 of eSPI Channel 1"]
pub type EnblWrGroup5ofEspichannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1224PRIC1_224\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1224pric12242116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1224pric12242116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1224pric12242116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1224PRIC12242116` reader - Enable Reset Tolerance of PRIC1224PRIC1_224\\[21:16\\]"]
pub type EnblRstToleranceOfPric1224pric12242116R =
    crate::BitReader<EnblRstToleranceOfPric1224pric12242116>;
impl EnblRstToleranceOfPric1224pric12242116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1224pric12242116 {
        match self.bits {
            false => EnblRstToleranceOfPric1224pric12242116::ResetBySrst,
            true => EnblRstToleranceOfPric1224pric12242116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1224pric12242116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1224pric12242116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1224PRIC12242116` writer - Enable Reset Tolerance of PRIC1224PRIC1_224\\[21:16\\]"]
pub type EnblRstToleranceOfPric1224pric12242116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1224pric12242116>;
impl<'a, REG> EnblRstToleranceOfPric1224pric12242116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1224pric12242116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1224pric12242116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1224PRIC12242216` reader - Enable Write Protection of PRIC1224PRIC1_224\\[22:16\\]"]
pub type EnblWrProtOfPric1224pric12242216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1224PRIC12242216` writer - Enable Write Protection of PRIC1224PRIC1_224\\[22:16\\]"]
pub type EnblWrProtOfPric1224pric12242216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfESPIChannel2` reader - Enable Write Group #0 of eSPI Channel 2"]
pub type EnblWrGroup0ofEspichannel2R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfESPIChannel2` writer - Enable Write Group #0 of eSPI Channel 2"]
pub type EnblWrGroup0ofEspichannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfESPIChannel2` reader - Enable Write Group #1 of eSPI Channel 2"]
pub type EnblWrGroup1ofEspichannel2R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfESPIChannel2` writer - Enable Write Group #1 of eSPI Channel 2"]
pub type EnblWrGroup1ofEspichannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfESPIChannel2` reader - Enable Write Group #2 of eSPI Channel 2"]
pub type EnblWrGroup2ofEspichannel2R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfESPIChannel2` writer - Enable Write Group #2 of eSPI Channel 2"]
pub type EnblWrGroup2ofEspichannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfESPIChannel2` reader - Enable Write Group #3 of eSPI Channel 2"]
pub type EnblWrGroup3ofEspichannel2R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfESPIChannel2` writer - Enable Write Group #3 of eSPI Channel 2"]
pub type EnblWrGroup3ofEspichannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfESPIChannel2` reader - Enable Write Group #4 of eSPI Channel 2"]
pub type EnblWrGroup4ofEspichannel2R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfESPIChannel2` writer - Enable Write Group #4 of eSPI Channel 2"]
pub type EnblWrGroup4ofEspichannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfESPIChannel2` reader - Enable Write Group #5 of eSPI Channel 2"]
pub type EnblWrGroup5ofEspichannel2R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfESPIChannel2` writer - Enable Write Group #5 of eSPI Channel 2"]
pub type EnblWrGroup5ofEspichannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1224PRIC1_224\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1224pric12242924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1224pric12242924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1224pric12242924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1224PRIC12242924` reader - Enable Reset Tolerance of PRIC1224PRIC1_224\\[29:24\\]"]
pub type EnblRstToleranceOfPric1224pric12242924R =
    crate::BitReader<EnblRstToleranceOfPric1224pric12242924>;
impl EnblRstToleranceOfPric1224pric12242924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1224pric12242924 {
        match self.bits {
            false => EnblRstToleranceOfPric1224pric12242924::ResetBySrst,
            true => EnblRstToleranceOfPric1224pric12242924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1224pric12242924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1224pric12242924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1224PRIC12242924` writer - Enable Reset Tolerance of PRIC1224PRIC1_224\\[29:24\\]"]
pub type EnblRstToleranceOfPric1224pric12242924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1224pric12242924>;
impl<'a, REG> EnblRstToleranceOfPric1224pric12242924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1224pric12242924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1224pric12242924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1224PRIC12243024` reader - Enable Write Protection of PRIC1224PRIC1_224\\[30:24\\]"]
pub type EnblWrProtOfPric1224pric12243024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1224PRIC12243024` writer - Enable Write Protection of PRIC1224PRIC1_224\\[30:24\\]"]
pub type EnblWrProtOfPric1224pric12243024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espiglobal(&self) -> EnblWrGroup0ofEspiglobalR {
        EnblWrGroup0ofEspiglobalR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espiglobal(&self) -> EnblWrGroup1ofEspiglobalR {
        EnblWrGroup1ofEspiglobalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espiglobal(&self) -> EnblWrGroup2ofEspiglobalR {
        EnblWrGroup2ofEspiglobalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espiglobal(&self) -> EnblWrGroup3ofEspiglobalR {
        EnblWrGroup3ofEspiglobalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espiglobal(&self) -> EnblWrGroup4ofEspiglobalR {
        EnblWrGroup4ofEspiglobalR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espiglobal(&self) -> EnblWrGroup5ofEspiglobalR {
        EnblWrGroup5ofEspiglobalR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1224PRIC1_224\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1224pric12240500(
        &self,
    ) -> EnblRstToleranceOfPric1224pric12240500R {
        EnblRstToleranceOfPric1224pric12240500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1224PRIC1_224\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1224pric12240600(&self) -> EnblWrProtOfPric1224pric12240600R {
        EnblWrProtOfPric1224pric12240600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espichannel0(&self) -> EnblWrGroup0ofEspichannel0R {
        EnblWrGroup0ofEspichannel0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espichannel0(&self) -> EnblWrGroup1ofEspichannel0R {
        EnblWrGroup1ofEspichannel0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espichannel0(&self) -> EnblWrGroup2ofEspichannel0R {
        EnblWrGroup2ofEspichannel0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espichannel0(&self) -> EnblWrGroup3ofEspichannel0R {
        EnblWrGroup3ofEspichannel0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espichannel0(&self) -> EnblWrGroup4ofEspichannel0R {
        EnblWrGroup4ofEspichannel0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espichannel0(&self) -> EnblWrGroup5ofEspichannel0R {
        EnblWrGroup5ofEspichannel0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1224PRIC1_224\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1224pric12241308(
        &self,
    ) -> EnblRstToleranceOfPric1224pric12241308R {
        EnblRstToleranceOfPric1224pric12241308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1224PRIC1_224\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1224pric12241408(&self) -> EnblWrProtOfPric1224pric12241408R {
        EnblWrProtOfPric1224pric12241408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espichannel1(&self) -> EnblWrGroup0ofEspichannel1R {
        EnblWrGroup0ofEspichannel1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espichannel1(&self) -> EnblWrGroup1ofEspichannel1R {
        EnblWrGroup1ofEspichannel1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espichannel1(&self) -> EnblWrGroup2ofEspichannel1R {
        EnblWrGroup2ofEspichannel1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espichannel1(&self) -> EnblWrGroup3ofEspichannel1R {
        EnblWrGroup3ofEspichannel1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espichannel1(&self) -> EnblWrGroup4ofEspichannel1R {
        EnblWrGroup4ofEspichannel1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espichannel1(&self) -> EnblWrGroup5ofEspichannel1R {
        EnblWrGroup5ofEspichannel1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1224PRIC1_224\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1224pric12242116(
        &self,
    ) -> EnblRstToleranceOfPric1224pric12242116R {
        EnblRstToleranceOfPric1224pric12242116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1224PRIC1_224\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1224pric12242216(&self) -> EnblWrProtOfPric1224pric12242216R {
        EnblWrProtOfPric1224pric12242216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espichannel2(&self) -> EnblWrGroup0ofEspichannel2R {
        EnblWrGroup0ofEspichannel2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espichannel2(&self) -> EnblWrGroup1ofEspichannel2R {
        EnblWrGroup1ofEspichannel2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espichannel2(&self) -> EnblWrGroup2ofEspichannel2R {
        EnblWrGroup2ofEspichannel2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espichannel2(&self) -> EnblWrGroup3ofEspichannel2R {
        EnblWrGroup3ofEspichannel2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espichannel2(&self) -> EnblWrGroup4ofEspichannel2R {
        EnblWrGroup4ofEspichannel2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espichannel2(&self) -> EnblWrGroup5ofEspichannel2R {
        EnblWrGroup5ofEspichannel2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1224PRIC1_224\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1224pric12242924(
        &self,
    ) -> EnblRstToleranceOfPric1224pric12242924R {
        EnblRstToleranceOfPric1224pric12242924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1224PRIC1_224\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1224pric12243024(&self) -> EnblWrProtOfPric1224pric12243024R {
        EnblWrProtOfPric1224pric12243024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espiglobal(&mut self) -> EnblWrGroup0ofEspiglobalW<PricIo224Spec> {
        EnblWrGroup0ofEspiglobalW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espiglobal(&mut self) -> EnblWrGroup1ofEspiglobalW<PricIo224Spec> {
        EnblWrGroup1ofEspiglobalW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espiglobal(&mut self) -> EnblWrGroup2ofEspiglobalW<PricIo224Spec> {
        EnblWrGroup2ofEspiglobalW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espiglobal(&mut self) -> EnblWrGroup3ofEspiglobalW<PricIo224Spec> {
        EnblWrGroup3ofEspiglobalW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espiglobal(&mut self) -> EnblWrGroup4ofEspiglobalW<PricIo224Spec> {
        EnblWrGroup4ofEspiglobalW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espiglobal(&mut self) -> EnblWrGroup5ofEspiglobalW<PricIo224Spec> {
        EnblWrGroup5ofEspiglobalW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1224PRIC1_224\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1224pric12240500(
        &mut self,
    ) -> EnblRstToleranceOfPric1224pric12240500W<PricIo224Spec> {
        EnblRstToleranceOfPric1224pric12240500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1224PRIC1_224\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1224pric12240600(
        &mut self,
    ) -> EnblWrProtOfPric1224pric12240600W<PricIo224Spec> {
        EnblWrProtOfPric1224pric12240600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espichannel0(&mut self) -> EnblWrGroup0ofEspichannel0W<PricIo224Spec> {
        EnblWrGroup0ofEspichannel0W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espichannel0(&mut self) -> EnblWrGroup1ofEspichannel0W<PricIo224Spec> {
        EnblWrGroup1ofEspichannel0W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espichannel0(&mut self) -> EnblWrGroup2ofEspichannel0W<PricIo224Spec> {
        EnblWrGroup2ofEspichannel0W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espichannel0(&mut self) -> EnblWrGroup3ofEspichannel0W<PricIo224Spec> {
        EnblWrGroup3ofEspichannel0W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espichannel0(&mut self) -> EnblWrGroup4ofEspichannel0W<PricIo224Spec> {
        EnblWrGroup4ofEspichannel0W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espichannel0(&mut self) -> EnblWrGroup5ofEspichannel0W<PricIo224Spec> {
        EnblWrGroup5ofEspichannel0W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1224PRIC1_224\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1224pric12241308(
        &mut self,
    ) -> EnblRstToleranceOfPric1224pric12241308W<PricIo224Spec> {
        EnblRstToleranceOfPric1224pric12241308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1224PRIC1_224\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1224pric12241408(
        &mut self,
    ) -> EnblWrProtOfPric1224pric12241408W<PricIo224Spec> {
        EnblWrProtOfPric1224pric12241408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espichannel1(&mut self) -> EnblWrGroup0ofEspichannel1W<PricIo224Spec> {
        EnblWrGroup0ofEspichannel1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espichannel1(&mut self) -> EnblWrGroup1ofEspichannel1W<PricIo224Spec> {
        EnblWrGroup1ofEspichannel1W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espichannel1(&mut self) -> EnblWrGroup2ofEspichannel1W<PricIo224Spec> {
        EnblWrGroup2ofEspichannel1W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espichannel1(&mut self) -> EnblWrGroup3ofEspichannel1W<PricIo224Spec> {
        EnblWrGroup3ofEspichannel1W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espichannel1(&mut self) -> EnblWrGroup4ofEspichannel1W<PricIo224Spec> {
        EnblWrGroup4ofEspichannel1W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espichannel1(&mut self) -> EnblWrGroup5ofEspichannel1W<PricIo224Spec> {
        EnblWrGroup5ofEspichannel1W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1224PRIC1_224\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1224pric12242116(
        &mut self,
    ) -> EnblRstToleranceOfPric1224pric12242116W<PricIo224Spec> {
        EnblRstToleranceOfPric1224pric12242116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1224PRIC1_224\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1224pric12242216(
        &mut self,
    ) -> EnblWrProtOfPric1224pric12242216W<PricIo224Spec> {
        EnblWrProtOfPric1224pric12242216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espichannel2(&mut self) -> EnblWrGroup0ofEspichannel2W<PricIo224Spec> {
        EnblWrGroup0ofEspichannel2W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espichannel2(&mut self) -> EnblWrGroup1ofEspichannel2W<PricIo224Spec> {
        EnblWrGroup1ofEspichannel2W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espichannel2(&mut self) -> EnblWrGroup2ofEspichannel2W<PricIo224Spec> {
        EnblWrGroup2ofEspichannel2W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espichannel2(&mut self) -> EnblWrGroup3ofEspichannel2W<PricIo224Spec> {
        EnblWrGroup3ofEspichannel2W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espichannel2(&mut self) -> EnblWrGroup4ofEspichannel2W<PricIo224Spec> {
        EnblWrGroup4ofEspichannel2W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espichannel2(&mut self) -> EnblWrGroup5ofEspichannel2W<PricIo224Spec> {
        EnblWrGroup5ofEspichannel2W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1224PRIC1_224\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1224pric12242924(
        &mut self,
    ) -> EnblRstToleranceOfPric1224pric12242924W<PricIo224Spec> {
        EnblRstToleranceOfPric1224pric12242924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1224PRIC1_224\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1224pric12243024(
        &mut self,
    ) -> EnblWrProtOfPric1224pric12243024W<PricIo224Spec> {
        EnblWrProtOfPric1224pric12243024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io224::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io224::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo224Spec;
impl crate::RegisterSpec for PricIo224Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io224::R`](R) reader structure"]
impl crate::Readable for PricIo224Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io224::W`](W) writer structure"]
impl crate::Writable for PricIo224Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO224 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo224Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
