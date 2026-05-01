#[doc = "Register `PRIC_IO20C` reader"]
pub type R = crate::R<PricIo20cSpec>;
#[doc = "Register `PRIC_IO20C` writer"]
pub type W = crate::W<PricIo20cSpec>;
#[doc = "Field `EnblWrGroup0OfPRICTRL` reader - Enable Write Group #0 of PRICTRL"]
pub type EnblWrGroup0ofPrictrlR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfPRICTRL` writer - Enable Write Group #0 of PRICTRL"]
pub type EnblWrGroup0ofPrictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfPRICTRL` reader - Enable Write Group #1 of PRICTRL"]
pub type EnblWrGroup1ofPrictrlR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfPRICTRL` writer - Enable Write Group #1 of PRICTRL"]
pub type EnblWrGroup1ofPrictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfPRICTRL` reader - Enable Write Group #2 of PRICTRL"]
pub type EnblWrGroup2ofPrictrlR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfPRICTRL` writer - Enable Write Group #2 of PRICTRL"]
pub type EnblWrGroup2ofPrictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfPRICTRL` reader - Enable Write Group #3 of PRICTRL"]
pub type EnblWrGroup3ofPrictrlR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfPRICTRL` writer - Enable Write Group #3 of PRICTRL"]
pub type EnblWrGroup3ofPrictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfPRICTRL` reader - Enable Write Group #4 of PRICTRL"]
pub type EnblWrGroup4ofPrictrlR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfPRICTRL` writer - Enable Write Group #4 of PRICTRL"]
pub type EnblWrGroup4ofPrictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfPRICTRL` reader - Enable Write Group #5 of PRICTRL"]
pub type EnblWrGroup5ofPrictrlR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfPRICTRL` writer - Enable Write Group #5 of PRICTRL"]
pub type EnblWrGroup5ofPrictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC120CPRIC1_20C\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric120cpric120c0500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric120cpric120c0500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric120cpric120c0500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC120CPRIC120C0500` reader - Enable Reset Tolerance of PRIC120CPRIC1_20C\\[05:00\\]"]
pub type EnblRstToleranceOfPric120cpric120c0500R =
    crate::BitReader<EnblRstToleranceOfPric120cpric120c0500>;
