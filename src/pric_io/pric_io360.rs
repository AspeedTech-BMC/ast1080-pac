#[doc = "Register `PRIC_IO360` reader"]
pub type R = crate::R<PricIo360Spec>;
#[doc = "Register `PRIC_IO360` writer"]
pub type W = crate::W<PricIo360Spec>;
#[doc = "Field `EnblReadGroup0OfI3C6` reader - Enable Read Group #0 of I3C6"]
pub type EnblReadGroup0ofI3c6R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI3C6` writer - Enable Read Group #0 of I3C6"]
pub type EnblReadGroup0ofI3c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI3C6` reader - Enable Read Group #1 of I3C6"]
pub type EnblReadGroup1ofI3c6R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI3C6` writer - Enable Read Group #1 of I3C6"]
pub type EnblReadGroup1ofI3c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI3C6` reader - Enable Read Group #2 of I3C6"]
pub type EnblReadGroup2ofI3c6R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI3C6` writer - Enable Read Group #2 of I3C6"]
pub type EnblReadGroup2ofI3c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI3C6` reader - Enable Read Group #3 of I3C6"]
pub type EnblReadGroup3ofI3c6R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI3C6` writer - Enable Read Group #3 of I3C6"]
pub type EnblReadGroup3ofI3c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI3C6` reader - Enable Read Group #4 of I3C6"]
pub type EnblReadGroup4ofI3c6R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI3C6` writer - Enable Read Group #4 of I3C6"]
pub type EnblReadGroup4ofI3c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI3C6` reader - Enable Read Group #5 of I3C6"]
pub type EnblReadGroup5ofI3c6R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI3C6` writer - Enable Read Group #5 of I3C6"]
pub type EnblReadGroup5ofI3c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1360PRIC1_360\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1360pric13600500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1360pric13600500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1360pric13600500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1360PRIC13600500` reader - Enable Reset Tolerance of PRIC1360PRIC1_360\\[05:00\\]"]
pub type EnblRstToleranceOfPric1360pric13600500R =
    crate::BitReader<EnblRstToleranceOfPric1360pric13600500>;
