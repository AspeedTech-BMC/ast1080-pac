#[doc = "Register `PRIC_IO38C` reader"]
pub type R = crate::R<PricIo38cSpec>;
#[doc = "Register `PRIC_IO38C` writer"]
pub type W = crate::W<PricIo38cSpec>;
#[doc = "Field `EnblReadGroup0OfTIMER3` reader - Enable Read Group #0 of TIMER3"]
pub type EnblReadGroup0ofTimer3R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfTIMER3` writer - Enable Read Group #0 of TIMER3"]
pub type EnblReadGroup0ofTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfTIMER3` reader - Enable Read Group #1 of TIMER3"]
pub type EnblReadGroup1ofTimer3R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfTIMER3` writer - Enable Read Group #1 of TIMER3"]
pub type EnblReadGroup1ofTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfTIMER3` reader - Enable Read Group #2 of TIMER3"]
pub type EnblReadGroup2ofTimer3R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfTIMER3` writer - Enable Read Group #2 of TIMER3"]
pub type EnblReadGroup2ofTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfTIMER3` reader - Enable Read Group #3 of TIMER3"]
pub type EnblReadGroup3ofTimer3R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfTIMER3` writer - Enable Read Group #3 of TIMER3"]
pub type EnblReadGroup3ofTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfTIMER3` reader - Enable Read Group #4 of TIMER3"]
pub type EnblReadGroup4ofTimer3R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfTIMER3` writer - Enable Read Group #4 of TIMER3"]
pub type EnblReadGroup4ofTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfTIMER3` reader - Enable Read Group #5 of TIMER3"]
pub type EnblReadGroup5ofTimer3R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfTIMER3` writer - Enable Read Group #5 of TIMER3"]
pub type EnblReadGroup5ofTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC138CPRIC1_38C\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric138cpric138c0500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric138cpric138c0500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric138cpric138c0500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC138CPRIC138C0500` reader - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[05:00\\]"]
pub type EnblRstToleranceOfPric138cpric138c0500R =
    crate::BitReader<EnblRstToleranceOfPric138cpric138c0500>;
