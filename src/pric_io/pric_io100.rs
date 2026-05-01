#[doc = "Register `PRIC_IO100` reader"]
pub type R = crate::R<PricIo100Spec>;
#[doc = "Register `PRIC_IO100` writer"]
pub type W = crate::W<PricIo100Spec>;
#[doc = "Field `EnblReadGroup0OfCaliptraSSMCUAccess` reader - Enable Read Group #0 of Caliptra SS MCU access"]
pub type EnblReadGroup0ofCaliptraSsmcuaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfCaliptraSSMCUAccess` writer - Enable Read Group #0 of Caliptra SS MCU access"]
pub type EnblReadGroup0ofCaliptraSsmcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfCaliptraSSMCUAccess` reader - Enable Read Group #1 of Caliptra SS MCU access"]
pub type EnblReadGroup1ofCaliptraSsmcuaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfCaliptraSSMCUAccess` writer - Enable Read Group #1 of Caliptra SS MCU access"]
pub type EnblReadGroup1ofCaliptraSsmcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfCaliptraSSMCUAccess` reader - Enable Read Group #2 of Caliptra SS MCU access"]
pub type EnblReadGroup2ofCaliptraSsmcuaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfCaliptraSSMCUAccess` writer - Enable Read Group #2 of Caliptra SS MCU access"]
pub type EnblReadGroup2ofCaliptraSsmcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfCaliptraSSMCUAccess` reader - Enable Read Group #3 of Caliptra SS MCU access"]
pub type EnblReadGroup3ofCaliptraSsmcuaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfCaliptraSSMCUAccess` writer - Enable Read Group #3 of Caliptra SS MCU access"]
pub type EnblReadGroup3ofCaliptraSsmcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfCaliptraSSMCUAccess` reader - Enable Read Group #4 of Caliptra SS MCU access"]
pub type EnblReadGroup4ofCaliptraSsmcuaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfCaliptraSSMCUAccess` writer - Enable Read Group #4 of Caliptra SS MCU access"]
pub type EnblReadGroup4ofCaliptraSsmcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfCaliptraSSMCUAccess` reader - Enable Read Group #5 of Caliptra SS MCU access"]
pub type EnblReadGroup5ofCaliptraSsmcuaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfCaliptraSSMCUAccess` writer - Enable Read Group #5 of Caliptra SS MCU access"]
pub type EnblReadGroup5ofCaliptraSsmcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1100PRIC1_100\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1100pric11000500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1100pric11000500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1100pric11000500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1100PRIC11000500` reader - Enable Reset Tolerance of PRIC1100PRIC1_100\\[05:00\\]"]
pub type EnblRstToleranceOfPric1100pric11000500R =
    crate::BitReader<EnblRstToleranceOfPric1100pric11000500>;
