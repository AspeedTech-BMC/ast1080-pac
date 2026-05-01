#[doc = "Register `PRIC_IO340` reader"]
pub type R = crate::R<PricIo340Spec>;
#[doc = "Register `PRIC_IO340` writer"]
pub type W = crate::W<PricIo340Spec>;
#[doc = "Field `EnblReadGroup0OfI2C8` reader - Enable Read Group #0 of I2C8"]
pub type EnblReadGroup0ofI2c8R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C8` writer - Enable Read Group #0 of I2C8"]
pub type EnblReadGroup0ofI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C8` reader - Enable Read Group #1 of I2C8"]
pub type EnblReadGroup1ofI2c8R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C8` writer - Enable Read Group #1 of I2C8"]
pub type EnblReadGroup1ofI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C8` reader - Enable Read Group #2 of I2C8"]
pub type EnblReadGroup2ofI2c8R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C8` writer - Enable Read Group #2 of I2C8"]
pub type EnblReadGroup2ofI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C8` reader - Enable Read Group #3 of I2C8"]
pub type EnblReadGroup3ofI2c8R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C8` writer - Enable Read Group #3 of I2C8"]
pub type EnblReadGroup3ofI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C8` reader - Enable Read Group #4 of I2C8"]
pub type EnblReadGroup4ofI2c8R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C8` writer - Enable Read Group #4 of I2C8"]
pub type EnblReadGroup4ofI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C8` reader - Enable Read Group #5 of I2C8"]
pub type EnblReadGroup5ofI2c8R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C8` writer - Enable Read Group #5 of I2C8"]
pub type EnblReadGroup5ofI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1340PRIC1_340\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1340pric13400500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1340pric13400500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1340pric13400500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1340PRIC13400500` reader - Enable Reset Tolerance of PRIC1340PRIC1_340\\[05:00\\]"]
pub type EnblRstToleranceOfPric1340pric13400500R =
    crate::BitReader<EnblRstToleranceOfPric1340pric13400500>;
