#[doc = "Register `PRIC_IO324` reader"]
pub type R = crate::R<PricIo324Spec>;
#[doc = "Register `PRIC_IO324` writer"]
pub type W = crate::W<PricIo324Spec>;
#[doc = "Field `EnblReadGroup0OfESPIGlobal` reader - Enable Read Group #0 of eSPI Global"]
pub type EnblReadGroup0ofEspiglobalR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfESPIGlobal` writer - Enable Read Group #0 of eSPI Global"]
pub type EnblReadGroup0ofEspiglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfESPIGlobal` reader - Enable Read Group #1 of eSPI Global"]
pub type EnblReadGroup1ofEspiglobalR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfESPIGlobal` writer - Enable Read Group #1 of eSPI Global"]
pub type EnblReadGroup1ofEspiglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfESPIGlobal` reader - Enable Read Group #2 of eSPI Global"]
pub type EnblReadGroup2ofEspiglobalR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfESPIGlobal` writer - Enable Read Group #2 of eSPI Global"]
pub type EnblReadGroup2ofEspiglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfESPIGlobal` reader - Enable Read Group #3 of eSPI Global"]
pub type EnblReadGroup3ofEspiglobalR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfESPIGlobal` writer - Enable Read Group #3 of eSPI Global"]
pub type EnblReadGroup3ofEspiglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfESPIGlobal` reader - Enable Read Group #4 of eSPI Global"]
pub type EnblReadGroup4ofEspiglobalR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfESPIGlobal` writer - Enable Read Group #4 of eSPI Global"]
pub type EnblReadGroup4ofEspiglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfESPIGlobal` reader - Enable Read Group #5 of eSPI Global"]
pub type EnblReadGroup5ofEspiglobalR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfESPIGlobal` writer - Enable Read Group #5 of eSPI Global"]
pub type EnblReadGroup5ofEspiglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1324PRIC1_324\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1324pric13240500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1324pric13240500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1324pric13240500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1324PRIC13240500` reader - Enable Reset Tolerance of PRIC1324PRIC1_324\\[05:00\\]"]
pub type EnblRstToleranceOfPric1324pric13240500R =
    crate::BitReader<EnblRstToleranceOfPric1324pric13240500>;
