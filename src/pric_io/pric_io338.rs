#[doc = "Register `PRIC_IO338` reader"]
pub type R = crate::R<PricIo338Spec>;
#[doc = "Register `PRIC_IO338` writer"]
pub type W = crate::W<PricIo338Spec>;
#[doc = "Field `EnblReadGroup0OfI2C0` reader - Enable Read Group #0 of I2C0"]
pub type EnblReadGroup0ofI2c0R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C0` writer - Enable Read Group #0 of I2C0"]
pub type EnblReadGroup0ofI2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C0` reader - Enable Read Group #1 of I2C0"]
pub type EnblReadGroup1ofI2c0R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C0` writer - Enable Read Group #1 of I2C0"]
pub type EnblReadGroup1ofI2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C0` reader - Enable Read Group #2 of I2C0"]
pub type EnblReadGroup2ofI2c0R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C0` writer - Enable Read Group #2 of I2C0"]
pub type EnblReadGroup2ofI2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C0` reader - Enable Read Group #3 of I2C0"]
pub type EnblReadGroup3ofI2c0R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C0` writer - Enable Read Group #3 of I2C0"]
pub type EnblReadGroup3ofI2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C0` reader - Enable Read Group #4 of I2C0"]
pub type EnblReadGroup4ofI2c0R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C0` writer - Enable Read Group #4 of I2C0"]
pub type EnblReadGroup4ofI2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C0` reader - Enable Read Group #5 of I2C0"]
pub type EnblReadGroup5ofI2c0R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C0` writer - Enable Read Group #5 of I2C0"]
pub type EnblReadGroup5ofI2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1338PRIC1_338\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1338pric13380500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1338pric13380500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1338pric13380500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1338PRIC13380500` reader - Enable Reset Tolerance of PRIC1338PRIC1_338\\[05:00\\]"]
pub type EnblRstToleranceOfPric1338pric13380500R =
    crate::BitReader<EnblRstToleranceOfPric1338pric13380500>;
