#[doc = "Register `PRIC_IO328` reader"]
pub type R = crate::R<PricIo328Spec>;
#[doc = "Register `PRIC_IO328` writer"]
pub type W = crate::W<PricIo328Spec>;
#[doc = "Field `EnblReadGroup0OfESPIChannel3` reader - Enable Read Group #0 of eSPI Channel 3"]
pub type EnblReadGroup0ofEspichannel3R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfESPIChannel3` writer - Enable Read Group #0 of eSPI Channel 3"]
pub type EnblReadGroup0ofEspichannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfESPIChannel3` reader - Enable Read Group #1 of eSPI Channel 3"]
pub type EnblReadGroup1ofEspichannel3R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfESPIChannel3` writer - Enable Read Group #1 of eSPI Channel 3"]
pub type EnblReadGroup1ofEspichannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfESPIChannel3` reader - Enable Read Group #2 of eSPI Channel 3"]
pub type EnblReadGroup2ofEspichannel3R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfESPIChannel3` writer - Enable Read Group #2 of eSPI Channel 3"]
pub type EnblReadGroup2ofEspichannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfESPIChannel3` reader - Enable Read Group #3 of eSPI Channel 3"]
pub type EnblReadGroup3ofEspichannel3R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfESPIChannel3` writer - Enable Read Group #3 of eSPI Channel 3"]
pub type EnblReadGroup3ofEspichannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfESPIChannel3` reader - Enable Read Group #4 of eSPI Channel 3"]
pub type EnblReadGroup4ofEspichannel3R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfESPIChannel3` writer - Enable Read Group #4 of eSPI Channel 3"]
pub type EnblReadGroup4ofEspichannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfESPIChannel3` reader - Enable Read Group #5 of eSPI Channel 3"]
pub type EnblReadGroup5ofEspichannel3R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfESPIChannel3` writer - Enable Read Group #5 of eSPI Channel 3"]
pub type EnblReadGroup5ofEspichannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1328PRIC1_328\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1328pric13280500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1328pric13280500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1328pric13280500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1328PRIC13280500` reader - Enable Reset Tolerance of PRIC1328PRIC1_328\\[05:00\\]"]
pub type EnblRstToleranceOfPric1328pric13280500R =
    crate::BitReader<EnblRstToleranceOfPric1328pric13280500>;
