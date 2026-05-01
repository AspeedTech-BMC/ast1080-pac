#[doc = "Register `PRIC_IO32C` reader"]
pub type R = crate::R<PricIo32cSpec>;
#[doc = "Register `PRIC_IO32C` writer"]
pub type W = crate::W<PricIo32cSpec>;
#[doc = "Field `EnblReadGroup0OfOTPC` reader - Enable Read Group #0 of OTPC"]
pub type EnblReadGroup0ofOtpcR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfOTPC` writer - Enable Read Group #0 of OTPC"]
pub type EnblReadGroup0ofOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfOTPC` reader - Enable Read Group #1 of OTPC"]
pub type EnblReadGroup1ofOtpcR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfOTPC` writer - Enable Read Group #1 of OTPC"]
pub type EnblReadGroup1ofOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfOTPC` reader - Enable Read Group #2 of OTPC"]
pub type EnblReadGroup2ofOtpcR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfOTPC` writer - Enable Read Group #2 of OTPC"]
pub type EnblReadGroup2ofOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfOTPC` reader - Enable Read Group #3 of OTPC"]
pub type EnblReadGroup3ofOtpcR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfOTPC` writer - Enable Read Group #3 of OTPC"]
pub type EnblReadGroup3ofOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfOTPC` reader - Enable Read Group #4 of OTPC"]
pub type EnblReadGroup4ofOtpcR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfOTPC` writer - Enable Read Group #4 of OTPC"]
pub type EnblReadGroup4ofOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfOTPC` reader - Enable Read Group #5 of OTPC"]
pub type EnblReadGroup5ofOtpcR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfOTPC` writer - Enable Read Group #5 of OTPC"]
pub type EnblReadGroup5ofOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC132CPRIC1_32C\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric132cpric132c0500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric132cpric132c0500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric132cpric132c0500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC132CPRIC132C0500` reader - Enable Reset Tolerance of PRIC132CPRIC1_32C\\[05:00\\]"]
pub type EnblRstToleranceOfPric132cpric132c0500R =
    crate::BitReader<EnblRstToleranceOfPric132cpric132c0500>;
