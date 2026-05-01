#[doc = "Register `PRIC_IO344` reader"]
pub type R = crate::R<PricIo344Spec>;
#[doc = "Register `PRIC_IO344` writer"]
pub type W = crate::W<PricIo344Spec>;
#[doc = "Field `EnblReadGroup0OfI2C12` reader - Enable Read Group #0 of I2C12"]
pub type EnblReadGroup0ofI2c12R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C12` writer - Enable Read Group #0 of I2C12"]
pub type EnblReadGroup0ofI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C12` reader - Enable Read Group #1 of I2C12"]
pub type EnblReadGroup1ofI2c12R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C12` writer - Enable Read Group #1 of I2C12"]
pub type EnblReadGroup1ofI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C12` reader - Enable Read Group #2 of I2C12"]
pub type EnblReadGroup2ofI2c12R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C12` writer - Enable Read Group #2 of I2C12"]
pub type EnblReadGroup2ofI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C12` reader - Enable Read Group #3 of I2C12"]
pub type EnblReadGroup3ofI2c12R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C12` writer - Enable Read Group #3 of I2C12"]
pub type EnblReadGroup3ofI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C12` reader - Enable Read Group #4 of I2C12"]
pub type EnblReadGroup4ofI2c12R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C12` writer - Enable Read Group #4 of I2C12"]
pub type EnblReadGroup4ofI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C12` reader - Enable Read Group #5 of I2C12"]
pub type EnblReadGroup5ofI2c12R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C12` writer - Enable Read Group #5 of I2C12"]
pub type EnblReadGroup5ofI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1344PRIC1_344\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1344pric13440500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1344pric13440500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1344pric13440500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1344PRIC13440500` reader - Enable Reset Tolerance of PRIC1344PRIC1_344\\[05:00\\]"]
pub type EnblRstToleranceOfPric1344pric13440500R =
    crate::BitReader<EnblRstToleranceOfPric1344pric13440500>;
