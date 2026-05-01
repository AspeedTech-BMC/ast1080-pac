#[doc = "Register `PRIC_IO000` reader"]
pub type R = crate::R<PricIo000Spec>;
#[doc = "Register `PRIC_IO000` writer"]
pub type W = crate::W<PricIo000Spec>;
#[doc = "Field `EnblWrGroup0OfCaliptraSSMCUAccess` reader - Enable Write Group #0 of Caliptra SS MCU access"]
pub type EnblWrGroup0ofCaliptraSsmcuaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfCaliptraSSMCUAccess` writer - Enable Write Group #0 of Caliptra SS MCU access"]
pub type EnblWrGroup0ofCaliptraSsmcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfCaliptraSSMCUAccess` reader - Enable Write Group #1 of Caliptra SS MCU access"]
pub type EnblWrGroup1ofCaliptraSsmcuaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfCaliptraSSMCUAccess` writer - Enable Write Group #1 of Caliptra SS MCU access"]
pub type EnblWrGroup1ofCaliptraSsmcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfCaliptraSSMCUAccess` reader - Enable Write Group #2 of Caliptra SS MCU access"]
pub type EnblWrGroup2ofCaliptraSsmcuaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfCaliptraSSMCUAccess` writer - Enable Write Group #2 of Caliptra SS MCU access"]
pub type EnblWrGroup2ofCaliptraSsmcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfCaliptraSSMCUAccess` reader - Enable Write Group #3 of Caliptra SS MCU access"]
pub type EnblWrGroup3ofCaliptraSsmcuaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfCaliptraSSMCUAccess` writer - Enable Write Group #3 of Caliptra SS MCU access"]
pub type EnblWrGroup3ofCaliptraSsmcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfCaliptraSSMCUAccess` reader - Enable Write Group #4 of Caliptra SS MCU access"]
pub type EnblWrGroup4ofCaliptraSsmcuaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfCaliptraSSMCUAccess` writer - Enable Write Group #4 of Caliptra SS MCU access"]
pub type EnblWrGroup4ofCaliptraSsmcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfCaliptraSSMCUAccess` reader - Enable Write Group #5 of Caliptra SS MCU access"]
pub type EnblWrGroup5ofCaliptraSsmcuaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfCaliptraSSMCUAccess` writer - Enable Write Group #5 of Caliptra SS MCU access"]
pub type EnblWrGroup5ofCaliptraSsmcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1000PRIC1_000\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1000pric10000500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1000pric10000500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1000pric10000500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1000PRIC10000500` reader - Enable Reset Tolerance of PRIC1000PRIC1_000\\[05:00\\]"]
pub type EnblRstToleranceOfPric1000pric10000500R =
    crate::BitReader<EnblRstToleranceOfPric1000pric10000500>;
