#[doc = "Register `PRIC_IO244` reader"]
pub type R = crate::R<PricIo244Spec>;
#[doc = "Register `PRIC_IO244` writer"]
pub type W = crate::W<PricIo244Spec>;
#[doc = "Field `EnblWrGroup0OfI2C12` reader - Enable Write Group #0 of I2C12"]
pub type EnblWrGroup0ofI2c12R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI2C12` writer - Enable Write Group #0 of I2C12"]
pub type EnblWrGroup0ofI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI2C12` reader - Enable Write Group #1 of I2C12"]
pub type EnblWrGroup1ofI2c12R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI2C12` writer - Enable Write Group #1 of I2C12"]
pub type EnblWrGroup1ofI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI2C12` reader - Enable Write Group #2 of I2C12"]
pub type EnblWrGroup2ofI2c12R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI2C12` writer - Enable Write Group #2 of I2C12"]
pub type EnblWrGroup2ofI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI2C12` reader - Enable Write Group #3 of I2C12"]
pub type EnblWrGroup3ofI2c12R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI2C12` writer - Enable Write Group #3 of I2C12"]
pub type EnblWrGroup3ofI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI2C12` reader - Enable Write Group #4 of I2C12"]
pub type EnblWrGroup4ofI2c12R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI2C12` writer - Enable Write Group #4 of I2C12"]
pub type EnblWrGroup4ofI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI2C12` reader - Enable Write Group #5 of I2C12"]
pub type EnblWrGroup5ofI2c12R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI2C12` writer - Enable Write Group #5 of I2C12"]
pub type EnblWrGroup5ofI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1244PRIC1_244\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1244pric12440500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1244pric12440500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1244pric12440500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1244PRIC12440500` reader - Enable Reset Tolerance of PRIC1244PRIC1_244\\[05:00\\]"]
pub type EnblRstToleranceOfPric1244pric12440500R =
    crate::BitReader<EnblRstToleranceOfPric1244pric12440500>;
