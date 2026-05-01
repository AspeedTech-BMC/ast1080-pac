#[doc = "Register `PRIC_IO240` reader"]
pub type R = crate::R<PricIo240Spec>;
#[doc = "Register `PRIC_IO240` writer"]
pub type W = crate::W<PricIo240Spec>;
#[doc = "Field `EnblWrGroup0OfI2C8` reader - Enable Write Group #0 of I2C8"]
pub type EnblWrGroup0ofI2c8R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI2C8` writer - Enable Write Group #0 of I2C8"]
pub type EnblWrGroup0ofI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI2C8` reader - Enable Write Group #1 of I2C8"]
pub type EnblWrGroup1ofI2c8R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI2C8` writer - Enable Write Group #1 of I2C8"]
pub type EnblWrGroup1ofI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI2C8` reader - Enable Write Group #2 of I2C8"]
pub type EnblWrGroup2ofI2c8R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI2C8` writer - Enable Write Group #2 of I2C8"]
pub type EnblWrGroup2ofI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI2C8` reader - Enable Write Group #3 of I2C8"]
pub type EnblWrGroup3ofI2c8R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI2C8` writer - Enable Write Group #3 of I2C8"]
pub type EnblWrGroup3ofI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI2C8` reader - Enable Write Group #4 of I2C8"]
pub type EnblWrGroup4ofI2c8R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI2C8` writer - Enable Write Group #4 of I2C8"]
pub type EnblWrGroup4ofI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI2C8` reader - Enable Write Group #5 of I2C8"]
pub type EnblWrGroup5ofI2c8R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI2C8` writer - Enable Write Group #5 of I2C8"]
pub type EnblWrGroup5ofI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1240PRIC1_240\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1240pric12400500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1240pric12400500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1240pric12400500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1240PRIC12400500` reader - Enable Reset Tolerance of PRIC1240PRIC1_240\\[05:00\\]"]
pub type EnblRstToleranceOfPric1240pric12400500R =
    crate::BitReader<EnblRstToleranceOfPric1240pric12400500>;
