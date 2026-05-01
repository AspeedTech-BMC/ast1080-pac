#[doc = "Register `PRIC_IO228` reader"]
pub type R = crate::R<PricIo228Spec>;
#[doc = "Register `PRIC_IO228` writer"]
pub type W = crate::W<PricIo228Spec>;
#[doc = "Field `EnblWrGroup0OfESPIChannel3` reader - Enable Write Group #0 of eSPI Channel 3"]
pub type EnblWrGroup0ofEspichannel3R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfESPIChannel3` writer - Enable Write Group #0 of eSPI Channel 3"]
pub type EnblWrGroup0ofEspichannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfESPIChannel3` reader - Enable Write Group #1 of eSPI Channel 3"]
pub type EnblWrGroup1ofEspichannel3R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfESPIChannel3` writer - Enable Write Group #1 of eSPI Channel 3"]
pub type EnblWrGroup1ofEspichannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfESPIChannel3` reader - Enable Write Group #2 of eSPI Channel 3"]
pub type EnblWrGroup2ofEspichannel3R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfESPIChannel3` writer - Enable Write Group #2 of eSPI Channel 3"]
pub type EnblWrGroup2ofEspichannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfESPIChannel3` reader - Enable Write Group #3 of eSPI Channel 3"]
pub type EnblWrGroup3ofEspichannel3R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfESPIChannel3` writer - Enable Write Group #3 of eSPI Channel 3"]
pub type EnblWrGroup3ofEspichannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfESPIChannel3` reader - Enable Write Group #4 of eSPI Channel 3"]
pub type EnblWrGroup4ofEspichannel3R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfESPIChannel3` writer - Enable Write Group #4 of eSPI Channel 3"]
pub type EnblWrGroup4ofEspichannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfESPIChannel3` reader - Enable Write Group #5 of eSPI Channel 3"]
pub type EnblWrGroup5ofEspichannel3R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfESPIChannel3` writer - Enable Write Group #5 of eSPI Channel 3"]
pub type EnblWrGroup5ofEspichannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1228PRIC1_228\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1228pric12280500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1228pric12280500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1228pric12280500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1228PRIC12280500` reader - Enable Reset Tolerance of PRIC1228PRIC1_228\\[05:00\\]"]
pub type EnblRstToleranceOfPric1228pric12280500R =
    crate::BitReader<EnblRstToleranceOfPric1228pric12280500>;
