#[doc = "Register `PRIC_IO290` reader"]
pub type R = crate::R<PricIo290Spec>;
#[doc = "Register `PRIC_IO290` writer"]
pub type W = crate::W<PricIo290Spec>;
#[doc = "Field `EnblWrGroup0OfTIMER7` reader - Enable Write Group #0 of TIMER7"]
pub type EnblWrGroup0ofTimer7R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfTIMER7` writer - Enable Write Group #0 of TIMER7"]
pub type EnblWrGroup0ofTimer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfTIMER7` reader - Enable Write Group #1 of TIMER7"]
pub type EnblWrGroup1ofTimer7R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfTIMER7` writer - Enable Write Group #1 of TIMER7"]
pub type EnblWrGroup1ofTimer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfTIMER7` reader - Enable Write Group #2 of TIMER7"]
pub type EnblWrGroup2ofTimer7R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfTIMER7` writer - Enable Write Group #2 of TIMER7"]
pub type EnblWrGroup2ofTimer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfTIMER7` reader - Enable Write Group #3 of TIMER7"]
pub type EnblWrGroup3ofTimer7R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfTIMER7` writer - Enable Write Group #3 of TIMER7"]
pub type EnblWrGroup3ofTimer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfTIMER7` reader - Enable Write Group #4 of TIMER7"]
pub type EnblWrGroup4ofTimer7R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfTIMER7` writer - Enable Write Group #4 of TIMER7"]
pub type EnblWrGroup4ofTimer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfTIMER7` reader - Enable Write Group #5 of TIMER7"]
pub type EnblWrGroup5ofTimer7R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfTIMER7` writer - Enable Write Group #5 of TIMER7"]
pub type EnblWrGroup5ofTimer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1290PRIC1_290\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1290pric12900500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1290pric12900500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1290pric12900500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1290PRIC12900500` reader - Enable Reset Tolerance of PRIC1290PRIC1_290\\[05:00\\]"]
pub type EnblRstToleranceOfPric1290pric12900500R =
    crate::BitReader<EnblRstToleranceOfPric1290pric12900500>;
