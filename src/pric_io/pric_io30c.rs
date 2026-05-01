#[doc = "Register `PRIC_IO30C` reader"]
pub type R = crate::R<PricIo30cSpec>;
#[doc = "Register `PRIC_IO30C` writer"]
pub type W = crate::W<PricIo30cSpec>;
#[doc = "Field `EnblReadGroup0OfPRICTRL` reader - Enable Read Group #0 of PRICTRL"]
pub type EnblReadGroup0ofPrictrlR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfPRICTRL` writer - Enable Read Group #0 of PRICTRL"]
pub type EnblReadGroup0ofPrictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfPRICTRL` reader - Enable Read Group #1 of PRICTRL"]
pub type EnblReadGroup1ofPrictrlR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfPRICTRL` writer - Enable Read Group #1 of PRICTRL"]
pub type EnblReadGroup1ofPrictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfPRICTRL` reader - Enable Read Group #2 of PRICTRL"]
pub type EnblReadGroup2ofPrictrlR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfPRICTRL` writer - Enable Read Group #2 of PRICTRL"]
pub type EnblReadGroup2ofPrictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfPRICTRL` reader - Enable Read Group #3 of PRICTRL"]
pub type EnblReadGroup3ofPrictrlR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfPRICTRL` writer - Enable Read Group #3 of PRICTRL"]
pub type EnblReadGroup3ofPrictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfPRICTRL` reader - Enable Read Group #4 of PRICTRL"]
pub type EnblReadGroup4ofPrictrlR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfPRICTRL` writer - Enable Read Group #4 of PRICTRL"]
pub type EnblReadGroup4ofPrictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfPRICTRL` reader - Enable Read Group #5 of PRICTRL"]
pub type EnblReadGroup5ofPrictrlR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfPRICTRL` writer - Enable Read Group #5 of PRICTRL"]
pub type EnblReadGroup5ofPrictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC130CPRIC1_30C\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric130cpric130c0500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric130cpric130c0500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric130cpric130c0500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC130CPRIC130C0500` reader - Enable Reset Tolerance of PRIC130CPRIC1_30C\\[05:00\\]"]
pub type EnblRstToleranceOfPric130cpric130c0500R =
    crate::BitReader<EnblRstToleranceOfPric130cpric130c0500>;
