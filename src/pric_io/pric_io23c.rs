#[doc = "Register `PRIC_IO23C` reader"]
pub type R = crate::R<PricIo23cSpec>;
#[doc = "Register `PRIC_IO23C` writer"]
pub type W = crate::W<PricIo23cSpec>;
#[doc = "Field `EnblWrGroup0OfI2C4` reader - Enable Write Group #0 of I2C4"]
pub type EnblWrGroup0ofI2c4R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI2C4` writer - Enable Write Group #0 of I2C4"]
pub type EnblWrGroup0ofI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI2C4` reader - Enable Write Group #1 of I2C4"]
pub type EnblWrGroup1ofI2c4R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI2C4` writer - Enable Write Group #1 of I2C4"]
pub type EnblWrGroup1ofI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI2C4` reader - Enable Write Group #2 of I2C4"]
pub type EnblWrGroup2ofI2c4R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI2C4` writer - Enable Write Group #2 of I2C4"]
pub type EnblWrGroup2ofI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI2C4` reader - Enable Write Group #3 of I2C4"]
pub type EnblWrGroup3ofI2c4R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI2C4` writer - Enable Write Group #3 of I2C4"]
pub type EnblWrGroup3ofI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI2C4` reader - Enable Write Group #4 of I2C4"]
pub type EnblWrGroup4ofI2c4R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI2C4` writer - Enable Write Group #4 of I2C4"]
pub type EnblWrGroup4ofI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI2C4` reader - Enable Write Group #5 of I2C4"]
pub type EnblWrGroup5ofI2c4R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI2C4` writer - Enable Write Group #5 of I2C4"]
pub type EnblWrGroup5ofI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC123CPRIC1_23C\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric123cpric123c0500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric123cpric123c0500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric123cpric123c0500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC123CPRIC123C0500` reader - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[05:00\\]"]
pub type EnblRstToleranceOfPric123cpric123c0500R =
    crate::BitReader<EnblRstToleranceOfPric123cpric123c0500>;
