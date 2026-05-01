#[doc = "Register `PRIC_IO33C` reader"]
pub type R = crate::R<PricIo33cSpec>;
#[doc = "Register `PRIC_IO33C` writer"]
pub type W = crate::W<PricIo33cSpec>;
#[doc = "Field `EnblReadGroup0OfI2C4` reader - Enable Read Group #0 of I2C4"]
pub type EnblReadGroup0ofI2c4R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C4` writer - Enable Read Group #0 of I2C4"]
pub type EnblReadGroup0ofI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C4` reader - Enable Read Group #1 of I2C4"]
pub type EnblReadGroup1ofI2c4R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C4` writer - Enable Read Group #1 of I2C4"]
pub type EnblReadGroup1ofI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C4` reader - Enable Read Group #2 of I2C4"]
pub type EnblReadGroup2ofI2c4R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C4` writer - Enable Read Group #2 of I2C4"]
pub type EnblReadGroup2ofI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C4` reader - Enable Read Group #3 of I2C4"]
pub type EnblReadGroup3ofI2c4R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C4` writer - Enable Read Group #3 of I2C4"]
pub type EnblReadGroup3ofI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C4` reader - Enable Read Group #4 of I2C4"]
pub type EnblReadGroup4ofI2c4R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C4` writer - Enable Read Group #4 of I2C4"]
pub type EnblReadGroup4ofI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C4` reader - Enable Read Group #5 of I2C4"]
pub type EnblReadGroup5ofI2c4R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C4` writer - Enable Read Group #5 of I2C4"]
pub type EnblReadGroup5ofI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC133CPRIC1_33C\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric133cpric133c0500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric133cpric133c0500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric133cpric133c0500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC133CPRIC133C0500` reader - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[05:00\\]"]
pub type EnblRstToleranceOfPric133cpric133c0500R =
    crate::BitReader<EnblRstToleranceOfPric133cpric133c0500>;