impl EnblRstToleranceOfPric132cpric132c0500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric132cpric132c0500 {
        match self.bits {
            false => EnblRstToleranceOfPric132cpric132c0500::ResetBySrst,
            true => EnblRstToleranceOfPric132cpric132c0500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric132cpric132c0500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric132cpric132c0500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC132CPRIC132C0500` writer - Enable Reset Tolerance of PRIC132CPRIC1_32C\\[05:00\\]"]
pub type EnblRstToleranceOfPric132cpric132c0500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric132cpric132c0500>;
impl<'a, REG> EnblRstToleranceOfPric132cpric132c0500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric132cpric132c0500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric132cpric132c0500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC132CPRIC132C0600` reader - Enable Write Protection of PRIC132CPRIC1_32C\\[06:00\\]"]
pub type EnblWrProtOfPric132cpric132c0600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC132CPRIC132C0600` writer - Enable Write Protection of PRIC132CPRIC1_32C\\[06:00\\]"]
pub type EnblWrProtOfPric132cpric132c0600W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `EnblReadGroup0OfJTAG1` reader - Enable Read Group #0 of JTAG1"]
pub type EnblReadGroup0ofJtag1R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfJTAG1` writer - Enable Read Group #0 of JTAG1"]
pub type EnblReadGroup0ofJtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfJTAG1` reader - Enable Read Group #1 of JTAG1"]
pub type EnblReadGroup1ofJtag1R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfJTAG1` writer - Enable Read Group #1 of JTAG1"]
pub type EnblReadGroup1ofJtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfJTAG1` reader - Enable Read Group #2 of JTAG1"]
pub type EnblReadGroup2ofJtag1R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfJTAG1` writer - Enable Read Group #2 of JTAG1"]
pub type EnblReadGroup2ofJtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfJTAG1` reader - Enable Read Group #3 of JTAG1"]
pub type EnblReadGroup3ofJtag1R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfJTAG1` writer - Enable Read Group #3 of JTAG1"]
pub type EnblReadGroup3ofJtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfJTAG1` reader - Enable Read Group #4 of JTAG1"]
pub type EnblReadGroup4ofJtag1R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfJTAG1` writer - Enable Read Group #4 of JTAG1"]
pub type EnblReadGroup4ofJtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfJTAG1` reader - Enable Read Group #5 of JTAG1"]
pub type EnblReadGroup5ofJtag1R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfJTAG1` writer - Enable Read Group #5 of JTAG1"]
pub type EnblReadGroup5ofJtag1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC132CPRIC1_32C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric132cpric132c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric132cpric132c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric132cpric132c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC132CPRIC132C2116` reader - Enable Reset Tolerance of PRIC132CPRIC1_32C\\[21:16\\]"]
pub type EnblRstToleranceOfPric132cpric132c2116R =
    crate::BitReader<EnblRstToleranceOfPric132cpric132c2116>;
impl EnblRstToleranceOfPric132cpric132c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric132cpric132c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric132cpric132c2116::ResetBySrst,
            true => EnblRstToleranceOfPric132cpric132c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric132cpric132c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric132cpric132c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC132CPRIC132C2116` writer - Enable Reset Tolerance of PRIC132CPRIC1_32C\\[21:16\\]"]
pub type EnblRstToleranceOfPric132cpric132c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric132cpric132c2116>;
impl<'a, REG> EnblRstToleranceOfPric132cpric132c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric132cpric132c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric132cpric132c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC132CPRIC132C2216` reader - Enable Write Protection of PRIC132CPRIC1_32C\\[22:16\\]"]
pub type EnblWrProtOfPric132cpric132c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC132CPRIC132C2216` writer - Enable Write Protection of PRIC132CPRIC1_32C\\[22:16\\]"]
pub type EnblWrProtOfPric132cpric132c2216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfSRAMReg` reader - Enable Read Group #0 of SRAM Register"]
pub type EnblReadGroup0ofSramregR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSRAMReg` writer - Enable Read Group #0 of SRAM Register"]
pub type EnblReadGroup0ofSramregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSRAMReg` reader - Enable Read Group #1 of SRAM Register"]
pub type EnblReadGroup1ofSramregR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSRAMReg` writer - Enable Read Group #1 of SRAM Register"]
pub type EnblReadGroup1ofSramregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSRAMReg` reader - Enable Read Group #2 of SRAM Register"]
pub type EnblReadGroup2ofSramregR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSRAMReg` writer - Enable Read Group #2 of SRAM Register"]
pub type EnblReadGroup2ofSramregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSRAMReg` reader - Enable Read Group #3 of SRAM Register"]
pub type EnblReadGroup3ofSramregR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSRAMReg` writer - Enable Read Group #3 of SRAM Register"]
pub type EnblReadGroup3ofSramregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSRAMReg` reader - Enable Read Group #4 of SRAM Register"]
pub type EnblReadGroup4ofSramregR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSRAMReg` writer - Enable Read Group #4 of SRAM Register"]
pub type EnblReadGroup4ofSramregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSRAMReg` reader - Enable Read Group #5 of SRAM Register"]
pub type EnblReadGroup5ofSramregR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSRAMReg` writer - Enable Read Group #5 of SRAM Register"]
pub type EnblReadGroup5ofSramregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC132CPRIC1_32C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric132cpric132c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric132cpric132c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric132cpric132c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC132CPRIC132C2924` reader - Enable Reset Tolerance of PRIC132CPRIC1_32C\\[29:24\\]"]
pub type EnblRstToleranceOfPric132cpric132c2924R =
    crate::BitReader<EnblRstToleranceOfPric132cpric132c2924>;
impl EnblRstToleranceOfPric132cpric132c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric132cpric132c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric132cpric132c2924::ResetBySrst,
            true => EnblRstToleranceOfPric132cpric132c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric132cpric132c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric132cpric132c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC132CPRIC132C2924` writer - Enable Reset Tolerance of PRIC132CPRIC1_32C\\[29:24\\]"]
pub type EnblRstToleranceOfPric132cpric132c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric132cpric132c2924>;
impl<'a, REG> EnblRstToleranceOfPric132cpric132c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric132cpric132c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric132cpric132c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC132CPRIC132C3024` reader - Enable Write Protection of PRIC132CPRIC1_32C\\[30:24\\]"]
pub type EnblWrProtOfPric132cpric132c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC132CPRIC132C3024` writer - Enable Write Protection of PRIC132CPRIC1_32C\\[30:24\\]"]
pub type EnblWrProtOfPric132cpric132c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of OTPC"]
    #[inline(always)]
    pub fn enbl_read_group0of_otpc(&self) -> EnblReadGroup0ofOtpcR {
        EnblReadGroup0ofOtpcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of OTPC"]
    #[inline(always)]
    pub fn enbl_read_group1of_otpc(&self) -> EnblReadGroup1ofOtpcR {
        EnblReadGroup1ofOtpcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of OTPC"]
    #[inline(always)]
    pub fn enbl_read_group2of_otpc(&self) -> EnblReadGroup2ofOtpcR {
        EnblReadGroup2ofOtpcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of OTPC"]
    #[inline(always)]
    pub fn enbl_read_group3of_otpc(&self) -> EnblReadGroup3ofOtpcR {
        EnblReadGroup3ofOtpcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of OTPC"]
    #[inline(always)]
    pub fn enbl_read_group4of_otpc(&self) -> EnblReadGroup4ofOtpcR {
        EnblReadGroup4ofOtpcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of OTPC"]
    #[inline(always)]
    pub fn enbl_read_group5of_otpc(&self) -> EnblReadGroup5ofOtpcR {
        EnblReadGroup5ofOtpcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC132CPRIC1_32C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric132cpric132c0500(
        &self,
    ) -> EnblRstToleranceOfPric132cpric132c0500R {
        EnblRstToleranceOfPric132cpric132c0500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC132CPRIC1_32C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric132cpric132c0600(&self) -> EnblWrProtOfPric132cpric132c0600R {
        EnblWrProtOfPric132cpric132c0600R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 16 - Enable Read Group #0 of JTAG1"]
    #[inline(always)]
    pub fn enbl_read_group0of_jtag1(&self) -> EnblReadGroup0ofJtag1R {
        EnblReadGroup0ofJtag1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of JTAG1"]
    #[inline(always)]
    pub fn enbl_read_group1of_jtag1(&self) -> EnblReadGroup1ofJtag1R {
        EnblReadGroup1ofJtag1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of JTAG1"]
    #[inline(always)]
    pub fn enbl_read_group2of_jtag1(&self) -> EnblReadGroup2ofJtag1R {
        EnblReadGroup2ofJtag1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of JTAG1"]
    #[inline(always)]
    pub fn enbl_read_group3of_jtag1(&self) -> EnblReadGroup3ofJtag1R {
        EnblReadGroup3ofJtag1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of JTAG1"]
    #[inline(always)]
    pub fn enbl_read_group4of_jtag1(&self) -> EnblReadGroup4ofJtag1R {
        EnblReadGroup4ofJtag1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of JTAG1"]
    #[inline(always)]
    pub fn enbl_read_group5of_jtag1(&self) -> EnblReadGroup5ofJtag1R {
        EnblReadGroup5ofJtag1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC132CPRIC1_32C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric132cpric132c2116(
        &self,
    ) -> EnblRstToleranceOfPric132cpric132c2116R {
        EnblRstToleranceOfPric132cpric132c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC132CPRIC1_32C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric132cpric132c2216(&self) -> EnblWrProtOfPric132cpric132c2216R {
        EnblWrProtOfPric132cpric132c2216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_read_group0of_sramreg(&self) -> EnblReadGroup0ofSramregR {
        EnblReadGroup0ofSramregR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_read_group1of_sramreg(&self) -> EnblReadGroup1ofSramregR {
        EnblReadGroup1ofSramregR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_read_group2of_sramreg(&self) -> EnblReadGroup2ofSramregR {
        EnblReadGroup2ofSramregR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_read_group3of_sramreg(&self) -> EnblReadGroup3ofSramregR {
        EnblReadGroup3ofSramregR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_read_group4of_sramreg(&self) -> EnblReadGroup4ofSramregR {
        EnblReadGroup4ofSramregR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_read_group5of_sramreg(&self) -> EnblReadGroup5ofSramregR {
        EnblReadGroup5ofSramregR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC132CPRIC1_32C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric132cpric132c2924(
        &self,
    ) -> EnblRstToleranceOfPric132cpric132c2924R {
        EnblRstToleranceOfPric132cpric132c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC132CPRIC1_32C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric132cpric132c3024(&self) -> EnblWrProtOfPric132cpric132c3024R {
        EnblWrProtOfPric132cpric132c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of OTPC"]
    #[inline(always)]
    pub fn enbl_read_group0of_otpc(&mut self) -> EnblReadGroup0ofOtpcW<PricIo32cSpec> {
        EnblReadGroup0ofOtpcW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of OTPC"]
    #[inline(always)]
    pub fn enbl_read_group1of_otpc(&mut self) -> EnblReadGroup1ofOtpcW<PricIo32cSpec> {
        EnblReadGroup1ofOtpcW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of OTPC"]
    #[inline(always)]
    pub fn enbl_read_group2of_otpc(&mut self) -> EnblReadGroup2ofOtpcW<PricIo32cSpec> {
        EnblReadGroup2ofOtpcW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of OTPC"]
    #[inline(always)]
    pub fn enbl_read_group3of_otpc(&mut self) -> EnblReadGroup3ofOtpcW<PricIo32cSpec> {
        EnblReadGroup3ofOtpcW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of OTPC"]
    #[inline(always)]
    pub fn enbl_read_group4of_otpc(&mut self) -> EnblReadGroup4ofOtpcW<PricIo32cSpec> {
        EnblReadGroup4ofOtpcW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of OTPC"]
    #[inline(always)]
    pub fn enbl_read_group5of_otpc(&mut self) -> EnblReadGroup5ofOtpcW<PricIo32cSpec> {
        EnblReadGroup5ofOtpcW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC132CPRIC1_32C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric132cpric132c0500(
        &mut self,
    ) -> EnblRstToleranceOfPric132cpric132c0500W<PricIo32cSpec> {
        EnblRstToleranceOfPric132cpric132c0500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC132CPRIC1_32C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric132cpric132c0600(
        &mut self,
    ) -> EnblWrProtOfPric132cpric132c0600W<PricIo32cSpec> {
        EnblWrProtOfPric132cpric132c0600W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo32cSpec> {
        Reserved7W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo32cSpec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo32cSpec> {
        Reserved5W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo32cSpec> {
        Reserved4W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo32cSpec> {
        Reserved3W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo32cSpec> {
        Reserved2W::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo32cSpec> {
        Reserved1W::new(self, 14)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of JTAG1"]
    #[inline(always)]
    pub fn enbl_read_group0of_jtag1(&mut self) -> EnblReadGroup0ofJtag1W<PricIo32cSpec> {
        EnblReadGroup0ofJtag1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of JTAG1"]
    #[inline(always)]
    pub fn enbl_read_group1of_jtag1(&mut self) -> EnblReadGroup1ofJtag1W<PricIo32cSpec> {
        EnblReadGroup1ofJtag1W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of JTAG1"]
    #[inline(always)]
    pub fn enbl_read_group2of_jtag1(&mut self) -> EnblReadGroup2ofJtag1W<PricIo32cSpec> {
        EnblReadGroup2ofJtag1W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of JTAG1"]
    #[inline(always)]
    pub fn enbl_read_group3of_jtag1(&mut self) -> EnblReadGroup3ofJtag1W<PricIo32cSpec> {
        EnblReadGroup3ofJtag1W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of JTAG1"]
    #[inline(always)]
    pub fn enbl_read_group4of_jtag1(&mut self) -> EnblReadGroup4ofJtag1W<PricIo32cSpec> {
        EnblReadGroup4ofJtag1W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of JTAG1"]
    #[inline(always)]
    pub fn enbl_read_group5of_jtag1(&mut self) -> EnblReadGroup5ofJtag1W<PricIo32cSpec> {
        EnblReadGroup5ofJtag1W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC132CPRIC1_32C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric132cpric132c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric132cpric132c2116W<PricIo32cSpec> {
        EnblRstToleranceOfPric132cpric132c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC132CPRIC1_32C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric132cpric132c2216(
        &mut self,
    ) -> EnblWrProtOfPric132cpric132c2216W<PricIo32cSpec> {
        EnblWrProtOfPric132cpric132c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_read_group0of_sramreg(&mut self) -> EnblReadGroup0ofSramregW<PricIo32cSpec> {
        EnblReadGroup0ofSramregW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_read_group1of_sramreg(&mut self) -> EnblReadGroup1ofSramregW<PricIo32cSpec> {
        EnblReadGroup1ofSramregW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_read_group2of_sramreg(&mut self) -> EnblReadGroup2ofSramregW<PricIo32cSpec> {
        EnblReadGroup2ofSramregW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_read_group3of_sramreg(&mut self) -> EnblReadGroup3ofSramregW<PricIo32cSpec> {
        EnblReadGroup3ofSramregW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_read_group4of_sramreg(&mut self) -> EnblReadGroup4ofSramregW<PricIo32cSpec> {
        EnblReadGroup4ofSramregW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of SRAM Register"]
    #[inline(always)]
    pub fn enbl_read_group5of_sramreg(&mut self) -> EnblReadGroup5ofSramregW<PricIo32cSpec> {
        EnblReadGroup5ofSramregW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC132CPRIC1_32C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric132cpric132c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric132cpric132c2924W<PricIo32cSpec> {
        EnblRstToleranceOfPric132cpric132c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC132CPRIC1_32C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric132cpric132c3024(
        &mut self,
    ) -> EnblWrProtOfPric132cpric132c3024W<PricIo32cSpec> {
        EnblWrProtOfPric132cpric132c3024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io32c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io32c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo32cSpec;
impl crate::RegisterSpec for PricIo32cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io32c::R`](R) reader structure"]
impl crate::Readable for PricIo32cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io32c::W`](W) writer structure"]
impl crate::Writable for PricIo32cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO32C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo32cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