impl EnblRstToleranceOfPric1228pric12280500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1228pric12280500 {
        match self.bits {
            false => EnblRstToleranceOfPric1228pric12280500::ResetBySrst,
            true => EnblRstToleranceOfPric1228pric12280500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1228pric12280500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1228pric12280500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1228PRIC12280500` writer - Enable Reset Tolerance of PRIC1228PRIC1_228\\[05:00\\]"]
pub type EnblRstToleranceOfPric1228pric12280500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1228pric12280500>;
impl<'a, REG> EnblRstToleranceOfPric1228pric12280500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1228pric12280500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1228pric12280500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1228PRIC12280600` reader - Enable Write Protection of PRIC1228PRIC1_228\\[06:00\\]"]
pub type EnblWrProtOfPric1228pric12280600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1228PRIC12280600` writer - Enable Write Protection of PRIC1228PRIC1_228\\[06:00\\]"]
pub type EnblWrProtOfPric1228pric12280600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfESPITAFSFilter` reader - Enable Write Group #0 of eSPI TAFS Filter"]
pub type EnblWrGroup0ofEspitafsfilterR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfESPITAFSFilter` writer - Enable Write Group #0 of eSPI TAFS Filter"]
pub type EnblWrGroup0ofEspitafsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfESPITAFSFilter` reader - Enable Write Group #1 of eSPI TAFS Filter"]
pub type EnblWrGroup1ofEspitafsfilterR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfESPITAFSFilter` writer - Enable Write Group #1 of eSPI TAFS Filter"]
pub type EnblWrGroup1ofEspitafsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfESPITAFSFilter` reader - Enable Write Group #2 of eSPI TAFS Filter"]
pub type EnblWrGroup2ofEspitafsfilterR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfESPITAFSFilter` writer - Enable Write Group #2 of eSPI TAFS Filter"]
pub type EnblWrGroup2ofEspitafsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfESPITAFSFilter` reader - Enable Write Group #3 of eSPI TAFS Filter"]
pub type EnblWrGroup3ofEspitafsfilterR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfESPITAFSFilter` writer - Enable Write Group #3 of eSPI TAFS Filter"]
pub type EnblWrGroup3ofEspitafsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfESPITAFSFilter` reader - Enable Write Group #4 of eSPI TAFS Filter"]
pub type EnblWrGroup4ofEspitafsfilterR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfESPITAFSFilter` writer - Enable Write Group #4 of eSPI TAFS Filter"]
pub type EnblWrGroup4ofEspitafsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfESPITAFSFilter` reader - Enable Write Group #5 of eSPI TAFS Filter"]
pub type EnblWrGroup5ofEspitafsfilterR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfESPITAFSFilter` writer - Enable Write Group #5 of eSPI TAFS Filter"]
pub type EnblWrGroup5ofEspitafsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1228PRIC1_228\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1228pric12281308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1228pric12281308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1228pric12281308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1228PRIC12281308` reader - Enable Reset Tolerance of PRIC1228PRIC1_228\\[13:08\\]"]
pub type EnblRstToleranceOfPric1228pric12281308R =
    crate::BitReader<EnblRstToleranceOfPric1228pric12281308>;
impl EnblRstToleranceOfPric1228pric12281308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1228pric12281308 {
        match self.bits {
            false => EnblRstToleranceOfPric1228pric12281308::ResetBySrst,
            true => EnblRstToleranceOfPric1228pric12281308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1228pric12281308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1228pric12281308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1228PRIC12281308` writer - Enable Reset Tolerance of PRIC1228PRIC1_228\\[13:08\\]"]
pub type EnblRstToleranceOfPric1228pric12281308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1228pric12281308>;
impl<'a, REG> EnblRstToleranceOfPric1228pric12281308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1228pric12281308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1228pric12281308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1228PRIC12281408` reader - Enable Write Protection of PRIC1228PRIC1_228\\[14:08\\]"]
pub type EnblWrProtOfPric1228pric12281408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1228PRIC12281408` writer - Enable Write Protection of PRIC1228PRIC1_228\\[14:08\\]"]
pub type EnblWrProtOfPric1228pric12281408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfESPI0MMBI` reader - Enable Write Group #0 of eSPI0 MMBI"]
pub type EnblWrGroup0ofEspi0mmbiR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfESPI0MMBI` writer - Enable Write Group #0 of eSPI0 MMBI"]
pub type EnblWrGroup0ofEspi0mmbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfESPI0MMBI` reader - Enable Write Group #1 of eSPI0 MMBI"]
pub type EnblWrGroup1ofEspi0mmbiR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfESPI0MMBI` writer - Enable Write Group #1 of eSPI0 MMBI"]
pub type EnblWrGroup1ofEspi0mmbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfESPI0MMBI` reader - Enable Write Group #2 of eSPI0 MMBI"]
pub type EnblWrGroup2ofEspi0mmbiR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfESPI0MMBI` writer - Enable Write Group #2 of eSPI0 MMBI"]
pub type EnblWrGroup2ofEspi0mmbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfESPI0MMBI` reader - Enable Write Group #3 of eSPI0 MMBI"]
pub type EnblWrGroup3ofEspi0mmbiR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfESPI0MMBI` writer - Enable Write Group #3 of eSPI0 MMBI"]
pub type EnblWrGroup3ofEspi0mmbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfESPI0MMBI` reader - Enable Write Group #4 of eSPI0 MMBI"]
pub type EnblWrGroup4ofEspi0mmbiR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfESPI0MMBI` writer - Enable Write Group #4 of eSPI0 MMBI"]
pub type EnblWrGroup4ofEspi0mmbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfESPI0MMBI` reader - Enable Write Group #5 of eSPI0 MMBI"]
pub type EnblWrGroup5ofEspi0mmbiR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfESPI0MMBI` writer - Enable Write Group #5 of eSPI0 MMBI"]
pub type EnblWrGroup5ofEspi0mmbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1228PRIC1_228\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1228pric12282116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1228pric12282116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1228pric12282116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1228PRIC12282116` reader - Enable Reset Tolerance of PRIC1228PRIC1_228\\[21:16\\]"]
pub type EnblRstToleranceOfPric1228pric12282116R =
    crate::BitReader<EnblRstToleranceOfPric1228pric12282116>;
impl EnblRstToleranceOfPric1228pric12282116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1228pric12282116 {
        match self.bits {
            false => EnblRstToleranceOfPric1228pric12282116::ResetBySrst,
            true => EnblRstToleranceOfPric1228pric12282116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1228pric12282116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1228pric12282116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1228PRIC12282116` writer - Enable Reset Tolerance of PRIC1228PRIC1_228\\[21:16\\]"]
pub type EnblRstToleranceOfPric1228pric12282116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1228pric12282116>;
impl<'a, REG> EnblRstToleranceOfPric1228pric12282116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1228pric12282116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1228pric12282116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1228PRIC12282216` reader - Enable Write Protection of PRIC1228PRIC1_228\\[22:16\\]"]
pub type EnblWrProtOfPric1228pric12282216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1228PRIC12282216` writer - Enable Write Protection of PRIC1228PRIC1_228\\[22:16\\]"]
pub type EnblWrProtOfPric1228pric12282216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espichannel3(&self) -> EnblWrGroup0ofEspichannel3R {
        EnblWrGroup0ofEspichannel3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espichannel3(&self) -> EnblWrGroup1ofEspichannel3R {
        EnblWrGroup1ofEspichannel3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espichannel3(&self) -> EnblWrGroup2ofEspichannel3R {
        EnblWrGroup2ofEspichannel3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espichannel3(&self) -> EnblWrGroup3ofEspichannel3R {
        EnblWrGroup3ofEspichannel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espichannel3(&self) -> EnblWrGroup4ofEspichannel3R {
        EnblWrGroup4ofEspichannel3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espichannel3(&self) -> EnblWrGroup5ofEspichannel3R {
        EnblWrGroup5ofEspichannel3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1228PRIC1_228\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1228pric12280500(
        &self,
    ) -> EnblRstToleranceOfPric1228pric12280500R {
        EnblRstToleranceOfPric1228pric12280500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1228PRIC1_228\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1228pric12280600(&self) -> EnblWrProtOfPric1228pric12280600R {
        EnblWrProtOfPric1228pric12280600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espitafsfilter(&self) -> EnblWrGroup0ofEspitafsfilterR {
        EnblWrGroup0ofEspitafsfilterR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espitafsfilter(&self) -> EnblWrGroup1ofEspitafsfilterR {
        EnblWrGroup1ofEspitafsfilterR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espitafsfilter(&self) -> EnblWrGroup2ofEspitafsfilterR {
        EnblWrGroup2ofEspitafsfilterR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espitafsfilter(&self) -> EnblWrGroup3ofEspitafsfilterR {
        EnblWrGroup3ofEspitafsfilterR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espitafsfilter(&self) -> EnblWrGroup4ofEspitafsfilterR {
        EnblWrGroup4ofEspitafsfilterR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espitafsfilter(&self) -> EnblWrGroup5ofEspitafsfilterR {
        EnblWrGroup5ofEspitafsfilterR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1228PRIC1_228\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1228pric12281308(
        &self,
    ) -> EnblRstToleranceOfPric1228pric12281308R {
        EnblRstToleranceOfPric1228pric12281308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1228PRIC1_228\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1228pric12281408(&self) -> EnblWrProtOfPric1228pric12281408R {
        EnblWrProtOfPric1228pric12281408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espi0mmbi(&self) -> EnblWrGroup0ofEspi0mmbiR {
        EnblWrGroup0ofEspi0mmbiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espi0mmbi(&self) -> EnblWrGroup1ofEspi0mmbiR {
        EnblWrGroup1ofEspi0mmbiR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espi0mmbi(&self) -> EnblWrGroup2ofEspi0mmbiR {
        EnblWrGroup2ofEspi0mmbiR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espi0mmbi(&self) -> EnblWrGroup3ofEspi0mmbiR {
        EnblWrGroup3ofEspi0mmbiR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espi0mmbi(&self) -> EnblWrGroup4ofEspi0mmbiR {
        EnblWrGroup4ofEspi0mmbiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espi0mmbi(&self) -> EnblWrGroup5ofEspi0mmbiR {
        EnblWrGroup5ofEspi0mmbiR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1228PRIC1_228\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1228pric12282116(
        &self,
    ) -> EnblRstToleranceOfPric1228pric12282116R {
        EnblRstToleranceOfPric1228pric12282116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1228PRIC1_228\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1228pric12282216(&self) -> EnblWrProtOfPric1228pric12282216R {
        EnblWrProtOfPric1228pric12282216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espichannel3(&mut self) -> EnblWrGroup0ofEspichannel3W<PricIo228Spec> {
        EnblWrGroup0ofEspichannel3W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espichannel3(&mut self) -> EnblWrGroup1ofEspichannel3W<PricIo228Spec> {
        EnblWrGroup1ofEspichannel3W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espichannel3(&mut self) -> EnblWrGroup2ofEspichannel3W<PricIo228Spec> {
        EnblWrGroup2ofEspichannel3W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espichannel3(&mut self) -> EnblWrGroup3ofEspichannel3W<PricIo228Spec> {
        EnblWrGroup3ofEspichannel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espichannel3(&mut self) -> EnblWrGroup4ofEspichannel3W<PricIo228Spec> {
        EnblWrGroup4ofEspichannel3W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of eSPI Channel 3"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espichannel3(&mut self) -> EnblWrGroup5ofEspichannel3W<PricIo228Spec> {
        EnblWrGroup5ofEspichannel3W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1228PRIC1_228\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1228pric12280500(
        &mut self,
    ) -> EnblRstToleranceOfPric1228pric12280500W<PricIo228Spec> {
        EnblRstToleranceOfPric1228pric12280500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1228PRIC1_228\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1228pric12280600(
        &mut self,
    ) -> EnblWrProtOfPric1228pric12280600W<PricIo228Spec> {
        EnblWrProtOfPric1228pric12280600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espitafsfilter(
        &mut self,
    ) -> EnblWrGroup0ofEspitafsfilterW<PricIo228Spec> {
        EnblWrGroup0ofEspitafsfilterW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espitafsfilter(
        &mut self,
    ) -> EnblWrGroup1ofEspitafsfilterW<PricIo228Spec> {
        EnblWrGroup1ofEspitafsfilterW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espitafsfilter(
        &mut self,
    ) -> EnblWrGroup2ofEspitafsfilterW<PricIo228Spec> {
        EnblWrGroup2ofEspitafsfilterW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espitafsfilter(
        &mut self,
    ) -> EnblWrGroup3ofEspitafsfilterW<PricIo228Spec> {
        EnblWrGroup3ofEspitafsfilterW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espitafsfilter(
        &mut self,
    ) -> EnblWrGroup4ofEspitafsfilterW<PricIo228Spec> {
        EnblWrGroup4ofEspitafsfilterW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of eSPI TAFS Filter"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espitafsfilter(
        &mut self,
    ) -> EnblWrGroup5ofEspitafsfilterW<PricIo228Spec> {
        EnblWrGroup5ofEspitafsfilterW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1228PRIC1_228\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1228pric12281308(
        &mut self,
    ) -> EnblRstToleranceOfPric1228pric12281308W<PricIo228Spec> {
        EnblRstToleranceOfPric1228pric12281308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1228PRIC1_228\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1228pric12281408(
        &mut self,
    ) -> EnblWrProtOfPric1228pric12281408W<PricIo228Spec> {
        EnblWrProtOfPric1228pric12281408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espi0mmbi(&mut self) -> EnblWrGroup0ofEspi0mmbiW<PricIo228Spec> {
        EnblWrGroup0ofEspi0mmbiW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espi0mmbi(&mut self) -> EnblWrGroup1ofEspi0mmbiW<PricIo228Spec> {
        EnblWrGroup1ofEspi0mmbiW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espi0mmbi(&mut self) -> EnblWrGroup2ofEspi0mmbiW<PricIo228Spec> {
        EnblWrGroup2ofEspi0mmbiW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espi0mmbi(&mut self) -> EnblWrGroup3ofEspi0mmbiW<PricIo228Spec> {
        EnblWrGroup3ofEspi0mmbiW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espi0mmbi(&mut self) -> EnblWrGroup4ofEspi0mmbiW<PricIo228Spec> {
        EnblWrGroup4ofEspi0mmbiW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of eSPI0 MMBI"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espi0mmbi(&mut self) -> EnblWrGroup5ofEspi0mmbiW<PricIo228Spec> {
        EnblWrGroup5ofEspi0mmbiW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1228PRIC1_228\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1228pric12282116(
        &mut self,
    ) -> EnblRstToleranceOfPric1228pric12282116W<PricIo228Spec> {
        EnblRstToleranceOfPric1228pric12282116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1228PRIC1_228\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1228pric12282216(
        &mut self,
    ) -> EnblWrProtOfPric1228pric12282216W<PricIo228Spec> {
        EnblWrProtOfPric1228pric12282216W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo228Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Slave Write Group Setting Register \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io228::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io228::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo228Spec;
impl crate::RegisterSpec for PricIo228Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io228::R`](R) reader structure"]
impl crate::Readable for PricIo228Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io228::W`](W) writer structure"]
impl crate::Writable for PricIo228Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO228 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo228Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
