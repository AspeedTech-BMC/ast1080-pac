#[doc = "Register `PRIC_IO22C` reader"]
pub type R = crate::R<PricIo22cSpec>;
#[doc = "Register `PRIC_IO22C` writer"]
pub type W = crate::W<PricIo22cSpec>;
#[doc = "Field `EnblWrGroup0OfOTPC` reader - Enable Write Group #0 of OTPC"]
pub type EnblWrGroup0ofOtpcR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfOTPC` writer - Enable Write Group #0 of OTPC"]
pub type EnblWrGroup0ofOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfOTPC` reader - Enable Write Group #1 of OTPC"]
pub type EnblWrGroup1ofOtpcR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfOTPC` writer - Enable Write Group #1 of OTPC"]
pub type EnblWrGroup1ofOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfOTPC` reader - Enable Write Group #2 of OTPC"]
pub type EnblWrGroup2ofOtpcR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfOTPC` writer - Enable Write Group #2 of OTPC"]
pub type EnblWrGroup2ofOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfOTPC` reader - Enable Write Group #3 of OTPC"]
pub type EnblWrGroup3ofOtpcR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfOTPC` writer - Enable Write Group #3 of OTPC"]
pub type EnblWrGroup3ofOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfOTPC` reader - Enable Write Group #4 of OTPC"]
pub type EnblWrGroup4ofOtpcR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfOTPC` writer - Enable Write Group #4 of OTPC"]
pub type EnblWrGroup4ofOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfOTPC` reader - Enable Write Group #5 of OTPC"]
pub type EnblWrGroup5ofOtpcR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfOTPC` writer - Enable Write Group #5 of OTPC"]
pub type EnblWrGroup5ofOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC122CPRIC1_22C\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric122cpric122c0500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric122cpric122c0500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric122cpric122c0500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC122CPRIC122C0500` reader - Enable Reset Tolerance of PRIC122CPRIC1_22C\\[05:00\\]"]
pub type EnblRstToleranceOfPric122cpric122c0500R =
    crate::BitReader<EnblRstToleranceOfPric122cpric122c0500>;