impl EnblRstToleranceOfPric130cpric130c0500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric130cpric130c0500 {
        match self.bits {
            false => EnblRstToleranceOfPric130cpric130c0500::ResetBySrst,
            true => EnblRstToleranceOfPric130cpric130c0500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric130cpric130c0500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric130cpric130c0500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC130CPRIC130C0500` writer - Enable Reset Tolerance of PRIC130CPRIC1_30C\\[05:00\\]"]
pub type EnblRstToleranceOfPric130cpric130c0500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric130cpric130c0500>;
impl<'a, REG> EnblRstToleranceOfPric130cpric130c0500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric130cpric130c0500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric130cpric130c0500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC130CPRIC130C0600` reader - Enable Write Protection of PRIC130CPRIC1_30C\\[06:00\\]"]
pub type EnblWrProtOfPric130cpric130c0600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC130CPRIC130C0600` writer - Enable Write Protection of PRIC130CPRIC1_30C\\[06:00\\]"]
pub type EnblWrProtOfPric130cpric130c0600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfCaliptra` reader - Enable Read Group #0 of Caliptra"]
pub type EnblReadGroup0ofCaliptraR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfCaliptra` writer - Enable Read Group #0 of Caliptra"]
pub type EnblReadGroup0ofCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfCaliptra` reader - Enable Read Group #1 of Caliptra"]
pub type EnblReadGroup1ofCaliptraR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfCaliptra` writer - Enable Read Group #1 of Caliptra"]
pub type EnblReadGroup1ofCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfCaliptra` reader - Enable Read Group #2 of Caliptra"]
pub type EnblReadGroup2ofCaliptraR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfCaliptra` writer - Enable Read Group #2 of Caliptra"]
pub type EnblReadGroup2ofCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfCaliptra` reader - Enable Read Group #3 of Caliptra"]
pub type EnblReadGroup3ofCaliptraR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfCaliptra` writer - Enable Read Group #3 of Caliptra"]
pub type EnblReadGroup3ofCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfCaliptra` reader - Enable Read Group #4 of Caliptra"]
pub type EnblReadGroup4ofCaliptraR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfCaliptra` writer - Enable Read Group #4 of Caliptra"]
pub type EnblReadGroup4ofCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfCaliptra` reader - Enable Read Group #5 of Caliptra"]
pub type EnblReadGroup5ofCaliptraR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfCaliptra` writer - Enable Read Group #5 of Caliptra"]
pub type EnblReadGroup5ofCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC130CPRIC1_30C\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric130cpric130c1308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric130cpric130c1308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric130cpric130c1308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC130CPRIC130C1308` reader - Enable Reset Tolerance of PRIC130CPRIC1_30C\\[13:08\\]"]
pub type EnblRstToleranceOfPric130cpric130c1308R =
    crate::BitReader<EnblRstToleranceOfPric130cpric130c1308>;
impl EnblRstToleranceOfPric130cpric130c1308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric130cpric130c1308 {
        match self.bits {
            false => EnblRstToleranceOfPric130cpric130c1308::ResetBySrst,
            true => EnblRstToleranceOfPric130cpric130c1308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric130cpric130c1308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric130cpric130c1308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC130CPRIC130C1308` writer - Enable Reset Tolerance of PRIC130CPRIC1_30C\\[13:08\\]"]
pub type EnblRstToleranceOfPric130cpric130c1308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric130cpric130c1308>;
impl<'a, REG> EnblRstToleranceOfPric130cpric130c1308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric130cpric130c1308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric130cpric130c1308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC130CPRIC130C1408` reader - Enable Write Protection of PRIC130CPRIC1_30C\\[14:08\\]"]
pub type EnblWrProtOfPric130cpric130c1408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC130CPRIC130C1408` writer - Enable Write Protection of PRIC130CPRIC1_30C\\[14:08\\]"]
pub type EnblWrProtOfPric130cpric130c1408W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `EnblReadGroup0OfSRAMMemory` reader - Enable Read Group #0 of SRAM Memory"]
pub type EnblReadGroup0ofSrammemoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSRAMMemory` writer - Enable Read Group #0 of SRAM Memory"]
pub type EnblReadGroup0ofSrammemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSRAMMemory` reader - Enable Read Group #1 of SRAM Memory"]
pub type EnblReadGroup1ofSrammemoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSRAMMemory` writer - Enable Read Group #1 of SRAM Memory"]
pub type EnblReadGroup1ofSrammemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSRAMMemory` reader - Enable Read Group #2 of SRAM Memory"]
pub type EnblReadGroup2ofSrammemoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSRAMMemory` writer - Enable Read Group #2 of SRAM Memory"]
pub type EnblReadGroup2ofSrammemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSRAMMemory` reader - Enable Read Group #3 of SRAM Memory"]
pub type EnblReadGroup3ofSrammemoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSRAMMemory` writer - Enable Read Group #3 of SRAM Memory"]
pub type EnblReadGroup3ofSrammemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSRAMMemory` reader - Enable Read Group #4 of SRAM Memory"]
pub type EnblReadGroup4ofSrammemoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSRAMMemory` writer - Enable Read Group #4 of SRAM Memory"]
pub type EnblReadGroup4ofSrammemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSRAMMemory` reader - Enable Read Group #5 of SRAM Memory"]
pub type EnblReadGroup5ofSrammemoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSRAMMemory` writer - Enable Read Group #5 of SRAM Memory"]
pub type EnblReadGroup5ofSrammemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC130CPRIC1_30C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric130cpric130c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric130cpric130c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric130cpric130c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC130CPRIC130C2924` reader - Enable Reset Tolerance of PRIC130CPRIC1_30C\\[29:24\\]"]
pub type EnblRstToleranceOfPric130cpric130c2924R =
    crate::BitReader<EnblRstToleranceOfPric130cpric130c2924>;
impl EnblRstToleranceOfPric130cpric130c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric130cpric130c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric130cpric130c2924::ResetBySrst,
            true => EnblRstToleranceOfPric130cpric130c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric130cpric130c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric130cpric130c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC130CPRIC130C2924` writer - Enable Reset Tolerance of PRIC130CPRIC1_30C\\[29:24\\]"]
pub type EnblRstToleranceOfPric130cpric130c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric130cpric130c2924>;
impl<'a, REG> EnblRstToleranceOfPric130cpric130c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric130cpric130c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric130cpric130c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC130CPRIC130C3024` reader - Enable Write Protection of PRIC130CPRIC1_30C\\[30:24\\]"]
pub type EnblWrProtOfPric130cpric130c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC130CPRIC130C3024` writer - Enable Write Protection of PRIC130CPRIC1_30C\\[30:24\\]"]
pub type EnblWrProtOfPric130cpric130c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_read_group0of_prictrl(&self) -> EnblReadGroup0ofPrictrlR {
        EnblReadGroup0ofPrictrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_read_group1of_prictrl(&self) -> EnblReadGroup1ofPrictrlR {
        EnblReadGroup1ofPrictrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_read_group2of_prictrl(&self) -> EnblReadGroup2ofPrictrlR {
        EnblReadGroup2ofPrictrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_read_group3of_prictrl(&self) -> EnblReadGroup3ofPrictrlR {
        EnblReadGroup3ofPrictrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_read_group4of_prictrl(&self) -> EnblReadGroup4ofPrictrlR {
        EnblReadGroup4ofPrictrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_read_group5of_prictrl(&self) -> EnblReadGroup5ofPrictrlR {
        EnblReadGroup5ofPrictrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC130CPRIC1_30C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric130cpric130c0500(
        &self,
    ) -> EnblRstToleranceOfPric130cpric130c0500R {
        EnblRstToleranceOfPric130cpric130c0500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC130CPRIC1_30C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric130cpric130c0600(&self) -> EnblWrProtOfPric130cpric130c0600R {
        EnblWrProtOfPric130cpric130c0600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of Caliptra"]
    #[inline(always)]
    pub fn enbl_read_group0of_caliptra(&self) -> EnblReadGroup0ofCaliptraR {
        EnblReadGroup0ofCaliptraR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of Caliptra"]
    #[inline(always)]
    pub fn enbl_read_group1of_caliptra(&self) -> EnblReadGroup1ofCaliptraR {
        EnblReadGroup1ofCaliptraR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of Caliptra"]
    #[inline(always)]
    pub fn enbl_read_group2of_caliptra(&self) -> EnblReadGroup2ofCaliptraR {
        EnblReadGroup2ofCaliptraR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of Caliptra"]
    #[inline(always)]
    pub fn enbl_read_group3of_caliptra(&self) -> EnblReadGroup3ofCaliptraR {
        EnblReadGroup3ofCaliptraR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of Caliptra"]
    #[inline(always)]
    pub fn enbl_read_group4of_caliptra(&self) -> EnblReadGroup4ofCaliptraR {
        EnblReadGroup4ofCaliptraR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of Caliptra"]
    #[inline(always)]
    pub fn enbl_read_group5of_caliptra(&self) -> EnblReadGroup5ofCaliptraR {
        EnblReadGroup5ofCaliptraR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC130CPRIC1_30C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric130cpric130c1308(
        &self,
    ) -> EnblRstToleranceOfPric130cpric130c1308R {
        EnblRstToleranceOfPric130cpric130c1308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC130CPRIC1_30C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric130cpric130c1408(&self) -> EnblWrProtOfPric130cpric130c1408R {
        EnblWrProtOfPric130cpric130c1408R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 24 - Enable Read Group #0 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_read_group0of_srammemory(&self) -> EnblReadGroup0ofSrammemoryR {
        EnblReadGroup0ofSrammemoryR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_read_group1of_srammemory(&self) -> EnblReadGroup1ofSrammemoryR {
        EnblReadGroup1ofSrammemoryR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_read_group2of_srammemory(&self) -> EnblReadGroup2ofSrammemoryR {
        EnblReadGroup2ofSrammemoryR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_read_group3of_srammemory(&self) -> EnblReadGroup3ofSrammemoryR {
        EnblReadGroup3ofSrammemoryR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_read_group4of_srammemory(&self) -> EnblReadGroup4ofSrammemoryR {
        EnblReadGroup4ofSrammemoryR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_read_group5of_srammemory(&self) -> EnblReadGroup5ofSrammemoryR {
        EnblReadGroup5ofSrammemoryR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC130CPRIC1_30C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric130cpric130c2924(
        &self,
    ) -> EnblRstToleranceOfPric130cpric130c2924R {
        EnblRstToleranceOfPric130cpric130c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC130CPRIC1_30C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric130cpric130c3024(&self) -> EnblWrProtOfPric130cpric130c3024R {
        EnblWrProtOfPric130cpric130c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_read_group0of_prictrl(&mut self) -> EnblReadGroup0ofPrictrlW<PricIo30cSpec> {
        EnblReadGroup0ofPrictrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_read_group1of_prictrl(&mut self) -> EnblReadGroup1ofPrictrlW<PricIo30cSpec> {
        EnblReadGroup1ofPrictrlW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_read_group2of_prictrl(&mut self) -> EnblReadGroup2ofPrictrlW<PricIo30cSpec> {
        EnblReadGroup2ofPrictrlW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_read_group3of_prictrl(&mut self) -> EnblReadGroup3ofPrictrlW<PricIo30cSpec> {
        EnblReadGroup3ofPrictrlW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_read_group4of_prictrl(&mut self) -> EnblReadGroup4ofPrictrlW<PricIo30cSpec> {
        EnblReadGroup4ofPrictrlW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of PRICTRL"]
    #[inline(always)]
    pub fn enbl_read_group5of_prictrl(&mut self) -> EnblReadGroup5ofPrictrlW<PricIo30cSpec> {
        EnblReadGroup5ofPrictrlW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC130CPRIC1_30C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric130cpric130c0500(
        &mut self,
    ) -> EnblRstToleranceOfPric130cpric130c0500W<PricIo30cSpec> {
        EnblRstToleranceOfPric130cpric130c0500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC130CPRIC1_30C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric130cpric130c0600(
        &mut self,
    ) -> EnblWrProtOfPric130cpric130c0600W<PricIo30cSpec> {
        EnblWrProtOfPric130cpric130c0600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of Caliptra"]
    #[inline(always)]
    pub fn enbl_read_group0of_caliptra(&mut self) -> EnblReadGroup0ofCaliptraW<PricIo30cSpec> {
        EnblReadGroup0ofCaliptraW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of Caliptra"]
    #[inline(always)]
    pub fn enbl_read_group1of_caliptra(&mut self) -> EnblReadGroup1ofCaliptraW<PricIo30cSpec> {
        EnblReadGroup1ofCaliptraW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of Caliptra"]
    #[inline(always)]
    pub fn enbl_read_group2of_caliptra(&mut self) -> EnblReadGroup2ofCaliptraW<PricIo30cSpec> {
        EnblReadGroup2ofCaliptraW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of Caliptra"]
    #[inline(always)]
    pub fn enbl_read_group3of_caliptra(&mut self) -> EnblReadGroup3ofCaliptraW<PricIo30cSpec> {
        EnblReadGroup3ofCaliptraW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of Caliptra"]
    #[inline(always)]
    pub fn enbl_read_group4of_caliptra(&mut self) -> EnblReadGroup4ofCaliptraW<PricIo30cSpec> {
        EnblReadGroup4ofCaliptraW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of Caliptra"]
    #[inline(always)]
    pub fn enbl_read_group5of_caliptra(&mut self) -> EnblReadGroup5ofCaliptraW<PricIo30cSpec> {
        EnblReadGroup5ofCaliptraW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC130CPRIC1_30C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric130cpric130c1308(
        &mut self,
    ) -> EnblRstToleranceOfPric130cpric130c1308W<PricIo30cSpec> {
        EnblRstToleranceOfPric130cpric130c1308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC130CPRIC1_30C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric130cpric130c1408(
        &mut self,
    ) -> EnblWrProtOfPric130cpric130c1408W<PricIo30cSpec> {
        EnblWrProtOfPric130cpric130c1408W::new(self, 15)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo30cSpec> {
        Reserved7W::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo30cSpec> {
        Reserved6W::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo30cSpec> {
        Reserved5W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo30cSpec> {
        Reserved4W::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo30cSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo30cSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo30cSpec> {
        Reserved1W::new(self, 22)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_read_group0of_srammemory(&mut self) -> EnblReadGroup0ofSrammemoryW<PricIo30cSpec> {
        EnblReadGroup0ofSrammemoryW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_read_group1of_srammemory(&mut self) -> EnblReadGroup1ofSrammemoryW<PricIo30cSpec> {
        EnblReadGroup1ofSrammemoryW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_read_group2of_srammemory(&mut self) -> EnblReadGroup2ofSrammemoryW<PricIo30cSpec> {
        EnblReadGroup2ofSrammemoryW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_read_group3of_srammemory(&mut self) -> EnblReadGroup3ofSrammemoryW<PricIo30cSpec> {
        EnblReadGroup3ofSrammemoryW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_read_group4of_srammemory(&mut self) -> EnblReadGroup4ofSrammemoryW<PricIo30cSpec> {
        EnblReadGroup4ofSrammemoryW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of SRAM Memory"]
    #[inline(always)]
    pub fn enbl_read_group5of_srammemory(&mut self) -> EnblReadGroup5ofSrammemoryW<PricIo30cSpec> {
        EnblReadGroup5ofSrammemoryW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC130CPRIC1_30C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric130cpric130c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric130cpric130c2924W<PricIo30cSpec> {
        EnblRstToleranceOfPric130cpric130c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC130CPRIC1_30C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric130cpric130c3024(
        &mut self,
    ) -> EnblWrProtOfPric130cpric130c3024W<PricIo30cSpec> {
        EnblWrProtOfPric130cpric130c3024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io30c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io30c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo30cSpec;
impl crate::RegisterSpec for PricIo30cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io30c::R`](R) reader structure"]
impl crate::Readable for PricIo30cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io30c::W`](W) writer structure"]
impl crate::Writable for PricIo30cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO30C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo30cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
