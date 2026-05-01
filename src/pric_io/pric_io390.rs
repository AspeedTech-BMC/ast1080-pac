#[doc = "Register `PRIC_IO390` reader"]
pub type R = crate::R<PricIo390Spec>;
#[doc = "Register `PRIC_IO390` writer"]
pub type W = crate::W<PricIo390Spec>;
#[doc = "Field `EnblReadGroup0OfTIMER7` reader - Enable Read Group #0 of TIMER7"]
pub type EnblReadGroup0ofTimer7R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfTIMER7` writer - Enable Read Group #0 of TIMER7"]
pub type EnblReadGroup0ofTimer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfTIMER7` reader - Enable Read Group #1 of TIMER7"]
pub type EnblReadGroup1ofTimer7R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfTIMER7` writer - Enable Read Group #1 of TIMER7"]
pub type EnblReadGroup1ofTimer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfTIMER7` reader - Enable Read Group #2 of TIMER7"]
pub type EnblReadGroup2ofTimer7R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfTIMER7` writer - Enable Read Group #2 of TIMER7"]
pub type EnblReadGroup2ofTimer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfTIMER7` reader - Enable Read Group #3 of TIMER7"]
pub type EnblReadGroup3ofTimer7R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfTIMER7` writer - Enable Read Group #3 of TIMER7"]
pub type EnblReadGroup3ofTimer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfTIMER7` reader - Enable Read Group #4 of TIMER7"]
pub type EnblReadGroup4ofTimer7R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfTIMER7` writer - Enable Read Group #4 of TIMER7"]
pub type EnblReadGroup4ofTimer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfTIMER7` reader - Enable Read Group #5 of TIMER7"]
pub type EnblReadGroup5ofTimer7R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfTIMER7` writer - Enable Read Group #5 of TIMER7"]
pub type EnblReadGroup5ofTimer7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1390PRIC1_390\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1390pric13900500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1390pric13900500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1390pric13900500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1390PRIC13900500` reader - Enable Reset Tolerance of PRIC1390PRIC1_390\\[05:00\\]"]
pub type EnblRstToleranceOfPric1390pric13900500R =
    crate::BitReader<EnblRstToleranceOfPric1390pric13900500>;