impl EnblRstToleranceOfPric1290pric12900500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1290pric12900500 {
        match self.bits {
            false => EnblRstToleranceOfPric1290pric12900500::ResetBySrst,
            true => EnblRstToleranceOfPric1290pric12900500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1290pric12900500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1290pric12900500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1290PRIC12900500` writer - Enable Reset Tolerance of PRIC1290PRIC1_290\\[05:00\\]"]
pub type EnblRstToleranceOfPric1290pric12900500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1290pric12900500>;
impl<'a, REG> EnblRstToleranceOfPric1290pric12900500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1290pric12900500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1290pric12900500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1290PRIC12900600` reader - Enable Write Protection of PRIC1290PRIC1_290\\[06:00\\]"]
pub type EnblWrProtOfPric1290pric12900600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1290PRIC12900600` writer - Enable Write Protection of PRIC1290PRIC1_290\\[06:00\\]"]
pub type EnblWrProtOfPric1290pric12900600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfOCPLOCK` reader - Enable Write Group #0 of OCP_LOCK"]
pub type EnblWrGroup0ofOcplockR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfOCPLOCK` writer - Enable Write Group #0 of OCP_LOCK"]
pub type EnblWrGroup0ofOcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfOCPLOCK` reader - Enable Write Group #1 of OCP_LOCK"]
pub type EnblWrGroup1ofOcplockR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfOCPLOCK` writer - Enable Write Group #1 of OCP_LOCK"]
pub type EnblWrGroup1ofOcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfOCPLOCK` reader - Enable Write Group #2 of OCP_LOCK"]
pub type EnblWrGroup2ofOcplockR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfOCPLOCK` writer - Enable Write Group #2 of OCP_LOCK"]
pub type EnblWrGroup2ofOcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfOCPLOCK` reader - Enable Write Group #3 of OCP_LOCK"]
pub type EnblWrGroup3ofOcplockR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfOCPLOCK` writer - Enable Write Group #3 of OCP_LOCK"]
pub type EnblWrGroup3ofOcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfOCPLOCK` reader - Enable Write Group #4 of OCP_LOCK"]
pub type EnblWrGroup4ofOcplockR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfOCPLOCK` writer - Enable Write Group #4 of OCP_LOCK"]
pub type EnblWrGroup4ofOcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfOCPLOCK` reader - Enable Write Group #5 of OCP_LOCK"]
pub type EnblWrGroup5ofOcplockR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfOCPLOCK` writer - Enable Write Group #5 of OCP_LOCK"]
pub type EnblWrGroup5ofOcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1290PRIC1_290\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1290pric12902116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1290pric12902116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1290pric12902116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1290PRIC12902116` reader - Enable Reset Tolerance of PRIC1290PRIC1_290\\[21:16\\]"]
pub type EnblRstToleranceOfPric1290pric12902116R =
    crate::BitReader<EnblRstToleranceOfPric1290pric12902116>;
impl EnblRstToleranceOfPric1290pric12902116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1290pric12902116 {
        match self.bits {
            false => EnblRstToleranceOfPric1290pric12902116::ResetBySrst,
            true => EnblRstToleranceOfPric1290pric12902116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1290pric12902116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1290pric12902116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1290PRIC12902116` writer - Enable Reset Tolerance of PRIC1290PRIC1_290\\[21:16\\]"]
pub type EnblRstToleranceOfPric1290pric12902116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1290pric12902116>;
impl<'a, REG> EnblRstToleranceOfPric1290pric12902116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1290pric12902116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1290pric12902116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1290PRIC12902216` reader - Enable Write Protection of PRIC1290PRIC1_290\\[22:16\\]"]
pub type EnblWrProtOfPric1290pric12902216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1290PRIC12902216` writer - Enable Write Protection of PRIC1290PRIC1_290\\[22:16\\]"]
pub type EnblWrProtOfPric1290pric12902216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of TIMER7"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer7(&self) -> EnblWrGroup0ofTimer7R {
        EnblWrGroup0ofTimer7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of TIMER7"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer7(&self) -> EnblWrGroup1ofTimer7R {
        EnblWrGroup1ofTimer7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of TIMER7"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer7(&self) -> EnblWrGroup2ofTimer7R {
        EnblWrGroup2ofTimer7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of TIMER7"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer7(&self) -> EnblWrGroup3ofTimer7R {
        EnblWrGroup3ofTimer7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of TIMER7"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer7(&self) -> EnblWrGroup4ofTimer7R {
        EnblWrGroup4ofTimer7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of TIMER7"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer7(&self) -> EnblWrGroup5ofTimer7R {
        EnblWrGroup5ofTimer7R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1290PRIC1_290\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1290pric12900500(
        &self,
    ) -> EnblRstToleranceOfPric1290pric12900500R {
        EnblRstToleranceOfPric1290pric12900500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1290PRIC1_290\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1290pric12900600(&self) -> EnblWrProtOfPric1290pric12900600R {
        EnblWrProtOfPric1290pric12900600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_wr_group0of_ocplock(&self) -> EnblWrGroup0ofOcplockR {
        EnblWrGroup0ofOcplockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_wr_group1of_ocplock(&self) -> EnblWrGroup1ofOcplockR {
        EnblWrGroup1ofOcplockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_wr_group2of_ocplock(&self) -> EnblWrGroup2ofOcplockR {
        EnblWrGroup2ofOcplockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_wr_group3of_ocplock(&self) -> EnblWrGroup3ofOcplockR {
        EnblWrGroup3ofOcplockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_wr_group4of_ocplock(&self) -> EnblWrGroup4ofOcplockR {
        EnblWrGroup4ofOcplockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_wr_group5of_ocplock(&self) -> EnblWrGroup5ofOcplockR {
        EnblWrGroup5ofOcplockR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1290PRIC1_290\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1290pric12902116(
        &self,
    ) -> EnblRstToleranceOfPric1290pric12902116R {
        EnblRstToleranceOfPric1290pric12902116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1290PRIC1_290\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1290pric12902216(&self) -> EnblWrProtOfPric1290pric12902216R {
        EnblWrProtOfPric1290pric12902216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of TIMER7"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer7(&mut self) -> EnblWrGroup0ofTimer7W<PricIo290Spec> {
        EnblWrGroup0ofTimer7W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of TIMER7"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer7(&mut self) -> EnblWrGroup1ofTimer7W<PricIo290Spec> {
        EnblWrGroup1ofTimer7W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of TIMER7"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer7(&mut self) -> EnblWrGroup2ofTimer7W<PricIo290Spec> {
        EnblWrGroup2ofTimer7W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of TIMER7"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer7(&mut self) -> EnblWrGroup3ofTimer7W<PricIo290Spec> {
        EnblWrGroup3ofTimer7W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of TIMER7"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer7(&mut self) -> EnblWrGroup4ofTimer7W<PricIo290Spec> {
        EnblWrGroup4ofTimer7W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of TIMER7"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer7(&mut self) -> EnblWrGroup5ofTimer7W<PricIo290Spec> {
        EnblWrGroup5ofTimer7W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1290PRIC1_290\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1290pric12900500(
        &mut self,
    ) -> EnblRstToleranceOfPric1290pric12900500W<PricIo290Spec> {
        EnblRstToleranceOfPric1290pric12900500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1290PRIC1_290\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1290pric12900600(
        &mut self,
    ) -> EnblWrProtOfPric1290pric12900600W<PricIo290Spec> {
        EnblWrProtOfPric1290pric12900600W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo290Spec> {
        Reserved3W::new(self, 8)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo290Spec> {
        Reserved2W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_wr_group0of_ocplock(&mut self) -> EnblWrGroup0ofOcplockW<PricIo290Spec> {
        EnblWrGroup0ofOcplockW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_wr_group1of_ocplock(&mut self) -> EnblWrGroup1ofOcplockW<PricIo290Spec> {
        EnblWrGroup1ofOcplockW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_wr_group2of_ocplock(&mut self) -> EnblWrGroup2ofOcplockW<PricIo290Spec> {
        EnblWrGroup2ofOcplockW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_wr_group3of_ocplock(&mut self) -> EnblWrGroup3ofOcplockW<PricIo290Spec> {
        EnblWrGroup3ofOcplockW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_wr_group4of_ocplock(&mut self) -> EnblWrGroup4ofOcplockW<PricIo290Spec> {
        EnblWrGroup4ofOcplockW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_wr_group5of_ocplock(&mut self) -> EnblWrGroup5ofOcplockW<PricIo290Spec> {
        EnblWrGroup5ofOcplockW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1290PRIC1_290\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1290pric12902116(
        &mut self,
    ) -> EnblRstToleranceOfPric1290pric12902116W<PricIo290Spec> {
        EnblRstToleranceOfPric1290pric12902116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1290PRIC1_290\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1290pric12902216(
        &mut self,
    ) -> EnblWrProtOfPric1290pric12902216W<PricIo290Spec> {
        EnblWrProtOfPric1290pric12902216W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo290Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Slave Write Group Setting Register \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io290::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io290::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo290Spec;
impl crate::RegisterSpec for PricIo290Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io290::R`](R) reader structure"]
impl crate::Readable for PricIo290Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io290::W`](W) writer structure"]
impl crate::Writable for PricIo290Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO290 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo290Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