impl EnblRstToleranceOfPric1328pric13280500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1328pric13280500 {
        match self.bits {
            false => EnblRstToleranceOfPric1328pric13280500::ResetBySrst,
            true => EnblRstToleranceOfPric1328pric13280500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1328pric13280500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1328pric13280500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1328PRIC13280500` writer - Enable Reset Tolerance of PRIC1328PRIC1_328\\[05:00\\]"]
pub type EnblRstToleranceOfPric1328pric13280500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1328pric13280500>;
impl<'a, REG> EnblRstToleranceOfPric1328pric13280500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1328pric13280500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1328pric13280500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1328PRIC13280600` reader - Enable Write Protection of PRIC1328PRIC1_328\\[06:00\\]"]
pub type EnblWrProtOfPric1328pric13280600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1328PRIC13280600` writer - Enable Write Protection of PRIC1328PRIC1_328\\[06:00\\]"]
pub type EnblWrProtOfPric1328pric13280600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfESPITAFSFilter` reader - Enable Read Group #0 of eSPI TAFS Filter"]
pub type EnblReadGroup0ofEspitafsfilterR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfESPITAFSFilter` writer - Enable Read Group #0 of eSPI TAFS Filter"]
pub type EnblReadGroup0ofEspitafsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfESPITAFSFilter` reader - Enable Read Group #1 of eSPI TAFS Filter"]
pub type EnblReadGroup1ofEspitafsfilterR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfESPITAFSFilter` writer - Enable Read Group #1 of eSPI TAFS Filter"]
pub type EnblReadGroup1ofEspitafsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfESPITAFSFilter` reader - Enable Read Group #2 of eSPI TAFS Filter"]
pub type EnblReadGroup2ofEspitafsfilterR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfESPITAFSFilter` writer - Enable Read Group #2 of eSPI TAFS Filter"]
pub type EnblReadGroup2ofEspitafsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfESPITAFSFilter` reader - Enable Read Group #3 of eSPI TAFS Filter"]
pub type EnblReadGroup3ofEspitafsfilterR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfESPITAFSFilter` writer - Enable Read Group #3 of eSPI TAFS Filter"]
pub type EnblReadGroup3ofEspitafsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfESPITAFSFilter` reader - Enable Read Group #4 of eSPI TAFS Filter"]
pub type EnblReadGroup4ofEspitafsfilterR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfESPITAFSFilter` writer - Enable Read Group #4 of eSPI TAFS Filter"]
pub type EnblReadGroup4ofEspitafsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfESPITAFSFilter` reader - Enable Read Group #5 of eSPI TAFS Filter"]
pub type EnblReadGroup5ofEspitafsfilterR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfESPITAFSFilter` writer - Enable Read Group #5 of eSPI TAFS Filter"]
pub type EnblReadGroup5ofEspitafsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1328PRIC1_328\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1328pric13281308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1328pric13281308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1328pric13281308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1328PRIC13281308` reader - Enable Reset Tolerance of PRIC1328PRIC1_328\\[13:08\\]"]
pub type EnblRstToleranceOfPric1328pric13281308R =
    crate::BitReader<EnblRstToleranceOfPric1328pric13281308>;
impl EnblRstToleranceOfPric1328pric13281308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1328pric13281308 {
        match self.bits {
            false => EnblRstToleranceOfPric1328pric13281308::ResetBySrst,
            true => EnblRstToleranceOfPric1328pric13281308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1328pric13281308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1328pric13281308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1328PRIC13281308` writer - Enable Reset Tolerance of PRIC1328PRIC1_328\\[13:08\\]"]
pub type EnblRstToleranceOfPric1328pric13281308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1328pric13281308>;
impl<'a, REG> EnblRstToleranceOfPric1328pric13281308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1328pric13281308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1328pric13281308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1328PRIC13281408` reader - Enable Write Protection of PRIC1328PRIC1_328\\[14:08\\]"]
pub type EnblWrProtOfPric1328pric13281408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1328PRIC13281408` writer - Enable Write Protection of PRIC1328PRIC1_328\\[14:08\\]"]
pub type EnblWrProtOfPric1328pric13281408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfESPI0MMBI` reader - Enable Read Group #0 of eSPI0 MMBI"]
pub type EnblReadGroup0ofEspi0mmbiR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfESPI0MMBI` writer - Enable Read Group #0 of eSPI0 MMBI"]
pub type EnblReadGroup0ofEspi0mmbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfESPI0MMBI` reader - Enable Read Group #1 of eSPI0 MMBI"]
pub type EnblReadGroup1ofEspi0mmbiR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfESPI0MMBI` writer - Enable Read Group #1 of eSPI0 MMBI"]
pub type EnblReadGroup1ofEspi0mmbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfESPI0MMBI` reader - Enable Read Group #2 of eSPI0 MMBI"]
pub type EnblReadGroup2ofEspi0mmbiR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfESPI0MMBI` writer - Enable Read Group #2 of eSPI0 MMBI"]
pub type EnblReadGroup2ofEspi0mmbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfESPI0MMBI` reader - Enable Read Group #3 of eSPI0 MMBI"]
pub type EnblReadGroup3ofEspi0mmbiR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfESPI0MMBI` writer - Enable Read Group #3 of eSPI0 MMBI"]
pub type EnblReadGroup3ofEspi0mmbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfESPI0MMBI` reader - Enable Read Group #4 of eSPI0 MMBI"]
pub type EnblReadGroup4ofEspi0mmbiR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfESPI0MMBI` writer - Enable Read Group #4 of eSPI0 MMBI"]
pub type EnblReadGroup4ofEspi0mmbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfESPI0MMBI` reader - Enable Read Group #5 of eSPI0 MMBI"]
pub type EnblReadGroup5ofEspi0mmbiR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfESPI0MMBI` writer - Enable Read Group #5 of eSPI0 MMBI"]
pub type EnblReadGroup5ofEspi0mmbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1328PRIC1_328\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1328pric13282116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1328pric13282116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1328pric13282116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1328PRIC13282116` reader - Enable Reset Tolerance of PRIC1328PRIC1_328\\[21:16\\]"]
pub type EnblRstToleranceOfPric1328pric13282116R =
    crate::BitReader<EnblRstToleranceOfPric1328pric13282116>;
impl EnblRstToleranceOfPric1328pric13282116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1328pric13282116 {
        match self.bits {
            false => EnblRstToleranceOfPric1328pric13282116::ResetBySrst,
            true => EnblRstToleranceOfPric1328pric13282116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1328pric13282116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1328pric13282116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1328PRIC13282116` writer - Enable Reset Tolerance of PRIC1328PRIC1_328\\[21:16\\]"]
pub type EnblRstToleranceOfPric1328pric13282116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1328pric13282116>;
impl<'a, REG> EnblRstToleranceOfPric1328pric13282116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1328pric13282116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1328pric13282116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1328PRIC13282216` reader - Enable Write Protection of PRIC1328PRIC1_328\\[22:16\\]"]
pub type EnblWrProtOfPric1328pric13282216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1328PRIC13282216` writer - Enable Write Protection of PRIC1328PRIC1_328\\[22:16\\]"]
pub type EnblWrProtOfPric1328pric13282216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_read_group0of_espichannel3(&self) -> EnblReadGroup0ofEspichannel3R {
        EnblReadGroup0ofEspichannel3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_read_group1of_espichannel3(&self) -> EnblReadGroup1ofEspichannel3R {
        EnblReadGroup1ofEspichannel3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_read_group2of_espichannel3(&self) -> EnblReadGroup2ofEspichannel3R {
        EnblReadGroup2ofEspichannel3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_read_group3of_espichannel3(&self) -> EnblReadGroup3ofEspichannel3R {
        EnblReadGroup3ofEspichannel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_read_group4of_espichannel3(&self) -> EnblReadGroup4ofEspichannel3R {
        EnblReadGroup4ofEspichannel3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_read_group5of_espichannel3(&self) -> EnblReadGroup5ofEspichannel3R {
        EnblReadGroup5ofEspichannel3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1328PRIC1_328\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1328pric13280500(
        &self,
    ) -> EnblRstToleranceOfPric1328pric13280500R {
        EnblRstToleranceOfPric1328pric13280500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1328PRIC1_328\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1328pric13280600(&self) -> EnblWrProtOfPric1328pric13280600R {
        EnblWrProtOfPric1328pric13280600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_read_group0of_espitafsfilter(&self) -> EnblReadGroup0ofEspitafsfilterR {
        EnblReadGroup0ofEspitafsfilterR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_read_group1of_espitafsfilter(&self) -> EnblReadGroup1ofEspitafsfilterR {
        EnblReadGroup1ofEspitafsfilterR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_read_group2of_espitafsfilter(&self) -> EnblReadGroup2ofEspitafsfilterR {
        EnblReadGroup2ofEspitafsfilterR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_read_group3of_espitafsfilter(&self) -> EnblReadGroup3ofEspitafsfilterR {
        EnblReadGroup3ofEspitafsfilterR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_read_group4of_espitafsfilter(&self) -> EnblReadGroup4ofEspitafsfilterR {
        EnblReadGroup4ofEspitafsfilterR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_read_group5of_espitafsfilter(&self) -> EnblReadGroup5ofEspitafsfilterR {
        EnblReadGroup5ofEspitafsfilterR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1328PRIC1_328\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1328pric13281308(
        &self,
    ) -> EnblRstToleranceOfPric1328pric13281308R {
        EnblRstToleranceOfPric1328pric13281308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1328PRIC1_328\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1328pric13281408(&self) -> EnblWrProtOfPric1328pric13281408R {
        EnblWrProtOfPric1328pric13281408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_read_group0of_espi0mmbi(&self) -> EnblReadGroup0ofEspi0mmbiR {
        EnblReadGroup0ofEspi0mmbiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_read_group1of_espi0mmbi(&self) -> EnblReadGroup1ofEspi0mmbiR {
        EnblReadGroup1ofEspi0mmbiR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_read_group2of_espi0mmbi(&self) -> EnblReadGroup2ofEspi0mmbiR {
        EnblReadGroup2ofEspi0mmbiR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_read_group3of_espi0mmbi(&self) -> EnblReadGroup3ofEspi0mmbiR {
        EnblReadGroup3ofEspi0mmbiR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_read_group4of_espi0mmbi(&self) -> EnblReadGroup4ofEspi0mmbiR {
        EnblReadGroup4ofEspi0mmbiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_read_group5of_espi0mmbi(&self) -> EnblReadGroup5ofEspi0mmbiR {
        EnblReadGroup5ofEspi0mmbiR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1328PRIC1_328\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1328pric13282116(
        &self,
    ) -> EnblRstToleranceOfPric1328pric13282116R {
        EnblRstToleranceOfPric1328pric13282116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1328PRIC1_328\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1328pric13282216(&self) -> EnblWrProtOfPric1328pric13282216R {
        EnblWrProtOfPric1328pric13282216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_read_group0of_espichannel3(
        &mut self,
    ) -> EnblReadGroup0ofEspichannel3W<PricIo328Spec> {
        EnblReadGroup0ofEspichannel3W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_read_group1of_espichannel3(
        &mut self,
    ) -> EnblReadGroup1ofEspichannel3W<PricIo328Spec> {
        EnblReadGroup1ofEspichannel3W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_read_group2of_espichannel3(
        &mut self,
    ) -> EnblReadGroup2ofEspichannel3W<PricIo328Spec> {
        EnblReadGroup2ofEspichannel3W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_read_group3of_espichannel3(
        &mut self,
    ) -> EnblReadGroup3ofEspichannel3W<PricIo328Spec> {
        EnblReadGroup3ofEspichannel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_read_group4of_espichannel3(
        &mut self,
    ) -> EnblReadGroup4ofEspichannel3W<PricIo328Spec> {
        EnblReadGroup4ofEspichannel3W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_read_group5of_espichannel3(
        &mut self,
    ) -> EnblReadGroup5ofEspichannel3W<PricIo328Spec> {
        EnblReadGroup5ofEspichannel3W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1328PRIC1_328\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1328pric13280500(
        &mut self,
    ) -> EnblRstToleranceOfPric1328pric13280500W<PricIo328Spec> {
        EnblRstToleranceOfPric1328pric13280500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1328PRIC1_328\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1328pric13280600(
        &mut self,
    ) -> EnblWrProtOfPric1328pric13280600W<PricIo328Spec> {
        EnblWrProtOfPric1328pric13280600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_read_group0of_espitafsfilter(
        &mut self,
    ) -> EnblReadGroup0ofEspitafsfilterW<PricIo328Spec> {
        EnblReadGroup0ofEspitafsfilterW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_read_group1of_espitafsfilter(
        &mut self,
    ) -> EnblReadGroup1ofEspitafsfilterW<PricIo328Spec> {
        EnblReadGroup1ofEspitafsfilterW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_read_group2of_espitafsfilter(
        &mut self,
    ) -> EnblReadGroup2ofEspitafsfilterW<PricIo328Spec> {
        EnblReadGroup2ofEspitafsfilterW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_read_group3of_espitafsfilter(
        &mut self,
    ) -> EnblReadGroup3ofEspitafsfilterW<PricIo328Spec> {
        EnblReadGroup3ofEspitafsfilterW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_read_group4of_espitafsfilter(
        &mut self,
    ) -> EnblReadGroup4ofEspitafsfilterW<PricIo328Spec> {
        EnblReadGroup4ofEspitafsfilterW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_read_group5of_espitafsfilter(
        &mut self,
    ) -> EnblReadGroup5ofEspitafsfilterW<PricIo328Spec> {
        EnblReadGroup5ofEspitafsfilterW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1328PRIC1_328\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1328pric13281308(
        &mut self,
    ) -> EnblRstToleranceOfPric1328pric13281308W<PricIo328Spec> {
        EnblRstToleranceOfPric1328pric13281308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1328PRIC1_328\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1328pric13281408(
        &mut self,
    ) -> EnblWrProtOfPric1328pric13281408W<PricIo328Spec> {
        EnblWrProtOfPric1328pric13281408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_read_group0of_espi0mmbi(&mut self) -> EnblReadGroup0ofEspi0mmbiW<PricIo328Spec> {
        EnblReadGroup0ofEspi0mmbiW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_read_group1of_espi0mmbi(&mut self) -> EnblReadGroup1ofEspi0mmbiW<PricIo328Spec> {
        EnblReadGroup1ofEspi0mmbiW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_read_group2of_espi0mmbi(&mut self) -> EnblReadGroup2ofEspi0mmbiW<PricIo328Spec> {
        EnblReadGroup2ofEspi0mmbiW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_read_group3of_espi0mmbi(&mut self) -> EnblReadGroup3ofEspi0mmbiW<PricIo328Spec> {
        EnblReadGroup3ofEspi0mmbiW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_read_group4of_espi0mmbi(&mut self) -> EnblReadGroup4ofEspi0mmbiW<PricIo328Spec> {
        EnblReadGroup4ofEspi0mmbiW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_read_group5of_espi0mmbi(&mut self) -> EnblReadGroup5ofEspi0mmbiW<PricIo328Spec> {
        EnblReadGroup5ofEspi0mmbiW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1328PRIC1_328\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1328pric13282116(
        &mut self,
    ) -> EnblRstToleranceOfPric1328pric13282116W<PricIo328Spec> {
        EnblRstToleranceOfPric1328pric13282116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1328PRIC1_328\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1328pric13282216(
        &mut self,
    ) -> EnblWrProtOfPric1328pric13282216W<PricIo328Spec> {
        EnblWrProtOfPric1328pric13282216W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo328Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Slave Read Group Setting Register \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io328::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io328::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo328Spec;
impl crate::RegisterSpec for PricIo328Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io328::R`](R) reader structure"]
impl crate::Readable for PricIo328Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io328::W`](W) writer structure"]
impl crate::Writable for PricIo328Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO328 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo328Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