impl EnblRstToleranceOfPric1344pric13440500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1344pric13440500 {
        match self.bits {
            false => EnblRstToleranceOfPric1344pric13440500::ResetBySrst,
            true => EnblRstToleranceOfPric1344pric13440500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1344pric13440500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1344pric13440500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1344PRIC13440500` writer - Enable Reset Tolerance of PRIC1344PRIC1_344\\[05:00\\]"]
pub type EnblRstToleranceOfPric1344pric13440500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1344pric13440500>;
impl<'a, REG> EnblRstToleranceOfPric1344pric13440500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1344pric13440500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1344pric13440500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1344PRIC13440600` reader - Enable Write Protection of PRIC1344PRIC1_344\\[06:00\\]"]
pub type EnblWrProtOfPric1344pric13440600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1344PRIC13440600` writer - Enable Write Protection of PRIC1344PRIC1_344\\[06:00\\]"]
pub type EnblWrProtOfPric1344pric13440600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI2C13` reader - Enable Read Group #0 of I2C13"]
pub type EnblReadGroup0ofI2c13R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2C13` writer - Enable Read Group #0 of I2C13"]
pub type EnblReadGroup0ofI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2C13` reader - Enable Read Group #1 of I2C13"]
pub type EnblReadGroup1ofI2c13R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2C13` writer - Enable Read Group #1 of I2C13"]
pub type EnblReadGroup1ofI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2C13` reader - Enable Read Group #2 of I2C13"]
pub type EnblReadGroup2ofI2c13R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2C13` writer - Enable Read Group #2 of I2C13"]
pub type EnblReadGroup2ofI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2C13` reader - Enable Read Group #3 of I2C13"]
pub type EnblReadGroup3ofI2c13R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2C13` writer - Enable Read Group #3 of I2C13"]
pub type EnblReadGroup3ofI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2C13` reader - Enable Read Group #4 of I2C13"]
pub type EnblReadGroup4ofI2c13R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2C13` writer - Enable Read Group #4 of I2C13"]
pub type EnblReadGroup4ofI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2C13` reader - Enable Read Group #5 of I2C13"]
pub type EnblReadGroup5ofI2c13R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2C13` writer - Enable Read Group #5 of I2C13"]
pub type EnblReadGroup5ofI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1344PRIC1_344\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1344pric13441308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1344pric13441308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1344pric13441308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1344PRIC13441308` reader - Enable Reset Tolerance of PRIC1344PRIC1_344\\[13:08\\]"]
pub type EnblRstToleranceOfPric1344pric13441308R =
    crate::BitReader<EnblRstToleranceOfPric1344pric13441308>;
impl EnblRstToleranceOfPric1344pric13441308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1344pric13441308 {
        match self.bits {
            false => EnblRstToleranceOfPric1344pric13441308::ResetBySrst,
            true => EnblRstToleranceOfPric1344pric13441308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1344pric13441308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1344pric13441308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1344PRIC13441308` writer - Enable Reset Tolerance of PRIC1344PRIC1_344\\[13:08\\]"]
pub type EnblRstToleranceOfPric1344pric13441308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1344pric13441308>;
impl<'a, REG> EnblRstToleranceOfPric1344pric13441308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1344pric13441308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1344pric13441308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1344PRIC13441408` reader - Enable Write Protection of PRIC1344PRIC1_344\\[14:08\\]"]
pub type EnblWrProtOfPric1344pric13441408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1344PRIC13441408` writer - Enable Write Protection of PRIC1344PRIC1_344\\[14:08\\]"]
pub type EnblWrProtOfPric1344pric13441408W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - Enable Read Group #0 of I2C12"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c12(&self) -> EnblReadGroup0ofI2c12R {
        EnblReadGroup0ofI2c12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I2C12"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c12(&self) -> EnblReadGroup1ofI2c12R {
        EnblReadGroup1ofI2c12R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I2C12"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c12(&self) -> EnblReadGroup2ofI2c12R {
        EnblReadGroup2ofI2c12R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I2C12"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c12(&self) -> EnblReadGroup3ofI2c12R {
        EnblReadGroup3ofI2c12R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I2C12"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c12(&self) -> EnblReadGroup4ofI2c12R {
        EnblReadGroup4ofI2c12R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I2C12"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c12(&self) -> EnblReadGroup5ofI2c12R {
        EnblReadGroup5ofI2c12R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1344PRIC1_344\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1344pric13440500(
        &self,
    ) -> EnblRstToleranceOfPric1344pric13440500R {
        EnblRstToleranceOfPric1344pric13440500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1344PRIC1_344\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1344pric13440600(&self) -> EnblWrProtOfPric1344pric13440600R {
        EnblWrProtOfPric1344pric13440600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of I2C13"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c13(&self) -> EnblReadGroup0ofI2c13R {
        EnblReadGroup0ofI2c13R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of I2C13"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c13(&self) -> EnblReadGroup1ofI2c13R {
        EnblReadGroup1ofI2c13R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of I2C13"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c13(&self) -> EnblReadGroup2ofI2c13R {
        EnblReadGroup2ofI2c13R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of I2C13"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c13(&self) -> EnblReadGroup3ofI2c13R {
        EnblReadGroup3ofI2c13R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of I2C13"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c13(&self) -> EnblReadGroup4ofI2c13R {
        EnblReadGroup4ofI2c13R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of I2C13"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c13(&self) -> EnblReadGroup5ofI2c13R {
        EnblReadGroup5ofI2c13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1344PRIC1_344\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1344pric13441308(
        &self,
    ) -> EnblRstToleranceOfPric1344pric13441308R {
        EnblRstToleranceOfPric1344pric13441308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1344PRIC1_344\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1344pric13441408(&self) -> EnblWrProtOfPric1344pric13441408R {
        EnblWrProtOfPric1344pric13441408R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 0 - Enable Read Group #0 of I2C12"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c12(&mut self) -> EnblReadGroup0ofI2c12W<PricIo344Spec> {
        EnblReadGroup0ofI2c12W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I2C12"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c12(&mut self) -> EnblReadGroup1ofI2c12W<PricIo344Spec> {
        EnblReadGroup1ofI2c12W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I2C12"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c12(&mut self) -> EnblReadGroup2ofI2c12W<PricIo344Spec> {
        EnblReadGroup2ofI2c12W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I2C12"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c12(&mut self) -> EnblReadGroup3ofI2c12W<PricIo344Spec> {
        EnblReadGroup3ofI2c12W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I2C12"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c12(&mut self) -> EnblReadGroup4ofI2c12W<PricIo344Spec> {
        EnblReadGroup4ofI2c12W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I2C12"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c12(&mut self) -> EnblReadGroup5ofI2c12W<PricIo344Spec> {
        EnblReadGroup5ofI2c12W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1344PRIC1_344\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1344pric13440500(
        &mut self,
    ) -> EnblRstToleranceOfPric1344pric13440500W<PricIo344Spec> {
        EnblRstToleranceOfPric1344pric13440500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1344PRIC1_344\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1344pric13440600(
        &mut self,
    ) -> EnblWrProtOfPric1344pric13440600W<PricIo344Spec> {
        EnblWrProtOfPric1344pric13440600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of I2C13"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2c13(&mut self) -> EnblReadGroup0ofI2c13W<PricIo344Spec> {
        EnblReadGroup0ofI2c13W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of I2C13"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2c13(&mut self) -> EnblReadGroup1ofI2c13W<PricIo344Spec> {
        EnblReadGroup1ofI2c13W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of I2C13"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2c13(&mut self) -> EnblReadGroup2ofI2c13W<PricIo344Spec> {
        EnblReadGroup2ofI2c13W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of I2C13"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2c13(&mut self) -> EnblReadGroup3ofI2c13W<PricIo344Spec> {
        EnblReadGroup3ofI2c13W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of I2C13"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2c13(&mut self) -> EnblReadGroup4ofI2c13W<PricIo344Spec> {
        EnblReadGroup4ofI2c13W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of I2C13"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2c13(&mut self) -> EnblReadGroup5ofI2c13W<PricIo344Spec> {
        EnblReadGroup5ofI2c13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1344PRIC1_344\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1344pric13441308(
        &mut self,
    ) -> EnblRstToleranceOfPric1344pric13441308W<PricIo344Spec> {
        EnblRstToleranceOfPric1344pric13441308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1344PRIC1_344\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1344pric13441408(
        &mut self,
    ) -> EnblWrProtOfPric1344pric13441408W<PricIo344Spec> {
        EnblWrProtOfPric1344pric13441408W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo344Spec> {
        Reserved3W::new(self, 16)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo344Spec> {
        Reserved2W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo344Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Slave Read Group Setting Register \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io344::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io344::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo344Spec;
impl crate::RegisterSpec for PricIo344Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io344::R`](R) reader structure"]
impl crate::Readable for PricIo344Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io344::W`](W) writer structure"]
impl crate::Writable for PricIo344Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO344 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo344Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
