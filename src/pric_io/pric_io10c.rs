#[doc = "Register `PRIC_IO10C` reader"]
pub type R = crate::R<PricIo10cSpec>;
#[doc = "Register `PRIC_IO10C` writer"]
pub type W = crate::W<PricIo10cSpec>;
#[doc = "Field `EnblReadGroup0OfI2CAccess` reader - Enable Read Group #0 of I2C access"]
pub type EnblReadGroup0ofI2caccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2CAccess` writer - Enable Read Group #0 of I2C access"]
pub type EnblReadGroup0ofI2caccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2CAccess` reader - Enable Read Group #1 of I2C access"]
pub type EnblReadGroup1ofI2caccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2CAccess` writer - Enable Read Group #1 of I2C access"]
pub type EnblReadGroup1ofI2caccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2CAccess` reader - Enable Read Group #2 of I2C access"]
pub type EnblReadGroup2ofI2caccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2CAccess` writer - Enable Read Group #2 of I2C access"]
pub type EnblReadGroup2ofI2caccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2CAccess` reader - Enable Read Group #3 of I2C access"]
pub type EnblReadGroup3ofI2caccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2CAccess` writer - Enable Read Group #3 of I2C access"]
pub type EnblReadGroup3ofI2caccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2CAccess` reader - Enable Read Group #4 of I2C access"]
pub type EnblReadGroup4ofI2caccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2CAccess` writer - Enable Read Group #4 of I2C access"]
pub type EnblReadGroup4ofI2caccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2CAccess` reader - Enable Read Group #5 of I2C access"]
pub type EnblReadGroup5ofI2caccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2CAccess` writer - Enable Read Group #5 of I2C access"]
pub type EnblReadGroup5ofI2caccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC110CPRIC1_10C\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric110cpric110c0500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric110cpric110c0500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric110cpric110c0500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC110CPRIC110C0500` reader - Enable Reset Tolerance of PRIC110CPRIC1_10C\\[05:00\\]"]
pub type EnblRstToleranceOfPric110cpric110c0500R =
    crate::BitReader<EnblRstToleranceOfPric110cpric110c0500>;
