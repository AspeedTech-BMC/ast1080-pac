#[doc = "Register `PRIC_IO00C` reader"]
pub type R = crate::R<PricIo00cSpec>;
#[doc = "Register `PRIC_IO00C` writer"]
pub type W = crate::W<PricIo00cSpec>;
#[doc = "Field `EnblWrGroup0OfI2CAccess` reader - Enable Write Group #0 of I2C access"]
pub type EnblWrGroup0ofI2caccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI2CAccess` writer - Enable Write Group #0 of I2C access"]
pub type EnblWrGroup0ofI2caccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI2CAccess` reader - Enable Write Group #1 of I2C access"]
pub type EnblWrGroup1ofI2caccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI2CAccess` writer - Enable Write Group #1 of I2C access"]
pub type EnblWrGroup1ofI2caccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI2CAccess` reader - Enable Write Group #2 of I2C access"]
pub type EnblWrGroup2ofI2caccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI2CAccess` writer - Enable Write Group #2 of I2C access"]
pub type EnblWrGroup2ofI2caccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI2CAccess` reader - Enable Write Group #3 of I2C access"]
pub type EnblWrGroup3ofI2caccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI2CAccess` writer - Enable Write Group #3 of I2C access"]
pub type EnblWrGroup3ofI2caccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI2CAccess` reader - Enable Write Group #4 of I2C access"]
pub type EnblWrGroup4ofI2caccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI2CAccess` writer - Enable Write Group #4 of I2C access"]
pub type EnblWrGroup4ofI2caccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI2CAccess` reader - Enable Write Group #5 of I2C access"]
pub type EnblWrGroup5ofI2caccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI2CAccess` writer - Enable Write Group #5 of I2C access"]
pub type EnblWrGroup5ofI2caccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC100CPRIC1_00C\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric100cpric100c0500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric100cpric100c0500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric100cpric100c0500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC100CPRIC100C0500` reader - Enable Reset Tolerance of PRIC100CPRIC1_00C\\[05:00\\]"]
pub type EnblRstToleranceOfPric100cpric100c0500R =
    crate::BitReader<EnblRstToleranceOfPric100cpric100c0500>;