impl EnblRstToleranceOfPric133cpric133c0500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric133cpric133c0500 {
        match self.bits {
            false => EnblRstToleranceOfPric133cpric133c0500::ResetBySrst,
            true => EnblRstToleranceOfPric133cpric133c0500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric133cpric133c0500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric133cpric133c0500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC133CPRIC133C0500` writer - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[05:00\\]"]
pub type EnblRstToleranceOfPric133cpric133c0500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric133cpric133c0500>;
impl<'a, REG> EnblRstToleranceOfPric133cpric133c0500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric133cpric133c0500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric133cpric133c0500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC133CPRIC133C0600` reader - Enable Write Protection of PRIC133CPRIC1_33C\\[06:00\\]"]
pub type EnblWrProtOfPric133cpric133c0600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC133CPRIC133C0600` writer - Enable Write Protection of PRIC133CPRIC1_33C\\[06:00\\]"]
pub type EnblWrProtOfPric133cpric133c0600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI2C5` reader - Enable Read Group #0 of I2C5"]
pub type EnblReadGroup0ofI2c5R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C5` writer - Enable Read Group #0 of I2C5"]
pub type EnblReadGroup0ofI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C5` reader - Enable Read Group #1 of I2C5"]
pub type EnblReadGroup1ofI2c5R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C5` writer - Enable Read Group #1 of I2C5"]
pub type EnblReadGroup1ofI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C5` reader - Enable Read Group #2 of I2C5"]
pub type EnblReadGroup2ofI2c5R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C5` writer - Enable Read Group #2 of I2C5"]
pub type EnblReadGroup2ofI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C5` reader - Enable Read Group #3 of I2C5"]
pub type EnblReadGroup3ofI2c5R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C5` writer - Enable Read Group #3 of I2C5"]
pub type EnblReadGroup3ofI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C5` reader - Enable Read Group #4 of I2C5"]
pub type EnblReadGroup4ofI2c5R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C5` writer - Enable Read Group #4 of I2C5"]
pub type EnblReadGroup4ofI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C5` reader - Enable Read Group #5 of I2C5"]
pub type EnblReadGroup5ofI2c5R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C5` writer - Enable Read Group #5 of I2C5"]
pub type EnblReadGroup5ofI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC133CPRIC1_33C\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric133cpric133c1308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric133cpric133c1308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric133cpric133c1308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC133CPRIC133C1308` reader - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[13:08\\]"]
pub type EnblRstToleranceOfPric133cpric133c1308R =
    crate::BitReader<EnblRstToleranceOfPric133cpric133c1308>;
impl EnblRstToleranceOfPric133cpric133c1308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric133cpric133c1308 {
        match self.bits {
            false => EnblRstToleranceOfPric133cpric133c1308::ResetBySrst,
            true => EnblRstToleranceOfPric133cpric133c1308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric133cpric133c1308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric133cpric133c1308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC133CPRIC133C1308` writer - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[13:08\\]"]
pub type EnblRstToleranceOfPric133cpric133c1308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric133cpric133c1308>;
impl<'a, REG> EnblRstToleranceOfPric133cpric133c1308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric133cpric133c1308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric133cpric133c1308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC133CPRIC133C1408` reader - Enable Write Protection of PRIC133CPRIC1_33C\\[14:08\\]"]
pub type EnblWrProtOfPric133cpric133c1408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC133CPRIC133C1408` writer - Enable Write Protection of PRIC133CPRIC1_33C\\[14:08\\]"]
pub type EnblWrProtOfPric133cpric133c1408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI2C6` reader - Enable Read Group #0 of I2C6"]
pub type EnblReadGroup0ofI2c6R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C6` writer - Enable Read Group #0 of I2C6"]
pub type EnblReadGroup0ofI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C6` reader - Enable Read Group #1 of I2C6"]
pub type EnblReadGroup1ofI2c6R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C6` writer - Enable Read Group #1 of I2C6"]
pub type EnblReadGroup1ofI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C6` reader - Enable Read Group #2 of I2C6"]
pub type EnblReadGroup2ofI2c6R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C6` writer - Enable Read Group #2 of I2C6"]
pub type EnblReadGroup2ofI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C6` reader - Enable Read Group #3 of I2C6"]
pub type EnblReadGroup3ofI2c6R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C6` writer - Enable Read Group #3 of I2C6"]
pub type EnblReadGroup3ofI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C6` reader - Enable Read Group #4 of I2C6"]
pub type EnblReadGroup4ofI2c6R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C6` writer - Enable Read Group #4 of I2C6"]
pub type EnblReadGroup4ofI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C6` reader - Enable Read Group #5 of I2C6"]
pub type EnblReadGroup5ofI2c6R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C6` writer - Enable Read Group #5 of I2C6"]
pub type EnblReadGroup5ofI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC133CPRIC1_33C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric133cpric133c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric133cpric133c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric133cpric133c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC133CPRIC133C2116` reader - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[21:16\\]"]
pub type EnblRstToleranceOfPric133cpric133c2116R =
    crate::BitReader<EnblRstToleranceOfPric133cpric133c2116>;
impl EnblRstToleranceOfPric133cpric133c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric133cpric133c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric133cpric133c2116::ResetBySrst,
            true => EnblRstToleranceOfPric133cpric133c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric133cpric133c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric133cpric133c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC133CPRIC133C2116` writer - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[21:16\\]"]
pub type EnblRstToleranceOfPric133cpric133c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric133cpric133c2116>;
impl<'a, REG> EnblRstToleranceOfPric133cpric133c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric133cpric133c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric133cpric133c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC133CPRIC133C2216` reader - Enable Write Protection of PRIC133CPRIC1_33C\\[22:16\\]"]
pub type EnblWrProtOfPric133cpric133c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC133CPRIC133C2216` writer - Enable Write Protection of PRIC133CPRIC1_33C\\[22:16\\]"]
pub type EnblWrProtOfPric133cpric133c2216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI2C7` reader - Enable Read Group #0 of I2C7"]
pub type EnblReadGroup0ofI2c7R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C7` writer - Enable Read Group #0 of I2C7"]
pub type EnblReadGroup0ofI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C7` reader - Enable Read Group #1 of I2C7"]
pub type EnblReadGroup1ofI2c7R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C7` writer - Enable Read Group #1 of I2C7"]
pub type EnblReadGroup1ofI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C7` reader - Enable Read Group #2 of I2C7"]
pub type EnblReadGroup2ofI2c7R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C7` writer - Enable Read Group #2 of I2C7"]
pub type EnblReadGroup2ofI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C7` reader - Enable Read Group #3 of I2C7"]
pub type EnblReadGroup3ofI2c7R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C7` writer - Enable Read Group #3 of I2C7"]
pub type EnblReadGroup3ofI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C7` reader - Enable Read Group #4 of I2C7"]
pub type EnblReadGroup4ofI2c7R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C7` writer - Enable Read Group #4 of I2C7"]
pub type EnblReadGroup4ofI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C7` reader - Enable Read Group #5 of I2C7"]
pub type EnblReadGroup5ofI2c7R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C7` writer - Enable Read Group #5 of I2C7"]
pub type EnblReadGroup5ofI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC133CPRIC1_33C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric133cpric133c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric133cpric133c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric133cpric133c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC133CPRIC133C2924` reader - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[29:24\\]"]
pub type EnblRstToleranceOfPric133cpric133c2924R =
    crate::BitReader<EnblRstToleranceOfPric133cpric133c2924>;
impl EnblRstToleranceOfPric133cpric133c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric133cpric133c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric133cpric133c2924::ResetBySrst,
            true => EnblRstToleranceOfPric133cpric133c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric133cpric133c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric133cpric133c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC133CPRIC133C2924` writer - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[29:24\\]"]
pub type EnblRstToleranceOfPric133cpric133c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric133cpric133c2924>;
impl<'a, REG> EnblRstToleranceOfPric133cpric133c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric133cpric133c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric133cpric133c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC133CPRIC133C3024` reader - Enable Write Protection of PRIC133CPRIC1_33C\\[30:24\\]"]
pub type EnblWrProtOfPric133cpric133c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC133CPRIC133C3024` writer - Enable Write Protection of PRIC133CPRIC1_33C\\[30:24\\]"]
pub type EnblWrProtOfPric133cpric133c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of I2C4"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c4(&self) -> EnblReadGroup0ofI2c4R {
        EnblReadGroup0ofI2c4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I2C4"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c4(&self) -> EnblReadGroup1ofI2c4R {
        EnblReadGroup1ofI2c4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I2C4"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c4(&self) -> EnblReadGroup2ofI2c4R {
        EnblReadGroup2ofI2c4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I2C4"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c4(&self) -> EnblReadGroup3ofI2c4R {
        EnblReadGroup3ofI2c4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I2C4"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c4(&self) -> EnblReadGroup4ofI2c4R {
        EnblReadGroup4ofI2c4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I2C4"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c4(&self) -> EnblReadGroup5ofI2c4R {
        EnblReadGroup5ofI2c4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric133cpric133c0500(
        &self,
    ) -> EnblRstToleranceOfPric133cpric133c0500R {
        EnblRstToleranceOfPric133cpric133c0500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC133CPRIC1_33C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric133cpric133c0600(&self) -> EnblWrProtOfPric133cpric133c0600R {
        EnblWrProtOfPric133cpric133c0600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of I2C5"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c5(&self) -> EnblReadGroup0ofI2c5R {
        EnblReadGroup0ofI2c5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of I2C5"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c5(&self) -> EnblReadGroup1ofI2c5R {
        EnblReadGroup1ofI2c5R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of I2C5"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c5(&self) -> EnblReadGroup2ofI2c5R {
        EnblReadGroup2ofI2c5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of I2C5"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c5(&self) -> EnblReadGroup3ofI2c5R {
        EnblReadGroup3ofI2c5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of I2C5"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c5(&self) -> EnblReadGroup4ofI2c5R {
        EnblReadGroup4ofI2c5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of I2C5"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c5(&self) -> EnblReadGroup5ofI2c5R {
        EnblReadGroup5ofI2c5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric133cpric133c1308(
        &self,
    ) -> EnblRstToleranceOfPric133cpric133c1308R {
        EnblRstToleranceOfPric133cpric133c1308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC133CPRIC1_33C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric133cpric133c1408(&self) -> EnblWrProtOfPric133cpric133c1408R {
        EnblWrProtOfPric133cpric133c1408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of I2C6"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c6(&self) -> EnblReadGroup0ofI2c6R {
        EnblReadGroup0ofI2c6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of I2C6"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c6(&self) -> EnblReadGroup1ofI2c6R {
        EnblReadGroup1ofI2c6R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of I2C6"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c6(&self) -> EnblReadGroup2ofI2c6R {
        EnblReadGroup2ofI2c6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of I2C6"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c6(&self) -> EnblReadGroup3ofI2c6R {
        EnblReadGroup3ofI2c6R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of I2C6"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c6(&self) -> EnblReadGroup4ofI2c6R {
        EnblReadGroup4ofI2c6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of I2C6"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c6(&self) -> EnblReadGroup5ofI2c6R {
        EnblReadGroup5ofI2c6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric133cpric133c2116(
        &self,
    ) -> EnblRstToleranceOfPric133cpric133c2116R {
        EnblRstToleranceOfPric133cpric133c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC133CPRIC1_33C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric133cpric133c2216(&self) -> EnblWrProtOfPric133cpric133c2216R {
        EnblWrProtOfPric133cpric133c2216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of I2C7"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c7(&self) -> EnblReadGroup0ofI2c7R {
        EnblReadGroup0ofI2c7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of I2C7"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c7(&self) -> EnblReadGroup1ofI2c7R {
        EnblReadGroup1ofI2c7R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of I2C7"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c7(&self) -> EnblReadGroup2ofI2c7R {
        EnblReadGroup2ofI2c7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of I2C7"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c7(&self) -> EnblReadGroup3ofI2c7R {
        EnblReadGroup3ofI2c7R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of I2C7"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c7(&self) -> EnblReadGroup4ofI2c7R {
        EnblReadGroup4ofI2c7R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of I2C7"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c7(&self) -> EnblReadGroup5ofI2c7R {
        EnblReadGroup5ofI2c7R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric133cpric133c2924(
        &self,
    ) -> EnblRstToleranceOfPric133cpric133c2924R {
        EnblRstToleranceOfPric133cpric133c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC133CPRIC1_33C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric133cpric133c3024(&self) -> EnblWrProtOfPric133cpric133c3024R {
        EnblWrProtOfPric133cpric133c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of I2C4"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c4(&mut self) -> EnblReadGroup0ofI2c4W<PricIo33cSpec> {
        EnblReadGroup0ofI2c4W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I2C4"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c4(&mut self) -> EnblReadGroup1ofI2c4W<PricIo33cSpec> {
        EnblReadGroup1ofI2c4W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I2C4"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c4(&mut self) -> EnblReadGroup2ofI2c4W<PricIo33cSpec> {
        EnblReadGroup2ofI2c4W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I2C4"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c4(&mut self) -> EnblReadGroup3ofI2c4W<PricIo33cSpec> {
        EnblReadGroup3ofI2c4W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I2C4"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c4(&mut self) -> EnblReadGroup4ofI2c4W<PricIo33cSpec> {
        EnblReadGroup4ofI2c4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I2C4"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c4(&mut self) -> EnblReadGroup5ofI2c4W<PricIo33cSpec> {
        EnblReadGroup5ofI2c4W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric133cpric133c0500(
        &mut self,
    ) -> EnblRstToleranceOfPric133cpric133c0500W<PricIo33cSpec> {
        EnblRstToleranceOfPric133cpric133c0500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC133CPRIC1_33C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric133cpric133c0600(
        &mut self,
    ) -> EnblWrProtOfPric133cpric133c0600W<PricIo33cSpec> {
        EnblWrProtOfPric133cpric133c0600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of I2C5"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c5(&mut self) -> EnblReadGroup0ofI2c5W<PricIo33cSpec> {
        EnblReadGroup0ofI2c5W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of I2C5"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c5(&mut self) -> EnblReadGroup1ofI2c5W<PricIo33cSpec> {
        EnblReadGroup1ofI2c5W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of I2C5"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c5(&mut self) -> EnblReadGroup2ofI2c5W<PricIo33cSpec> {
        EnblReadGroup2ofI2c5W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of I2C5"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c5(&mut self) -> EnblReadGroup3ofI2c5W<PricIo33cSpec> {
        EnblReadGroup3ofI2c5W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of I2C5"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c5(&mut self) -> EnblReadGroup4ofI2c5W<PricIo33cSpec> {
        EnblReadGroup4ofI2c5W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of I2C5"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c5(&mut self) -> EnblReadGroup5ofI2c5W<PricIo33cSpec> {
        EnblReadGroup5ofI2c5W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric133cpric133c1308(
        &mut self,
    ) -> EnblRstToleranceOfPric133cpric133c1308W<PricIo33cSpec> {
        EnblRstToleranceOfPric133cpric133c1308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC133CPRIC1_33C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric133cpric133c1408(
        &mut self,
    ) -> EnblWrProtOfPric133cpric133c1408W<PricIo33cSpec> {
        EnblWrProtOfPric133cpric133c1408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of I2C6"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c6(&mut self) -> EnblReadGroup0ofI2c6W<PricIo33cSpec> {
        EnblReadGroup0ofI2c6W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of I2C6"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c6(&mut self) -> EnblReadGroup1ofI2c6W<PricIo33cSpec> {
        EnblReadGroup1ofI2c6W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of I2C6"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c6(&mut self) -> EnblReadGroup2ofI2c6W<PricIo33cSpec> {
        EnblReadGroup2ofI2c6W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of I2C6"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c6(&mut self) -> EnblReadGroup3ofI2c6W<PricIo33cSpec> {
        EnblReadGroup3ofI2c6W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of I2C6"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c6(&mut self) -> EnblReadGroup4ofI2c6W<PricIo33cSpec> {
        EnblReadGroup4ofI2c6W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of I2C6"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c6(&mut self) -> EnblReadGroup5ofI2c6W<PricIo33cSpec> {
        EnblReadGroup5ofI2c6W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric133cpric133c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric133cpric133c2116W<PricIo33cSpec> {
        EnblRstToleranceOfPric133cpric133c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC133CPRIC1_33C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric133cpric133c2216(
        &mut self,
    ) -> EnblWrProtOfPric133cpric133c2216W<PricIo33cSpec> {
        EnblWrProtOfPric133cpric133c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of I2C7"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c7(&mut self) -> EnblReadGroup0ofI2c7W<PricIo33cSpec> {
        EnblReadGroup0ofI2c7W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of I2C7"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c7(&mut self) -> EnblReadGroup1ofI2c7W<PricIo33cSpec> {
        EnblReadGroup1ofI2c7W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of I2C7"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c7(&mut self) -> EnblReadGroup2ofI2c7W<PricIo33cSpec> {
        EnblReadGroup2ofI2c7W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of I2C7"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c7(&mut self) -> EnblReadGroup3ofI2c7W<PricIo33cSpec> {
        EnblReadGroup3ofI2c7W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of I2C7"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c7(&mut self) -> EnblReadGroup4ofI2c7W<PricIo33cSpec> {
        EnblReadGroup4ofI2c7W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of I2C7"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c7(&mut self) -> EnblReadGroup5ofI2c7W<PricIo33cSpec> {
        EnblReadGroup5ofI2c7W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC133CPRIC1_33C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric133cpric133c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric133cpric133c2924W<PricIo33cSpec> {
        EnblRstToleranceOfPric133cpric133c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC133CPRIC1_33C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric133cpric133c3024(
        &mut self,
    ) -> EnblWrProtOfPric133cpric133c3024W<PricIo33cSpec> {
        EnblWrProtOfPric133cpric133c3024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io33c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io33c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo33cSpec;
impl crate::RegisterSpec for PricIo33cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io33c::R`](R) reader structure"]
impl crate::Readable for PricIo33cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io33c::W`](W) writer structure"]
impl crate::Writable for PricIo33cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO33C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo33cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