impl EnblRstToleranceOfPric1324pric13240500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1324pric13240500 {
        match self.bits {
            false => EnblRstToleranceOfPric1324pric13240500::ResetBySrst,
            true => EnblRstToleranceOfPric1324pric13240500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1324pric13240500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1324pric13240500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1324PRIC13240500` writer - Enable Reset Tolerance of PRIC1324PRIC1_324\\[05:00\\]"]
pub type EnblRstToleranceOfPric1324pric13240500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1324pric13240500>;
impl<'a, REG> EnblRstToleranceOfPric1324pric13240500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1324pric13240500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1324pric13240500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1324PRIC13240600` reader - Enable Write Protection of PRIC1324PRIC1_324\\[06:00\\]"]
pub type EnblWrProtOfPric1324pric13240600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1324PRIC13240600` writer - Enable Write Protection of PRIC1324PRIC1_324\\[06:00\\]"]
pub type EnblWrProtOfPric1324pric13240600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfESPIChannel0` reader - Enable Read Group #0 of eSPI Channel 0"]
pub type EnblReadGroup0ofEspichannel0R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfESPIChannel0` writer - Enable Read Group #0 of eSPI Channel 0"]
pub type EnblReadGroup0ofEspichannel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfESPIChannel0` reader - Enable Read Group #1 of eSPI Channel 0"]
pub type EnblReadGroup1ofEspichannel0R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfESPIChannel0` writer - Enable Read Group #1 of eSPI Channel 0"]
pub type EnblReadGroup1ofEspichannel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfESPIChannel0` reader - Enable Read Group #2 of eSPI Channel 0"]
pub type EnblReadGroup2ofEspichannel0R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfESPIChannel0` writer - Enable Read Group #2 of eSPI Channel 0"]
pub type EnblReadGroup2ofEspichannel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfESPIChannel0` reader - Enable Read Group #3 of eSPI Channel 0"]
pub type EnblReadGroup3ofEspichannel0R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfESPIChannel0` writer - Enable Read Group #3 of eSPI Channel 0"]
pub type EnblReadGroup3ofEspichannel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfESPIChannel0` reader - Enable Read Group #4 of eSPI Channel 0"]
pub type EnblReadGroup4ofEspichannel0R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfESPIChannel0` writer - Enable Read Group #4 of eSPI Channel 0"]
pub type EnblReadGroup4ofEspichannel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfESPIChannel0` reader - Enable Read Group #5 of eSPI Channel 0"]
pub type EnblReadGroup5ofEspichannel0R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfESPIChannel0` writer - Enable Read Group #5 of eSPI Channel 0"]
pub type EnblReadGroup5ofEspichannel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1324PRIC1_324\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1324pric13241308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1324pric13241308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1324pric13241308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1324PRIC13241308` reader - Enable Reset Tolerance of PRIC1324PRIC1_324\\[13:08\\]"]
pub type EnblRstToleranceOfPric1324pric13241308R =
    crate::BitReader<EnblRstToleranceOfPric1324pric13241308>;
impl EnblRstToleranceOfPric1324pric13241308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1324pric13241308 {
        match self.bits {
            false => EnblRstToleranceOfPric1324pric13241308::ResetBySrst,
            true => EnblRstToleranceOfPric1324pric13241308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1324pric13241308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1324pric13241308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1324PRIC13241308` writer - Enable Reset Tolerance of PRIC1324PRIC1_324\\[13:08\\]"]
pub type EnblRstToleranceOfPric1324pric13241308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1324pric13241308>;
impl<'a, REG> EnblRstToleranceOfPric1324pric13241308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1324pric13241308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1324pric13241308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1324PRIC13241408` reader - Enable Write Protection of PRIC1324PRIC1_324\\[14:08\\]"]
pub type EnblWrProtOfPric1324pric13241408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1324PRIC13241408` writer - Enable Write Protection of PRIC1324PRIC1_324\\[14:08\\]"]
pub type EnblWrProtOfPric1324pric13241408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfESPIChannel1` reader - Enable Read Group #0 of eSPI Channel 1"]
pub type EnblReadGroup0ofEspichannel1R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfESPIChannel1` writer - Enable Read Group #0 of eSPI Channel 1"]
pub type EnblReadGroup0ofEspichannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfESPIChannel1` reader - Enable Read Group #1 of eSPI Channel 1"]
pub type EnblReadGroup1ofEspichannel1R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfESPIChannel1` writer - Enable Read Group #1 of eSPI Channel 1"]
pub type EnblReadGroup1ofEspichannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfESPIChannel1` reader - Enable Read Group #2 of eSPI Channel 1"]
pub type EnblReadGroup2ofEspichannel1R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfESPIChannel1` writer - Enable Read Group #2 of eSPI Channel 1"]
pub type EnblReadGroup2ofEspichannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfESPIChannel1` reader - Enable Read Group #3 of eSPI Channel 1"]
pub type EnblReadGroup3ofEspichannel1R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfESPIChannel1` writer - Enable Read Group #3 of eSPI Channel 1"]
pub type EnblReadGroup3ofEspichannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfESPIChannel1` reader - Enable Read Group #4 of eSPI Channel 1"]
pub type EnblReadGroup4ofEspichannel1R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfESPIChannel1` writer - Enable Read Group #4 of eSPI Channel 1"]
pub type EnblReadGroup4ofEspichannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfESPIChannel1` reader - Enable Read Group #5 of eSPI Channel 1"]
pub type EnblReadGroup5ofEspichannel1R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfESPIChannel1` writer - Enable Read Group #5 of eSPI Channel 1"]
pub type EnblReadGroup5ofEspichannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1324PRIC1_324\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1324pric13242116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1324pric13242116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1324pric13242116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1324PRIC13242116` reader - Enable Reset Tolerance of PRIC1324PRIC1_324\\[21:16\\]"]
pub type EnblRstToleranceOfPric1324pric13242116R =
    crate::BitReader<EnblRstToleranceOfPric1324pric13242116>;
impl EnblRstToleranceOfPric1324pric13242116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1324pric13242116 {
        match self.bits {
            false => EnblRstToleranceOfPric1324pric13242116::ResetBySrst,
            true => EnblRstToleranceOfPric1324pric13242116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1324pric13242116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1324pric13242116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1324PRIC13242116` writer - Enable Reset Tolerance of PRIC1324PRIC1_324\\[21:16\\]"]
pub type EnblRstToleranceOfPric1324pric13242116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1324pric13242116>;
impl<'a, REG> EnblRstToleranceOfPric1324pric13242116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1324pric13242116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1324pric13242116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1324PRIC13242216` reader - Enable Write Protection of PRIC1324PRIC1_324\\[22:16\\]"]
pub type EnblWrProtOfPric1324pric13242216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1324PRIC13242216` writer - Enable Write Protection of PRIC1324PRIC1_324\\[22:16\\]"]
pub type EnblWrProtOfPric1324pric13242216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfESPIChannel2` reader - Enable Read Group #0 of eSPI Channel 2"]
pub type EnblReadGroup0ofEspichannel2R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfESPIChannel2` writer - Enable Read Group #0 of eSPI Channel 2"]
pub type EnblReadGroup0ofEspichannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfESPIChannel2` reader - Enable Read Group #1 of eSPI Channel 2"]
pub type EnblReadGroup1ofEspichannel2R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfESPIChannel2` writer - Enable Read Group #1 of eSPI Channel 2"]
pub type EnblReadGroup1ofEspichannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfESPIChannel2` reader - Enable Read Group #2 of eSPI Channel 2"]
pub type EnblReadGroup2ofEspichannel2R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfESPIChannel2` writer - Enable Read Group #2 of eSPI Channel 2"]
pub type EnblReadGroup2ofEspichannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfESPIChannel2` reader - Enable Read Group #3 of eSPI Channel 2"]
pub type EnblReadGroup3ofEspichannel2R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfESPIChannel2` writer - Enable Read Group #3 of eSPI Channel 2"]
pub type EnblReadGroup3ofEspichannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfESPIChannel2` reader - Enable Read Group #4 of eSPI Channel 2"]
pub type EnblReadGroup4ofEspichannel2R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfESPIChannel2` writer - Enable Read Group #4 of eSPI Channel 2"]
pub type EnblReadGroup4ofEspichannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfESPIChannel2` reader - Enable Read Group #5 of eSPI Channel 2"]
pub type EnblReadGroup5ofEspichannel2R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfESPIChannel2` writer - Enable Read Group #5 of eSPI Channel 2"]
pub type EnblReadGroup5ofEspichannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1324PRIC1_324\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1324pric13242924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1324pric13242924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1324pric13242924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1324PRIC13242924` reader - Enable Reset Tolerance of PRIC1324PRIC1_324\\[29:24\\]"]
pub type EnblRstToleranceOfPric1324pric13242924R =
    crate::BitReader<EnblRstToleranceOfPric1324pric13242924>;
impl EnblRstToleranceOfPric1324pric13242924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1324pric13242924 {
        match self.bits {
            false => EnblRstToleranceOfPric1324pric13242924::ResetBySrst,
            true => EnblRstToleranceOfPric1324pric13242924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1324pric13242924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1324pric13242924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1324PRIC13242924` writer - Enable Reset Tolerance of PRIC1324PRIC1_324\\[29:24\\]"]
pub type EnblRstToleranceOfPric1324pric13242924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1324pric13242924>;
impl<'a, REG> EnblRstToleranceOfPric1324pric13242924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1324pric13242924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1324pric13242924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1324PRIC13243024` reader - Enable Write Protection of PRIC1324PRIC1_324\\[30:24\\]"]
pub type EnblWrProtOfPric1324pric13243024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1324PRIC13243024` writer - Enable Write Protection of PRIC1324PRIC1_324\\[30:24\\]"]
pub type EnblWrProtOfPric1324pric13243024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_read_group0of_espiglobal(&self) -> EnblReadGroup0ofEspiglobalR {
        EnblReadGroup0ofEspiglobalR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_read_group1of_espiglobal(&self) -> EnblReadGroup1ofEspiglobalR {
        EnblReadGroup1ofEspiglobalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_read_group2of_espiglobal(&self) -> EnblReadGroup2ofEspiglobalR {
        EnblReadGroup2ofEspiglobalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_read_group3of_espiglobal(&self) -> EnblReadGroup3ofEspiglobalR {
        EnblReadGroup3ofEspiglobalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_read_group4of_espiglobal(&self) -> EnblReadGroup4ofEspiglobalR {
        EnblReadGroup4ofEspiglobalR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_read_group5of_espiglobal(&self) -> EnblReadGroup5ofEspiglobalR {
        EnblReadGroup5ofEspiglobalR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1324PRIC1_324\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1324pric13240500(
        &self,
    ) -> EnblRstToleranceOfPric1324pric13240500R {
        EnblRstToleranceOfPric1324pric13240500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1324PRIC1_324\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1324pric13240600(&self) -> EnblWrProtOfPric1324pric13240600R {
        EnblWrProtOfPric1324pric13240600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_read_group0of_espichannel0(&self) -> EnblReadGroup0ofEspichannel0R {
        EnblReadGroup0ofEspichannel0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_read_group1of_espichannel0(&self) -> EnblReadGroup1ofEspichannel0R {
        EnblReadGroup1ofEspichannel0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_read_group2of_espichannel0(&self) -> EnblReadGroup2ofEspichannel0R {
        EnblReadGroup2ofEspichannel0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_read_group3of_espichannel0(&self) -> EnblReadGroup3ofEspichannel0R {
        EnblReadGroup3ofEspichannel0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_read_group4of_espichannel0(&self) -> EnblReadGroup4ofEspichannel0R {
        EnblReadGroup4ofEspichannel0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_read_group5of_espichannel0(&self) -> EnblReadGroup5ofEspichannel0R {
        EnblReadGroup5ofEspichannel0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1324PRIC1_324\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1324pric13241308(
        &self,
    ) -> EnblRstToleranceOfPric1324pric13241308R {
        EnblRstToleranceOfPric1324pric13241308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1324PRIC1_324\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1324pric13241408(&self) -> EnblWrProtOfPric1324pric13241408R {
        EnblWrProtOfPric1324pric13241408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_read_group0of_espichannel1(&self) -> EnblReadGroup0ofEspichannel1R {
        EnblReadGroup0ofEspichannel1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_read_group1of_espichannel1(&self) -> EnblReadGroup1ofEspichannel1R {
        EnblReadGroup1ofEspichannel1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_read_group2of_espichannel1(&self) -> EnblReadGroup2ofEspichannel1R {
        EnblReadGroup2ofEspichannel1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_read_group3of_espichannel1(&self) -> EnblReadGroup3ofEspichannel1R {
        EnblReadGroup3ofEspichannel1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_read_group4of_espichannel1(&self) -> EnblReadGroup4ofEspichannel1R {
        EnblReadGroup4ofEspichannel1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_read_group5of_espichannel1(&self) -> EnblReadGroup5ofEspichannel1R {
        EnblReadGroup5ofEspichannel1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1324PRIC1_324\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1324pric13242116(
        &self,
    ) -> EnblRstToleranceOfPric1324pric13242116R {
        EnblRstToleranceOfPric1324pric13242116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1324PRIC1_324\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1324pric13242216(&self) -> EnblWrProtOfPric1324pric13242216R {
        EnblWrProtOfPric1324pric13242216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_read_group0of_espichannel2(&self) -> EnblReadGroup0ofEspichannel2R {
        EnblReadGroup0ofEspichannel2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_read_group1of_espichannel2(&self) -> EnblReadGroup1ofEspichannel2R {
        EnblReadGroup1ofEspichannel2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_read_group2of_espichannel2(&self) -> EnblReadGroup2ofEspichannel2R {
        EnblReadGroup2ofEspichannel2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_read_group3of_espichannel2(&self) -> EnblReadGroup3ofEspichannel2R {
        EnblReadGroup3ofEspichannel2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_read_group4of_espichannel2(&self) -> EnblReadGroup4ofEspichannel2R {
        EnblReadGroup4ofEspichannel2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_read_group5of_espichannel2(&self) -> EnblReadGroup5ofEspichannel2R {
        EnblReadGroup5ofEspichannel2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1324PRIC1_324\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1324pric13242924(
        &self,
    ) -> EnblRstToleranceOfPric1324pric13242924R {
        EnblRstToleranceOfPric1324pric13242924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1324PRIC1_324\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1324pric13243024(&self) -> EnblWrProtOfPric1324pric13243024R {
        EnblWrProtOfPric1324pric13243024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_read_group0of_espiglobal(&mut self) -> EnblReadGroup0ofEspiglobalW<PricIo324Spec> {
        EnblReadGroup0ofEspiglobalW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_read_group1of_espiglobal(&mut self) -> EnblReadGroup1ofEspiglobalW<PricIo324Spec> {
        EnblReadGroup1ofEspiglobalW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_read_group2of_espiglobal(&mut self) -> EnblReadGroup2ofEspiglobalW<PricIo324Spec> {
        EnblReadGroup2ofEspiglobalW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_read_group3of_espiglobal(&mut self) -> EnblReadGroup3ofEspiglobalW<PricIo324Spec> {
        EnblReadGroup3ofEspiglobalW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_read_group4of_espiglobal(&mut self) -> EnblReadGroup4ofEspiglobalW<PricIo324Spec> {
        EnblReadGroup4ofEspiglobalW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of eSPI Global"]
    #[inline(always)]
    pub fn enbl_read_group5of_espiglobal(&mut self) -> EnblReadGroup5ofEspiglobalW<PricIo324Spec> {
        EnblReadGroup5ofEspiglobalW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1324PRIC1_324\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1324pric13240500(
        &mut self,
    ) -> EnblRstToleranceOfPric1324pric13240500W<PricIo324Spec> {
        EnblRstToleranceOfPric1324pric13240500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1324PRIC1_324\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1324pric13240600(
        &mut self,
    ) -> EnblWrProtOfPric1324pric13240600W<PricIo324Spec> {
        EnblWrProtOfPric1324pric13240600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_read_group0of_espichannel0(
        &mut self,
    ) -> EnblReadGroup0ofEspichannel0W<PricIo324Spec> {
        EnblReadGroup0ofEspichannel0W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_read_group1of_espichannel0(
        &mut self,
    ) -> EnblReadGroup1ofEspichannel0W<PricIo324Spec> {
        EnblReadGroup1ofEspichannel0W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_read_group2of_espichannel0(
        &mut self,
    ) -> EnblReadGroup2ofEspichannel0W<PricIo324Spec> {
        EnblReadGroup2ofEspichannel0W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_read_group3of_espichannel0(
        &mut self,
    ) -> EnblReadGroup3ofEspichannel0W<PricIo324Spec> {
        EnblReadGroup3ofEspichannel0W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_read_group4of_espichannel0(
        &mut self,
    ) -> EnblReadGroup4ofEspichannel0W<PricIo324Spec> {
        EnblReadGroup4ofEspichannel0W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of eSPI Channel 0"]
    #[inline(always)]
    pub fn enbl_read_group5of_espichannel0(
        &mut self,
    ) -> EnblReadGroup5ofEspichannel0W<PricIo324Spec> {
        EnblReadGroup5ofEspichannel0W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1324PRIC1_324\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1324pric13241308(
        &mut self,
    ) -> EnblRstToleranceOfPric1324pric13241308W<PricIo324Spec> {
        EnblRstToleranceOfPric1324pric13241308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1324PRIC1_324\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1324pric13241408(
        &mut self,
    ) -> EnblWrProtOfPric1324pric13241408W<PricIo324Spec> {
        EnblWrProtOfPric1324pric13241408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_read_group0of_espichannel1(
        &mut self,
    ) -> EnblReadGroup0ofEspichannel1W<PricIo324Spec> {
        EnblReadGroup0ofEspichannel1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_read_group1of_espichannel1(
        &mut self,
    ) -> EnblReadGroup1ofEspichannel1W<PricIo324Spec> {
        EnblReadGroup1ofEspichannel1W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_read_group2of_espichannel1(
        &mut self,
    ) -> EnblReadGroup2ofEspichannel1W<PricIo324Spec> {
        EnblReadGroup2ofEspichannel1W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_read_group3of_espichannel1(
        &mut self,
    ) -> EnblReadGroup3ofEspichannel1W<PricIo324Spec> {
        EnblReadGroup3ofEspichannel1W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_read_group4of_espichannel1(
        &mut self,
    ) -> EnblReadGroup4ofEspichannel1W<PricIo324Spec> {
        EnblReadGroup4ofEspichannel1W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of eSPI Channel 1"]
    #[inline(always)]
    pub fn enbl_read_group5of_espichannel1(
        &mut self,
    ) -> EnblReadGroup5ofEspichannel1W<PricIo324Spec> {
        EnblReadGroup5ofEspichannel1W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1324PRIC1_324\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1324pric13242116(
        &mut self,
    ) -> EnblRstToleranceOfPric1324pric13242116W<PricIo324Spec> {
        EnblRstToleranceOfPric1324pric13242116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1324PRIC1_324\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1324pric13242216(
        &mut self,
    ) -> EnblWrProtOfPric1324pric13242216W<PricIo324Spec> {
        EnblWrProtOfPric1324pric13242216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_read_group0of_espichannel2(
        &mut self,
    ) -> EnblReadGroup0ofEspichannel2W<PricIo324Spec> {
        EnblReadGroup0ofEspichannel2W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_read_group1of_espichannel2(
        &mut self,
    ) -> EnblReadGroup1ofEspichannel2W<PricIo324Spec> {
        EnblReadGroup1ofEspichannel2W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_read_group2of_espichannel2(
        &mut self,
    ) -> EnblReadGroup2ofEspichannel2W<PricIo324Spec> {
        EnblReadGroup2ofEspichannel2W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_read_group3of_espichannel2(
        &mut self,
    ) -> EnblReadGroup3ofEspichannel2W<PricIo324Spec> {
        EnblReadGroup3ofEspichannel2W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_read_group4of_espichannel2(
        &mut self,
    ) -> EnblReadGroup4ofEspichannel2W<PricIo324Spec> {
        EnblReadGroup4ofEspichannel2W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of eSPI Channel 2"]
    #[inline(always)]
    pub fn enbl_read_group5of_espichannel2(
        &mut self,
    ) -> EnblReadGroup5ofEspichannel2W<PricIo324Spec> {
        EnblReadGroup5ofEspichannel2W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1324PRIC1_324\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1324pric13242924(
        &mut self,
    ) -> EnblRstToleranceOfPric1324pric13242924W<PricIo324Spec> {
        EnblRstToleranceOfPric1324pric13242924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1324PRIC1_324\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1324pric13243024(
        &mut self,
    ) -> EnblWrProtOfPric1324pric13243024W<PricIo324Spec> {
        EnblWrProtOfPric1324pric13243024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io324::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io324::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo324Spec;
impl crate::RegisterSpec for PricIo324Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io324::R`](R) reader structure"]
impl crate::Readable for PricIo324Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io324::W`](W) writer structure"]
impl crate::Writable for PricIo324Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO324 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo324Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