impl EnblRstToleranceOfPric1340pric13400500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1340pric13400500 {
        match self.bits {
            false => EnblRstToleranceOfPric1340pric13400500::ResetBySrst,
            true => EnblRstToleranceOfPric1340pric13400500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1340pric13400500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1340pric13400500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1340PRIC13400500` writer - Enable Reset Tolerance of PRIC1340PRIC1_340\\[05:00\\]"]
pub type EnblRstToleranceOfPric1340pric13400500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1340pric13400500>;
impl<'a, REG> EnblRstToleranceOfPric1340pric13400500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1340pric13400500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1340pric13400500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1340PRIC13400600` reader - Enable Write Protection of PRIC1340PRIC1_340\\[06:00\\]"]
pub type EnblWrProtOfPric1340pric13400600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1340PRIC13400600` writer - Enable Write Protection of PRIC1340PRIC1_340\\[06:00\\]"]
pub type EnblWrProtOfPric1340pric13400600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI2C9` reader - Enable Read Group #0 of I2C9"]
pub type EnblReadGroup0ofI2c9R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C9` writer - Enable Read Group #0 of I2C9"]
pub type EnblReadGroup0ofI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C9` reader - Enable Read Group #1 of I2C9"]
pub type EnblReadGroup1ofI2c9R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C9` writer - Enable Read Group #1 of I2C9"]
pub type EnblReadGroup1ofI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C9` reader - Enable Read Group #2 of I2C9"]
pub type EnblReadGroup2ofI2c9R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C9` writer - Enable Read Group #2 of I2C9"]
pub type EnblReadGroup2ofI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C9` reader - Enable Read Group #3 of I2C9"]
pub type EnblReadGroup3ofI2c9R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C9` writer - Enable Read Group #3 of I2C9"]
pub type EnblReadGroup3ofI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C9` reader - Enable Read Group #4 of I2C9"]
pub type EnblReadGroup4ofI2c9R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C9` writer - Enable Read Group #4 of I2C9"]
pub type EnblReadGroup4ofI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C9` reader - Enable Read Group #5 of I2C9"]
pub type EnblReadGroup5ofI2c9R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C9` writer - Enable Read Group #5 of I2C9"]
pub type EnblReadGroup5ofI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1340PRIC1_340\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1340pric13401308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1340pric13401308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1340pric13401308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1340PRIC13401308` reader - Enable Reset Tolerance of PRIC1340PRIC1_340\\[13:08\\]"]
pub type EnblRstToleranceOfPric1340pric13401308R =
    crate::BitReader<EnblRstToleranceOfPric1340pric13401308>;
impl EnblRstToleranceOfPric1340pric13401308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1340pric13401308 {
        match self.bits {
            false => EnblRstToleranceOfPric1340pric13401308::ResetBySrst,
            true => EnblRstToleranceOfPric1340pric13401308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1340pric13401308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1340pric13401308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1340PRIC13401308` writer - Enable Reset Tolerance of PRIC1340PRIC1_340\\[13:08\\]"]
pub type EnblRstToleranceOfPric1340pric13401308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1340pric13401308>;
impl<'a, REG> EnblRstToleranceOfPric1340pric13401308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1340pric13401308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1340pric13401308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1340PRIC13401408` reader - Enable Write Protection of PRIC1340PRIC1_340\\[14:08\\]"]
pub type EnblWrProtOfPric1340pric13401408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1340PRIC13401408` writer - Enable Write Protection of PRIC1340PRIC1_340\\[14:08\\]"]
pub type EnblWrProtOfPric1340pric13401408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI2C10` reader - Enable Read Group #0 of I2C10"]
pub type EnblReadGroup0ofI2c10R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C10` writer - Enable Read Group #0 of I2C10"]
pub type EnblReadGroup0ofI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C10` reader - Enable Read Group #1 of I2C10"]
pub type EnblReadGroup1ofI2c10R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C10` writer - Enable Read Group #1 of I2C10"]
pub type EnblReadGroup1ofI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C10` reader - Enable Read Group #2 of I2C10"]
pub type EnblReadGroup2ofI2c10R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C10` writer - Enable Read Group #2 of I2C10"]
pub type EnblReadGroup2ofI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C10` reader - Enable Read Group #3 of I2C10"]
pub type EnblReadGroup3ofI2c10R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C10` writer - Enable Read Group #3 of I2C10"]
pub type EnblReadGroup3ofI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C10` reader - Enable Read Group #4 of I2C10"]
pub type EnblReadGroup4ofI2c10R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C10` writer - Enable Read Group #4 of I2C10"]
pub type EnblReadGroup4ofI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C10` reader - Enable Read Group #5 of I2C10"]
pub type EnblReadGroup5ofI2c10R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C10` writer - Enable Read Group #5 of I2C10"]
pub type EnblReadGroup5ofI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1340PRIC1_340\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1340pric13402116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1340pric13402116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1340pric13402116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1340PRIC13402116` reader - Enable Reset Tolerance of PRIC1340PRIC1_340\\[21:16\\]"]
pub type EnblRstToleranceOfPric1340pric13402116R =
    crate::BitReader<EnblRstToleranceOfPric1340pric13402116>;
impl EnblRstToleranceOfPric1340pric13402116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1340pric13402116 {
        match self.bits {
            false => EnblRstToleranceOfPric1340pric13402116::ResetBySrst,
            true => EnblRstToleranceOfPric1340pric13402116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1340pric13402116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1340pric13402116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1340PRIC13402116` writer - Enable Reset Tolerance of PRIC1340PRIC1_340\\[21:16\\]"]
pub type EnblRstToleranceOfPric1340pric13402116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1340pric13402116>;
impl<'a, REG> EnblRstToleranceOfPric1340pric13402116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1340pric13402116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1340pric13402116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1340PRIC13402216` reader - Enable Write Protection of PRIC1340PRIC1_340\\[22:16\\]"]
pub type EnblWrProtOfPric1340pric13402216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1340PRIC13402216` writer - Enable Write Protection of PRIC1340PRIC1_340\\[22:16\\]"]
pub type EnblWrProtOfPric1340pric13402216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI2C11` reader - Enable Read Group #0 of I2C11"]
pub type EnblReadGroup0ofI2c11R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C11` writer - Enable Read Group #0 of I2C11"]
pub type EnblReadGroup0ofI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C11` reader - Enable Read Group #1 of I2C11"]
pub type EnblReadGroup1ofI2c11R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C11` writer - Enable Read Group #1 of I2C11"]
pub type EnblReadGroup1ofI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C11` reader - Enable Read Group #2 of I2C11"]
pub type EnblReadGroup2ofI2c11R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C11` writer - Enable Read Group #2 of I2C11"]
pub type EnblReadGroup2ofI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C11` reader - Enable Read Group #3 of I2C11"]
pub type EnblReadGroup3ofI2c11R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C11` writer - Enable Read Group #3 of I2C11"]
pub type EnblReadGroup3ofI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C11` reader - Enable Read Group #4 of I2C11"]
pub type EnblReadGroup4ofI2c11R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C11` writer - Enable Read Group #4 of I2C11"]
pub type EnblReadGroup4ofI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C11` reader - Enable Read Group #5 of I2C11"]
pub type EnblReadGroup5ofI2c11R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C11` writer - Enable Read Group #5 of I2C11"]
pub type EnblReadGroup5ofI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1340PRIC1_340\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1340pric13402924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1340pric13402924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1340pric13402924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1340PRIC13402924` reader - Enable Reset Tolerance of PRIC1340PRIC1_340\\[29:24\\]"]
pub type EnblRstToleranceOfPric1340pric13402924R =
    crate::BitReader<EnblRstToleranceOfPric1340pric13402924>;
impl EnblRstToleranceOfPric1340pric13402924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1340pric13402924 {
        match self.bits {
            false => EnblRstToleranceOfPric1340pric13402924::ResetBySrst,
            true => EnblRstToleranceOfPric1340pric13402924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1340pric13402924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1340pric13402924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1340PRIC13402924` writer - Enable Reset Tolerance of PRIC1340PRIC1_340\\[29:24\\]"]
pub type EnblRstToleranceOfPric1340pric13402924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1340pric13402924>;
impl<'a, REG> EnblRstToleranceOfPric1340pric13402924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1340pric13402924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1340pric13402924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1340PRIC13403024` reader - Enable Write Protection of PRIC1340PRIC1_340\\[30:24\\]"]
pub type EnblWrProtOfPric1340pric13403024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1340PRIC13403024` writer - Enable Write Protection of PRIC1340PRIC1_340\\[30:24\\]"]
pub type EnblWrProtOfPric1340pric13403024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of I2C8"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c8(&self) -> EnblReadGroup0ofI2c8R {
        EnblReadGroup0ofI2c8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I2C8"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c8(&self) -> EnblReadGroup1ofI2c8R {
        EnblReadGroup1ofI2c8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I2C8"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c8(&self) -> EnblReadGroup2ofI2c8R {
        EnblReadGroup2ofI2c8R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I2C8"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c8(&self) -> EnblReadGroup3ofI2c8R {
        EnblReadGroup3ofI2c8R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I2C8"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c8(&self) -> EnblReadGroup4ofI2c8R {
        EnblReadGroup4ofI2c8R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I2C8"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c8(&self) -> EnblReadGroup5ofI2c8R {
        EnblReadGroup5ofI2c8R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1340PRIC1_340\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1340pric13400500(
        &self,
    ) -> EnblRstToleranceOfPric1340pric13400500R {
        EnblRstToleranceOfPric1340pric13400500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1340PRIC1_340\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1340pric13400600(&self) -> EnblWrProtOfPric1340pric13400600R {
        EnblWrProtOfPric1340pric13400600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of I2C9"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c9(&self) -> EnblReadGroup0ofI2c9R {
        EnblReadGroup0ofI2c9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of I2C9"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c9(&self) -> EnblReadGroup1ofI2c9R {
        EnblReadGroup1ofI2c9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of I2C9"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c9(&self) -> EnblReadGroup2ofI2c9R {
        EnblReadGroup2ofI2c9R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of I2C9"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c9(&self) -> EnblReadGroup3ofI2c9R {
        EnblReadGroup3ofI2c9R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of I2C9"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c9(&self) -> EnblReadGroup4ofI2c9R {
        EnblReadGroup4ofI2c9R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of I2C9"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c9(&self) -> EnblReadGroup5ofI2c9R {
        EnblReadGroup5ofI2c9R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1340PRIC1_340\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1340pric13401308(
        &self,
    ) -> EnblRstToleranceOfPric1340pric13401308R {
        EnblRstToleranceOfPric1340pric13401308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1340PRIC1_340\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1340pric13401408(&self) -> EnblWrProtOfPric1340pric13401408R {
        EnblWrProtOfPric1340pric13401408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of I2C10"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c10(&self) -> EnblReadGroup0ofI2c10R {
        EnblReadGroup0ofI2c10R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of I2C10"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c10(&self) -> EnblReadGroup1ofI2c10R {
        EnblReadGroup1ofI2c10R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of I2C10"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c10(&self) -> EnblReadGroup2ofI2c10R {
        EnblReadGroup2ofI2c10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of I2C10"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c10(&self) -> EnblReadGroup3ofI2c10R {
        EnblReadGroup3ofI2c10R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of I2C10"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c10(&self) -> EnblReadGroup4ofI2c10R {
        EnblReadGroup4ofI2c10R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of I2C10"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c10(&self) -> EnblReadGroup5ofI2c10R {
        EnblReadGroup5ofI2c10R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1340PRIC1_340\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1340pric13402116(
        &self,
    ) -> EnblRstToleranceOfPric1340pric13402116R {
        EnblRstToleranceOfPric1340pric13402116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1340PRIC1_340\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1340pric13402216(&self) -> EnblWrProtOfPric1340pric13402216R {
        EnblWrProtOfPric1340pric13402216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of I2C11"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c11(&self) -> EnblReadGroup0ofI2c11R {
        EnblReadGroup0ofI2c11R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of I2C11"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c11(&self) -> EnblReadGroup1ofI2c11R {
        EnblReadGroup1ofI2c11R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of I2C11"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c11(&self) -> EnblReadGroup2ofI2c11R {
        EnblReadGroup2ofI2c11R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of I2C11"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c11(&self) -> EnblReadGroup3ofI2c11R {
        EnblReadGroup3ofI2c11R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of I2C11"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c11(&self) -> EnblReadGroup4ofI2c11R {
        EnblReadGroup4ofI2c11R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of I2C11"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c11(&self) -> EnblReadGroup5ofI2c11R {
        EnblReadGroup5ofI2c11R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1340PRIC1_340\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1340pric13402924(
        &self,
    ) -> EnblRstToleranceOfPric1340pric13402924R {
        EnblRstToleranceOfPric1340pric13402924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1340PRIC1_340\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1340pric13403024(&self) -> EnblWrProtOfPric1340pric13403024R {
        EnblWrProtOfPric1340pric13403024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of I2C8"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c8(&mut self) -> EnblReadGroup0ofI2c8W<PricIo340Spec> {
        EnblReadGroup0ofI2c8W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I2C8"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c8(&mut self) -> EnblReadGroup1ofI2c8W<PricIo340Spec> {
        EnblReadGroup1ofI2c8W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I2C8"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c8(&mut self) -> EnblReadGroup2ofI2c8W<PricIo340Spec> {
        EnblReadGroup2ofI2c8W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I2C8"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c8(&mut self) -> EnblReadGroup3ofI2c8W<PricIo340Spec> {
        EnblReadGroup3ofI2c8W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I2C8"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c8(&mut self) -> EnblReadGroup4ofI2c8W<PricIo340Spec> {
        EnblReadGroup4ofI2c8W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I2C8"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c8(&mut self) -> EnblReadGroup5ofI2c8W<PricIo340Spec> {
        EnblReadGroup5ofI2c8W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1340PRIC1_340\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1340pric13400500(
        &mut self,
    ) -> EnblRstToleranceOfPric1340pric13400500W<PricIo340Spec> {
        EnblRstToleranceOfPric1340pric13400500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1340PRIC1_340\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1340pric13400600(
        &mut self,
    ) -> EnblWrProtOfPric1340pric13400600W<PricIo340Spec> {
        EnblWrProtOfPric1340pric13400600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of I2C9"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c9(&mut self) -> EnblReadGroup0ofI2c9W<PricIo340Spec> {
        EnblReadGroup0ofI2c9W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of I2C9"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c9(&mut self) -> EnblReadGroup1ofI2c9W<PricIo340Spec> {
        EnblReadGroup1ofI2c9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of I2C9"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c9(&mut self) -> EnblReadGroup2ofI2c9W<PricIo340Spec> {
        EnblReadGroup2ofI2c9W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of I2C9"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c9(&mut self) -> EnblReadGroup3ofI2c9W<PricIo340Spec> {
        EnblReadGroup3ofI2c9W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of I2C9"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c9(&mut self) -> EnblReadGroup4ofI2c9W<PricIo340Spec> {
        EnblReadGroup4ofI2c9W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of I2C9"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c9(&mut self) -> EnblReadGroup5ofI2c9W<PricIo340Spec> {
        EnblReadGroup5ofI2c9W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1340PRIC1_340\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1340pric13401308(
        &mut self,
    ) -> EnblRstToleranceOfPric1340pric13401308W<PricIo340Spec> {
        EnblRstToleranceOfPric1340pric13401308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1340PRIC1_340\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1340pric13401408(
        &mut self,
    ) -> EnblWrProtOfPric1340pric13401408W<PricIo340Spec> {
        EnblWrProtOfPric1340pric13401408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of I2C10"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c10(&mut self) -> EnblReadGroup0ofI2c10W<PricIo340Spec> {
        EnblReadGroup0ofI2c10W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of I2C10"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c10(&mut self) -> EnblReadGroup1ofI2c10W<PricIo340Spec> {
        EnblReadGroup1ofI2c10W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of I2C10"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c10(&mut self) -> EnblReadGroup2ofI2c10W<PricIo340Spec> {
        EnblReadGroup2ofI2c10W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of I2C10"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c10(&mut self) -> EnblReadGroup3ofI2c10W<PricIo340Spec> {
        EnblReadGroup3ofI2c10W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of I2C10"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c10(&mut self) -> EnblReadGroup4ofI2c10W<PricIo340Spec> {
        EnblReadGroup4ofI2c10W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of I2C10"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c10(&mut self) -> EnblReadGroup5ofI2c10W<PricIo340Spec> {
        EnblReadGroup5ofI2c10W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1340PRIC1_340\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1340pric13402116(
        &mut self,
    ) -> EnblRstToleranceOfPric1340pric13402116W<PricIo340Spec> {
        EnblRstToleranceOfPric1340pric13402116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1340PRIC1_340\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1340pric13402216(
        &mut self,
    ) -> EnblWrProtOfPric1340pric13402216W<PricIo340Spec> {
        EnblWrProtOfPric1340pric13402216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of I2C11"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c11(&mut self) -> EnblReadGroup0ofI2c11W<PricIo340Spec> {
        EnblReadGroup0ofI2c11W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of I2C11"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c11(&mut self) -> EnblReadGroup1ofI2c11W<PricIo340Spec> {
        EnblReadGroup1ofI2c11W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of I2C11"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c11(&mut self) -> EnblReadGroup2ofI2c11W<PricIo340Spec> {
        EnblReadGroup2ofI2c11W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of I2C11"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c11(&mut self) -> EnblReadGroup3ofI2c11W<PricIo340Spec> {
        EnblReadGroup3ofI2c11W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of I2C11"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c11(&mut self) -> EnblReadGroup4ofI2c11W<PricIo340Spec> {
        EnblReadGroup4ofI2c11W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of I2C11"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c11(&mut self) -> EnblReadGroup5ofI2c11W<PricIo340Spec> {
        EnblReadGroup5ofI2c11W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1340PRIC1_340\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1340pric13402924(
        &mut self,
    ) -> EnblRstToleranceOfPric1340pric13402924W<PricIo340Spec> {
        EnblRstToleranceOfPric1340pric13402924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1340PRIC1_340\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1340pric13403024(
        &mut self,
    ) -> EnblWrProtOfPric1340pric13403024W<PricIo340Spec> {
        EnblWrProtOfPric1340pric13403024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io340::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io340::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo340Spec;
impl crate::RegisterSpec for PricIo340Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io340::R`](R) reader structure"]
impl crate::Readable for PricIo340Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io340::W`](W) writer structure"]
impl crate::Writable for PricIo340Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO340 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo340Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