impl EnblRstToleranceOfPric123cpric123c0500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric123cpric123c0500 {
        match self.bits {
            false => EnblRstToleranceOfPric123cpric123c0500::ResetBySrst,
            true => EnblRstToleranceOfPric123cpric123c0500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric123cpric123c0500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric123cpric123c0500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC123CPRIC123C0500` writer - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[05:00\\]"]
pub type EnblRstToleranceOfPric123cpric123c0500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric123cpric123c0500>;
impl<'a, REG> EnblRstToleranceOfPric123cpric123c0500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric123cpric123c0500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric123cpric123c0500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC123CPRIC123C0600` reader - Enable Write Protection of PRIC123CPRIC1_23C\\[06:00\\]"]
pub type EnblWrProtOfPric123cpric123c0600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC123CPRIC123C0600` writer - Enable Write Protection of PRIC123CPRIC1_23C\\[06:00\\]"]
pub type EnblWrProtOfPric123cpric123c0600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfI2C5` reader - Enable Write Group #0 of I2C5"]
pub type EnblWrGroup0ofI2c5R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI2C5` writer - Enable Write Group #0 of I2C5"]
pub type EnblWrGroup0ofI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI2C5` reader - Enable Write Group #1 of I2C5"]
pub type EnblWrGroup1ofI2c5R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI2C5` writer - Enable Write Group #1 of I2C5"]
pub type EnblWrGroup1ofI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI2C5` reader - Enable Write Group #2 of I2C5"]
pub type EnblWrGroup2ofI2c5R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI2C5` writer - Enable Write Group #2 of I2C5"]
pub type EnblWrGroup2ofI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI2C5` reader - Enable Write Group #3 of I2C5"]
pub type EnblWrGroup3ofI2c5R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI2C5` writer - Enable Write Group #3 of I2C5"]
pub type EnblWrGroup3ofI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI2C5` reader - Enable Write Group #4 of I2C5"]
pub type EnblWrGroup4ofI2c5R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI2C5` writer - Enable Write Group #4 of I2C5"]
pub type EnblWrGroup4ofI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI2C5` reader - Enable Write Group #5 of I2C5"]
pub type EnblWrGroup5ofI2c5R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI2C5` writer - Enable Write Group #5 of I2C5"]
pub type EnblWrGroup5ofI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC123CPRIC1_23C\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric123cpric123c1308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric123cpric123c1308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric123cpric123c1308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC123CPRIC123C1308` reader - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[13:08\\]"]
pub type EnblRstToleranceOfPric123cpric123c1308R =
    crate::BitReader<EnblRstToleranceOfPric123cpric123c1308>;
impl EnblRstToleranceOfPric123cpric123c1308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric123cpric123c1308 {
        match self.bits {
            false => EnblRstToleranceOfPric123cpric123c1308::ResetBySrst,
            true => EnblRstToleranceOfPric123cpric123c1308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric123cpric123c1308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric123cpric123c1308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC123CPRIC123C1308` writer - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[13:08\\]"]
pub type EnblRstToleranceOfPric123cpric123c1308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric123cpric123c1308>;
impl<'a, REG> EnblRstToleranceOfPric123cpric123c1308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric123cpric123c1308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric123cpric123c1308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC123CPRIC123C1408` reader - Enable Write Protection of PRIC123CPRIC1_23C\\[14:08\\]"]
pub type EnblWrProtOfPric123cpric123c1408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC123CPRIC123C1408` writer - Enable Write Protection of PRIC123CPRIC1_23C\\[14:08\\]"]
pub type EnblWrProtOfPric123cpric123c1408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfI2C6` reader - Enable Write Group #0 of I2C6"]
pub type EnblWrGroup0ofI2c6R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI2C6` writer - Enable Write Group #0 of I2C6"]
pub type EnblWrGroup0ofI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI2C6` reader - Enable Write Group #1 of I2C6"]
pub type EnblWrGroup1ofI2c6R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI2C6` writer - Enable Write Group #1 of I2C6"]
pub type EnblWrGroup1ofI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI2C6` reader - Enable Write Group #2 of I2C6"]
pub type EnblWrGroup2ofI2c6R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI2C6` writer - Enable Write Group #2 of I2C6"]
pub type EnblWrGroup2ofI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI2C6` reader - Enable Write Group #3 of I2C6"]
pub type EnblWrGroup3ofI2c6R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI2C6` writer - Enable Write Group #3 of I2C6"]
pub type EnblWrGroup3ofI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI2C6` reader - Enable Write Group #4 of I2C6"]
pub type EnblWrGroup4ofI2c6R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI2C6` writer - Enable Write Group #4 of I2C6"]
pub type EnblWrGroup4ofI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI2C6` reader - Enable Write Group #5 of I2C6"]
pub type EnblWrGroup5ofI2c6R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI2C6` writer - Enable Write Group #5 of I2C6"]
pub type EnblWrGroup5ofI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC123CPRIC1_23C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric123cpric123c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric123cpric123c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric123cpric123c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC123CPRIC123C2116` reader - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[21:16\\]"]
pub type EnblRstToleranceOfPric123cpric123c2116R =
    crate::BitReader<EnblRstToleranceOfPric123cpric123c2116>;
impl EnblRstToleranceOfPric123cpric123c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric123cpric123c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric123cpric123c2116::ResetBySrst,
            true => EnblRstToleranceOfPric123cpric123c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric123cpric123c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric123cpric123c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC123CPRIC123C2116` writer - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[21:16\\]"]
pub type EnblRstToleranceOfPric123cpric123c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric123cpric123c2116>;
impl<'a, REG> EnblRstToleranceOfPric123cpric123c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric123cpric123c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric123cpric123c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC123CPRIC123C2216` reader - Enable Write Protection of PRIC123CPRIC1_23C\\[22:16\\]"]
pub type EnblWrProtOfPric123cpric123c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC123CPRIC123C2216` writer - Enable Write Protection of PRIC123CPRIC1_23C\\[22:16\\]"]
pub type EnblWrProtOfPric123cpric123c2216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfI2C7` reader - Enable Write Group #0 of I2C7"]
pub type EnblWrGroup0ofI2c7R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI2C7` writer - Enable Write Group #0 of I2C7"]
pub type EnblWrGroup0ofI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI2C7` reader - Enable Write Group #1 of I2C7"]
pub type EnblWrGroup1ofI2c7R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI2C7` writer - Enable Write Group #1 of I2C7"]
pub type EnblWrGroup1ofI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI2C7` reader - Enable Write Group #2 of I2C7"]
pub type EnblWrGroup2ofI2c7R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI2C7` writer - Enable Write Group #2 of I2C7"]
pub type EnblWrGroup2ofI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI2C7` reader - Enable Write Group #3 of I2C7"]
pub type EnblWrGroup3ofI2c7R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI2C7` writer - Enable Write Group #3 of I2C7"]
pub type EnblWrGroup3ofI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI2C7` reader - Enable Write Group #4 of I2C7"]
pub type EnblWrGroup4ofI2c7R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI2C7` writer - Enable Write Group #4 of I2C7"]
pub type EnblWrGroup4ofI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI2C7` reader - Enable Write Group #5 of I2C7"]
pub type EnblWrGroup5ofI2c7R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI2C7` writer - Enable Write Group #5 of I2C7"]
pub type EnblWrGroup5ofI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC123CPRIC1_23C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric123cpric123c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric123cpric123c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric123cpric123c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC123CPRIC123C2924` reader - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[29:24\\]"]
pub type EnblRstToleranceOfPric123cpric123c2924R =
    crate::BitReader<EnblRstToleranceOfPric123cpric123c2924>;
impl EnblRstToleranceOfPric123cpric123c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric123cpric123c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric123cpric123c2924::ResetBySrst,
            true => EnblRstToleranceOfPric123cpric123c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric123cpric123c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric123cpric123c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC123CPRIC123C2924` writer - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[29:24\\]"]
pub type EnblRstToleranceOfPric123cpric123c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric123cpric123c2924>;
impl<'a, REG> EnblRstToleranceOfPric123cpric123c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric123cpric123c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric123cpric123c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC123CPRIC123C3024` reader - Enable Write Protection of PRIC123CPRIC1_23C\\[30:24\\]"]
pub type EnblWrProtOfPric123cpric123c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC123CPRIC123C3024` writer - Enable Write Protection of PRIC123CPRIC1_23C\\[30:24\\]"]
pub type EnblWrProtOfPric123cpric123c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of I2C4"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c4(&self) -> EnblWrGroup0ofI2c4R {
        EnblWrGroup0ofI2c4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of I2C4"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c4(&self) -> EnblWrGroup1ofI2c4R {
        EnblWrGroup1ofI2c4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of I2C4"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c4(&self) -> EnblWrGroup2ofI2c4R {
        EnblWrGroup2ofI2c4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of I2C4"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c4(&self) -> EnblWrGroup3ofI2c4R {
        EnblWrGroup3ofI2c4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of I2C4"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c4(&self) -> EnblWrGroup4ofI2c4R {
        EnblWrGroup4ofI2c4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of I2C4"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c4(&self) -> EnblWrGroup5ofI2c4R {
        EnblWrGroup5ofI2c4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric123cpric123c0500(
        &self,
    ) -> EnblRstToleranceOfPric123cpric123c0500R {
        EnblRstToleranceOfPric123cpric123c0500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC123CPRIC1_23C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric123cpric123c0600(&self) -> EnblWrProtOfPric123cpric123c0600R {
        EnblWrProtOfPric123cpric123c0600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of I2C5"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c5(&self) -> EnblWrGroup0ofI2c5R {
        EnblWrGroup0ofI2c5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of I2C5"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c5(&self) -> EnblWrGroup1ofI2c5R {
        EnblWrGroup1ofI2c5R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of I2C5"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c5(&self) -> EnblWrGroup2ofI2c5R {
        EnblWrGroup2ofI2c5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of I2C5"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c5(&self) -> EnblWrGroup3ofI2c5R {
        EnblWrGroup3ofI2c5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of I2C5"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c5(&self) -> EnblWrGroup4ofI2c5R {
        EnblWrGroup4ofI2c5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of I2C5"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c5(&self) -> EnblWrGroup5ofI2c5R {
        EnblWrGroup5ofI2c5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric123cpric123c1308(
        &self,
    ) -> EnblRstToleranceOfPric123cpric123c1308R {
        EnblRstToleranceOfPric123cpric123c1308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC123CPRIC1_23C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric123cpric123c1408(&self) -> EnblWrProtOfPric123cpric123c1408R {
        EnblWrProtOfPric123cpric123c1408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of I2C6"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c6(&self) -> EnblWrGroup0ofI2c6R {
        EnblWrGroup0ofI2c6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of I2C6"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c6(&self) -> EnblWrGroup1ofI2c6R {
        EnblWrGroup1ofI2c6R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of I2C6"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c6(&self) -> EnblWrGroup2ofI2c6R {
        EnblWrGroup2ofI2c6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of I2C6"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c6(&self) -> EnblWrGroup3ofI2c6R {
        EnblWrGroup3ofI2c6R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of I2C6"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c6(&self) -> EnblWrGroup4ofI2c6R {
        EnblWrGroup4ofI2c6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of I2C6"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c6(&self) -> EnblWrGroup5ofI2c6R {
        EnblWrGroup5ofI2c6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric123cpric123c2116(
        &self,
    ) -> EnblRstToleranceOfPric123cpric123c2116R {
        EnblRstToleranceOfPric123cpric123c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC123CPRIC1_23C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric123cpric123c2216(&self) -> EnblWrProtOfPric123cpric123c2216R {
        EnblWrProtOfPric123cpric123c2216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of I2C7"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c7(&self) -> EnblWrGroup0ofI2c7R {
        EnblWrGroup0ofI2c7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of I2C7"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c7(&self) -> EnblWrGroup1ofI2c7R {
        EnblWrGroup1ofI2c7R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of I2C7"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c7(&self) -> EnblWrGroup2ofI2c7R {
        EnblWrGroup2ofI2c7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of I2C7"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c7(&self) -> EnblWrGroup3ofI2c7R {
        EnblWrGroup3ofI2c7R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of I2C7"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c7(&self) -> EnblWrGroup4ofI2c7R {
        EnblWrGroup4ofI2c7R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of I2C7"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c7(&self) -> EnblWrGroup5ofI2c7R {
        EnblWrGroup5ofI2c7R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric123cpric123c2924(
        &self,
    ) -> EnblRstToleranceOfPric123cpric123c2924R {
        EnblRstToleranceOfPric123cpric123c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC123CPRIC1_23C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric123cpric123c3024(&self) -> EnblWrProtOfPric123cpric123c3024R {
        EnblWrProtOfPric123cpric123c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of I2C4"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c4(&mut self) -> EnblWrGroup0ofI2c4W<PricIo23cSpec> {
        EnblWrGroup0ofI2c4W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of I2C4"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c4(&mut self) -> EnblWrGroup1ofI2c4W<PricIo23cSpec> {
        EnblWrGroup1ofI2c4W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of I2C4"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c4(&mut self) -> EnblWrGroup2ofI2c4W<PricIo23cSpec> {
        EnblWrGroup2ofI2c4W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of I2C4"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c4(&mut self) -> EnblWrGroup3ofI2c4W<PricIo23cSpec> {
        EnblWrGroup3ofI2c4W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of I2C4"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c4(&mut self) -> EnblWrGroup4ofI2c4W<PricIo23cSpec> {
        EnblWrGroup4ofI2c4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of I2C4"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c4(&mut self) -> EnblWrGroup5ofI2c4W<PricIo23cSpec> {
        EnblWrGroup5ofI2c4W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric123cpric123c0500(
        &mut self,
    ) -> EnblRstToleranceOfPric123cpric123c0500W<PricIo23cSpec> {
        EnblRstToleranceOfPric123cpric123c0500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC123CPRIC1_23C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric123cpric123c0600(
        &mut self,
    ) -> EnblWrProtOfPric123cpric123c0600W<PricIo23cSpec> {
        EnblWrProtOfPric123cpric123c0600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of I2C5"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c5(&mut self) -> EnblWrGroup0ofI2c5W<PricIo23cSpec> {
        EnblWrGroup0ofI2c5W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of I2C5"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c5(&mut self) -> EnblWrGroup1ofI2c5W<PricIo23cSpec> {
        EnblWrGroup1ofI2c5W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of I2C5"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c5(&mut self) -> EnblWrGroup2ofI2c5W<PricIo23cSpec> {
        EnblWrGroup2ofI2c5W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of I2C5"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c5(&mut self) -> EnblWrGroup3ofI2c5W<PricIo23cSpec> {
        EnblWrGroup3ofI2c5W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of I2C5"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c5(&mut self) -> EnblWrGroup4ofI2c5W<PricIo23cSpec> {
        EnblWrGroup4ofI2c5W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of I2C5"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c5(&mut self) -> EnblWrGroup5ofI2c5W<PricIo23cSpec> {
        EnblWrGroup5ofI2c5W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric123cpric123c1308(
        &mut self,
    ) -> EnblRstToleranceOfPric123cpric123c1308W<PricIo23cSpec> {
        EnblRstToleranceOfPric123cpric123c1308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC123CPRIC1_23C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric123cpric123c1408(
        &mut self,
    ) -> EnblWrProtOfPric123cpric123c1408W<PricIo23cSpec> {
        EnblWrProtOfPric123cpric123c1408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of I2C6"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c6(&mut self) -> EnblWrGroup0ofI2c6W<PricIo23cSpec> {
        EnblWrGroup0ofI2c6W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of I2C6"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c6(&mut self) -> EnblWrGroup1ofI2c6W<PricIo23cSpec> {
        EnblWrGroup1ofI2c6W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of I2C6"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c6(&mut self) -> EnblWrGroup2ofI2c6W<PricIo23cSpec> {
        EnblWrGroup2ofI2c6W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of I2C6"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c6(&mut self) -> EnblWrGroup3ofI2c6W<PricIo23cSpec> {
        EnblWrGroup3ofI2c6W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of I2C6"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c6(&mut self) -> EnblWrGroup4ofI2c6W<PricIo23cSpec> {
        EnblWrGroup4ofI2c6W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of I2C6"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c6(&mut self) -> EnblWrGroup5ofI2c6W<PricIo23cSpec> {
        EnblWrGroup5ofI2c6W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric123cpric123c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric123cpric123c2116W<PricIo23cSpec> {
        EnblRstToleranceOfPric123cpric123c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC123CPRIC1_23C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric123cpric123c2216(
        &mut self,
    ) -> EnblWrProtOfPric123cpric123c2216W<PricIo23cSpec> {
        EnblWrProtOfPric123cpric123c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of I2C7"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c7(&mut self) -> EnblWrGroup0ofI2c7W<PricIo23cSpec> {
        EnblWrGroup0ofI2c7W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of I2C7"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c7(&mut self) -> EnblWrGroup1ofI2c7W<PricIo23cSpec> {
        EnblWrGroup1ofI2c7W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of I2C7"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c7(&mut self) -> EnblWrGroup2ofI2c7W<PricIo23cSpec> {
        EnblWrGroup2ofI2c7W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of I2C7"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c7(&mut self) -> EnblWrGroup3ofI2c7W<PricIo23cSpec> {
        EnblWrGroup3ofI2c7W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of I2C7"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c7(&mut self) -> EnblWrGroup4ofI2c7W<PricIo23cSpec> {
        EnblWrGroup4ofI2c7W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of I2C7"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c7(&mut self) -> EnblWrGroup5ofI2c7W<PricIo23cSpec> {
        EnblWrGroup5ofI2c7W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC123CPRIC1_23C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric123cpric123c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric123cpric123c2924W<PricIo23cSpec> {
        EnblRstToleranceOfPric123cpric123c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC123CPRIC1_23C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric123cpric123c3024(
        &mut self,
    ) -> EnblWrProtOfPric123cpric123c3024W<PricIo23cSpec> {
        EnblWrProtOfPric123cpric123c3024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io23c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io23c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo23cSpec;
impl crate::RegisterSpec for PricIo23cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io23c::R`](R) reader structure"]
impl crate::Readable for PricIo23cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io23c::W`](W) writer structure"]
impl crate::Writable for PricIo23cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO23C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo23cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