impl EnblRstToleranceOfPric100cpric100c0500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric100cpric100c0500 {
        match self.bits {
            false => EnblRstToleranceOfPric100cpric100c0500::ResetBySrst,
            true => EnblRstToleranceOfPric100cpric100c0500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric100cpric100c0500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric100cpric100c0500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC100CPRIC100C0500` writer - Enable Reset Tolerance of PRIC100CPRIC1_00C\\[05:00\\]"]
pub type EnblRstToleranceOfPric100cpric100c0500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric100cpric100c0500>;
impl<'a, REG> EnblRstToleranceOfPric100cpric100c0500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric100cpric100c0500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric100cpric100c0500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC100CPRIC100C0600` reader - Enable Write Protection of PRIC100CPRIC1_00C\\[06:00\\]"]
pub type EnblWrProtOfPric100cpric100c0600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC100CPRIC100C0600` writer - Enable Write Protection of PRIC100CPRIC1_00C\\[06:00\\]"]
pub type EnblWrProtOfPric100cpric100c0600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfTAFSBridgeAccess` reader - Enable Write Group #0 of TAFS Bridge access"]
pub type EnblWrGroup0ofTafsbridgeAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfTAFSBridgeAccess` writer - Enable Write Group #0 of TAFS Bridge access"]
pub type EnblWrGroup0ofTafsbridgeAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfTAFSBridgeAccess` reader - Enable Write Group #1 of TAFS Bridge access"]
pub type EnblWrGroup1ofTafsbridgeAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfTAFSBridgeAccess` writer - Enable Write Group #1 of TAFS Bridge access"]
pub type EnblWrGroup1ofTafsbridgeAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfTAFSBridgeAccess` reader - Enable Write Group #2 of TAFS Bridge access"]
pub type EnblWrGroup2ofTafsbridgeAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfTAFSBridgeAccess` writer - Enable Write Group #2 of TAFS Bridge access"]
pub type EnblWrGroup2ofTafsbridgeAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfTAFSBridgeAccess` reader - Enable Write Group #3 of TAFS Bridge access"]
pub type EnblWrGroup3ofTafsbridgeAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfTAFSBridgeAccess` writer - Enable Write Group #3 of TAFS Bridge access"]
pub type EnblWrGroup3ofTafsbridgeAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfTAFSBridgeAccess` reader - Enable Write Group #4 of TAFS Bridge access"]
pub type EnblWrGroup4ofTafsbridgeAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfTAFSBridgeAccess` writer - Enable Write Group #4 of TAFS Bridge access"]
pub type EnblWrGroup4ofTafsbridgeAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfTAFSBridgeAccess` reader - Enable Write Group #5 of TAFS Bridge access"]
pub type EnblWrGroup5ofTafsbridgeAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfTAFSBridgeAccess` writer - Enable Write Group #5 of TAFS Bridge access"]
pub type EnblWrGroup5ofTafsbridgeAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC100CPRIC1_00C\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric100cpric100c1308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric100cpric100c1308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric100cpric100c1308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC100CPRIC100C1308` reader - Enable Reset Tolerance of PRIC100CPRIC1_00C\\[13:08\\]"]
pub type EnblRstToleranceOfPric100cpric100c1308R =
    crate::BitReader<EnblRstToleranceOfPric100cpric100c1308>;
impl EnblRstToleranceOfPric100cpric100c1308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric100cpric100c1308 {
        match self.bits {
            false => EnblRstToleranceOfPric100cpric100c1308::ResetBySrst,
            true => EnblRstToleranceOfPric100cpric100c1308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric100cpric100c1308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric100cpric100c1308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC100CPRIC100C1308` writer - Enable Reset Tolerance of PRIC100CPRIC1_00C\\[13:08\\]"]
pub type EnblRstToleranceOfPric100cpric100c1308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric100cpric100c1308>;
impl<'a, REG> EnblRstToleranceOfPric100cpric100c1308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric100cpric100c1308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric100cpric100c1308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC100CPRIC100C1408` reader - Enable Write Protection of PRIC100CPRIC1_00C\\[14:08\\]"]
pub type EnblWrProtOfPric100cpric100c1408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC100CPRIC100C1408` writer - Enable Write Protection of PRIC100CPRIC1_00C\\[14:08\\]"]
pub type EnblWrProtOfPric100cpric100c1408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUartDmaAccess` reader - Enable Write Group #0 of UartDma access"]
pub type EnblWrGroup0ofUartDmaAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUartDmaAccess` writer - Enable Write Group #0 of UartDma access"]
pub type EnblWrGroup0ofUartDmaAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUartDmaAccess` reader - Enable Write Group #1 of UartDma access"]
pub type EnblWrGroup1ofUartDmaAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUartDmaAccess` writer - Enable Write Group #1 of UartDma access"]
pub type EnblWrGroup1ofUartDmaAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUartDmaAccess` reader - Enable Write Group #2 of UartDma access"]
pub type EnblWrGroup2ofUartDmaAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUartDmaAccess` writer - Enable Write Group #2 of UartDma access"]
pub type EnblWrGroup2ofUartDmaAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUartDmaAccess` reader - Enable Write Group #3 of UartDma access"]
pub type EnblWrGroup3ofUartDmaAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUartDmaAccess` writer - Enable Write Group #3 of UartDma access"]
pub type EnblWrGroup3ofUartDmaAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUartDmaAccess` reader - Enable Write Group #4 of UartDma access"]
pub type EnblWrGroup4ofUartDmaAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUartDmaAccess` writer - Enable Write Group #4 of UartDma access"]
pub type EnblWrGroup4ofUartDmaAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUartDmaAccess` reader - Enable Write Group #5 of UartDma access"]
pub type EnblWrGroup5ofUartDmaAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUartDmaAccess` writer - Enable Write Group #5 of UartDma access"]
pub type EnblWrGroup5ofUartDmaAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC100CPRIC1_00C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric100cpric100c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric100cpric100c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric100cpric100c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC100CPRIC100C2116` reader - Enable Reset Tolerance of PRIC100CPRIC1_00C\\[21:16\\]"]
pub type EnblRstToleranceOfPric100cpric100c2116R =
    crate::BitReader<EnblRstToleranceOfPric100cpric100c2116>;
impl EnblRstToleranceOfPric100cpric100c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric100cpric100c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric100cpric100c2116::ResetBySrst,
            true => EnblRstToleranceOfPric100cpric100c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric100cpric100c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric100cpric100c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC100CPRIC100C2116` writer - Enable Reset Tolerance of PRIC100CPRIC1_00C\\[21:16\\]"]
pub type EnblRstToleranceOfPric100cpric100c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric100cpric100c2116>;
impl<'a, REG> EnblRstToleranceOfPric100cpric100c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric100cpric100c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric100cpric100c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC100CPRIC100C2216` reader - Enable Write Protection of PRIC100CPRIC1_00C\\[22:16\\]"]
pub type EnblWrProtOfPric100cpric100c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC100CPRIC100C2216` writer - Enable Write Protection of PRIC100CPRIC1_00C\\[22:16\\]"]
pub type EnblWrProtOfPric100cpric100c2216W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of I2C access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2caccess(&self) -> EnblWrGroup0ofI2caccessR {
        EnblWrGroup0ofI2caccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of I2C access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2caccess(&self) -> EnblWrGroup1ofI2caccessR {
        EnblWrGroup1ofI2caccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of I2C access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2caccess(&self) -> EnblWrGroup2ofI2caccessR {
        EnblWrGroup2ofI2caccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of I2C access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2caccess(&self) -> EnblWrGroup3ofI2caccessR {
        EnblWrGroup3ofI2caccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of I2C access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2caccess(&self) -> EnblWrGroup4ofI2caccessR {
        EnblWrGroup4ofI2caccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of I2C access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2caccess(&self) -> EnblWrGroup5ofI2caccessR {
        EnblWrGroup5ofI2caccessR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC100CPRIC1_00C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric100cpric100c0500(
        &self,
    ) -> EnblRstToleranceOfPric100cpric100c0500R {
        EnblRstToleranceOfPric100cpric100c0500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC100CPRIC1_00C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric100cpric100c0600(&self) -> EnblWrProtOfPric100cpric100c0600R {
        EnblWrProtOfPric100cpric100c0600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_tafsbridge_access(&self) -> EnblWrGroup0ofTafsbridgeAccessR {
        EnblWrGroup0ofTafsbridgeAccessR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_tafsbridge_access(&self) -> EnblWrGroup1ofTafsbridgeAccessR {
        EnblWrGroup1ofTafsbridgeAccessR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_tafsbridge_access(&self) -> EnblWrGroup2ofTafsbridgeAccessR {
        EnblWrGroup2ofTafsbridgeAccessR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_tafsbridge_access(&self) -> EnblWrGroup3ofTafsbridgeAccessR {
        EnblWrGroup3ofTafsbridgeAccessR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_tafsbridge_access(&self) -> EnblWrGroup4ofTafsbridgeAccessR {
        EnblWrGroup4ofTafsbridgeAccessR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_tafsbridge_access(&self) -> EnblWrGroup5ofTafsbridgeAccessR {
        EnblWrGroup5ofTafsbridgeAccessR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC100CPRIC1_00C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric100cpric100c1308(
        &self,
    ) -> EnblRstToleranceOfPric100cpric100c1308R {
        EnblRstToleranceOfPric100cpric100c1308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC100CPRIC1_00C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric100cpric100c1408(&self) -> EnblWrProtOfPric100cpric100c1408R {
        EnblWrProtOfPric100cpric100c1408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of UartDma access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart_dma_access(&self) -> EnblWrGroup0ofUartDmaAccessR {
        EnblWrGroup0ofUartDmaAccessR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of UartDma access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart_dma_access(&self) -> EnblWrGroup1ofUartDmaAccessR {
        EnblWrGroup1ofUartDmaAccessR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of UartDma access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart_dma_access(&self) -> EnblWrGroup2ofUartDmaAccessR {
        EnblWrGroup2ofUartDmaAccessR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of UartDma access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart_dma_access(&self) -> EnblWrGroup3ofUartDmaAccessR {
        EnblWrGroup3ofUartDmaAccessR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of UartDma access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart_dma_access(&self) -> EnblWrGroup4ofUartDmaAccessR {
        EnblWrGroup4ofUartDmaAccessR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of UartDma access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart_dma_access(&self) -> EnblWrGroup5ofUartDmaAccessR {
        EnblWrGroup5ofUartDmaAccessR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC100CPRIC1_00C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric100cpric100c2116(
        &self,
    ) -> EnblRstToleranceOfPric100cpric100c2116R {
        EnblRstToleranceOfPric100cpric100c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC100CPRIC1_00C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric100cpric100c2216(&self) -> EnblWrProtOfPric100cpric100c2216R {
        EnblWrProtOfPric100cpric100c2216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of I2C access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2caccess(&mut self) -> EnblWrGroup0ofI2caccessW<PricIo00cSpec> {
        EnblWrGroup0ofI2caccessW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of I2C access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2caccess(&mut self) -> EnblWrGroup1ofI2caccessW<PricIo00cSpec> {
        EnblWrGroup1ofI2caccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of I2C access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2caccess(&mut self) -> EnblWrGroup2ofI2caccessW<PricIo00cSpec> {
        EnblWrGroup2ofI2caccessW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of I2C access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2caccess(&mut self) -> EnblWrGroup3ofI2caccessW<PricIo00cSpec> {
        EnblWrGroup3ofI2caccessW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of I2C access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2caccess(&mut self) -> EnblWrGroup4ofI2caccessW<PricIo00cSpec> {
        EnblWrGroup4ofI2caccessW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of I2C access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2caccess(&mut self) -> EnblWrGroup5ofI2caccessW<PricIo00cSpec> {
        EnblWrGroup5ofI2caccessW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC100CPRIC1_00C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric100cpric100c0500(
        &mut self,
    ) -> EnblRstToleranceOfPric100cpric100c0500W<PricIo00cSpec> {
        EnblRstToleranceOfPric100cpric100c0500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC100CPRIC1_00C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric100cpric100c0600(
        &mut self,
    ) -> EnblWrProtOfPric100cpric100c0600W<PricIo00cSpec> {
        EnblWrProtOfPric100cpric100c0600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_tafsbridge_access(
        &mut self,
    ) -> EnblWrGroup0ofTafsbridgeAccessW<PricIo00cSpec> {
        EnblWrGroup0ofTafsbridgeAccessW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_tafsbridge_access(
        &mut self,
    ) -> EnblWrGroup1ofTafsbridgeAccessW<PricIo00cSpec> {
        EnblWrGroup1ofTafsbridgeAccessW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_tafsbridge_access(
        &mut self,
    ) -> EnblWrGroup2ofTafsbridgeAccessW<PricIo00cSpec> {
        EnblWrGroup2ofTafsbridgeAccessW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_tafsbridge_access(
        &mut self,
    ) -> EnblWrGroup3ofTafsbridgeAccessW<PricIo00cSpec> {
        EnblWrGroup3ofTafsbridgeAccessW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_tafsbridge_access(
        &mut self,
    ) -> EnblWrGroup4ofTafsbridgeAccessW<PricIo00cSpec> {
        EnblWrGroup4ofTafsbridgeAccessW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_tafsbridge_access(
        &mut self,
    ) -> EnblWrGroup5ofTafsbridgeAccessW<PricIo00cSpec> {
        EnblWrGroup5ofTafsbridgeAccessW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC100CPRIC1_00C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric100cpric100c1308(
        &mut self,
    ) -> EnblRstToleranceOfPric100cpric100c1308W<PricIo00cSpec> {
        EnblRstToleranceOfPric100cpric100c1308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC100CPRIC1_00C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric100cpric100c1408(
        &mut self,
    ) -> EnblWrProtOfPric100cpric100c1408W<PricIo00cSpec> {
        EnblWrProtOfPric100cpric100c1408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of UartDma access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart_dma_access(
        &mut self,
    ) -> EnblWrGroup0ofUartDmaAccessW<PricIo00cSpec> {
        EnblWrGroup0ofUartDmaAccessW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of UartDma access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart_dma_access(
        &mut self,
    ) -> EnblWrGroup1ofUartDmaAccessW<PricIo00cSpec> {
        EnblWrGroup1ofUartDmaAccessW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of UartDma access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart_dma_access(
        &mut self,
    ) -> EnblWrGroup2ofUartDmaAccessW<PricIo00cSpec> {
        EnblWrGroup2ofUartDmaAccessW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of UartDma access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart_dma_access(
        &mut self,
    ) -> EnblWrGroup3ofUartDmaAccessW<PricIo00cSpec> {
        EnblWrGroup3ofUartDmaAccessW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of UartDma access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart_dma_access(
        &mut self,
    ) -> EnblWrGroup4ofUartDmaAccessW<PricIo00cSpec> {
        EnblWrGroup4ofUartDmaAccessW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of UartDma access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart_dma_access(
        &mut self,
    ) -> EnblWrGroup5ofUartDmaAccessW<PricIo00cSpec> {
        EnblWrGroup5ofUartDmaAccessW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC100CPRIC1_00C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric100cpric100c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric100cpric100c2116W<PricIo00cSpec> {
        EnblRstToleranceOfPric100cpric100c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC100CPRIC1_00C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric100cpric100c2216(
        &mut self,
    ) -> EnblWrProtOfPric100cpric100c2216W<PricIo00cSpec> {
        EnblWrProtOfPric100cpric100c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo00cSpec> {
        Reserved7W::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo00cSpec> {
        Reserved6W::new(self, 25)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo00cSpec> {
        Reserved5W::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo00cSpec> {
        Reserved4W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo00cSpec> {
        Reserved3W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo00cSpec> {
        Reserved2W::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo00cSpec> {
        Reserved1W::new(self, 30)
    }
}
#[doc = "Master Write Group Setting Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo00cSpec;
impl crate::RegisterSpec for PricIo00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io00c::R`](R) reader structure"]
impl crate::Readable for PricIo00cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io00c::W`](W) writer structure"]
impl crate::Writable for PricIo00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO00C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo00cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