impl EnblRstToleranceOfPric120cpric120c0500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric120cpric120c0500 {
        match self.bits {
            false => EnblRstToleranceOfPric120cpric120c0500::ResetBySrst,
            true => EnblRstToleranceOfPric120cpric120c0500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric120cpric120c0500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric120cpric120c0500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC120CPRIC120C0500` writer - Enable Reset Tolerance of PRIC120CPRIC1_20C\\[05:00\\]"]
pub type EnblRstToleranceOfPric120cpric120c0500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric120cpric120c0500>;
impl<'a, REG> EnblRstToleranceOfPric120cpric120c0500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric120cpric120c0500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric120cpric120c0500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC120CPRIC120C0600` reader - Enable Write Protection of PRIC120CPRIC1_20C\\[06:00\\]"]
pub type EnblWrProtOfPric120cpric120c0600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC120CPRIC120C0600` writer - Enable Write Protection of PRIC120CPRIC1_20C\\[06:00\\]"]
pub type EnblWrProtOfPric120cpric120c0600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfCaliptra` reader - Enable Write Group #0 of Caliptra"]
pub type EnblWrGroup0ofCaliptraR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfCaliptra` writer - Enable Write Group #0 of Caliptra"]
pub type EnblWrGroup0ofCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfCaliptra` reader - Enable Write Group #1 of Caliptra"]
pub type EnblWrGroup1ofCaliptraR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfCaliptra` writer - Enable Write Group #1 of Caliptra"]
pub type EnblWrGroup1ofCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfCaliptra` reader - Enable Write Group #2 of Caliptra"]
pub type EnblWrGroup2ofCaliptraR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfCaliptra` writer - Enable Write Group #2 of Caliptra"]
pub type EnblWrGroup2ofCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfCaliptra` reader - Enable Write Group #3 of Caliptra"]
pub type EnblWrGroup3ofCaliptraR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfCaliptra` writer - Enable Write Group #3 of Caliptra"]
pub type EnblWrGroup3ofCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfCaliptra` reader - Enable Write Group #4 of Caliptra"]
pub type EnblWrGroup4ofCaliptraR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfCaliptra` writer - Enable Write Group #4 of Caliptra"]
pub type EnblWrGroup4ofCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfCaliptra` reader - Enable Write Group #5 of Caliptra"]
pub type EnblWrGroup5ofCaliptraR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfCaliptra` writer - Enable Write Group #5 of Caliptra"]
pub type EnblWrGroup5ofCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC120CPRIC1_20C\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric120cpric120c1308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric120cpric120c1308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric120cpric120c1308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC120CPRIC120C1308` reader - Enable Reset Tolerance of PRIC120CPRIC1_20C\\[13:08\\]"]
pub type EnblRstToleranceOfPric120cpric120c1308R =
    crate::BitReader<EnblRstToleranceOfPric120cpric120c1308>;
impl EnblRstToleranceOfPric120cpric120c1308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric120cpric120c1308 {
        match self.bits {
            false => EnblRstToleranceOfPric120cpric120c1308::ResetBySrst,
            true => EnblRstToleranceOfPric120cpric120c1308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric120cpric120c1308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric120cpric120c1308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC120CPRIC120C1308` writer - Enable Reset Tolerance of PRIC120CPRIC1_20C\\[13:08\\]"]
pub type EnblRstToleranceOfPric120cpric120c1308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric120cpric120c1308>;
impl<'a, REG> EnblRstToleranceOfPric120cpric120c1308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric120cpric120c1308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric120cpric120c1308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC120CPRIC120C1408` reader - Enable Write Protection of PRIC120CPRIC1_20C\\[14:08\\]"]
pub type EnblWrProtOfPric120cpric120c1408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC120CPRIC120C1408` writer - Enable Write Protection of PRIC120CPRIC1_20C\\[14:08\\]"]
pub type EnblWrProtOfPric120cpric120c1408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSRAMMemory` reader - Enable Write Group #0 of SRAM Memory"]
pub type EnblWrGroup0ofSrammemoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSRAMMemory` writer - Enable Write Group #0 of SRAM Memory"]
pub type EnblWrGroup0ofSrammemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSRAMMemory` reader - Enable Write Group #1 of SRAM Memory"]
pub type EnblWrGroup1ofSrammemoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSRAMMemory` writer - Enable Write Group #1 of SRAM Memory"]
pub type EnblWrGroup1ofSrammemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSRAMMemory` reader - Enable Write Group #2 of SRAM Memory"]
pub type EnblWrGroup2ofSrammemoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSRAMMemory` writer - Enable Write Group #2 of SRAM Memory"]
pub type EnblWrGroup2ofSrammemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSRAMMemory` reader - Enable Write Group #3 of SRAM Memory"]
pub type EnblWrGroup3ofSrammemoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSRAMMemory` writer - Enable Write Group #3 of SRAM Memory"]
pub type EnblWrGroup3ofSrammemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSRAMMemory` reader - Enable Write Group #4 of SRAM Memory"]
pub type EnblWrGroup4ofSrammemoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSRAMMemory` writer - Enable Write Group #4 of SRAM Memory"]
pub type EnblWrGroup4ofSrammemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSRAMMemory` reader - Enable Write Group #5 of SRAM Memory"]
pub type EnblWrGroup5ofSrammemoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSRAMMemory` writer - Enable Write Group #5 of SRAM Memory"]
pub type EnblWrGroup5ofSrammemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC120CPRIC1_20C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric120cpric120c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric120cpric120c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric120cpric120c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC120CPRIC120C2924` reader - Enable Reset Tolerance of PRIC120CPRIC1_20C\\[29:24\\]"]
pub type EnblRstToleranceOfPric120cpric120c2924R =
    crate::BitReader<EnblRstToleranceOfPric120cpric120c2924>;
impl EnblRstToleranceOfPric120cpric120c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric120cpric120c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric120cpric120c2924::ResetBySrst,
            true => EnblRstToleranceOfPric120cpric120c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric120cpric120c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric120cpric120c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC120CPRIC120C2924` writer - Enable Reset Tolerance of PRIC120CPRIC1_20C\\[29:24\\]"]
pub type EnblRstToleranceOfPric120cpric120c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric120cpric120c2924>;
impl<'a, REG> EnblRstToleranceOfPric120cpric120c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric120cpric120c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric120cpric120c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC120CPRIC120C3024` reader - Enable Write Protection of PRIC120CPRIC1_20C\\[30:24\\]"]
pub type EnblWrProtOfPric120cpric120c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC120CPRIC120C3024` writer - Enable Write Protection of PRIC120CPRIC1_20C\\[30:24\\]"]
pub type EnblWrProtOfPric120cpric120c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_wr_group0of_prictrl(&self) -> EnblWrGroup0ofPrictrlR {
        EnblWrGroup0ofPrictrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_wr_group1of_prictrl(&self) -> EnblWrGroup1ofPrictrlR {
        EnblWrGroup1ofPrictrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_wr_group2of_prictrl(&self) -> EnblWrGroup2ofPrictrlR {
        EnblWrGroup2ofPrictrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_wr_group3of_prictrl(&self) -> EnblWrGroup3ofPrictrlR {
        EnblWrGroup3ofPrictrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_wr_group4of_prictrl(&self) -> EnblWrGroup4ofPrictrlR {
        EnblWrGroup4ofPrictrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_wr_group5of_prictrl(&self) -> EnblWrGroup5ofPrictrlR {
        EnblWrGroup5ofPrictrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC120CPRIC1_20C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric120cpric120c0500(
        &self,
    ) -> EnblRstToleranceOfPric120cpric120c0500R {
        EnblRstToleranceOfPric120cpric120c0500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC120CPRIC1_20C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric120cpric120c0600(&self) -> EnblWrProtOfPric120cpric120c0600R {
        EnblWrProtOfPric120cpric120c0600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of Caliptra"]
    #[inline(always)]
    pub fn enbl_wr_group0of_caliptra(&self) -> EnblWrGroup0ofCaliptraR {
        EnblWrGroup0ofCaliptraR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of Caliptra"]
    #[inline(always)]
    pub fn enbl_wr_group1of_caliptra(&self) -> EnblWrGroup1ofCaliptraR {
        EnblWrGroup1ofCaliptraR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of Caliptra"]
    #[inline(always)]
    pub fn enbl_wr_group2of_caliptra(&self) -> EnblWrGroup2ofCaliptraR {
        EnblWrGroup2ofCaliptraR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of Caliptra"]
    #[inline(always)]
    pub fn enbl_wr_group3of_caliptra(&self) -> EnblWrGroup3ofCaliptraR {
        EnblWrGroup3ofCaliptraR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of Caliptra"]
    #[inline(always)]
    pub fn enbl_wr_group4of_caliptra(&self) -> EnblWrGroup4ofCaliptraR {
        EnblWrGroup4ofCaliptraR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of Caliptra"]
    #[inline(always)]
    pub fn enbl_wr_group5of_caliptra(&self) -> EnblWrGroup5ofCaliptraR {
        EnblWrGroup5ofCaliptraR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC120CPRIC1_20C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric120cpric120c1308(
        &self,
    ) -> EnblRstToleranceOfPric120cpric120c1308R {
        EnblRstToleranceOfPric120cpric120c1308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC120CPRIC1_20C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric120cpric120c1408(&self) -> EnblWrProtOfPric120cpric120c1408R {
        EnblWrProtOfPric120cpric120c1408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_wr_group0of_srammemory(&self) -> EnblWrGroup0ofSrammemoryR {
        EnblWrGroup0ofSrammemoryR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_wr_group1of_srammemory(&self) -> EnblWrGroup1ofSrammemoryR {
        EnblWrGroup1ofSrammemoryR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_wr_group2of_srammemory(&self) -> EnblWrGroup2ofSrammemoryR {
        EnblWrGroup2ofSrammemoryR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_wr_group3of_srammemory(&self) -> EnblWrGroup3ofSrammemoryR {
        EnblWrGroup3ofSrammemoryR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_wr_group4of_srammemory(&self) -> EnblWrGroup4ofSrammemoryR {
        EnblWrGroup4ofSrammemoryR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_wr_group5of_srammemory(&self) -> EnblWrGroup5ofSrammemoryR {
        EnblWrGroup5ofSrammemoryR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC120CPRIC1_20C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric120cpric120c2924(
        &self,
    ) -> EnblRstToleranceOfPric120cpric120c2924R {
        EnblRstToleranceOfPric120cpric120c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC120CPRIC1_20C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric120cpric120c3024(&self) -> EnblWrProtOfPric120cpric120c3024R {
        EnblWrProtOfPric120cpric120c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_wr_group0of_prictrl(&mut self) -> EnblWrGroup0ofPrictrlW<PricIo20cSpec> {
        EnblWrGroup0ofPrictrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_wr_group1of_prictrl(&mut self) -> EnblWrGroup1ofPrictrlW<PricIo20cSpec> {
        EnblWrGroup1ofPrictrlW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_wr_group2of_prictrl(&mut self) -> EnblWrGroup2ofPrictrlW<PricIo20cSpec> {
        EnblWrGroup2ofPrictrlW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_wr_group3of_prictrl(&mut self) -> EnblWrGroup3ofPrictrlW<PricIo20cSpec> {
        EnblWrGroup3ofPrictrlW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_wr_group4of_prictrl(&mut self) -> EnblWrGroup4ofPrictrlW<PricIo20cSpec> {
        EnblWrGroup4ofPrictrlW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_wr_group5of_prictrl(&mut self) -> EnblWrGroup5ofPrictrlW<PricIo20cSpec> {
        EnblWrGroup5ofPrictrlW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC120CPRIC1_20C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric120cpric120c0500(
        &mut self,
    ) -> EnblRstToleranceOfPric120cpric120c0500W<PricIo20cSpec> {
        EnblRstToleranceOfPric120cpric120c0500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC120CPRIC1_20C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric120cpric120c0600(
        &mut self,
    ) -> EnblWrProtOfPric120cpric120c0600W<PricIo20cSpec> {
        EnblWrProtOfPric120cpric120c0600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of Caliptra"]
    #[inline(always)]
    pub fn enbl_wr_group0of_caliptra(&mut self) -> EnblWrGroup0ofCaliptraW<PricIo20cSpec> {
        EnblWrGroup0ofCaliptraW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of Caliptra"]
    #[inline(always)]
    pub fn enbl_wr_group1of_caliptra(&mut self) -> EnblWrGroup1ofCaliptraW<PricIo20cSpec> {
        EnblWrGroup1ofCaliptraW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of Caliptra"]
    #[inline(always)]
    pub fn enbl_wr_group2of_caliptra(&mut self) -> EnblWrGroup2ofCaliptraW<PricIo20cSpec> {
        EnblWrGroup2ofCaliptraW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of Caliptra"]
    #[inline(always)]
    pub fn enbl_wr_group3of_caliptra(&mut self) -> EnblWrGroup3ofCaliptraW<PricIo20cSpec> {
        EnblWrGroup3ofCaliptraW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of Caliptra"]
    #[inline(always)]
    pub fn enbl_wr_group4of_caliptra(&mut self) -> EnblWrGroup4ofCaliptraW<PricIo20cSpec> {
        EnblWrGroup4ofCaliptraW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of Caliptra"]
    #[inline(always)]
    pub fn enbl_wr_group5of_caliptra(&mut self) -> EnblWrGroup5ofCaliptraW<PricIo20cSpec> {
        EnblWrGroup5ofCaliptraW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC120CPRIC1_20C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric120cpric120c1308(
        &mut self,
    ) -> EnblRstToleranceOfPric120cpric120c1308W<PricIo20cSpec> {
        EnblRstToleranceOfPric120cpric120c1308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC120CPRIC1_20C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric120cpric120c1408(
        &mut self,
    ) -> EnblWrProtOfPric120cpric120c1408W<PricIo20cSpec> {
        EnblWrProtOfPric120cpric120c1408W::new(self, 15)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo20cSpec> {
        Reserved7W::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo20cSpec> {
        Reserved6W::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo20cSpec> {
        Reserved5W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo20cSpec> {
        Reserved4W::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo20cSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo20cSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo20cSpec> {
        Reserved1W::new(self, 22)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_wr_group0of_srammemory(&mut self) -> EnblWrGroup0ofSrammemoryW<PricIo20cSpec> {
        EnblWrGroup0ofSrammemoryW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_wr_group1of_srammemory(&mut self) -> EnblWrGroup1ofSrammemoryW<PricIo20cSpec> {
        EnblWrGroup1ofSrammemoryW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_wr_group2of_srammemory(&mut self) -> EnblWrGroup2ofSrammemoryW<PricIo20cSpec> {
        EnblWrGroup2ofSrammemoryW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_wr_group3of_srammemory(&mut self) -> EnblWrGroup3ofSrammemoryW<PricIo20cSpec> {
        EnblWrGroup3ofSrammemoryW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_wr_group4of_srammemory(&mut self) -> EnblWrGroup4ofSrammemoryW<PricIo20cSpec> {
        EnblWrGroup4ofSrammemoryW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_wr_group5of_srammemory(&mut self) -> EnblWrGroup5ofSrammemoryW<PricIo20cSpec> {
        EnblWrGroup5ofSrammemoryW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC120CPRIC1_20C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric120cpric120c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric120cpric120c2924W<PricIo20cSpec> {
        EnblRstToleranceOfPric120cpric120c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC120CPRIC1_20C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric120cpric120c3024(
        &mut self,
    ) -> EnblWrProtOfPric120cpric120c3024W<PricIo20cSpec> {
        EnblWrProtOfPric120cpric120c3024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io20c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io20c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo20cSpec;
impl crate::RegisterSpec for PricIo20cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io20c::R`](R) reader structure"]
impl crate::Readable for PricIo20cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io20c::W`](W) writer structure"]
impl crate::Writable for PricIo20cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO20C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo20cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