impl EnblRstToleranceOfPric110cpric110c0500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric110cpric110c0500 {
        match self.bits {
            false => EnblRstToleranceOfPric110cpric110c0500::ResetBySrst,
            true => EnblRstToleranceOfPric110cpric110c0500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric110cpric110c0500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric110cpric110c0500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC110CPRIC110C0500` writer - Enable Reset Tolerance of PRIC110CPRIC1_10C\\[05:00\\]"]
pub type EnblRstToleranceOfPric110cpric110c0500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric110cpric110c0500>;
impl<'a, REG> EnblRstToleranceOfPric110cpric110c0500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric110cpric110c0500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric110cpric110c0500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC110CPRIC110C0600` reader - Enable Write Protection of PRIC110CPRIC1_10C\\[06:00\\]"]
pub type EnblWrProtOfPric110cpric110c0600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC110CPRIC110C0600` writer - Enable Write Protection of PRIC110CPRIC1_10C\\[06:00\\]"]
pub type EnblWrProtOfPric110cpric110c0600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfTAFSBridgeAccess` reader - Enable Read Group #0 of TAFS Bridge access"]
pub type EnblReadGroup0ofTafsbridgeAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfTAFSBridgeAccess` writer - Enable Read Group #0 of TAFS Bridge access"]
pub type EnblReadGroup0ofTafsbridgeAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfTAFSBridgeAccess` reader - Enable Read Group #1 of TAFS Bridge access"]
pub type EnblReadGroup1ofTafsbridgeAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfTAFSBridgeAccess` writer - Enable Read Group #1 of TAFS Bridge access"]
pub type EnblReadGroup1ofTafsbridgeAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfTAFSBridgeAccess` reader - Enable Read Group #2 of TAFS Bridge access"]
pub type EnblReadGroup2ofTafsbridgeAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfTAFSBridgeAccess` writer - Enable Read Group #2 of TAFS Bridge access"]
pub type EnblReadGroup2ofTafsbridgeAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfTAFSBridgeAccess` reader - Enable Read Group #3 of TAFS Bridge access"]
pub type EnblReadGroup3ofTafsbridgeAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfTAFSBridgeAccess` writer - Enable Read Group #3 of TAFS Bridge access"]
pub type EnblReadGroup3ofTafsbridgeAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfTAFSBridgeAccess` reader - Enable Read Group #4 of TAFS Bridge access"]
pub type EnblReadGroup4ofTafsbridgeAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfTAFSBridgeAccess` writer - Enable Read Group #4 of TAFS Bridge access"]
pub type EnblReadGroup4ofTafsbridgeAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfTAFSBridgeAccess` reader - Enable Read Group #5 of TAFS Bridge access"]
pub type EnblReadGroup5ofTafsbridgeAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfTAFSBridgeAccess` writer - Enable Read Group #5 of TAFS Bridge access"]
pub type EnblReadGroup5ofTafsbridgeAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC110CPRIC1_10C\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric110cpric110c1308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric110cpric110c1308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric110cpric110c1308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC110CPRIC110C1308` reader - Enable Reset Tolerance of PRIC110CPRIC1_10C\\[13:08\\]"]
pub type EnblRstToleranceOfPric110cpric110c1308R =
    crate::BitReader<EnblRstToleranceOfPric110cpric110c1308>;
impl EnblRstToleranceOfPric110cpric110c1308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric110cpric110c1308 {
        match self.bits {
            false => EnblRstToleranceOfPric110cpric110c1308::ResetBySrst,
            true => EnblRstToleranceOfPric110cpric110c1308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric110cpric110c1308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric110cpric110c1308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC110CPRIC110C1308` writer - Enable Reset Tolerance of PRIC110CPRIC1_10C\\[13:08\\]"]
pub type EnblRstToleranceOfPric110cpric110c1308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric110cpric110c1308>;
impl<'a, REG> EnblRstToleranceOfPric110cpric110c1308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric110cpric110c1308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric110cpric110c1308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC110CPRIC110C1408` reader - Enable Write Protection of PRIC110CPRIC1_10C\\[14:08\\]"]
pub type EnblWrProtOfPric110cpric110c1408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC110CPRIC110C1408` writer - Enable Write Protection of PRIC110CPRIC1_10C\\[14:08\\]"]
pub type EnblWrProtOfPric110cpric110c1408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUartDmaAccess` reader - Enable Read Group #0 of UartDma access"]
pub type EnblReadGroup0ofUartDmaAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUartDmaAccess` writer - Enable Read Group #0 of UartDma access"]
pub type EnblReadGroup0ofUartDmaAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUartDmaAccess` reader - Enable Read Group #1 of UartDma access"]
pub type EnblReadGroup1ofUartDmaAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUartDmaAccess` writer - Enable Read Group #1 of UartDma access"]
pub type EnblReadGroup1ofUartDmaAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUartDmaAccess` reader - Enable Read Group #2 of UartDma access"]
pub type EnblReadGroup2ofUartDmaAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUartDmaAccess` writer - Enable Read Group #2 of UartDma access"]
pub type EnblReadGroup2ofUartDmaAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUartDmaAccess` reader - Enable Read Group #3 of UartDma access"]
pub type EnblReadGroup3ofUartDmaAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUartDmaAccess` writer - Enable Read Group #3 of UartDma access"]
pub type EnblReadGroup3ofUartDmaAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUartDmaAccess` reader - Enable Read Group #4 of UartDma access"]
pub type EnblReadGroup4ofUartDmaAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUartDmaAccess` writer - Enable Read Group #4 of UartDma access"]
pub type EnblReadGroup4ofUartDmaAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUartDmaAccess` reader - Enable Read Group #5 of UartDma access"]
pub type EnblReadGroup5ofUartDmaAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUartDmaAccess` writer - Enable Read Group #5 of UartDma access"]
pub type EnblReadGroup5ofUartDmaAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC110CPRIC1_10C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric110cpric110c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric110cpric110c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric110cpric110c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC110CPRIC110C2116` reader - Enable Reset Tolerance of PRIC110CPRIC1_10C\\[21:16\\]"]
pub type EnblRstToleranceOfPric110cpric110c2116R =
    crate::BitReader<EnblRstToleranceOfPric110cpric110c2116>;
impl EnblRstToleranceOfPric110cpric110c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric110cpric110c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric110cpric110c2116::ResetBySrst,
            true => EnblRstToleranceOfPric110cpric110c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric110cpric110c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric110cpric110c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC110CPRIC110C2116` writer - Enable Reset Tolerance of PRIC110CPRIC1_10C\\[21:16\\]"]
pub type EnblRstToleranceOfPric110cpric110c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric110cpric110c2116>;
impl<'a, REG> EnblRstToleranceOfPric110cpric110c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric110cpric110c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric110cpric110c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC110CPRIC110C2216` reader - Enable Write Protection of PRIC110CPRIC1_10C\\[22:16\\]"]
pub type EnblWrProtOfPric110cpric110c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC110CPRIC110C2216` writer - Enable Write Protection of PRIC110CPRIC1_10C\\[22:16\\]"]
pub type EnblWrProtOfPric110cpric110c2216W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - Enable Read Group #0 of I2C access"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2caccess(&self) -> EnblReadGroup0ofI2caccessR {
        EnblReadGroup0ofI2caccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I2C access"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2caccess(&self) -> EnblReadGroup1ofI2caccessR {
        EnblReadGroup1ofI2caccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I2C access"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2caccess(&self) -> EnblReadGroup2ofI2caccessR {
        EnblReadGroup2ofI2caccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I2C access"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2caccess(&self) -> EnblReadGroup3ofI2caccessR {
        EnblReadGroup3ofI2caccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I2C access"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2caccess(&self) -> EnblReadGroup4ofI2caccessR {
        EnblReadGroup4ofI2caccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I2C access"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2caccess(&self) -> EnblReadGroup5ofI2caccessR {
        EnblReadGroup5ofI2caccessR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC110CPRIC1_10C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric110cpric110c0500(
        &self,
    ) -> EnblRstToleranceOfPric110cpric110c0500R {
        EnblRstToleranceOfPric110cpric110c0500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC110CPRIC1_10C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric110cpric110c0600(&self) -> EnblWrProtOfPric110cpric110c0600R {
        EnblWrProtOfPric110cpric110c0600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_read_group0of_tafsbridge_access(&self) -> EnblReadGroup0ofTafsbridgeAccessR {
        EnblReadGroup0ofTafsbridgeAccessR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_read_group1of_tafsbridge_access(&self) -> EnblReadGroup1ofTafsbridgeAccessR {
        EnblReadGroup1ofTafsbridgeAccessR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_read_group2of_tafsbridge_access(&self) -> EnblReadGroup2ofTafsbridgeAccessR {
        EnblReadGroup2ofTafsbridgeAccessR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_read_group3of_tafsbridge_access(&self) -> EnblReadGroup3ofTafsbridgeAccessR {
        EnblReadGroup3ofTafsbridgeAccessR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_read_group4of_tafsbridge_access(&self) -> EnblReadGroup4ofTafsbridgeAccessR {
        EnblReadGroup4ofTafsbridgeAccessR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_read_group5of_tafsbridge_access(&self) -> EnblReadGroup5ofTafsbridgeAccessR {
        EnblReadGroup5ofTafsbridgeAccessR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC110CPRIC1_10C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric110cpric110c1308(
        &self,
    ) -> EnblRstToleranceOfPric110cpric110c1308R {
        EnblRstToleranceOfPric110cpric110c1308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC110CPRIC1_10C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric110cpric110c1408(&self) -> EnblWrProtOfPric110cpric110c1408R {
        EnblWrProtOfPric110cpric110c1408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of UartDma access"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart_dma_access(&self) -> EnblReadGroup0ofUartDmaAccessR {
        EnblReadGroup0ofUartDmaAccessR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of UartDma access"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart_dma_access(&self) -> EnblReadGroup1ofUartDmaAccessR {
        EnblReadGroup1ofUartDmaAccessR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of UartDma access"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart_dma_access(&self) -> EnblReadGroup2ofUartDmaAccessR {
        EnblReadGroup2ofUartDmaAccessR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of UartDma access"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart_dma_access(&self) -> EnblReadGroup3ofUartDmaAccessR {
        EnblReadGroup3ofUartDmaAccessR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of UartDma access"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart_dma_access(&self) -> EnblReadGroup4ofUartDmaAccessR {
        EnblReadGroup4ofUartDmaAccessR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of UartDma access"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart_dma_access(&self) -> EnblReadGroup5ofUartDmaAccessR {
        EnblReadGroup5ofUartDmaAccessR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC110CPRIC1_10C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric110cpric110c2116(
        &self,
    ) -> EnblRstToleranceOfPric110cpric110c2116R {
        EnblRstToleranceOfPric110cpric110c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC110CPRIC1_10C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric110cpric110c2216(&self) -> EnblWrProtOfPric110cpric110c2216R {
        EnblWrProtOfPric110cpric110c2216R::new(((self.bits >> 23) & 1) != 0)
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
    #[doc = "Bit 0 - Enable Read Group #0 of I2C access"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2caccess(&mut self) -> EnblReadGroup0ofI2caccessW<PricIo10cSpec> {
        EnblReadGroup0ofI2caccessW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I2C access"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2caccess(&mut self) -> EnblReadGroup1ofI2caccessW<PricIo10cSpec> {
        EnblReadGroup1ofI2caccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I2C access"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2caccess(&mut self) -> EnblReadGroup2ofI2caccessW<PricIo10cSpec> {
        EnblReadGroup2ofI2caccessW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I2C access"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2caccess(&mut self) -> EnblReadGroup3ofI2caccessW<PricIo10cSpec> {
        EnblReadGroup3ofI2caccessW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I2C access"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2caccess(&mut self) -> EnblReadGroup4ofI2caccessW<PricIo10cSpec> {
        EnblReadGroup4ofI2caccessW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I2C access"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2caccess(&mut self) -> EnblReadGroup5ofI2caccessW<PricIo10cSpec> {
        EnblReadGroup5ofI2caccessW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC110CPRIC1_10C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric110cpric110c0500(
        &mut self,
    ) -> EnblRstToleranceOfPric110cpric110c0500W<PricIo10cSpec> {
        EnblRstToleranceOfPric110cpric110c0500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC110CPRIC1_10C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric110cpric110c0600(
        &mut self,
    ) -> EnblWrProtOfPric110cpric110c0600W<PricIo10cSpec> {
        EnblWrProtOfPric110cpric110c0600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_read_group0of_tafsbridge_access(
        &mut self,
    ) -> EnblReadGroup0ofTafsbridgeAccessW<PricIo10cSpec> {
        EnblReadGroup0ofTafsbridgeAccessW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_read_group1of_tafsbridge_access(
        &mut self,
    ) -> EnblReadGroup1ofTafsbridgeAccessW<PricIo10cSpec> {
        EnblReadGroup1ofTafsbridgeAccessW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_read_group2of_tafsbridge_access(
        &mut self,
    ) -> EnblReadGroup2ofTafsbridgeAccessW<PricIo10cSpec> {
        EnblReadGroup2ofTafsbridgeAccessW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_read_group3of_tafsbridge_access(
        &mut self,
    ) -> EnblReadGroup3ofTafsbridgeAccessW<PricIo10cSpec> {
        EnblReadGroup3ofTafsbridgeAccessW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_read_group4of_tafsbridge_access(
        &mut self,
    ) -> EnblReadGroup4ofTafsbridgeAccessW<PricIo10cSpec> {
        EnblReadGroup4ofTafsbridgeAccessW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of TAFS Bridge access"]
    #[inline(always)]
    pub fn enbl_read_group5of_tafsbridge_access(
        &mut self,
    ) -> EnblReadGroup5ofTafsbridgeAccessW<PricIo10cSpec> {
        EnblReadGroup5ofTafsbridgeAccessW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC110CPRIC1_10C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric110cpric110c1308(
        &mut self,
    ) -> EnblRstToleranceOfPric110cpric110c1308W<PricIo10cSpec> {
        EnblRstToleranceOfPric110cpric110c1308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC110CPRIC1_10C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric110cpric110c1408(
        &mut self,
    ) -> EnblWrProtOfPric110cpric110c1408W<PricIo10cSpec> {
        EnblWrProtOfPric110cpric110c1408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of UartDma access"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart_dma_access(
        &mut self,
    ) -> EnblReadGroup0ofUartDmaAccessW<PricIo10cSpec> {
        EnblReadGroup0ofUartDmaAccessW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of UartDma access"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart_dma_access(
        &mut self,
    ) -> EnblReadGroup1ofUartDmaAccessW<PricIo10cSpec> {
        EnblReadGroup1ofUartDmaAccessW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of UartDma access"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart_dma_access(
        &mut self,
    ) -> EnblReadGroup2ofUartDmaAccessW<PricIo10cSpec> {
        EnblReadGroup2ofUartDmaAccessW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of UartDma access"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart_dma_access(
        &mut self,
    ) -> EnblReadGroup3ofUartDmaAccessW<PricIo10cSpec> {
        EnblReadGroup3ofUartDmaAccessW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of UartDma access"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart_dma_access(
        &mut self,
    ) -> EnblReadGroup4ofUartDmaAccessW<PricIo10cSpec> {
        EnblReadGroup4ofUartDmaAccessW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of UartDma access"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart_dma_access(
        &mut self,
    ) -> EnblReadGroup5ofUartDmaAccessW<PricIo10cSpec> {
        EnblReadGroup5ofUartDmaAccessW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC110CPRIC1_10C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric110cpric110c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric110cpric110c2116W<PricIo10cSpec> {
        EnblRstToleranceOfPric110cpric110c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC110CPRIC1_10C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric110cpric110c2216(
        &mut self,
    ) -> EnblWrProtOfPric110cpric110c2216W<PricIo10cSpec> {
        EnblWrProtOfPric110cpric110c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo10cSpec> {
        Reserved7W::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo10cSpec> {
        Reserved6W::new(self, 25)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo10cSpec> {
        Reserved5W::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo10cSpec> {
        Reserved4W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo10cSpec> {
        Reserved3W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo10cSpec> {
        Reserved2W::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo10cSpec> {
        Reserved1W::new(self, 30)
    }
}
#[doc = "Master Read Group Setting Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io10c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io10c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo10cSpec;
impl crate::RegisterSpec for PricIo10cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io10c::R`](R) reader structure"]
impl crate::Readable for PricIo10cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io10c::W`](W) writer structure"]
impl crate::Writable for PricIo10cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO10C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo10cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