impl EnblRstToleranceOfPric1240pric12400500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1240pric12400500 {
        match self.bits {
            false => EnblRstToleranceOfPric1240pric12400500::ResetBySrst,
            true => EnblRstToleranceOfPric1240pric12400500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1240pric12400500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1240pric12400500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1240PRIC12400500` writer - Enable Reset Tolerance of PRIC1240PRIC1_240\\[05:00\\]"]
pub type EnblRstToleranceOfPric1240pric12400500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1240pric12400500>;
impl<'a, REG> EnblRstToleranceOfPric1240pric12400500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1240pric12400500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1240pric12400500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1240PRIC12400600` reader - Enable Write Protection of PRIC1240PRIC1_240\\[06:00\\]"]
pub type EnblWrProtOfPric1240pric12400600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1240PRIC12400600` writer - Enable Write Protection of PRIC1240PRIC1_240\\[06:00\\]"]
pub type EnblWrProtOfPric1240pric12400600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfI2C9` reader - Enable Write Group #0 of I2C9"]
pub type EnblWrGroup0ofI2c9R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI2C9` writer - Enable Write Group #0 of I2C9"]
pub type EnblWrGroup0ofI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI2C9` reader - Enable Write Group #1 of I2C9"]
pub type EnblWrGroup1ofI2c9R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI2C9` writer - Enable Write Group #1 of I2C9"]
pub type EnblWrGroup1ofI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI2C9` reader - Enable Write Group #2 of I2C9"]
pub type EnblWrGroup2ofI2c9R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI2C9` writer - Enable Write Group #2 of I2C9"]
pub type EnblWrGroup2ofI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI2C9` reader - Enable Write Group #3 of I2C9"]
pub type EnblWrGroup3ofI2c9R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI2C9` writer - Enable Write Group #3 of I2C9"]
pub type EnblWrGroup3ofI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI2C9` reader - Enable Write Group #4 of I2C9"]
pub type EnblWrGroup4ofI2c9R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI2C9` writer - Enable Write Group #4 of I2C9"]
pub type EnblWrGroup4ofI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI2C9` reader - Enable Write Group #5 of I2C9"]
pub type EnblWrGroup5ofI2c9R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI2C9` writer - Enable Write Group #5 of I2C9"]
pub type EnblWrGroup5ofI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1240PRIC1_240\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1240pric12401308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1240pric12401308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1240pric12401308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1240PRIC12401308` reader - Enable Reset Tolerance of PRIC1240PRIC1_240\\[13:08\\]"]
pub type EnblRstToleranceOfPric1240pric12401308R =
    crate::BitReader<EnblRstToleranceOfPric1240pric12401308>;
impl EnblRstToleranceOfPric1240pric12401308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1240pric12401308 {
        match self.bits {
            false => EnblRstToleranceOfPric1240pric12401308::ResetBySrst,
            true => EnblRstToleranceOfPric1240pric12401308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1240pric12401308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1240pric12401308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1240PRIC12401308` writer - Enable Reset Tolerance of PRIC1240PRIC1_240\\[13:08\\]"]
pub type EnblRstToleranceOfPric1240pric12401308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1240pric12401308>;
impl<'a, REG> EnblRstToleranceOfPric1240pric12401308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1240pric12401308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1240pric12401308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1240PRIC12401408` reader - Enable Write Protection of PRIC1240PRIC1_240\\[14:08\\]"]
pub type EnblWrProtOfPric1240pric12401408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1240PRIC12401408` writer - Enable Write Protection of PRIC1240PRIC1_240\\[14:08\\]"]
pub type EnblWrProtOfPric1240pric12401408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfI2C10` reader - Enable Write Group #0 of I2C10"]
pub type EnblWrGroup0ofI2c10R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI2C10` writer - Enable Write Group #0 of I2C10"]
pub type EnblWrGroup0ofI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI2C10` reader - Enable Write Group #1 of I2C10"]
pub type EnblWrGroup1ofI2c10R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI2C10` writer - Enable Write Group #1 of I2C10"]
pub type EnblWrGroup1ofI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI2C10` reader - Enable Write Group #2 of I2C10"]
pub type EnblWrGroup2ofI2c10R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI2C10` writer - Enable Write Group #2 of I2C10"]
pub type EnblWrGroup2ofI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI2C10` reader - Enable Write Group #3 of I2C10"]
pub type EnblWrGroup3ofI2c10R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI2C10` writer - Enable Write Group #3 of I2C10"]
pub type EnblWrGroup3ofI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI2C10` reader - Enable Write Group #4 of I2C10"]
pub type EnblWrGroup4ofI2c10R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI2C10` writer - Enable Write Group #4 of I2C10"]
pub type EnblWrGroup4ofI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI2C10` reader - Enable Write Group #5 of I2C10"]
pub type EnblWrGroup5ofI2c10R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI2C10` writer - Enable Write Group #5 of I2C10"]
pub type EnblWrGroup5ofI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1240PRIC1_240\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1240pric12402116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1240pric12402116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1240pric12402116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1240PRIC12402116` reader - Enable Reset Tolerance of PRIC1240PRIC1_240\\[21:16\\]"]
pub type EnblRstToleranceOfPric1240pric12402116R =
    crate::BitReader<EnblRstToleranceOfPric1240pric12402116>;
impl EnblRstToleranceOfPric1240pric12402116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1240pric12402116 {
        match self.bits {
            false => EnblRstToleranceOfPric1240pric12402116::ResetBySrst,
            true => EnblRstToleranceOfPric1240pric12402116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1240pric12402116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1240pric12402116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1240PRIC12402116` writer - Enable Reset Tolerance of PRIC1240PRIC1_240\\[21:16\\]"]
pub type EnblRstToleranceOfPric1240pric12402116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1240pric12402116>;
impl<'a, REG> EnblRstToleranceOfPric1240pric12402116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1240pric12402116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1240pric12402116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1240PRIC12402216` reader - Enable Write Protection of PRIC1240PRIC1_240\\[22:16\\]"]
pub type EnblWrProtOfPric1240pric12402216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1240PRIC12402216` writer - Enable Write Protection of PRIC1240PRIC1_240\\[22:16\\]"]
pub type EnblWrProtOfPric1240pric12402216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfI2C11` reader - Enable Write Group #0 of I2C11"]
pub type EnblWrGroup0ofI2c11R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI2C11` writer - Enable Write Group #0 of I2C11"]
pub type EnblWrGroup0ofI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI2C11` reader - Enable Write Group #1 of I2C11"]
pub type EnblWrGroup1ofI2c11R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI2C11` writer - Enable Write Group #1 of I2C11"]
pub type EnblWrGroup1ofI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI2C11` reader - Enable Write Group #2 of I2C11"]
pub type EnblWrGroup2ofI2c11R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI2C11` writer - Enable Write Group #2 of I2C11"]
pub type EnblWrGroup2ofI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI2C11` reader - Enable Write Group #3 of I2C11"]
pub type EnblWrGroup3ofI2c11R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI2C11` writer - Enable Write Group #3 of I2C11"]
pub type EnblWrGroup3ofI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI2C11` reader - Enable Write Group #4 of I2C11"]
pub type EnblWrGroup4ofI2c11R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI2C11` writer - Enable Write Group #4 of I2C11"]
pub type EnblWrGroup4ofI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI2C11` reader - Enable Write Group #5 of I2C11"]
pub type EnblWrGroup5ofI2c11R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI2C11` writer - Enable Write Group #5 of I2C11"]
pub type EnblWrGroup5ofI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1240PRIC1_240\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1240pric12402924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1240pric12402924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1240pric12402924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1240PRIC12402924` reader - Enable Reset Tolerance of PRIC1240PRIC1_240\\[29:24\\]"]
pub type EnblRstToleranceOfPric1240pric12402924R =
    crate::BitReader<EnblRstToleranceOfPric1240pric12402924>;
impl EnblRstToleranceOfPric1240pric12402924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1240pric12402924 {
        match self.bits {
            false => EnblRstToleranceOfPric1240pric12402924::ResetBySrst,
            true => EnblRstToleranceOfPric1240pric12402924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1240pric12402924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1240pric12402924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1240PRIC12402924` writer - Enable Reset Tolerance of PRIC1240PRIC1_240\\[29:24\\]"]
pub type EnblRstToleranceOfPric1240pric12402924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1240pric12402924>;
impl<'a, REG> EnblRstToleranceOfPric1240pric12402924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1240pric12402924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1240pric12402924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1240PRIC12403024` reader - Enable Write Protection of PRIC1240PRIC1_240\\[30:24\\]"]
pub type EnblWrProtOfPric1240pric12403024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1240PRIC12403024` writer - Enable Write Protection of PRIC1240PRIC1_240\\[30:24\\]"]
pub type EnblWrProtOfPric1240pric12403024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of I2C8"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c8(&self) -> EnblWrGroup0ofI2c8R {
        EnblWrGroup0ofI2c8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of I2C8"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c8(&self) -> EnblWrGroup1ofI2c8R {
        EnblWrGroup1ofI2c8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of I2C8"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c8(&self) -> EnblWrGroup2ofI2c8R {
        EnblWrGroup2ofI2c8R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of I2C8"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c8(&self) -> EnblWrGroup3ofI2c8R {
        EnblWrGroup3ofI2c8R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of I2C8"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c8(&self) -> EnblWrGroup4ofI2c8R {
        EnblWrGroup4ofI2c8R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of I2C8"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c8(&self) -> EnblWrGroup5ofI2c8R {
        EnblWrGroup5ofI2c8R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1240PRIC1_240\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1240pric12400500(
        &self,
    ) -> EnblRstToleranceOfPric1240pric12400500R {
        EnblRstToleranceOfPric1240pric12400500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1240PRIC1_240\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1240pric12400600(&self) -> EnblWrProtOfPric1240pric12400600R {
        EnblWrProtOfPric1240pric12400600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of I2C9"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c9(&self) -> EnblWrGroup0ofI2c9R {
        EnblWrGroup0ofI2c9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of I2C9"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c9(&self) -> EnblWrGroup1ofI2c9R {
        EnblWrGroup1ofI2c9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of I2C9"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c9(&self) -> EnblWrGroup2ofI2c9R {
        EnblWrGroup2ofI2c9R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of I2C9"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c9(&self) -> EnblWrGroup3ofI2c9R {
        EnblWrGroup3ofI2c9R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of I2C9"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c9(&self) -> EnblWrGroup4ofI2c9R {
        EnblWrGroup4ofI2c9R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of I2C9"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c9(&self) -> EnblWrGroup5ofI2c9R {
        EnblWrGroup5ofI2c9R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1240PRIC1_240\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1240pric12401308(
        &self,
    ) -> EnblRstToleranceOfPric1240pric12401308R {
        EnblRstToleranceOfPric1240pric12401308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1240PRIC1_240\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1240pric12401408(&self) -> EnblWrProtOfPric1240pric12401408R {
        EnblWrProtOfPric1240pric12401408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of I2C10"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c10(&self) -> EnblWrGroup0ofI2c10R {
        EnblWrGroup0ofI2c10R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of I2C10"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c10(&self) -> EnblWrGroup1ofI2c10R {
        EnblWrGroup1ofI2c10R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of I2C10"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c10(&self) -> EnblWrGroup2ofI2c10R {
        EnblWrGroup2ofI2c10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of I2C10"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c10(&self) -> EnblWrGroup3ofI2c10R {
        EnblWrGroup3ofI2c10R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of I2C10"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c10(&self) -> EnblWrGroup4ofI2c10R {
        EnblWrGroup4ofI2c10R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of I2C10"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c10(&self) -> EnblWrGroup5ofI2c10R {
        EnblWrGroup5ofI2c10R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1240PRIC1_240\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1240pric12402116(
        &self,
    ) -> EnblRstToleranceOfPric1240pric12402116R {
        EnblRstToleranceOfPric1240pric12402116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1240PRIC1_240\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1240pric12402216(&self) -> EnblWrProtOfPric1240pric12402216R {
        EnblWrProtOfPric1240pric12402216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of I2C11"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c11(&self) -> EnblWrGroup0ofI2c11R {
        EnblWrGroup0ofI2c11R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of I2C11"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c11(&self) -> EnblWrGroup1ofI2c11R {
        EnblWrGroup1ofI2c11R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of I2C11"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c11(&self) -> EnblWrGroup2ofI2c11R {
        EnblWrGroup2ofI2c11R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of I2C11"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c11(&self) -> EnblWrGroup3ofI2c11R {
        EnblWrGroup3ofI2c11R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of I2C11"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c11(&self) -> EnblWrGroup4ofI2c11R {
        EnblWrGroup4ofI2c11R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of I2C11"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c11(&self) -> EnblWrGroup5ofI2c11R {
        EnblWrGroup5ofI2c11R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1240PRIC1_240\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1240pric12402924(
        &self,
    ) -> EnblRstToleranceOfPric1240pric12402924R {
        EnblRstToleranceOfPric1240pric12402924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1240PRIC1_240\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1240pric12403024(&self) -> EnblWrProtOfPric1240pric12403024R {
        EnblWrProtOfPric1240pric12403024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of I2C8"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c8(&mut self) -> EnblWrGroup0ofI2c8W<PricIo240Spec> {
        EnblWrGroup0ofI2c8W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of I2C8"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c8(&mut self) -> EnblWrGroup1ofI2c8W<PricIo240Spec> {
        EnblWrGroup1ofI2c8W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of I2C8"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c8(&mut self) -> EnblWrGroup2ofI2c8W<PricIo240Spec> {
        EnblWrGroup2ofI2c8W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of I2C8"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c8(&mut self) -> EnblWrGroup3ofI2c8W<PricIo240Spec> {
        EnblWrGroup3ofI2c8W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of I2C8"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c8(&mut self) -> EnblWrGroup4ofI2c8W<PricIo240Spec> {
        EnblWrGroup4ofI2c8W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of I2C8"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c8(&mut self) -> EnblWrGroup5ofI2c8W<PricIo240Spec> {
        EnblWrGroup5ofI2c8W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1240PRIC1_240\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1240pric12400500(
        &mut self,
    ) -> EnblRstToleranceOfPric1240pric12400500W<PricIo240Spec> {
        EnblRstToleranceOfPric1240pric12400500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1240PRIC1_240\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1240pric12400600(
        &mut self,
    ) -> EnblWrProtOfPric1240pric12400600W<PricIo240Spec> {
        EnblWrProtOfPric1240pric12400600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of I2C9"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c9(&mut self) -> EnblWrGroup0ofI2c9W<PricIo240Spec> {
        EnblWrGroup0ofI2c9W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of I2C9"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c9(&mut self) -> EnblWrGroup1ofI2c9W<PricIo240Spec> {
        EnblWrGroup1ofI2c9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of I2C9"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c9(&mut self) -> EnblWrGroup2ofI2c9W<PricIo240Spec> {
        EnblWrGroup2ofI2c9W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of I2C9"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c9(&mut self) -> EnblWrGroup3ofI2c9W<PricIo240Spec> {
        EnblWrGroup3ofI2c9W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of I2C9"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c9(&mut self) -> EnblWrGroup4ofI2c9W<PricIo240Spec> {
        EnblWrGroup4ofI2c9W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of I2C9"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c9(&mut self) -> EnblWrGroup5ofI2c9W<PricIo240Spec> {
        EnblWrGroup5ofI2c9W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1240PRIC1_240\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1240pric12401308(
        &mut self,
    ) -> EnblRstToleranceOfPric1240pric12401308W<PricIo240Spec> {
        EnblRstToleranceOfPric1240pric12401308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1240PRIC1_240\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1240pric12401408(
        &mut self,
    ) -> EnblWrProtOfPric1240pric12401408W<PricIo240Spec> {
        EnblWrProtOfPric1240pric12401408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of I2C10"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c10(&mut self) -> EnblWrGroup0ofI2c10W<PricIo240Spec> {
        EnblWrGroup0ofI2c10W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of I2C10"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c10(&mut self) -> EnblWrGroup1ofI2c10W<PricIo240Spec> {
        EnblWrGroup1ofI2c10W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of I2C10"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c10(&mut self) -> EnblWrGroup2ofI2c10W<PricIo240Spec> {
        EnblWrGroup2ofI2c10W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of I2C10"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c10(&mut self) -> EnblWrGroup3ofI2c10W<PricIo240Spec> {
        EnblWrGroup3ofI2c10W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of I2C10"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c10(&mut self) -> EnblWrGroup4ofI2c10W<PricIo240Spec> {
        EnblWrGroup4ofI2c10W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of I2C10"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c10(&mut self) -> EnblWrGroup5ofI2c10W<PricIo240Spec> {
        EnblWrGroup5ofI2c10W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1240PRIC1_240\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1240pric12402116(
        &mut self,
    ) -> EnblRstToleranceOfPric1240pric12402116W<PricIo240Spec> {
        EnblRstToleranceOfPric1240pric12402116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1240PRIC1_240\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1240pric12402216(
        &mut self,
    ) -> EnblWrProtOfPric1240pric12402216W<PricIo240Spec> {
        EnblWrProtOfPric1240pric12402216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of I2C11"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c11(&mut self) -> EnblWrGroup0ofI2c11W<PricIo240Spec> {
        EnblWrGroup0ofI2c11W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of I2C11"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c11(&mut self) -> EnblWrGroup1ofI2c11W<PricIo240Spec> {
        EnblWrGroup1ofI2c11W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of I2C11"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c11(&mut self) -> EnblWrGroup2ofI2c11W<PricIo240Spec> {
        EnblWrGroup2ofI2c11W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of I2C11"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c11(&mut self) -> EnblWrGroup3ofI2c11W<PricIo240Spec> {
        EnblWrGroup3ofI2c11W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of I2C11"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c11(&mut self) -> EnblWrGroup4ofI2c11W<PricIo240Spec> {
        EnblWrGroup4ofI2c11W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of I2C11"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c11(&mut self) -> EnblWrGroup5ofI2c11W<PricIo240Spec> {
        EnblWrGroup5ofI2c11W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1240PRIC1_240\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1240pric12402924(
        &mut self,
    ) -> EnblRstToleranceOfPric1240pric12402924W<PricIo240Spec> {
        EnblRstToleranceOfPric1240pric12402924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1240PRIC1_240\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1240pric12403024(
        &mut self,
    ) -> EnblWrProtOfPric1240pric12403024W<PricIo240Spec> {
        EnblWrProtOfPric1240pric12403024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io240::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io240::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo240Spec;
impl crate::RegisterSpec for PricIo240Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io240::R`](R) reader structure"]
impl crate::Readable for PricIo240Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io240::W`](W) writer structure"]
impl crate::Writable for PricIo240Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO240 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo240Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