impl EnblRstToleranceOfPric1244pric12440500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1244pric12440500 {
        match self.bits {
            false => EnblRstToleranceOfPric1244pric12440500::ResetBySrst,
            true => EnblRstToleranceOfPric1244pric12440500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1244pric12440500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1244pric12440500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1244PRIC12440500` writer - Enable Reset Tolerance of PRIC1244PRIC1_244\\[05:00\\]"]
pub type EnblRstToleranceOfPric1244pric12440500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1244pric12440500>;
impl<'a, REG> EnblRstToleranceOfPric1244pric12440500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1244pric12440500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1244pric12440500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1244PRIC12440600` reader - Enable Write Protection of PRIC1244PRIC1_244\\[06:00\\]"]
pub type EnblWrProtOfPric1244pric12440600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1244PRIC12440600` writer - Enable Write Protection of PRIC1244PRIC1_244\\[06:00\\]"]
pub type EnblWrProtOfPric1244pric12440600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfI2C13` reader - Enable Write Group #0 of I2C13"]
pub type EnblWrGroup0ofI2c13R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI2C13` writer - Enable Write Group #0 of I2C13"]
pub type EnblWrGroup0ofI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI2C13` reader - Enable Write Group #1 of I2C13"]
pub type EnblWrGroup1ofI2c13R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI2C13` writer - Enable Write Group #1 of I2C13"]
pub type EnblWrGroup1ofI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI2C13` reader - Enable Write Group #2 of I2C13"]
pub type EnblWrGroup2ofI2c13R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI2C13` writer - Enable Write Group #2 of I2C13"]
pub type EnblWrGroup2ofI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI2C13` reader - Enable Write Group #3 of I2C13"]
pub type EnblWrGroup3ofI2c13R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI2C13` writer - Enable Write Group #3 of I2C13"]
pub type EnblWrGroup3ofI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI2C13` reader - Enable Write Group #4 of I2C13"]
pub type EnblWrGroup4ofI2c13R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI2C13` writer - Enable Write Group #4 of I2C13"]
pub type EnblWrGroup4ofI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI2C13` reader - Enable Write Group #5 of I2C13"]
pub type EnblWrGroup5ofI2c13R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI2C13` writer - Enable Write Group #5 of I2C13"]
pub type EnblWrGroup5ofI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1244PRIC1_244\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1244pric12441308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1244pric12441308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1244pric12441308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1244PRIC12441308` reader - Enable Reset Tolerance of PRIC1244PRIC1_244\\[13:08\\]"]
pub type EnblRstToleranceOfPric1244pric12441308R =
    crate::BitReader<EnblRstToleranceOfPric1244pric12441308>;
impl EnblRstToleranceOfPric1244pric12441308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1244pric12441308 {
        match self.bits {
            false => EnblRstToleranceOfPric1244pric12441308::ResetBySrst,
            true => EnblRstToleranceOfPric1244pric12441308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1244pric12441308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1244pric12441308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1244PRIC12441308` writer - Enable Reset Tolerance of PRIC1244PRIC1_244\\[13:08\\]"]
pub type EnblRstToleranceOfPric1244pric12441308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1244pric12441308>;
impl<'a, REG> EnblRstToleranceOfPric1244pric12441308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1244pric12441308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1244pric12441308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1244PRIC12441408` reader - Enable Write Protection of PRIC1244PRIC1_244\\[14:08\\]"]
pub type EnblWrProtOfPric1244pric12441408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1244PRIC12441408` writer - Enable Write Protection of PRIC1244PRIC1_244\\[14:08\\]"]
pub type EnblWrProtOfPric1244pric12441408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of I2C12"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c12(&self) -> EnblWrGroup0ofI2c12R {
        EnblWrGroup0ofI2c12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of I2C12"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c12(&self) -> EnblWrGroup1ofI2c12R {
        EnblWrGroup1ofI2c12R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of I2C12"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c12(&self) -> EnblWrGroup2ofI2c12R {
        EnblWrGroup2ofI2c12R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of I2C12"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c12(&self) -> EnblWrGroup3ofI2c12R {
        EnblWrGroup3ofI2c12R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of I2C12"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c12(&self) -> EnblWrGroup4ofI2c12R {
        EnblWrGroup4ofI2c12R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of I2C12"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c12(&self) -> EnblWrGroup5ofI2c12R {
        EnblWrGroup5ofI2c12R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1244PRIC1_244\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1244pric12440500(
        &self,
    ) -> EnblRstToleranceOfPric1244pric12440500R {
        EnblRstToleranceOfPric1244pric12440500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1244PRIC1_244\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1244pric12440600(&self) -> EnblWrProtOfPric1244pric12440600R {
        EnblWrProtOfPric1244pric12440600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of I2C13"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c13(&self) -> EnblWrGroup0ofI2c13R {
        EnblWrGroup0ofI2c13R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of I2C13"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c13(&self) -> EnblWrGroup1ofI2c13R {
        EnblWrGroup1ofI2c13R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of I2C13"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c13(&self) -> EnblWrGroup2ofI2c13R {
        EnblWrGroup2ofI2c13R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of I2C13"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c13(&self) -> EnblWrGroup3ofI2c13R {
        EnblWrGroup3ofI2c13R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of I2C13"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c13(&self) -> EnblWrGroup4ofI2c13R {
        EnblWrGroup4ofI2c13R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of I2C13"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c13(&self) -> EnblWrGroup5ofI2c13R {
        EnblWrGroup5ofI2c13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1244PRIC1_244\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1244pric12441308(
        &self,
    ) -> EnblRstToleranceOfPric1244pric12441308R {
        EnblRstToleranceOfPric1244pric12441308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1244PRIC1_244\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1244pric12441408(&self) -> EnblWrProtOfPric1244pric12441408R {
        EnblWrProtOfPric1244pric12441408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of I2C12"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c12(&mut self) -> EnblWrGroup0ofI2c12W<PricIo244Spec> {
        EnblWrGroup0ofI2c12W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of I2C12"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c12(&mut self) -> EnblWrGroup1ofI2c12W<PricIo244Spec> {
        EnblWrGroup1ofI2c12W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of I2C12"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c12(&mut self) -> EnblWrGroup2ofI2c12W<PricIo244Spec> {
        EnblWrGroup2ofI2c12W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of I2C12"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c12(&mut self) -> EnblWrGroup3ofI2c12W<PricIo244Spec> {
        EnblWrGroup3ofI2c12W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of I2C12"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c12(&mut self) -> EnblWrGroup4ofI2c12W<PricIo244Spec> {
        EnblWrGroup4ofI2c12W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of I2C12"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c12(&mut self) -> EnblWrGroup5ofI2c12W<PricIo244Spec> {
        EnblWrGroup5ofI2c12W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1244PRIC1_244\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1244pric12440500(
        &mut self,
    ) -> EnblRstToleranceOfPric1244pric12440500W<PricIo244Spec> {
        EnblRstToleranceOfPric1244pric12440500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1244PRIC1_244\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1244pric12440600(
        &mut self,
    ) -> EnblWrProtOfPric1244pric12440600W<PricIo244Spec> {
        EnblWrProtOfPric1244pric12440600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of I2C13"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2c13(&mut self) -> EnblWrGroup0ofI2c13W<PricIo244Spec> {
        EnblWrGroup0ofI2c13W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of I2C13"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2c13(&mut self) -> EnblWrGroup1ofI2c13W<PricIo244Spec> {
        EnblWrGroup1ofI2c13W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of I2C13"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2c13(&mut self) -> EnblWrGroup2ofI2c13W<PricIo244Spec> {
        EnblWrGroup2ofI2c13W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of I2C13"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2c13(&mut self) -> EnblWrGroup3ofI2c13W<PricIo244Spec> {
        EnblWrGroup3ofI2c13W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of I2C13"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2c13(&mut self) -> EnblWrGroup4ofI2c13W<PricIo244Spec> {
        EnblWrGroup4ofI2c13W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of I2C13"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2c13(&mut self) -> EnblWrGroup5ofI2c13W<PricIo244Spec> {
        EnblWrGroup5ofI2c13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1244PRIC1_244\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1244pric12441308(
        &mut self,
    ) -> EnblRstToleranceOfPric1244pric12441308W<PricIo244Spec> {
        EnblRstToleranceOfPric1244pric12441308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1244PRIC1_244\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1244pric12441408(
        &mut self,
    ) -> EnblWrProtOfPric1244pric12441408W<PricIo244Spec> {
        EnblWrProtOfPric1244pric12441408W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo244Spec> {
        Reserved3W::new(self, 16)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo244Spec> {
        Reserved2W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo244Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Slave Write Group Setting Register \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io244::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io244::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo244Spec;
impl crate::RegisterSpec for PricIo244Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io244::R`](R) reader structure"]
impl crate::Readable for PricIo244Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io244::W`](W) writer structure"]
impl crate::Writable for PricIo244Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO244 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo244Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