impl EnblRstToleranceOfPric1338pric13380500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1338pric13380500 {
        match self.bits {
            false => EnblRstToleranceOfPric1338pric13380500::ResetBySrst,
            true => EnblRstToleranceOfPric1338pric13380500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1338pric13380500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1338pric13380500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1338PRIC13380500` writer - Enable Reset Tolerance of PRIC1338PRIC1_338\\[05:00\\]"]
pub type EnblRstToleranceOfPric1338pric13380500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1338pric13380500>;
impl<'a, REG> EnblRstToleranceOfPric1338pric13380500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1338pric13380500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1338pric13380500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1338PRIC13380600` reader - Enable Write Protection of PRIC1338PRIC1_338\\[06:00\\]"]
pub type EnblWrProtOfPric1338pric13380600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1338PRIC13380600` writer - Enable Write Protection of PRIC1338PRIC1_338\\[06:00\\]"]
pub type EnblWrProtOfPric1338pric13380600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI2C1` reader - Enable Read Group #0 of I2C1"]
pub type EnblReadGroup0ofI2c1R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C1` writer - Enable Read Group #0 of I2C1"]
pub type EnblReadGroup0ofI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C1` reader - Enable Read Group #1 of I2C1"]
pub type EnblReadGroup1ofI2c1R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C1` writer - Enable Read Group #1 of I2C1"]
pub type EnblReadGroup1ofI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C1` reader - Enable Read Group #2 of I2C1"]
pub type EnblReadGroup2ofI2c1R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C1` writer - Enable Read Group #2 of I2C1"]
pub type EnblReadGroup2ofI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C1` reader - Enable Read Group #3 of I2C1"]
pub type EnblReadGroup3ofI2c1R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C1` writer - Enable Read Group #3 of I2C1"]
pub type EnblReadGroup3ofI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C1` reader - Enable Read Group #4 of I2C1"]
pub type EnblReadGroup4ofI2c1R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C1` writer - Enable Read Group #4 of I2C1"]
pub type EnblReadGroup4ofI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C1` reader - Enable Read Group #5 of I2C1"]
pub type EnblReadGroup5ofI2c1R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C1` writer - Enable Read Group #5 of I2C1"]
pub type EnblReadGroup5ofI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1338PRIC1_338\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1338pric13381308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1338pric13381308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1338pric13381308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1338PRIC13381308` reader - Enable Reset Tolerance of PRIC1338PRIC1_338\\[13:08\\]"]
pub type EnblRstToleranceOfPric1338pric13381308R =
    crate::BitReader<EnblRstToleranceOfPric1338pric13381308>;
impl EnblRstToleranceOfPric1338pric13381308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1338pric13381308 {
        match self.bits {
            false => EnblRstToleranceOfPric1338pric13381308::ResetBySrst,
            true => EnblRstToleranceOfPric1338pric13381308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1338pric13381308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1338pric13381308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1338PRIC13381308` writer - Enable Reset Tolerance of PRIC1338PRIC1_338\\[13:08\\]"]
pub type EnblRstToleranceOfPric1338pric13381308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1338pric13381308>;
impl<'a, REG> EnblRstToleranceOfPric1338pric13381308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1338pric13381308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1338pric13381308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1338PRIC13381408` reader - Enable Write Protection of PRIC1338PRIC1_338\\[14:08\\]"]
pub type EnblWrProtOfPric1338pric13381408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1338PRIC13381408` writer - Enable Write Protection of PRIC1338PRIC1_338\\[14:08\\]"]
pub type EnblWrProtOfPric1338pric13381408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI2C2` reader - Enable Read Group #0 of I2C2"]
pub type EnblReadGroup0ofI2c2R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C2` writer - Enable Read Group #0 of I2C2"]
pub type EnblReadGroup0ofI2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C2` reader - Enable Read Group #1 of I2C2"]
pub type EnblReadGroup1ofI2c2R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C2` writer - Enable Read Group #1 of I2C2"]
pub type EnblReadGroup1ofI2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C2` reader - Enable Read Group #2 of I2C2"]
pub type EnblReadGroup2ofI2c2R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C2` writer - Enable Read Group #2 of I2C2"]
pub type EnblReadGroup2ofI2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C2` reader - Enable Read Group #3 of I2C2"]
pub type EnblReadGroup3ofI2c2R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C2` writer - Enable Read Group #3 of I2C2"]
pub type EnblReadGroup3ofI2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C2` reader - Enable Read Group #4 of I2C2"]
pub type EnblReadGroup4ofI2c2R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C2` writer - Enable Read Group #4 of I2C2"]
pub type EnblReadGroup4ofI2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C2` reader - Enable Read Group #5 of I2C2"]
pub type EnblReadGroup5ofI2c2R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C2` writer - Enable Read Group #5 of I2C2"]
pub type EnblReadGroup5ofI2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1338PRIC1_338\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1338pric13382116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1338pric13382116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1338pric13382116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1338PRIC13382116` reader - Enable Reset Tolerance of PRIC1338PRIC1_338\\[21:16\\]"]
pub type EnblRstToleranceOfPric1338pric13382116R =
    crate::BitReader<EnblRstToleranceOfPric1338pric13382116>;
impl EnblRstToleranceOfPric1338pric13382116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1338pric13382116 {
        match self.bits {
            false => EnblRstToleranceOfPric1338pric13382116::ResetBySrst,
            true => EnblRstToleranceOfPric1338pric13382116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1338pric13382116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1338pric13382116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1338PRIC13382116` writer - Enable Reset Tolerance of PRIC1338PRIC1_338\\[21:16\\]"]
pub type EnblRstToleranceOfPric1338pric13382116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1338pric13382116>;
impl<'a, REG> EnblRstToleranceOfPric1338pric13382116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1338pric13382116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1338pric13382116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1338PRIC13382216` reader - Enable Write Protection of PRIC1338PRIC1_338\\[22:16\\]"]
pub type EnblWrProtOfPric1338pric13382216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1338PRIC13382216` writer - Enable Write Protection of PRIC1338PRIC1_338\\[22:16\\]"]
pub type EnblWrProtOfPric1338pric13382216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI2C3` reader - Enable Read Group #0 of I2C3"]
pub type EnblReadGroup0ofI2c3R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C3` writer - Enable Read Group #0 of I2C3"]
pub type EnblReadGroup0ofI2c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C3` reader - Enable Read Group #1 of I2C3"]
pub type EnblReadGroup1ofI2c3R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C3` writer - Enable Read Group #1 of I2C3"]
pub type EnblReadGroup1ofI2c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C3` reader - Enable Read Group #2 of I2C3"]
pub type EnblReadGroup2ofI2c3R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C3` writer - Enable Read Group #2 of I2C3"]
pub type EnblReadGroup2ofI2c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C3` reader - Enable Read Group #3 of I2C3"]
pub type EnblReadGroup3ofI2c3R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C3` writer - Enable Read Group #3 of I2C3"]
pub type EnblReadGroup3ofI2c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C3` reader - Enable Read Group #4 of I2C3"]
pub type EnblReadGroup4ofI2c3R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C3` writer - Enable Read Group #4 of I2C3"]
pub type EnblReadGroup4ofI2c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C3` reader - Enable Read Group #5 of I2C3"]
pub type EnblReadGroup5ofI2c3R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C3` writer - Enable Read Group #5 of I2C3"]
pub type EnblReadGroup5ofI2c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1338PRIC1_338\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1338pric13382924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1338pric13382924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1338pric13382924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1338PRIC13382924` reader - Enable Reset Tolerance of PRIC1338PRIC1_338\\[29:24\\]"]
pub type EnblRstToleranceOfPric1338pric13382924R =
    crate::BitReader<EnblRstToleranceOfPric1338pric13382924>;
impl EnblRstToleranceOfPric1338pric13382924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1338pric13382924 {
        match self.bits {
            false => EnblRstToleranceOfPric1338pric13382924::ResetBySrst,
            true => EnblRstToleranceOfPric1338pric13382924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1338pric13382924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1338pric13382924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1338PRIC13382924` writer - Enable Reset Tolerance of PRIC1338PRIC1_338\\[29:24\\]"]
pub type EnblRstToleranceOfPric1338pric13382924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1338pric13382924>;
impl<'a, REG> EnblRstToleranceOfPric1338pric13382924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1338pric13382924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1338pric13382924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1338PRIC13383024` reader - Enable Write Protection of PRIC1338PRIC1_338\\[30:24\\]"]
pub type EnblWrProtOfPric1338pric13383024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1338PRIC13383024` writer - Enable Write Protection of PRIC1338PRIC1_338\\[30:24\\]"]
pub type EnblWrProtOfPric1338pric13383024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of I2C0"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c0(&self) -> EnblReadGroup0ofI2c0R {
        EnblReadGroup0ofI2c0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I2C0"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c0(&self) -> EnblReadGroup1ofI2c0R {
        EnblReadGroup1ofI2c0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I2C0"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c0(&self) -> EnblReadGroup2ofI2c0R {
        EnblReadGroup2ofI2c0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I2C0"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c0(&self) -> EnblReadGroup3ofI2c0R {
        EnblReadGroup3ofI2c0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I2C0"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c0(&self) -> EnblReadGroup4ofI2c0R {
        EnblReadGroup4ofI2c0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I2C0"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c0(&self) -> EnblReadGroup5ofI2c0R {
        EnblReadGroup5ofI2c0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1338PRIC1_338\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1338pric13380500(
        &self,
    ) -> EnblRstToleranceOfPric1338pric13380500R {
        EnblRstToleranceOfPric1338pric13380500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1338PRIC1_338\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1338pric13380600(&self) -> EnblWrProtOfPric1338pric13380600R {
        EnblWrProtOfPric1338pric13380600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of I2C1"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c1(&self) -> EnblReadGroup0ofI2c1R {
        EnblReadGroup0ofI2c1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of I2C1"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c1(&self) -> EnblReadGroup1ofI2c1R {
        EnblReadGroup1ofI2c1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of I2C1"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c1(&self) -> EnblReadGroup2ofI2c1R {
        EnblReadGroup2ofI2c1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of I2C1"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c1(&self) -> EnblReadGroup3ofI2c1R {
        EnblReadGroup3ofI2c1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of I2C1"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c1(&self) -> EnblReadGroup4ofI2c1R {
        EnblReadGroup4ofI2c1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of I2C1"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c1(&self) -> EnblReadGroup5ofI2c1R {
        EnblReadGroup5ofI2c1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1338PRIC1_338\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1338pric13381308(
        &self,
    ) -> EnblRstToleranceOfPric1338pric13381308R {
        EnblRstToleranceOfPric1338pric13381308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1338PRIC1_338\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1338pric13381408(&self) -> EnblWrProtOfPric1338pric13381408R {
        EnblWrProtOfPric1338pric13381408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of I2C2"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c2(&self) -> EnblReadGroup0ofI2c2R {
        EnblReadGroup0ofI2c2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of I2C2"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c2(&self) -> EnblReadGroup1ofI2c2R {
        EnblReadGroup1ofI2c2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of I2C2"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c2(&self) -> EnblReadGroup2ofI2c2R {
        EnblReadGroup2ofI2c2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of I2C2"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c2(&self) -> EnblReadGroup3ofI2c2R {
        EnblReadGroup3ofI2c2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of I2C2"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c2(&self) -> EnblReadGroup4ofI2c2R {
        EnblReadGroup4ofI2c2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of I2C2"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c2(&self) -> EnblReadGroup5ofI2c2R {
        EnblReadGroup5ofI2c2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1338PRIC1_338\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1338pric13382116(
        &self,
    ) -> EnblRstToleranceOfPric1338pric13382116R {
        EnblRstToleranceOfPric1338pric13382116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1338PRIC1_338\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1338pric13382216(&self) -> EnblWrProtOfPric1338pric13382216R {
        EnblWrProtOfPric1338pric13382216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of I2C3"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c3(&self) -> EnblReadGroup0ofI2c3R {
        EnblReadGroup0ofI2c3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of I2C3"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c3(&self) -> EnblReadGroup1ofI2c3R {
        EnblReadGroup1ofI2c3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of I2C3"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c3(&self) -> EnblReadGroup2ofI2c3R {
        EnblReadGroup2ofI2c3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of I2C3"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c3(&self) -> EnblReadGroup3ofI2c3R {
        EnblReadGroup3ofI2c3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of I2C3"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c3(&self) -> EnblReadGroup4ofI2c3R {
        EnblReadGroup4ofI2c3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of I2C3"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c3(&self) -> EnblReadGroup5ofI2c3R {
        EnblReadGroup5ofI2c3R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1338PRIC1_338\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1338pric13382924(
        &self,
    ) -> EnblRstToleranceOfPric1338pric13382924R {
        EnblRstToleranceOfPric1338pric13382924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1338PRIC1_338\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1338pric13383024(&self) -> EnblWrProtOfPric1338pric13383024R {
        EnblWrProtOfPric1338pric13383024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of I2C0"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c0(&mut self) -> EnblReadGroup0ofI2c0W<PricIo338Spec> {
        EnblReadGroup0ofI2c0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I2C0"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c0(&mut self) -> EnblReadGroup1ofI2c0W<PricIo338Spec> {
        EnblReadGroup1ofI2c0W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I2C0"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c0(&mut self) -> EnblReadGroup2ofI2c0W<PricIo338Spec> {
        EnblReadGroup2ofI2c0W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I2C0"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c0(&mut self) -> EnblReadGroup3ofI2c0W<PricIo338Spec> {
        EnblReadGroup3ofI2c0W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I2C0"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c0(&mut self) -> EnblReadGroup4ofI2c0W<PricIo338Spec> {
        EnblReadGroup4ofI2c0W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I2C0"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c0(&mut self) -> EnblReadGroup5ofI2c0W<PricIo338Spec> {
        EnblReadGroup5ofI2c0W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1338PRIC1_338\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1338pric13380500(
        &mut self,
    ) -> EnblRstToleranceOfPric1338pric13380500W<PricIo338Spec> {
        EnblRstToleranceOfPric1338pric13380500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1338PRIC1_338\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1338pric13380600(
        &mut self,
    ) -> EnblWrProtOfPric1338pric13380600W<PricIo338Spec> {
        EnblWrProtOfPric1338pric13380600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of I2C1"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c1(&mut self) -> EnblReadGroup0ofI2c1W<PricIo338Spec> {
        EnblReadGroup0ofI2c1W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of I2C1"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c1(&mut self) -> EnblReadGroup1ofI2c1W<PricIo338Spec> {
        EnblReadGroup1ofI2c1W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of I2C1"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c1(&mut self) -> EnblReadGroup2ofI2c1W<PricIo338Spec> {
        EnblReadGroup2ofI2c1W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of I2C1"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c1(&mut self) -> EnblReadGroup3ofI2c1W<PricIo338Spec> {
        EnblReadGroup3ofI2c1W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of I2C1"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c1(&mut self) -> EnblReadGroup4ofI2c1W<PricIo338Spec> {
        EnblReadGroup4ofI2c1W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of I2C1"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c1(&mut self) -> EnblReadGroup5ofI2c1W<PricIo338Spec> {
        EnblReadGroup5ofI2c1W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1338PRIC1_338\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1338pric13381308(
        &mut self,
    ) -> EnblRstToleranceOfPric1338pric13381308W<PricIo338Spec> {
        EnblRstToleranceOfPric1338pric13381308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1338PRIC1_338\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1338pric13381408(
        &mut self,
    ) -> EnblWrProtOfPric1338pric13381408W<PricIo338Spec> {
        EnblWrProtOfPric1338pric13381408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of I2C2"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c2(&mut self) -> EnblReadGroup0ofI2c2W<PricIo338Spec> {
        EnblReadGroup0ofI2c2W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of I2C2"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c2(&mut self) -> EnblReadGroup1ofI2c2W<PricIo338Spec> {
        EnblReadGroup1ofI2c2W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of I2C2"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c2(&mut self) -> EnblReadGroup2ofI2c2W<PricIo338Spec> {
        EnblReadGroup2ofI2c2W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of I2C2"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c2(&mut self) -> EnblReadGroup3ofI2c2W<PricIo338Spec> {
        EnblReadGroup3ofI2c2W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of I2C2"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c2(&mut self) -> EnblReadGroup4ofI2c2W<PricIo338Spec> {
        EnblReadGroup4ofI2c2W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of I2C2"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c2(&mut self) -> EnblReadGroup5ofI2c2W<PricIo338Spec> {
        EnblReadGroup5ofI2c2W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1338PRIC1_338\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1338pric13382116(
        &mut self,
    ) -> EnblRstToleranceOfPric1338pric13382116W<PricIo338Spec> {
        EnblRstToleranceOfPric1338pric13382116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1338PRIC1_338\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1338pric13382216(
        &mut self,
    ) -> EnblWrProtOfPric1338pric13382216W<PricIo338Spec> {
        EnblWrProtOfPric1338pric13382216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of I2C3"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c3(&mut self) -> EnblReadGroup0ofI2c3W<PricIo338Spec> {
        EnblReadGroup0ofI2c3W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of I2C3"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c3(&mut self) -> EnblReadGroup1ofI2c3W<PricIo338Spec> {
        EnblReadGroup1ofI2c3W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of I2C3"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c3(&mut self) -> EnblReadGroup2ofI2c3W<PricIo338Spec> {
        EnblReadGroup2ofI2c3W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of I2C3"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c3(&mut self) -> EnblReadGroup3ofI2c3W<PricIo338Spec> {
        EnblReadGroup3ofI2c3W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of I2C3"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c3(&mut self) -> EnblReadGroup4ofI2c3W<PricIo338Spec> {
        EnblReadGroup4ofI2c3W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of I2C3"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c3(&mut self) -> EnblReadGroup5ofI2c3W<PricIo338Spec> {
        EnblReadGroup5ofI2c3W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1338PRIC1_338\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1338pric13382924(
        &mut self,
    ) -> EnblRstToleranceOfPric1338pric13382924W<PricIo338Spec> {
        EnblRstToleranceOfPric1338pric13382924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1338PRIC1_338\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1338pric13383024(
        &mut self,
    ) -> EnblWrProtOfPric1338pric13383024W<PricIo338Spec> {
        EnblWrProtOfPric1338pric13383024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io338::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io338::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo338Spec;
impl crate::RegisterSpec for PricIo338Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io338::R`](R) reader structure"]
impl crate::Readable for PricIo338Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io338::W`](W) writer structure"]
impl crate::Writable for PricIo338Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO338 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo338Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