impl EnblRstToleranceOfPric1100pric11000500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1100pric11000500 {
        match self.bits {
            false => EnblRstToleranceOfPric1100pric11000500::ResetBySrst,
            true => EnblRstToleranceOfPric1100pric11000500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1100pric11000500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1100pric11000500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1100PRIC11000500` writer - Enable Reset Tolerance of PRIC1100PRIC1_100\\[05:00\\]"]
pub type EnblRstToleranceOfPric1100pric11000500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1100pric11000500>;
impl<'a, REG> EnblRstToleranceOfPric1100pric11000500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1100pric11000500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1100pric11000500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1100PRIC11000600` reader - Enable Write Protection of PRIC1100PRIC1_100\\[06:00\\]"]
pub type EnblWrProtOfPric1100pric11000600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1100PRIC11000600` writer - Enable Write Protection of PRIC1100PRIC1_100\\[06:00\\]"]
pub type EnblWrProtOfPric1100pric11000600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfCaliptraMCUAccess` reader - Enable Read Group #0 of Caliptra MCU access"]
pub type EnblReadGroup0ofCaliptraMcuaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfCaliptraMCUAccess` writer - Enable Read Group #0 of Caliptra MCU access"]
pub type EnblReadGroup0ofCaliptraMcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfCaliptraMCUAccess` reader - Enable Read Group #1 of Caliptra MCU access"]
pub type EnblReadGroup1ofCaliptraMcuaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfCaliptraMCUAccess` writer - Enable Read Group #1 of Caliptra MCU access"]
pub type EnblReadGroup1ofCaliptraMcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfCaliptraMCUAccess` reader - Enable Read Group #2 of Caliptra MCU access"]
pub type EnblReadGroup2ofCaliptraMcuaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfCaliptraMCUAccess` writer - Enable Read Group #2 of Caliptra MCU access"]
pub type EnblReadGroup2ofCaliptraMcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfCaliptraMCUAccess` reader - Enable Read Group #3 of Caliptra MCU access"]
pub type EnblReadGroup3ofCaliptraMcuaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfCaliptraMCUAccess` writer - Enable Read Group #3 of Caliptra MCU access"]
pub type EnblReadGroup3ofCaliptraMcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfCaliptraMCUAccess` reader - Enable Read Group #4 of Caliptra MCU access"]
pub type EnblReadGroup4ofCaliptraMcuaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfCaliptraMCUAccess` writer - Enable Read Group #4 of Caliptra MCU access"]
pub type EnblReadGroup4ofCaliptraMcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfCaliptraMCUAccess` reader - Enable Read Group #5 of Caliptra MCU access"]
pub type EnblReadGroup5ofCaliptraMcuaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfCaliptraMCUAccess` writer - Enable Read Group #5 of Caliptra MCU access"]
pub type EnblReadGroup5ofCaliptraMcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1100PRIC1_100\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1100pric11001308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1100pric11001308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1100pric11001308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1100PRIC11001308` reader - Enable Reset Tolerance of PRIC1100PRIC1_100\\[13:08\\]"]
pub type EnblRstToleranceOfPric1100pric11001308R =
    crate::BitReader<EnblRstToleranceOfPric1100pric11001308>;
impl EnblRstToleranceOfPric1100pric11001308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1100pric11001308 {
        match self.bits {
            false => EnblRstToleranceOfPric1100pric11001308::ResetBySrst,
            true => EnblRstToleranceOfPric1100pric11001308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1100pric11001308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1100pric11001308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1100PRIC11001308` writer - Enable Reset Tolerance of PRIC1100PRIC1_100\\[13:08\\]"]
pub type EnblRstToleranceOfPric1100pric11001308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1100pric11001308>;
impl<'a, REG> EnblRstToleranceOfPric1100pric11001308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1100pric11001308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1100pric11001308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1100PRIC11001408` reader - Enable Write Protection of PRIC1100PRIC1_100\\[14:08\\]"]
pub type EnblWrProtOfPric1100pric11001408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1100PRIC11001408` writer - Enable Write Protection of PRIC1100PRIC1_100\\[14:08\\]"]
pub type EnblWrProtOfPric1100pric11001408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfPSPSAccess` reader - Enable Read Group #0 of PSP-S access"]
pub type EnblReadGroup0ofPspsaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfPSPSAccess` writer - Enable Read Group #0 of PSP-S access"]
pub type EnblReadGroup0ofPspsaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfPSPSAccess` reader - Enable Read Group #1 of PSP-S access"]
pub type EnblReadGroup1ofPspsaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfPSPSAccess` writer - Enable Read Group #1 of PSP-S access"]
pub type EnblReadGroup1ofPspsaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfPSPSAccess` reader - Enable Read Group #2 of PSP-S access"]
pub type EnblReadGroup2ofPspsaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfPSPSAccess` writer - Enable Read Group #2 of PSP-S access"]
pub type EnblReadGroup2ofPspsaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfPSPSAccess` reader - Enable Read Group #3 of PSP-S access"]
pub type EnblReadGroup3ofPspsaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfPSPSAccess` writer - Enable Read Group #3 of PSP-S access"]
pub type EnblReadGroup3ofPspsaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfPSPSAccess` reader - Enable Read Group #4 of PSP-S access"]
pub type EnblReadGroup4ofPspsaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfPSPSAccess` writer - Enable Read Group #4 of PSP-S access"]
pub type EnblReadGroup4ofPspsaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfPSPSAccess` reader - Enable Read Group #5 of PSP-S access"]
pub type EnblReadGroup5ofPspsaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfPSPSAccess` writer - Enable Read Group #5 of PSP-S access"]
pub type EnblReadGroup5ofPspsaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1100PRIC1_100\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1100pric11002116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1100pric11002116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1100pric11002116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1100PRIC11002116` reader - Enable Reset Tolerance of PRIC1100PRIC1_100\\[21:16\\]"]
pub type EnblRstToleranceOfPric1100pric11002116R =
    crate::BitReader<EnblRstToleranceOfPric1100pric11002116>;
impl EnblRstToleranceOfPric1100pric11002116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1100pric11002116 {
        match self.bits {
            false => EnblRstToleranceOfPric1100pric11002116::ResetBySrst,
            true => EnblRstToleranceOfPric1100pric11002116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1100pric11002116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1100pric11002116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1100PRIC11002116` writer - Enable Reset Tolerance of PRIC1100PRIC1_100\\[21:16\\]"]
pub type EnblRstToleranceOfPric1100pric11002116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1100pric11002116>;
impl<'a, REG> EnblRstToleranceOfPric1100pric11002116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1100pric11002116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1100pric11002116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1100PRIC11002216` reader - Enable Write Protection of PRIC1100PRIC1_100\\[22:16\\]"]
pub type EnblWrProtOfPric1100pric11002216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1100PRIC11002216` writer - Enable Write Protection of PRIC1100PRIC1_100\\[22:16\\]"]
pub type EnblWrProtOfPric1100pric11002216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_read_group0of_caliptra_ssmcuaccess(&self) -> EnblReadGroup0ofCaliptraSsmcuaccessR {
        EnblReadGroup0ofCaliptraSsmcuaccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_read_group1of_caliptra_ssmcuaccess(&self) -> EnblReadGroup1ofCaliptraSsmcuaccessR {
        EnblReadGroup1ofCaliptraSsmcuaccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_read_group2of_caliptra_ssmcuaccess(&self) -> EnblReadGroup2ofCaliptraSsmcuaccessR {
        EnblReadGroup2ofCaliptraSsmcuaccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_read_group3of_caliptra_ssmcuaccess(&self) -> EnblReadGroup3ofCaliptraSsmcuaccessR {
        EnblReadGroup3ofCaliptraSsmcuaccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_read_group4of_caliptra_ssmcuaccess(&self) -> EnblReadGroup4ofCaliptraSsmcuaccessR {
        EnblReadGroup4ofCaliptraSsmcuaccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_read_group5of_caliptra_ssmcuaccess(&self) -> EnblReadGroup5ofCaliptraSsmcuaccessR {
        EnblReadGroup5ofCaliptraSsmcuaccessR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1100PRIC1_100\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1100pric11000500(
        &self,
    ) -> EnblRstToleranceOfPric1100pric11000500R {
        EnblRstToleranceOfPric1100pric11000500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1100PRIC1_100\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1100pric11000600(&self) -> EnblWrProtOfPric1100pric11000600R {
        EnblWrProtOfPric1100pric11000600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_read_group0of_caliptra_mcuaccess(&self) -> EnblReadGroup0ofCaliptraMcuaccessR {
        EnblReadGroup0ofCaliptraMcuaccessR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_read_group1of_caliptra_mcuaccess(&self) -> EnblReadGroup1ofCaliptraMcuaccessR {
        EnblReadGroup1ofCaliptraMcuaccessR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_read_group2of_caliptra_mcuaccess(&self) -> EnblReadGroup2ofCaliptraMcuaccessR {
        EnblReadGroup2ofCaliptraMcuaccessR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_read_group3of_caliptra_mcuaccess(&self) -> EnblReadGroup3ofCaliptraMcuaccessR {
        EnblReadGroup3ofCaliptraMcuaccessR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_read_group4of_caliptra_mcuaccess(&self) -> EnblReadGroup4ofCaliptraMcuaccessR {
        EnblReadGroup4ofCaliptraMcuaccessR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_read_group5of_caliptra_mcuaccess(&self) -> EnblReadGroup5ofCaliptraMcuaccessR {
        EnblReadGroup5ofCaliptraMcuaccessR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1100PRIC1_100\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1100pric11001308(
        &self,
    ) -> EnblRstToleranceOfPric1100pric11001308R {
        EnblRstToleranceOfPric1100pric11001308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1100PRIC1_100\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1100pric11001408(&self) -> EnblWrProtOfPric1100pric11001408R {
        EnblWrProtOfPric1100pric11001408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_read_group0of_pspsaccess(&self) -> EnblReadGroup0ofPspsaccessR {
        EnblReadGroup0ofPspsaccessR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_read_group1of_pspsaccess(&self) -> EnblReadGroup1ofPspsaccessR {
        EnblReadGroup1ofPspsaccessR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_read_group2of_pspsaccess(&self) -> EnblReadGroup2ofPspsaccessR {
        EnblReadGroup2ofPspsaccessR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_read_group3of_pspsaccess(&self) -> EnblReadGroup3ofPspsaccessR {
        EnblReadGroup3ofPspsaccessR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_read_group4of_pspsaccess(&self) -> EnblReadGroup4ofPspsaccessR {
        EnblReadGroup4ofPspsaccessR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_read_group5of_pspsaccess(&self) -> EnblReadGroup5ofPspsaccessR {
        EnblReadGroup5ofPspsaccessR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1100PRIC1_100\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1100pric11002116(
        &self,
    ) -> EnblRstToleranceOfPric1100pric11002116R {
        EnblRstToleranceOfPric1100pric11002116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1100PRIC1_100\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1100pric11002216(&self) -> EnblWrProtOfPric1100pric11002216R {
        EnblWrProtOfPric1100pric11002216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_read_group0of_caliptra_ssmcuaccess(
        &mut self,
    ) -> EnblReadGroup0ofCaliptraSsmcuaccessW<PricIo100Spec> {
        EnblReadGroup0ofCaliptraSsmcuaccessW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_read_group1of_caliptra_ssmcuaccess(
        &mut self,
    ) -> EnblReadGroup1ofCaliptraSsmcuaccessW<PricIo100Spec> {
        EnblReadGroup1ofCaliptraSsmcuaccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_read_group2of_caliptra_ssmcuaccess(
        &mut self,
    ) -> EnblReadGroup2ofCaliptraSsmcuaccessW<PricIo100Spec> {
        EnblReadGroup2ofCaliptraSsmcuaccessW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_read_group3of_caliptra_ssmcuaccess(
        &mut self,
    ) -> EnblReadGroup3ofCaliptraSsmcuaccessW<PricIo100Spec> {
        EnblReadGroup3ofCaliptraSsmcuaccessW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_read_group4of_caliptra_ssmcuaccess(
        &mut self,
    ) -> EnblReadGroup4ofCaliptraSsmcuaccessW<PricIo100Spec> {
        EnblReadGroup4ofCaliptraSsmcuaccessW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_read_group5of_caliptra_ssmcuaccess(
        &mut self,
    ) -> EnblReadGroup5ofCaliptraSsmcuaccessW<PricIo100Spec> {
        EnblReadGroup5ofCaliptraSsmcuaccessW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1100PRIC1_100\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1100pric11000500(
        &mut self,
    ) -> EnblRstToleranceOfPric1100pric11000500W<PricIo100Spec> {
        EnblRstToleranceOfPric1100pric11000500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1100PRIC1_100\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1100pric11000600(
        &mut self,
    ) -> EnblWrProtOfPric1100pric11000600W<PricIo100Spec> {
        EnblWrProtOfPric1100pric11000600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_read_group0of_caliptra_mcuaccess(
        &mut self,
    ) -> EnblReadGroup0ofCaliptraMcuaccessW<PricIo100Spec> {
        EnblReadGroup0ofCaliptraMcuaccessW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_read_group1of_caliptra_mcuaccess(
        &mut self,
    ) -> EnblReadGroup1ofCaliptraMcuaccessW<PricIo100Spec> {
        EnblReadGroup1ofCaliptraMcuaccessW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_read_group2of_caliptra_mcuaccess(
        &mut self,
    ) -> EnblReadGroup2ofCaliptraMcuaccessW<PricIo100Spec> {
        EnblReadGroup2ofCaliptraMcuaccessW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_read_group3of_caliptra_mcuaccess(
        &mut self,
    ) -> EnblReadGroup3ofCaliptraMcuaccessW<PricIo100Spec> {
        EnblReadGroup3ofCaliptraMcuaccessW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_read_group4of_caliptra_mcuaccess(
        &mut self,
    ) -> EnblReadGroup4ofCaliptraMcuaccessW<PricIo100Spec> {
        EnblReadGroup4ofCaliptraMcuaccessW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_read_group5of_caliptra_mcuaccess(
        &mut self,
    ) -> EnblReadGroup5ofCaliptraMcuaccessW<PricIo100Spec> {
        EnblReadGroup5ofCaliptraMcuaccessW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1100PRIC1_100\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1100pric11001308(
        &mut self,
    ) -> EnblRstToleranceOfPric1100pric11001308W<PricIo100Spec> {
        EnblRstToleranceOfPric1100pric11001308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1100PRIC1_100\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1100pric11001408(
        &mut self,
    ) -> EnblWrProtOfPric1100pric11001408W<PricIo100Spec> {
        EnblWrProtOfPric1100pric11001408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_read_group0of_pspsaccess(&mut self) -> EnblReadGroup0ofPspsaccessW<PricIo100Spec> {
        EnblReadGroup0ofPspsaccessW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_read_group1of_pspsaccess(&mut self) -> EnblReadGroup1ofPspsaccessW<PricIo100Spec> {
        EnblReadGroup1ofPspsaccessW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_read_group2of_pspsaccess(&mut self) -> EnblReadGroup2ofPspsaccessW<PricIo100Spec> {
        EnblReadGroup2ofPspsaccessW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_read_group3of_pspsaccess(&mut self) -> EnblReadGroup3ofPspsaccessW<PricIo100Spec> {
        EnblReadGroup3ofPspsaccessW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_read_group4of_pspsaccess(&mut self) -> EnblReadGroup4ofPspsaccessW<PricIo100Spec> {
        EnblReadGroup4ofPspsaccessW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_read_group5of_pspsaccess(&mut self) -> EnblReadGroup5ofPspsaccessW<PricIo100Spec> {
        EnblReadGroup5ofPspsaccessW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1100PRIC1_100\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1100pric11002116(
        &mut self,
    ) -> EnblRstToleranceOfPric1100pric11002116W<PricIo100Spec> {
        EnblRstToleranceOfPric1100pric11002116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1100PRIC1_100\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1100pric11002216(
        &mut self,
    ) -> EnblWrProtOfPric1100pric11002216W<PricIo100Spec> {
        EnblWrProtOfPric1100pric11002216W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo100Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Master Read Group Setting Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io100::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io100::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo100Spec;
impl crate::RegisterSpec for PricIo100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io100::R`](R) reader structure"]
impl crate::Readable for PricIo100Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io100::W`](W) writer structure"]
impl crate::Writable for PricIo100Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO100 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo100Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