impl EnblRstToleranceOfPric1360pric13600500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1360pric13600500 {
        match self.bits {
            false => EnblRstToleranceOfPric1360pric13600500::ResetBySrst,
            true => EnblRstToleranceOfPric1360pric13600500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1360pric13600500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1360pric13600500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1360PRIC13600500` writer - Enable Reset Tolerance of PRIC1360PRIC1_360\\[05:00\\]"]
pub type EnblRstToleranceOfPric1360pric13600500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1360pric13600500>;
impl<'a, REG> EnblRstToleranceOfPric1360pric13600500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1360pric13600500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1360pric13600500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1360PRIC13600600` reader - Enable Write Protection of PRIC1360PRIC1_360\\[06:00\\]"]
pub type EnblWrProtOfPric1360pric13600600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1360PRIC13600600` writer - Enable Write Protection of PRIC1360PRIC1_360\\[06:00\\]"]
pub type EnblWrProtOfPric1360pric13600600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI3C7` reader - Enable Read Group #0 of I3C7"]
pub type EnblReadGroup0ofI3c7R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI3C7` writer - Enable Read Group #0 of I3C7"]
pub type EnblReadGroup0ofI3c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI3C7` reader - Enable Read Group #1 of I3C7"]
pub type EnblReadGroup1ofI3c7R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI3C7` writer - Enable Read Group #1 of I3C7"]
pub type EnblReadGroup1ofI3c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI3C7` reader - Enable Read Group #2 of I3C7"]
pub type EnblReadGroup2ofI3c7R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI3C7` writer - Enable Read Group #2 of I3C7"]
pub type EnblReadGroup2ofI3c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI3C7` reader - Enable Read Group #3 of I3C7"]
pub type EnblReadGroup3ofI3c7R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI3C7` writer - Enable Read Group #3 of I3C7"]
pub type EnblReadGroup3ofI3c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI3C7` reader - Enable Read Group #4 of I3C7"]
pub type EnblReadGroup4ofI3c7R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI3C7` writer - Enable Read Group #4 of I3C7"]
pub type EnblReadGroup4ofI3c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI3C7` reader - Enable Read Group #5 of I3C7"]
pub type EnblReadGroup5ofI3c7R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI3C7` writer - Enable Read Group #5 of I3C7"]
pub type EnblReadGroup5ofI3c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1360PRIC1_360\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1360pric13601308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1360pric13601308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1360pric13601308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1360PRIC13601308` reader - Enable Reset Tolerance of PRIC1360PRIC1_360\\[13:08\\]"]
pub type EnblRstToleranceOfPric1360pric13601308R =
    crate::BitReader<EnblRstToleranceOfPric1360pric13601308>;
impl EnblRstToleranceOfPric1360pric13601308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1360pric13601308 {
        match self.bits {
            false => EnblRstToleranceOfPric1360pric13601308::ResetBySrst,
            true => EnblRstToleranceOfPric1360pric13601308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1360pric13601308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1360pric13601308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1360PRIC13601308` writer - Enable Reset Tolerance of PRIC1360PRIC1_360\\[13:08\\]"]
pub type EnblRstToleranceOfPric1360pric13601308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1360pric13601308>;
impl<'a, REG> EnblRstToleranceOfPric1360pric13601308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1360pric13601308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1360pric13601308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1360PRIC13601408` reader - Enable Write Protection of PRIC1360PRIC1_360\\[14:08\\]"]
pub type EnblWrProtOfPric1360pric13601408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1360PRIC13601408` writer - Enable Write Protection of PRIC1360PRIC1_360\\[14:08\\]"]
pub type EnblWrProtOfPric1360pric13601408W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - Enable Read Group #0 of I3C6"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c6(&self) -> EnblReadGroup0ofI3c6R {
        EnblReadGroup0ofI3c6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I3C6"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c6(&self) -> EnblReadGroup1ofI3c6R {
        EnblReadGroup1ofI3c6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I3C6"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c6(&self) -> EnblReadGroup2ofI3c6R {
        EnblReadGroup2ofI3c6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I3C6"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c6(&self) -> EnblReadGroup3ofI3c6R {
        EnblReadGroup3ofI3c6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I3C6"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c6(&self) -> EnblReadGroup4ofI3c6R {
        EnblReadGroup4ofI3c6R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I3C6"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c6(&self) -> EnblReadGroup5ofI3c6R {
        EnblReadGroup5ofI3c6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1360PRIC1_360\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1360pric13600500(
        &self,
    ) -> EnblRstToleranceOfPric1360pric13600500R {
        EnblRstToleranceOfPric1360pric13600500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1360PRIC1_360\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1360pric13600600(&self) -> EnblWrProtOfPric1360pric13600600R {
        EnblWrProtOfPric1360pric13600600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of I3C7"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c7(&self) -> EnblReadGroup0ofI3c7R {
        EnblReadGroup0ofI3c7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of I3C7"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c7(&self) -> EnblReadGroup1ofI3c7R {
        EnblReadGroup1ofI3c7R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of I3C7"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c7(&self) -> EnblReadGroup2ofI3c7R {
        EnblReadGroup2ofI3c7R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of I3C7"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c7(&self) -> EnblReadGroup3ofI3c7R {
        EnblReadGroup3ofI3c7R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of I3C7"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c7(&self) -> EnblReadGroup4ofI3c7R {
        EnblReadGroup4ofI3c7R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of I3C7"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c7(&self) -> EnblReadGroup5ofI3c7R {
        EnblReadGroup5ofI3c7R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1360PRIC1_360\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1360pric13601308(
        &self,
    ) -> EnblRstToleranceOfPric1360pric13601308R {
        EnblRstToleranceOfPric1360pric13601308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1360PRIC1_360\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1360pric13601408(&self) -> EnblWrProtOfPric1360pric13601408R {
        EnblWrProtOfPric1360pric13601408R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 0 - Enable Read Group #0 of I3C6"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c6(&mut self) -> EnblReadGroup0ofI3c6W<PricIo360Spec> {
        EnblReadGroup0ofI3c6W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I3C6"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c6(&mut self) -> EnblReadGroup1ofI3c6W<PricIo360Spec> {
        EnblReadGroup1ofI3c6W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I3C6"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c6(&mut self) -> EnblReadGroup2ofI3c6W<PricIo360Spec> {
        EnblReadGroup2ofI3c6W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I3C6"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c6(&mut self) -> EnblReadGroup3ofI3c6W<PricIo360Spec> {
        EnblReadGroup3ofI3c6W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I3C6"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c6(&mut self) -> EnblReadGroup4ofI3c6W<PricIo360Spec> {
        EnblReadGroup4ofI3c6W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I3C6"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c6(&mut self) -> EnblReadGroup5ofI3c6W<PricIo360Spec> {
        EnblReadGroup5ofI3c6W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1360PRIC1_360\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1360pric13600500(
        &mut self,
    ) -> EnblRstToleranceOfPric1360pric13600500W<PricIo360Spec> {
        EnblRstToleranceOfPric1360pric13600500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1360PRIC1_360\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1360pric13600600(
        &mut self,
    ) -> EnblWrProtOfPric1360pric13600600W<PricIo360Spec> {
        EnblWrProtOfPric1360pric13600600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of I3C7"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c7(&mut self) -> EnblReadGroup0ofI3c7W<PricIo360Spec> {
        EnblReadGroup0ofI3c7W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of I3C7"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c7(&mut self) -> EnblReadGroup1ofI3c7W<PricIo360Spec> {
        EnblReadGroup1ofI3c7W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of I3C7"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c7(&mut self) -> EnblReadGroup2ofI3c7W<PricIo360Spec> {
        EnblReadGroup2ofI3c7W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of I3C7"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c7(&mut self) -> EnblReadGroup3ofI3c7W<PricIo360Spec> {
        EnblReadGroup3ofI3c7W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of I3C7"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c7(&mut self) -> EnblReadGroup4ofI3c7W<PricIo360Spec> {
        EnblReadGroup4ofI3c7W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of I3C7"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c7(&mut self) -> EnblReadGroup5ofI3c7W<PricIo360Spec> {
        EnblReadGroup5ofI3c7W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1360PRIC1_360\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1360pric13601308(
        &mut self,
    ) -> EnblRstToleranceOfPric1360pric13601308W<PricIo360Spec> {
        EnblRstToleranceOfPric1360pric13601308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1360PRIC1_360\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1360pric13601408(
        &mut self,
    ) -> EnblWrProtOfPric1360pric13601408W<PricIo360Spec> {
        EnblWrProtOfPric1360pric13601408W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo360Spec> {
        Reserved3W::new(self, 16)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo360Spec> {
        Reserved2W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo360Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Slave Read Group Setting Register \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io360::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io360::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo360Spec;
impl crate::RegisterSpec for PricIo360Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io360::R`](R) reader structure"]
impl crate::Readable for PricIo360Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io360::W`](W) writer structure"]
impl crate::Writable for PricIo360Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO360 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo360Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