impl EnblRstToleranceOfPric138cpric138c0500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric138cpric138c0500 {
        match self.bits {
            false => EnblRstToleranceOfPric138cpric138c0500::ResetBySrst,
            true => EnblRstToleranceOfPric138cpric138c0500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric138cpric138c0500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric138cpric138c0500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC138CPRIC138C0500` writer - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[05:00\\]"]
pub type EnblRstToleranceOfPric138cpric138c0500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric138cpric138c0500>;
impl<'a, REG> EnblRstToleranceOfPric138cpric138c0500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric138cpric138c0500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric138cpric138c0500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC138CPRIC138C0600` reader - Enable Write Protection of PRIC138CPRIC1_38C\\[06:00\\]"]
pub type EnblWrProtOfPric138cpric138c0600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC138CPRIC138C0600` writer - Enable Write Protection of PRIC138CPRIC1_38C\\[06:00\\]"]
pub type EnblWrProtOfPric138cpric138c0600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfTIMER4` reader - Enable Read Group #0 of TIMER4"]
pub type EnblReadGroup0ofTimer4R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfTIMER4` writer - Enable Read Group #0 of TIMER4"]
pub type EnblReadGroup0ofTimer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfTIMER4` reader - Enable Read Group #1 of TIMER4"]
pub type EnblReadGroup1ofTimer4R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfTIMER4` writer - Enable Read Group #1 of TIMER4"]
pub type EnblReadGroup1ofTimer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfTIMER4` reader - Enable Read Group #2 of TIMER4"]
pub type EnblReadGroup2ofTimer4R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfTIMER4` writer - Enable Read Group #2 of TIMER4"]
pub type EnblReadGroup2ofTimer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfTIMER4` reader - Enable Read Group #3 of TIMER4"]
pub type EnblReadGroup3ofTimer4R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfTIMER4` writer - Enable Read Group #3 of TIMER4"]
pub type EnblReadGroup3ofTimer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfTIMER4` reader - Enable Read Group #4 of TIMER4"]
pub type EnblReadGroup4ofTimer4R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfTIMER4` writer - Enable Read Group #4 of TIMER4"]
pub type EnblReadGroup4ofTimer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfTIMER4` reader - Enable Read Group #5 of TIMER4"]
pub type EnblReadGroup5ofTimer4R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfTIMER4` writer - Enable Read Group #5 of TIMER4"]
pub type EnblReadGroup5ofTimer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC138CPRIC1_38C\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric138cpric138c1308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric138cpric138c1308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric138cpric138c1308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC138CPRIC138C1308` reader - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[13:08\\]"]
pub type EnblRstToleranceOfPric138cpric138c1308R =
    crate::BitReader<EnblRstToleranceOfPric138cpric138c1308>;
impl EnblRstToleranceOfPric138cpric138c1308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric138cpric138c1308 {
        match self.bits {
            false => EnblRstToleranceOfPric138cpric138c1308::ResetBySrst,
            true => EnblRstToleranceOfPric138cpric138c1308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric138cpric138c1308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric138cpric138c1308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC138CPRIC138C1308` writer - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[13:08\\]"]
pub type EnblRstToleranceOfPric138cpric138c1308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric138cpric138c1308>;
impl<'a, REG> EnblRstToleranceOfPric138cpric138c1308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric138cpric138c1308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric138cpric138c1308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC138CPRIC138C1408` reader - Enable Write Protection of PRIC138CPRIC1_38C\\[14:08\\]"]
pub type EnblWrProtOfPric138cpric138c1408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC138CPRIC138C1408` writer - Enable Write Protection of PRIC138CPRIC1_38C\\[14:08\\]"]
pub type EnblWrProtOfPric138cpric138c1408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfTIMER5` reader - Enable Read Group #0 of TIMER5"]
pub type EnblReadGroup0ofTimer5R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfTIMER5` writer - Enable Read Group #0 of TIMER5"]
pub type EnblReadGroup0ofTimer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfTIMER5` reader - Enable Read Group #1 of TIMER5"]
pub type EnblReadGroup1ofTimer5R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfTIMER5` writer - Enable Read Group #1 of TIMER5"]
pub type EnblReadGroup1ofTimer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfTIMER5` reader - Enable Read Group #2 of TIMER5"]
pub type EnblReadGroup2ofTimer5R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfTIMER5` writer - Enable Read Group #2 of TIMER5"]
pub type EnblReadGroup2ofTimer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfTIMER5` reader - Enable Read Group #3 of TIMER5"]
pub type EnblReadGroup3ofTimer5R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfTIMER5` writer - Enable Read Group #3 of TIMER5"]
pub type EnblReadGroup3ofTimer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfTIMER5` reader - Enable Read Group #4 of TIMER5"]
pub type EnblReadGroup4ofTimer5R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfTIMER5` writer - Enable Read Group #4 of TIMER5"]
pub type EnblReadGroup4ofTimer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfTIMER5` reader - Enable Read Group #5 of TIMER5"]
pub type EnblReadGroup5ofTimer5R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfTIMER5` writer - Enable Read Group #5 of TIMER5"]
pub type EnblReadGroup5ofTimer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC138CPRIC1_38C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric138cpric138c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric138cpric138c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric138cpric138c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC138CPRIC138C2116` reader - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[21:16\\]"]
pub type EnblRstToleranceOfPric138cpric138c2116R =
    crate::BitReader<EnblRstToleranceOfPric138cpric138c2116>;
impl EnblRstToleranceOfPric138cpric138c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric138cpric138c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric138cpric138c2116::ResetBySrst,
            true => EnblRstToleranceOfPric138cpric138c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric138cpric138c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric138cpric138c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC138CPRIC138C2116` writer - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[21:16\\]"]
pub type EnblRstToleranceOfPric138cpric138c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric138cpric138c2116>;
impl<'a, REG> EnblRstToleranceOfPric138cpric138c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric138cpric138c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric138cpric138c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC138CPRIC138C2216` reader - Enable Write Protection of PRIC138CPRIC1_38C\\[22:16\\]"]
pub type EnblWrProtOfPric138cpric138c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC138CPRIC138C2216` writer - Enable Write Protection of PRIC138CPRIC1_38C\\[22:16\\]"]
pub type EnblWrProtOfPric138cpric138c2216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfTIMER6` reader - Enable Read Group #0 of TIMER6"]
pub type EnblReadGroup0ofTimer6R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfTIMER6` writer - Enable Read Group #0 of TIMER6"]
pub type EnblReadGroup0ofTimer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfTIMER6` reader - Enable Read Group #1 of TIMER6"]
pub type EnblReadGroup1ofTimer6R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfTIMER6` writer - Enable Read Group #1 of TIMER6"]
pub type EnblReadGroup1ofTimer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfTIMER6` reader - Enable Read Group #2 of TIMER6"]
pub type EnblReadGroup2ofTimer6R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfTIMER6` writer - Enable Read Group #2 of TIMER6"]
pub type EnblReadGroup2ofTimer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfTIMER6` reader - Enable Read Group #3 of TIMER6"]
pub type EnblReadGroup3ofTimer6R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfTIMER6` writer - Enable Read Group #3 of TIMER6"]
pub type EnblReadGroup3ofTimer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfTIMER6` reader - Enable Read Group #4 of TIMER6"]
pub type EnblReadGroup4ofTimer6R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfTIMER6` writer - Enable Read Group #4 of TIMER6"]
pub type EnblReadGroup4ofTimer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfTIMER6` reader - Enable Read Group #5 of TIMER6"]
pub type EnblReadGroup5ofTimer6R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfTIMER6` writer - Enable Read Group #5 of TIMER6"]
pub type EnblReadGroup5ofTimer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC138CPRIC1_38C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric138cpric138c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric138cpric138c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric138cpric138c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC138CPRIC138C2924` reader - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[29:24\\]"]
pub type EnblRstToleranceOfPric138cpric138c2924R =
    crate::BitReader<EnblRstToleranceOfPric138cpric138c2924>;
impl EnblRstToleranceOfPric138cpric138c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric138cpric138c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric138cpric138c2924::ResetBySrst,
            true => EnblRstToleranceOfPric138cpric138c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric138cpric138c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric138cpric138c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC138CPRIC138C2924` writer - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[29:24\\]"]
pub type EnblRstToleranceOfPric138cpric138c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric138cpric138c2924>;
impl<'a, REG> EnblRstToleranceOfPric138cpric138c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric138cpric138c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric138cpric138c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC138CPRIC138C3024` reader - Enable Write Protection of PRIC138CPRIC1_38C\\[30:24\\]"]
pub type EnblWrProtOfPric138cpric138c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC138CPRIC138C3024` writer - Enable Write Protection of PRIC138CPRIC1_38C\\[30:24\\]"]
pub type EnblWrProtOfPric138cpric138c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of TIMER3"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer3(&self) -> EnblReadGroup0ofTimer3R {
        EnblReadGroup0ofTimer3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of TIMER3"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer3(&self) -> EnblReadGroup1ofTimer3R {
        EnblReadGroup1ofTimer3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of TIMER3"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer3(&self) -> EnblReadGroup2ofTimer3R {
        EnblReadGroup2ofTimer3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of TIMER3"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer3(&self) -> EnblReadGroup3ofTimer3R {
        EnblReadGroup3ofTimer3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of TIMER3"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer3(&self) -> EnblReadGroup4ofTimer3R {
        EnblReadGroup4ofTimer3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of TIMER3"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer3(&self) -> EnblReadGroup5ofTimer3R {
        EnblReadGroup5ofTimer3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric138cpric138c0500(
        &self,
    ) -> EnblRstToleranceOfPric138cpric138c0500R {
        EnblRstToleranceOfPric138cpric138c0500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC138CPRIC1_38C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric138cpric138c0600(&self) -> EnblWrProtOfPric138cpric138c0600R {
        EnblWrProtOfPric138cpric138c0600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of TIMER4"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer4(&self) -> EnblReadGroup0ofTimer4R {
        EnblReadGroup0ofTimer4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of TIMER4"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer4(&self) -> EnblReadGroup1ofTimer4R {
        EnblReadGroup1ofTimer4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of TIMER4"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer4(&self) -> EnblReadGroup2ofTimer4R {
        EnblReadGroup2ofTimer4R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of TIMER4"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer4(&self) -> EnblReadGroup3ofTimer4R {
        EnblReadGroup3ofTimer4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of TIMER4"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer4(&self) -> EnblReadGroup4ofTimer4R {
        EnblReadGroup4ofTimer4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of TIMER4"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer4(&self) -> EnblReadGroup5ofTimer4R {
        EnblReadGroup5ofTimer4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric138cpric138c1308(
        &self,
    ) -> EnblRstToleranceOfPric138cpric138c1308R {
        EnblRstToleranceOfPric138cpric138c1308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC138CPRIC1_38C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric138cpric138c1408(&self) -> EnblWrProtOfPric138cpric138c1408R {
        EnblWrProtOfPric138cpric138c1408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of TIMER5"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer5(&self) -> EnblReadGroup0ofTimer5R {
        EnblReadGroup0ofTimer5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of TIMER5"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer5(&self) -> EnblReadGroup1ofTimer5R {
        EnblReadGroup1ofTimer5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of TIMER5"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer5(&self) -> EnblReadGroup2ofTimer5R {
        EnblReadGroup2ofTimer5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of TIMER5"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer5(&self) -> EnblReadGroup3ofTimer5R {
        EnblReadGroup3ofTimer5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of TIMER5"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer5(&self) -> EnblReadGroup4ofTimer5R {
        EnblReadGroup4ofTimer5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of TIMER5"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer5(&self) -> EnblReadGroup5ofTimer5R {
        EnblReadGroup5ofTimer5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric138cpric138c2116(
        &self,
    ) -> EnblRstToleranceOfPric138cpric138c2116R {
        EnblRstToleranceOfPric138cpric138c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC138CPRIC1_38C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric138cpric138c2216(&self) -> EnblWrProtOfPric138cpric138c2216R {
        EnblWrProtOfPric138cpric138c2216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of TIMER6"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer6(&self) -> EnblReadGroup0ofTimer6R {
        EnblReadGroup0ofTimer6R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of TIMER6"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer6(&self) -> EnblReadGroup1ofTimer6R {
        EnblReadGroup1ofTimer6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of TIMER6"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer6(&self) -> EnblReadGroup2ofTimer6R {
        EnblReadGroup2ofTimer6R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of TIMER6"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer6(&self) -> EnblReadGroup3ofTimer6R {
        EnblReadGroup3ofTimer6R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of TIMER6"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer6(&self) -> EnblReadGroup4ofTimer6R {
        EnblReadGroup4ofTimer6R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of TIMER6"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer6(&self) -> EnblReadGroup5ofTimer6R {
        EnblReadGroup5ofTimer6R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric138cpric138c2924(
        &self,
    ) -> EnblRstToleranceOfPric138cpric138c2924R {
        EnblRstToleranceOfPric138cpric138c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC138CPRIC1_38C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric138cpric138c3024(&self) -> EnblWrProtOfPric138cpric138c3024R {
        EnblWrProtOfPric138cpric138c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of TIMER3"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer3(&mut self) -> EnblReadGroup0ofTimer3W<PricIo38cSpec> {
        EnblReadGroup0ofTimer3W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of TIMER3"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer3(&mut self) -> EnblReadGroup1ofTimer3W<PricIo38cSpec> {
        EnblReadGroup1ofTimer3W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of TIMER3"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer3(&mut self) -> EnblReadGroup2ofTimer3W<PricIo38cSpec> {
        EnblReadGroup2ofTimer3W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of TIMER3"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer3(&mut self) -> EnblReadGroup3ofTimer3W<PricIo38cSpec> {
        EnblReadGroup3ofTimer3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of TIMER3"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer3(&mut self) -> EnblReadGroup4ofTimer3W<PricIo38cSpec> {
        EnblReadGroup4ofTimer3W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of TIMER3"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer3(&mut self) -> EnblReadGroup5ofTimer3W<PricIo38cSpec> {
        EnblReadGroup5ofTimer3W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric138cpric138c0500(
        &mut self,
    ) -> EnblRstToleranceOfPric138cpric138c0500W<PricIo38cSpec> {
        EnblRstToleranceOfPric138cpric138c0500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC138CPRIC1_38C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric138cpric138c0600(
        &mut self,
    ) -> EnblWrProtOfPric138cpric138c0600W<PricIo38cSpec> {
        EnblWrProtOfPric138cpric138c0600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of TIMER4"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer4(&mut self) -> EnblReadGroup0ofTimer4W<PricIo38cSpec> {
        EnblReadGroup0ofTimer4W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of TIMER4"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer4(&mut self) -> EnblReadGroup1ofTimer4W<PricIo38cSpec> {
        EnblReadGroup1ofTimer4W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of TIMER4"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer4(&mut self) -> EnblReadGroup2ofTimer4W<PricIo38cSpec> {
        EnblReadGroup2ofTimer4W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of TIMER4"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer4(&mut self) -> EnblReadGroup3ofTimer4W<PricIo38cSpec> {
        EnblReadGroup3ofTimer4W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of TIMER4"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer4(&mut self) -> EnblReadGroup4ofTimer4W<PricIo38cSpec> {
        EnblReadGroup4ofTimer4W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of TIMER4"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer4(&mut self) -> EnblReadGroup5ofTimer4W<PricIo38cSpec> {
        EnblReadGroup5ofTimer4W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric138cpric138c1308(
        &mut self,
    ) -> EnblRstToleranceOfPric138cpric138c1308W<PricIo38cSpec> {
        EnblRstToleranceOfPric138cpric138c1308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC138CPRIC1_38C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric138cpric138c1408(
        &mut self,
    ) -> EnblWrProtOfPric138cpric138c1408W<PricIo38cSpec> {
        EnblWrProtOfPric138cpric138c1408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of TIMER5"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer5(&mut self) -> EnblReadGroup0ofTimer5W<PricIo38cSpec> {
        EnblReadGroup0ofTimer5W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of TIMER5"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer5(&mut self) -> EnblReadGroup1ofTimer5W<PricIo38cSpec> {
        EnblReadGroup1ofTimer5W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of TIMER5"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer5(&mut self) -> EnblReadGroup2ofTimer5W<PricIo38cSpec> {
        EnblReadGroup2ofTimer5W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of TIMER5"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer5(&mut self) -> EnblReadGroup3ofTimer5W<PricIo38cSpec> {
        EnblReadGroup3ofTimer5W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of TIMER5"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer5(&mut self) -> EnblReadGroup4ofTimer5W<PricIo38cSpec> {
        EnblReadGroup4ofTimer5W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of TIMER5"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer5(&mut self) -> EnblReadGroup5ofTimer5W<PricIo38cSpec> {
        EnblReadGroup5ofTimer5W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric138cpric138c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric138cpric138c2116W<PricIo38cSpec> {
        EnblRstToleranceOfPric138cpric138c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC138CPRIC1_38C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric138cpric138c2216(
        &mut self,
    ) -> EnblWrProtOfPric138cpric138c2216W<PricIo38cSpec> {
        EnblWrProtOfPric138cpric138c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of TIMER6"]
    #[inline(always)]
    pub fn enbl_read_group0of_timer6(&mut self) -> EnblReadGroup0ofTimer6W<PricIo38cSpec> {
        EnblReadGroup0ofTimer6W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of TIMER6"]
    #[inline(always)]
    pub fn enbl_read_group1of_timer6(&mut self) -> EnblReadGroup1ofTimer6W<PricIo38cSpec> {
        EnblReadGroup1ofTimer6W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of TIMER6"]
    #[inline(always)]
    pub fn enbl_read_group2of_timer6(&mut self) -> EnblReadGroup2ofTimer6W<PricIo38cSpec> {
        EnblReadGroup2ofTimer6W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of TIMER6"]
    #[inline(always)]
    pub fn enbl_read_group3of_timer6(&mut self) -> EnblReadGroup3ofTimer6W<PricIo38cSpec> {
        EnblReadGroup3ofTimer6W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of TIMER6"]
    #[inline(always)]
    pub fn enbl_read_group4of_timer6(&mut self) -> EnblReadGroup4ofTimer6W<PricIo38cSpec> {
        EnblReadGroup4ofTimer6W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of TIMER6"]
    #[inline(always)]
    pub fn enbl_read_group5of_timer6(&mut self) -> EnblReadGroup5ofTimer6W<PricIo38cSpec> {
        EnblReadGroup5ofTimer6W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC138CPRIC1_38C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric138cpric138c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric138cpric138c2924W<PricIo38cSpec> {
        EnblRstToleranceOfPric138cpric138c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC138CPRIC1_38C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric138cpric138c3024(
        &mut self,
    ) -> EnblWrProtOfPric138cpric138c3024W<PricIo38cSpec> {
        EnblWrProtOfPric138cpric138c3024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io38c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io38c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo38cSpec;
impl crate::RegisterSpec for PricIo38cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io38c::R`](R) reader structure"]
impl crate::Readable for PricIo38cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io38c::W`](W) writer structure"]
impl crate::Writable for PricIo38cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO38C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo38cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