impl EnblRstToleranceOfPric1390pric13900500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1390pric13900500 {
        match self.bits {
            false => EnblRstToleranceOfPric1390pric13900500::ResetBySrst,
            true => EnblRstToleranceOfPric1390pric13900500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1390pric13900500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1390pric13900500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1390PRIC13900500` writer - Enable Reset Tolerance of PRIC1390PRIC1_390\\[05:00\\]"]
pub type EnblRstToleranceOfPric1390pric13900500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1390pric13900500>;
impl<'a, REG> EnblRstToleranceOfPric1390pric13900500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1390pric13900500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1390pric13900500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1390PRIC13900600` reader - Enable Write Protection of PRIC1390PRIC1_390\\[06:00\\]"]
pub type EnblWrProtOfPric1390pric13900600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1390PRIC13900600` writer - Enable Write Protection of PRIC1390PRIC1_390\\[06:00\\]"]
pub type EnblWrProtOfPric1390pric13900600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfOCPLOCK` reader - Enable Read Group #0 of OCP_LOCK"]
pub type EnblReadGroup0ofOcplockR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfOCPLOCK` writer - Enable Read Group #0 of OCP_LOCK"]
pub type EnblReadGroup0ofOcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfOCPLOCK` reader - Enable Read Group #1 of OCP_LOCK"]
pub type EnblReadGroup1ofOcplockR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfOCPLOCK` writer - Enable Read Group #1 of OCP_LOCK"]
pub type EnblReadGroup1ofOcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfOCPLOCK` reader - Enable Read Group #2 of OCP_LOCK"]
pub type EnblReadGroup2ofOcplockR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfOCPLOCK` writer - Enable Read Group #2 of OCP_LOCK"]
pub type EnblReadGroup2ofOcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfOCPLOCK` reader - Enable Read Group #3 of OCP_LOCK"]
pub type EnblReadGroup3ofOcplockR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfOCPLOCK` writer - Enable Read Group #3 of OCP_LOCK"]
pub type EnblReadGroup3ofOcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfOCPLOCK` reader - Enable Read Group #4 of OCP_LOCK"]
pub type EnblReadGroup4ofOcplockR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfOCPLOCK` writer - Enable Read Group #4 of OCP_LOCK"]
pub type EnblReadGroup4ofOcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfOCPLOCK` reader - Enable Read Group #5 of OCP_LOCK"]
pub type EnblReadGroup5ofOcplockR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfOCPLOCK` writer - Enable Read Group #5 of OCP_LOCK"]
pub type EnblReadGroup5ofOcplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1390PRIC1_390\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1390pric13902116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1390pric13902116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1390pric13902116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1390PRIC13902116` reader - Enable Reset Tolerance of PRIC1390PRIC1_390\\[21:16\\]"]
pub type EnblRstToleranceOfPric1390pric13902116R =
    crate::BitReader<EnblRstToleranceOfPric1390pric13902116>;
impl EnblRstToleranceOfPric1390pric13902116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1390pric13902116 {
        match self.bits {
            false => EnblRstToleranceOfPric1390pric13902116::ResetBySrst,
            true => EnblRstToleranceOfPric1390pric13902116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1390pric13902116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1390pric13902116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1390PRIC13902116` writer - Enable Reset Tolerance of PRIC1390PRIC1_390\\[21:16\\]"]
pub type EnblRstToleranceOfPric1390pric13902116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1390pric13902116>;
impl<'a, REG> EnblRstToleranceOfPric1390pric13902116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1390pric13902116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1390pric13902116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1390PRIC13902216` reader - Enable Write Protection of PRIC1390PRIC1_390\\[22:16\\]"]
pub type EnblWrProtOfPric1390pric13902216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1390PRIC13902216` writer - Enable Write Protection of PRIC1390PRIC1_390\\[22:16\\]"]
pub type EnblWrProtOfPric1390pric13902216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of TIMER7"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer7(&self) -> EnblReadGroup0ofTimer7R {
        EnblReadGroup0ofTimer7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of TIMER7"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer7(&self) -> EnblReadGroup1ofTimer7R {
        EnblReadGroup1ofTimer7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of TIMER7"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer7(&self) -> EnblReadGroup2ofTimer7R {
        EnblReadGroup2ofTimer7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of TIMER7"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer7(&self) -> EnblReadGroup3ofTimer7R {
        EnblReadGroup3ofTimer7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of TIMER7"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer7(&self) -> EnblReadGroup4ofTimer7R {
        EnblReadGroup4ofTimer7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of TIMER7"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer7(&self) -> EnblReadGroup5ofTimer7R {
        EnblReadGroup5ofTimer7R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1390PRIC1_390\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1390pric13900500(
        &self,
    ) -> EnblRstToleranceOfPric1390pric13900500R {
        EnblRstToleranceOfPric1390pric13900500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1390PRIC1_390\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1390pric13900600(&self) -> EnblWrProtOfPric1390pric13900600R {
        EnblWrProtOfPric1390pric13900600R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 16 - Enable Read Group #0 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_read_group0of_ocplock(&self) -> EnblReadGroup0ofOcplockR {
        EnblReadGroup0ofOcplockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_read_group1of_ocplock(&self) -> EnblReadGroup1ofOcplockR {
        EnblReadGroup1ofOcplockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_read_group2of_ocplock(&self) -> EnblReadGroup2ofOcplockR {
        EnblReadGroup2ofOcplockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_read_group3of_ocplock(&self) -> EnblReadGroup3ofOcplockR {
        EnblReadGroup3ofOcplockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_read_group4of_ocplock(&self) -> EnblReadGroup4ofOcplockR {
        EnblReadGroup4ofOcplockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_read_group5of_ocplock(&self) -> EnblReadGroup5ofOcplockR {
        EnblReadGroup5ofOcplockR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1390PRIC1_390\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1390pric13902116(
        &self,
    ) -> EnblRstToleranceOfPric1390pric13902116R {
        EnblRstToleranceOfPric1390pric13902116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1390PRIC1_390\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1390pric13902216(&self) -> EnblWrProtOfPric1390pric13902216R {
        EnblWrProtOfPric1390pric13902216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of TIMER7"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer7(&mut self) -> EnblReadGroup0ofTimer7W<PricIo390Spec> {
        EnblReadGroup0ofTimer7W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of TIMER7"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer7(&mut self) -> EnblReadGroup1ofTimer7W<PricIo390Spec> {
        EnblReadGroup1ofTimer7W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of TIMER7"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer7(&mut self) -> EnblReadGroup2ofTimer7W<PricIo390Spec> {
        EnblReadGroup2ofTimer7W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of TIMER7"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer7(&mut self) -> EnblReadGroup3ofTimer7W<PricIo390Spec> {
        EnblReadGroup3ofTimer7W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of TIMER7"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer7(&mut self) -> EnblReadGroup4ofTimer7W<PricIo390Spec> {
        EnblReadGroup4ofTimer7W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of TIMER7"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer7(&mut self) -> EnblReadGroup5ofTimer7W<PricIo390Spec> {
        EnblReadGroup5ofTimer7W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1390PRIC1_390\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1390pric13900500(
        &mut self,
    ) -> EnblRstToleranceOfPric1390pric13900500W<PricIo390Spec> {
        EnblRstToleranceOfPric1390pric13900500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1390PRIC1_390\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1390pric13900600(
        &mut self,
    ) -> EnblWrProtOfPric1390pric13900600W<PricIo390Spec> {
        EnblWrProtOfPric1390pric13900600W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo390Spec> {
        Reserved3W::new(self, 8)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo390Spec> {
        Reserved2W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_read_group0of_ocplock(&mut self) -> EnblReadGroup0ofOcplockW<PricIo390Spec> {
        EnblReadGroup0ofOcplockW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_read_group1of_ocplock(&mut self) -> EnblReadGroup1ofOcplockW<PricIo390Spec> {
        EnblReadGroup1ofOcplockW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_read_group2of_ocplock(&mut self) -> EnblReadGroup2ofOcplockW<PricIo390Spec> {
        EnblReadGroup2ofOcplockW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_read_group3of_ocplock(&mut self) -> EnblReadGroup3ofOcplockW<PricIo390Spec> {
        EnblReadGroup3ofOcplockW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_read_group4of_ocplock(&mut self) -> EnblReadGroup4ofOcplockW<PricIo390Spec> {
        EnblReadGroup4ofOcplockW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of OCP_LOCK"]
    #[inline(always)]
    pub fn enbl_read_group5of_ocplock(&mut self) -> EnblReadGroup5ofOcplockW<PricIo390Spec> {
        EnblReadGroup5ofOcplockW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1390PRIC1_390\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1390pric13902116(
        &mut self,
    ) -> EnblRstToleranceOfPric1390pric13902116W<PricIo390Spec> {
        EnblRstToleranceOfPric1390pric13902116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1390PRIC1_390\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1390pric13902216(
        &mut self,
    ) -> EnblWrProtOfPric1390pric13902216W<PricIo390Spec> {
        EnblWrProtOfPric1390pric13902216W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo390Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Slave Read Group Setting Register \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io390::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io390::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo390Spec;
impl crate::RegisterSpec for PricIo390Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io390::R`](R) reader structure"]
impl crate::Readable for PricIo390Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io390::W`](W) writer structure"]
impl crate::Writable for PricIo390Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO390 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo390Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