impl EnblRstToleranceOfPric122cpric122c0500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric122cpric122c0500 {
        match self.bits {
            false => EnblRstToleranceOfPric122cpric122c0500::ResetBySrst,
            true => EnblRstToleranceOfPric122cpric122c0500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric122cpric122c0500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric122cpric122c0500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC122CPRIC122C0500` writer - Enable Reset Tolerance of PRIC122CPRIC1_22C\\[05:00\\]"]
pub type EnblRstToleranceOfPric122cpric122c0500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric122cpric122c0500>;
impl<'a, REG> EnblRstToleranceOfPric122cpric122c0500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric122cpric122c0500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric122cpric122c0500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC122CPRIC122C0600` reader - Enable Write Protection of PRIC122CPRIC1_22C\\[06:00\\]"]
pub type EnblWrProtOfPric122cpric122c0600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC122CPRIC122C0600` writer - Enable Write Protection of PRIC122CPRIC1_22C\\[06:00\\]"]
pub type EnblWrProtOfPric122cpric122c0600W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `EnblWrGroup0OfJTAG1` reader - Enable Write Group #0 of JTAG1"]
pub type EnblWrGroup0ofJtag1R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfJTAG1` writer - Enable Write Group #0 of JTAG1"]
pub type EnblWrGroup0ofJtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfJTAG1` reader - Enable Write Group #1 of JTAG1"]
pub type EnblWrGroup1ofJtag1R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfJTAG1` writer - Enable Write Group #1 of JTAG1"]
pub type EnblWrGroup1ofJtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfJTAG1` reader - Enable Write Group #2 of JTAG1"]
pub type EnblWrGroup2ofJtag1R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfJTAG1` writer - Enable Write Group #2 of JTAG1"]
pub type EnblWrGroup2ofJtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfJTAG1` reader - Enable Write Group #3 of JTAG1"]
pub type EnblWrGroup3ofJtag1R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfJTAG1` writer - Enable Write Group #3 of JTAG1"]
pub type EnblWrGroup3ofJtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfJTAG1` reader - Enable Write Group #4 of JTAG1"]
pub type EnblWrGroup4ofJtag1R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfJTAG1` writer - Enable Write Group #4 of JTAG1"]
pub type EnblWrGroup4ofJtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfJTAG1` reader - Enable Write Group #5 of JTAG1"]
pub type EnblWrGroup5ofJtag1R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfJTAG1` writer - Enable Write Group #5 of JTAG1"]
pub type EnblWrGroup5ofJtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC122CPRIC1_22C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric122cpric122c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric122cpric122c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric122cpric122c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC122CPRIC122C2116` reader - Enable Reset Tolerance of PRIC122CPRIC1_22C\\[21:16\\]"]
pub type EnblRstToleranceOfPric122cpric122c2116R =
    crate::BitReader<EnblRstToleranceOfPric122cpric122c2116>;
impl EnblRstToleranceOfPric122cpric122c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric122cpric122c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric122cpric122c2116::ResetBySrst,
            true => EnblRstToleranceOfPric122cpric122c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric122cpric122c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric122cpric122c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC122CPRIC122C2116` writer - Enable Reset Tolerance of PRIC122CPRIC1_22C\\[21:16\\]"]
pub type EnblRstToleranceOfPric122cpric122c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric122cpric122c2116>;
impl<'a, REG> EnblRstToleranceOfPric122cpric122c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric122cpric122c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric122cpric122c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC122CPRIC122C2216` reader - Enable Write Protection of PRIC122CPRIC1_22C\\[22:16\\]"]
pub type EnblWrProtOfPric122cpric122c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC122CPRIC122C2216` writer - Enable Write Protection of PRIC122CPRIC1_22C\\[22:16\\]"]
pub type EnblWrProtOfPric122cpric122c2216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSRAMReg` reader - Enable Write Group #0 of SRAM Register"]
pub type EnblWrGroup0ofSramregR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSRAMReg` writer - Enable Write Group #0 of SRAM Register"]
pub type EnblWrGroup0ofSramregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSRAMReg` reader - Enable Write Group #1 of SRAM Register"]
pub type EnblWrGroup1ofSramregR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSRAMReg` writer - Enable Write Group #1 of SRAM Register"]
pub type EnblWrGroup1ofSramregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSRAMReg` reader - Enable Write Group #2 of SRAM Register"]
pub type EnblWrGroup2ofSramregR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSRAMReg` writer - Enable Write Group #2 of SRAM Register"]
pub type EnblWrGroup2ofSramregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSRAMReg` reader - Enable Write Group #3 of SRAM Register"]
pub type EnblWrGroup3ofSramregR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSRAMReg` writer - Enable Write Group #3 of SRAM Register"]
pub type EnblWrGroup3ofSramregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSRAMReg` reader - Enable Write Group #4 of SRAM Register"]
pub type EnblWrGroup4ofSramregR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSRAMReg` writer - Enable Write Group #4 of SRAM Register"]
pub type EnblWrGroup4ofSramregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSRAMReg` reader - Enable Write Group #5 of SRAM Register"]
pub type EnblWrGroup5ofSramregR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSRAMReg` writer - Enable Write Group #5 of SRAM Register"]
pub type EnblWrGroup5ofSramregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC122CPRIC1_22C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric122cpric122c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric122cpric122c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric122cpric122c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC122CPRIC122C2924` reader - Enable Reset Tolerance of PRIC122CPRIC1_22C\\[29:24\\]"]
pub type EnblRstToleranceOfPric122cpric122c2924R =
    crate::BitReader<EnblRstToleranceOfPric122cpric122c2924>;
impl EnblRstToleranceOfPric122cpric122c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric122cpric122c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric122cpric122c2924::ResetBySrst,
            true => EnblRstToleranceOfPric122cpric122c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric122cpric122c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric122cpric122c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC122CPRIC122C2924` writer - Enable Reset Tolerance of PRIC122CPRIC1_22C\\[29:24\\]"]
pub type EnblRstToleranceOfPric122cpric122c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric122cpric122c2924>;
impl<'a, REG> EnblRstToleranceOfPric122cpric122c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric122cpric122c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric122cpric122c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC122CPRIC122C3024` reader - Enable Write Protection of PRIC122CPRIC1_22C\\[30:24\\]"]
pub type EnblWrProtOfPric122cpric122c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC122CPRIC122C3024` writer - Enable Write Protection of PRIC122CPRIC1_22C\\[30:24\\]"]
pub type EnblWrProtOfPric122cpric122c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of OTPC"]
    #[inline(always)]
    pub fn enbl_wr_group0of_otpc(&self) -> EnblWrGroup0ofOtpcR {
        EnblWrGroup0ofOtpcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of OTPC"]
    #[inline(always)]
    pub fn enbl_wr_group1of_otpc(&self) -> EnblWrGroup1ofOtpcR {
        EnblWrGroup1ofOtpcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of OTPC"]
    #[inline(always)]
    pub fn enbl_wr_group2of_otpc(&self) -> EnblWrGroup2ofOtpcR {
        EnblWrGroup2ofOtpcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of OTPC"]
    #[inline(always)]
    pub fn enbl_wr_group3of_otpc(&self) -> EnblWrGroup3ofOtpcR {
        EnblWrGroup3ofOtpcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of OTPC"]
    #[inline(always)]
    pub fn enbl_wr_group4of_otpc(&self) -> EnblWrGroup4ofOtpcR {
        EnblWrGroup4ofOtpcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of OTPC"]
    #[inline(always)]
    pub fn enbl_wr_group5of_otpc(&self) -> EnblWrGroup5ofOtpcR {
        EnblWrGroup5ofOtpcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC122CPRIC1_22C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric122cpric122c0500(
        &self,
    ) -> EnblRstToleranceOfPric122cpric122c0500R {
        EnblRstToleranceOfPric122cpric122c0500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC122CPRIC1_22C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric122cpric122c0600(&self) -> EnblWrProtOfPric122cpric122c0600R {
        EnblWrProtOfPric122cpric122c0600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of JTAG1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_jtag1(&self) -> EnblWrGroup0ofJtag1R {
        EnblWrGroup0ofJtag1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of JTAG1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_jtag1(&self) -> EnblWrGroup1ofJtag1R {
        EnblWrGroup1ofJtag1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of JTAG1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_jtag1(&self) -> EnblWrGroup2ofJtag1R {
        EnblWrGroup2ofJtag1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of JTAG1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_jtag1(&self) -> EnblWrGroup3ofJtag1R {
        EnblWrGroup3ofJtag1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of JTAG1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_jtag1(&self) -> EnblWrGroup4ofJtag1R {
        EnblWrGroup4ofJtag1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of JTAG1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_jtag1(&self) -> EnblWrGroup5ofJtag1R {
        EnblWrGroup5ofJtag1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC122CPRIC1_22C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric122cpric122c2116(
        &self,
    ) -> EnblRstToleranceOfPric122cpric122c2116R {
        EnblRstToleranceOfPric122cpric122c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC122CPRIC1_22C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric122cpric122c2216(&self) -> EnblWrProtOfPric122cpric122c2216R {
        EnblWrProtOfPric122cpric122c2216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sramreg(&self) -> EnblWrGroup0ofSramregR {
        EnblWrGroup0ofSramregR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sramreg(&self) -> EnblWrGroup1ofSramregR {
        EnblWrGroup1ofSramregR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sramreg(&self) -> EnblWrGroup2ofSramregR {
        EnblWrGroup2ofSramregR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sramreg(&self) -> EnblWrGroup3ofSramregR {
        EnblWrGroup3ofSramregR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sramreg(&self) -> EnblWrGroup4ofSramregR {
        EnblWrGroup4ofSramregR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sramreg(&self) -> EnblWrGroup5ofSramregR {
        EnblWrGroup5ofSramregR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC122CPRIC1_22C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric122cpric122c2924(
        &self,
    ) -> EnblRstToleranceOfPric122cpric122c2924R {
        EnblRstToleranceOfPric122cpric122c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC122CPRIC1_22C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric122cpric122c3024(&self) -> EnblWrProtOfPric122cpric122c3024R {
        EnblWrProtOfPric122cpric122c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of OTPC"]
    #[inline(always)]
    pub fn enbl_wr_group0of_otpc(&mut self) -> EnblWrGroup0ofOtpcW<PricIo22cSpec> {
        EnblWrGroup0ofOtpcW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of OTPC"]
    #[inline(always)]
    pub fn enbl_wr_group1of_otpc(&mut self) -> EnblWrGroup1ofOtpcW<PricIo22cSpec> {
        EnblWrGroup1ofOtpcW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of OTPC"]
    #[inline(always)]
    pub fn enbl_wr_group2of_otpc(&mut self) -> EnblWrGroup2ofOtpcW<PricIo22cSpec> {
        EnblWrGroup2ofOtpcW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of OTPC"]
    #[inline(always)]
    pub fn enbl_wr_group3of_otpc(&mut self) -> EnblWrGroup3ofOtpcW<PricIo22cSpec> {
        EnblWrGroup3ofOtpcW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of OTPC"]
    #[inline(always)]
    pub fn enbl_wr_group4of_otpc(&mut self) -> EnblWrGroup4ofOtpcW<PricIo22cSpec> {
        EnblWrGroup4ofOtpcW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of OTPC"]
    #[inline(always)]
    pub fn enbl_wr_group5of_otpc(&mut self) -> EnblWrGroup5ofOtpcW<PricIo22cSpec> {
        EnblWrGroup5ofOtpcW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC122CPRIC1_22C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric122cpric122c0500(
        &mut self,
    ) -> EnblRstToleranceOfPric122cpric122c0500W<PricIo22cSpec> {
        EnblRstToleranceOfPric122cpric122c0500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC122CPRIC1_22C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric122cpric122c0600(
        &mut self,
    ) -> EnblWrProtOfPric122cpric122c0600W<PricIo22cSpec> {
        EnblWrProtOfPric122cpric122c0600W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo22cSpec> {
        Reserved7W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo22cSpec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo22cSpec> {
        Reserved5W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo22cSpec> {
        Reserved4W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo22cSpec> {
        Reserved3W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo22cSpec> {
        Reserved2W::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo22cSpec> {
        Reserved1W::new(self, 14)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of JTAG1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_jtag1(&mut self) -> EnblWrGroup0ofJtag1W<PricIo22cSpec> {
        EnblWrGroup0ofJtag1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of JTAG1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_jtag1(&mut self) -> EnblWrGroup1ofJtag1W<PricIo22cSpec> {
        EnblWrGroup1ofJtag1W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of JTAG1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_jtag1(&mut self) -> EnblWrGroup2ofJtag1W<PricIo22cSpec> {
        EnblWrGroup2ofJtag1W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of JTAG1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_jtag1(&mut self) -> EnblWrGroup3ofJtag1W<PricIo22cSpec> {
        EnblWrGroup3ofJtag1W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of JTAG1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_jtag1(&mut self) -> EnblWrGroup4ofJtag1W<PricIo22cSpec> {
        EnblWrGroup4ofJtag1W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of JTAG1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_jtag1(&mut self) -> EnblWrGroup5ofJtag1W<PricIo22cSpec> {
        EnblWrGroup5ofJtag1W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC122CPRIC1_22C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric122cpric122c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric122cpric122c2116W<PricIo22cSpec> {
        EnblRstToleranceOfPric122cpric122c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC122CPRIC1_22C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric122cpric122c2216(
        &mut self,
    ) -> EnblWrProtOfPric122cpric122c2216W<PricIo22cSpec> {
        EnblWrProtOfPric122cpric122c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sramreg(&mut self) -> EnblWrGroup0ofSramregW<PricIo22cSpec> {
        EnblWrGroup0ofSramregW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sramreg(&mut self) -> EnblWrGroup1ofSramregW<PricIo22cSpec> {
        EnblWrGroup1ofSramregW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sramreg(&mut self) -> EnblWrGroup2ofSramregW<PricIo22cSpec> {
        EnblWrGroup2ofSramregW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sramreg(&mut self) -> EnblWrGroup3ofSramregW<PricIo22cSpec> {
        EnblWrGroup3ofSramregW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sramreg(&mut self) -> EnblWrGroup4ofSramregW<PricIo22cSpec> {
        EnblWrGroup4ofSramregW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sramreg(&mut self) -> EnblWrGroup5ofSramregW<PricIo22cSpec> {
        EnblWrGroup5ofSramregW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC122CPRIC1_22C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric122cpric122c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric122cpric122c2924W<PricIo22cSpec> {
        EnblRstToleranceOfPric122cpric122c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC122CPRIC1_22C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric122cpric122c3024(
        &mut self,
    ) -> EnblWrProtOfPric122cpric122c3024W<PricIo22cSpec> {
        EnblWrProtOfPric122cpric122c3024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io22c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io22c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo22cSpec;
impl crate::RegisterSpec for PricIo22cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io22c::R`](R) reader structure"]
impl crate::Readable for PricIo22cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io22c::W`](W) writer structure"]
impl crate::Writable for PricIo22cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO22C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo22cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