impl EnblRstToleranceOfPric1000pric10000500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1000pric10000500 {
        match self.bits {
            false => EnblRstToleranceOfPric1000pric10000500::ResetBySrst,
            true => EnblRstToleranceOfPric1000pric10000500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1000pric10000500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1000pric10000500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1000PRIC10000500` writer - Enable Reset Tolerance of PRIC1000PRIC1_000\\[05:00\\]"]
pub type EnblRstToleranceOfPric1000pric10000500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1000pric10000500>;
impl<'a, REG> EnblRstToleranceOfPric1000pric10000500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1000pric10000500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1000pric10000500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1000PRIC10000600` reader - Enable Write Protection of PRIC1000PRIC1_000\\[06:00\\]"]
pub type EnblWrProtOfPric1000pric10000600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1000PRIC10000600` writer - Enable Write Protection of PRIC1000PRIC1_000\\[06:00\\]"]
pub type EnblWrProtOfPric1000pric10000600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfCaliptraMCUAccess` reader - Enable Write Group #0 of Caliptra MCU access"]
pub type EnblWrGroup0ofCaliptraMcuaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfCaliptraMCUAccess` writer - Enable Write Group #0 of Caliptra MCU access"]
pub type EnblWrGroup0ofCaliptraMcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfCaliptraMCUAccess` reader - Enable Write Group #1 of Caliptra MCU access"]
pub type EnblWrGroup1ofCaliptraMcuaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfCaliptraMCUAccess` writer - Enable Write Group #1 of Caliptra MCU access"]
pub type EnblWrGroup1ofCaliptraMcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfCaliptraMCUAccess` reader - Enable Write Group #2 of Caliptra MCU access"]
pub type EnblWrGroup2ofCaliptraMcuaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfCaliptraMCUAccess` writer - Enable Write Group #2 of Caliptra MCU access"]
pub type EnblWrGroup2ofCaliptraMcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfCaliptraMCUAccess` reader - Enable Write Group #3 of Caliptra MCU access"]
pub type EnblWrGroup3ofCaliptraMcuaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfCaliptraMCUAccess` writer - Enable Write Group #3 of Caliptra MCU access"]
pub type EnblWrGroup3ofCaliptraMcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfCaliptraMCUAccess` reader - Enable Write Group #4 of Caliptra MCU access"]
pub type EnblWrGroup4ofCaliptraMcuaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfCaliptraMCUAccess` writer - Enable Write Group #4 of Caliptra MCU access"]
pub type EnblWrGroup4ofCaliptraMcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfCaliptraMCUAccess` reader - Enable Write Group #5 of Caliptra MCU access"]
pub type EnblWrGroup5ofCaliptraMcuaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfCaliptraMCUAccess` writer - Enable Write Group #5 of Caliptra MCU access"]
pub type EnblWrGroup5ofCaliptraMcuaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1000PRIC1_000\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1000pric10001308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1000pric10001308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1000pric10001308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1000PRIC10001308` reader - Enable Reset Tolerance of PRIC1000PRIC1_000\\[13:08\\]"]
pub type EnblRstToleranceOfPric1000pric10001308R =
    crate::BitReader<EnblRstToleranceOfPric1000pric10001308>;
impl EnblRstToleranceOfPric1000pric10001308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1000pric10001308 {
        match self.bits {
            false => EnblRstToleranceOfPric1000pric10001308::ResetBySrst,
            true => EnblRstToleranceOfPric1000pric10001308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1000pric10001308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1000pric10001308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1000PRIC10001308` writer - Enable Reset Tolerance of PRIC1000PRIC1_000\\[13:08\\]"]
pub type EnblRstToleranceOfPric1000pric10001308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1000pric10001308>;
impl<'a, REG> EnblRstToleranceOfPric1000pric10001308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1000pric10001308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1000pric10001308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1000PRIC10001408` reader - Enable Write Protection of PRIC1000PRIC1_000\\[14:08\\]"]
pub type EnblWrProtOfPric1000pric10001408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1000PRIC10001408` writer - Enable Write Protection of PRIC1000PRIC1_000\\[14:08\\]"]
pub type EnblWrProtOfPric1000pric10001408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfPSPSAccess` reader - Enable Write Group #0 of PSP-S access"]
pub type EnblWrGroup0ofPspsaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfPSPSAccess` writer - Enable Write Group #0 of PSP-S access"]
pub type EnblWrGroup0ofPspsaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfPSPSAccess` reader - Enable Write Group #1 of PSP-S access"]
pub type EnblWrGroup1ofPspsaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfPSPSAccess` writer - Enable Write Group #1 of PSP-S access"]
pub type EnblWrGroup1ofPspsaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfPSPSAccess` reader - Enable Write Group #2 of PSP-S access"]
pub type EnblWrGroup2ofPspsaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfPSPSAccess` writer - Enable Write Group #2 of PSP-S access"]
pub type EnblWrGroup2ofPspsaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfPSPSAccess` reader - Enable Write Group #3 of PSP-S access"]
pub type EnblWrGroup3ofPspsaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfPSPSAccess` writer - Enable Write Group #3 of PSP-S access"]
pub type EnblWrGroup3ofPspsaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfPSPSAccess` reader - Enable Write Group #4 of PSP-S access"]
pub type EnblWrGroup4ofPspsaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfPSPSAccess` writer - Enable Write Group #4 of PSP-S access"]
pub type EnblWrGroup4ofPspsaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfPSPSAccess` reader - Enable Write Group #5 of PSP-S access"]
pub type EnblWrGroup5ofPspsaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfPSPSAccess` writer - Enable Write Group #5 of PSP-S access"]
pub type EnblWrGroup5ofPspsaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1000PRIC1_000\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1000pric10002116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1000pric10002116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1000pric10002116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1000PRIC10002116` reader - Enable Reset Tolerance of PRIC1000PRIC1_000\\[21:16\\]"]
pub type EnblRstToleranceOfPric1000pric10002116R =
    crate::BitReader<EnblRstToleranceOfPric1000pric10002116>;
impl EnblRstToleranceOfPric1000pric10002116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1000pric10002116 {
        match self.bits {
            false => EnblRstToleranceOfPric1000pric10002116::ResetBySrst,
            true => EnblRstToleranceOfPric1000pric10002116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1000pric10002116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1000pric10002116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1000PRIC10002116` writer - Enable Reset Tolerance of PRIC1000PRIC1_000\\[21:16\\]"]
pub type EnblRstToleranceOfPric1000pric10002116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1000pric10002116>;
impl<'a, REG> EnblRstToleranceOfPric1000pric10002116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1000pric10002116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1000pric10002116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1000PRIC10002216` reader - Enable Write Protection of PRIC1000PRIC1_000\\[22:16\\]"]
pub type EnblWrProtOfPric1000pric10002216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1000PRIC10002216` writer - Enable Write Protection of PRIC1000PRIC1_000\\[22:16\\]"]
pub type EnblWrProtOfPric1000pric10002216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_caliptra_ssmcuaccess(&self) -> EnblWrGroup0ofCaliptraSsmcuaccessR {
        EnblWrGroup0ofCaliptraSsmcuaccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_caliptra_ssmcuaccess(&self) -> EnblWrGroup1ofCaliptraSsmcuaccessR {
        EnblWrGroup1ofCaliptraSsmcuaccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_caliptra_ssmcuaccess(&self) -> EnblWrGroup2ofCaliptraSsmcuaccessR {
        EnblWrGroup2ofCaliptraSsmcuaccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_caliptra_ssmcuaccess(&self) -> EnblWrGroup3ofCaliptraSsmcuaccessR {
        EnblWrGroup3ofCaliptraSsmcuaccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_caliptra_ssmcuaccess(&self) -> EnblWrGroup4ofCaliptraSsmcuaccessR {
        EnblWrGroup4ofCaliptraSsmcuaccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_caliptra_ssmcuaccess(&self) -> EnblWrGroup5ofCaliptraSsmcuaccessR {
        EnblWrGroup5ofCaliptraSsmcuaccessR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1000PRIC1_000\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1000pric10000500(
        &self,
    ) -> EnblRstToleranceOfPric1000pric10000500R {
        EnblRstToleranceOfPric1000pric10000500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1000PRIC1_000\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1000pric10000600(&self) -> EnblWrProtOfPric1000pric10000600R {
        EnblWrProtOfPric1000pric10000600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_caliptra_mcuaccess(&self) -> EnblWrGroup0ofCaliptraMcuaccessR {
        EnblWrGroup0ofCaliptraMcuaccessR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_caliptra_mcuaccess(&self) -> EnblWrGroup1ofCaliptraMcuaccessR {
        EnblWrGroup1ofCaliptraMcuaccessR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_caliptra_mcuaccess(&self) -> EnblWrGroup2ofCaliptraMcuaccessR {
        EnblWrGroup2ofCaliptraMcuaccessR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_caliptra_mcuaccess(&self) -> EnblWrGroup3ofCaliptraMcuaccessR {
        EnblWrGroup3ofCaliptraMcuaccessR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_caliptra_mcuaccess(&self) -> EnblWrGroup4ofCaliptraMcuaccessR {
        EnblWrGroup4ofCaliptraMcuaccessR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_caliptra_mcuaccess(&self) -> EnblWrGroup5ofCaliptraMcuaccessR {
        EnblWrGroup5ofCaliptraMcuaccessR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1000PRIC1_000\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1000pric10001308(
        &self,
    ) -> EnblRstToleranceOfPric1000pric10001308R {
        EnblRstToleranceOfPric1000pric10001308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1000PRIC1_000\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1000pric10001408(&self) -> EnblWrProtOfPric1000pric10001408R {
        EnblWrProtOfPric1000pric10001408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_pspsaccess(&self) -> EnblWrGroup0ofPspsaccessR {
        EnblWrGroup0ofPspsaccessR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_pspsaccess(&self) -> EnblWrGroup1ofPspsaccessR {
        EnblWrGroup1ofPspsaccessR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_pspsaccess(&self) -> EnblWrGroup2ofPspsaccessR {
        EnblWrGroup2ofPspsaccessR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_pspsaccess(&self) -> EnblWrGroup3ofPspsaccessR {
        EnblWrGroup3ofPspsaccessR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_pspsaccess(&self) -> EnblWrGroup4ofPspsaccessR {
        EnblWrGroup4ofPspsaccessR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_pspsaccess(&self) -> EnblWrGroup5ofPspsaccessR {
        EnblWrGroup5ofPspsaccessR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1000PRIC1_000\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1000pric10002116(
        &self,
    ) -> EnblRstToleranceOfPric1000pric10002116R {
        EnblRstToleranceOfPric1000pric10002116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1000PRIC1_000\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1000pric10002216(&self) -> EnblWrProtOfPric1000pric10002216R {
        EnblWrProtOfPric1000pric10002216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_caliptra_ssmcuaccess(
        &mut self,
    ) -> EnblWrGroup0ofCaliptraSsmcuaccessW<PricIo000Spec> {
        EnblWrGroup0ofCaliptraSsmcuaccessW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_caliptra_ssmcuaccess(
        &mut self,
    ) -> EnblWrGroup1ofCaliptraSsmcuaccessW<PricIo000Spec> {
        EnblWrGroup1ofCaliptraSsmcuaccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_caliptra_ssmcuaccess(
        &mut self,
    ) -> EnblWrGroup2ofCaliptraSsmcuaccessW<PricIo000Spec> {
        EnblWrGroup2ofCaliptraSsmcuaccessW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_caliptra_ssmcuaccess(
        &mut self,
    ) -> EnblWrGroup3ofCaliptraSsmcuaccessW<PricIo000Spec> {
        EnblWrGroup3ofCaliptraSsmcuaccessW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_caliptra_ssmcuaccess(
        &mut self,
    ) -> EnblWrGroup4ofCaliptraSsmcuaccessW<PricIo000Spec> {
        EnblWrGroup4ofCaliptraSsmcuaccessW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of Caliptra SS MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_caliptra_ssmcuaccess(
        &mut self,
    ) -> EnblWrGroup5ofCaliptraSsmcuaccessW<PricIo000Spec> {
        EnblWrGroup5ofCaliptraSsmcuaccessW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1000PRIC1_000\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1000pric10000500(
        &mut self,
    ) -> EnblRstToleranceOfPric1000pric10000500W<PricIo000Spec> {
        EnblRstToleranceOfPric1000pric10000500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1000PRIC1_000\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1000pric10000600(
        &mut self,
    ) -> EnblWrProtOfPric1000pric10000600W<PricIo000Spec> {
        EnblWrProtOfPric1000pric10000600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_caliptra_mcuaccess(
        &mut self,
    ) -> EnblWrGroup0ofCaliptraMcuaccessW<PricIo000Spec> {
        EnblWrGroup0ofCaliptraMcuaccessW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_caliptra_mcuaccess(
        &mut self,
    ) -> EnblWrGroup1ofCaliptraMcuaccessW<PricIo000Spec> {
        EnblWrGroup1ofCaliptraMcuaccessW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_caliptra_mcuaccess(
        &mut self,
    ) -> EnblWrGroup2ofCaliptraMcuaccessW<PricIo000Spec> {
        EnblWrGroup2ofCaliptraMcuaccessW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_caliptra_mcuaccess(
        &mut self,
    ) -> EnblWrGroup3ofCaliptraMcuaccessW<PricIo000Spec> {
        EnblWrGroup3ofCaliptraMcuaccessW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_caliptra_mcuaccess(
        &mut self,
    ) -> EnblWrGroup4ofCaliptraMcuaccessW<PricIo000Spec> {
        EnblWrGroup4ofCaliptraMcuaccessW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of Caliptra MCU access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_caliptra_mcuaccess(
        &mut self,
    ) -> EnblWrGroup5ofCaliptraMcuaccessW<PricIo000Spec> {
        EnblWrGroup5ofCaliptraMcuaccessW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1000PRIC1_000\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1000pric10001308(
        &mut self,
    ) -> EnblRstToleranceOfPric1000pric10001308W<PricIo000Spec> {
        EnblRstToleranceOfPric1000pric10001308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1000PRIC1_000\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1000pric10001408(
        &mut self,
    ) -> EnblWrProtOfPric1000pric10001408W<PricIo000Spec> {
        EnblWrProtOfPric1000pric10001408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_pspsaccess(&mut self) -> EnblWrGroup0ofPspsaccessW<PricIo000Spec> {
        EnblWrGroup0ofPspsaccessW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_pspsaccess(&mut self) -> EnblWrGroup1ofPspsaccessW<PricIo000Spec> {
        EnblWrGroup1ofPspsaccessW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_pspsaccess(&mut self) -> EnblWrGroup2ofPspsaccessW<PricIo000Spec> {
        EnblWrGroup2ofPspsaccessW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_pspsaccess(&mut self) -> EnblWrGroup3ofPspsaccessW<PricIo000Spec> {
        EnblWrGroup3ofPspsaccessW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_pspsaccess(&mut self) -> EnblWrGroup4ofPspsaccessW<PricIo000Spec> {
        EnblWrGroup4ofPspsaccessW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of PSP-S access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_pspsaccess(&mut self) -> EnblWrGroup5ofPspsaccessW<PricIo000Spec> {
        EnblWrGroup5ofPspsaccessW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1000PRIC1_000\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1000pric10002116(
        &mut self,
    ) -> EnblRstToleranceOfPric1000pric10002116W<PricIo000Spec> {
        EnblRstToleranceOfPric1000pric10002116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1000PRIC1_000\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1000pric10002216(
        &mut self,
    ) -> EnblWrProtOfPric1000pric10002216W<PricIo000Spec> {
        EnblWrProtOfPric1000pric10002216W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo000Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Master Write Group Setting Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo000Spec;
impl crate::RegisterSpec for PricIo000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io000::R`](R) reader structure"]
impl crate::Readable for PricIo000Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io000::W`](W) writer structure"]
impl crate::Writable for PricIo000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO000 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo000Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
