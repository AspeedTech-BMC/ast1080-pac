#[doc = "Register `PRIC_IO28C` reader"]
pub type R = crate::R<PricIo28cSpec>;
#[doc = "Register `PRIC_IO28C` writer"]
pub type W = crate::W<PricIo28cSpec>;
#[doc = "Field `EnblWrGroup0OfTIMER3` reader - Enable Write Group #0 of TIMER3"]
pub type EnblWrGroup0ofTimer3R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfTIMER3` writer - Enable Write Group #0 of TIMER3"]
pub type EnblWrGroup0ofTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfTIMER3` reader - Enable Write Group #1 of TIMER3"]
pub type EnblWrGroup1ofTimer3R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfTIMER3` writer - Enable Write Group #1 of TIMER3"]
pub type EnblWrGroup1ofTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfTIMER3` reader - Enable Write Group #2 of TIMER3"]
pub type EnblWrGroup2ofTimer3R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfTIMER3` writer - Enable Write Group #2 of TIMER3"]
pub type EnblWrGroup2ofTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfTIMER3` reader - Enable Write Group #3 of TIMER3"]
pub type EnblWrGroup3ofTimer3R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfTIMER3` writer - Enable Write Group #3 of TIMER3"]
pub type EnblWrGroup3ofTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfTIMER3` reader - Enable Write Group #4 of TIMER3"]
pub type EnblWrGroup4ofTimer3R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfTIMER3` writer - Enable Write Group #4 of TIMER3"]
pub type EnblWrGroup4ofTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfTIMER3` reader - Enable Write Group #5 of TIMER3"]
pub type EnblWrGroup5ofTimer3R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfTIMER3` writer - Enable Write Group #5 of TIMER3"]
pub type EnblWrGroup5ofTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC128CPRIC1_28C\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric128cpric128c0500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric128cpric128c0500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric128cpric128c0500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC128CPRIC128C0500` reader - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[05:00\\]"]
pub type EnblRstToleranceOfPric128cpric128c0500R =
    crate::BitReader<EnblRstToleranceOfPric128cpric128c0500>;
impl EnblRstToleranceOfPric128cpric128c0500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric128cpric128c0500 {
        match self.bits {
            false => EnblRstToleranceOfPric128cpric128c0500::ResetBySrst,
            true => EnblRstToleranceOfPric128cpric128c0500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric128cpric128c0500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric128cpric128c0500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC128CPRIC128C0500` writer - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[05:00\\]"]
pub type EnblRstToleranceOfPric128cpric128c0500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric128cpric128c0500>;
impl<'a, REG> EnblRstToleranceOfPric128cpric128c0500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric128cpric128c0500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric128cpric128c0500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC128CPRIC128C0600` reader - Enable Write Protection of PRIC128CPRIC1_28C\\[06:00\\]"]
pub type EnblWrProtOfPric128cpric128c0600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC128CPRIC128C0600` writer - Enable Write Protection of PRIC128CPRIC1_28C\\[06:00\\]"]
pub type EnblWrProtOfPric128cpric128c0600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfTIMER4` reader - Enable Write Group #0 of TIMER4"]
pub type EnblWrGroup0ofTimer4R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfTIMER4` writer - Enable Write Group #0 of TIMER4"]
pub type EnblWrGroup0ofTimer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfTIMER4` reader - Enable Write Group #1 of TIMER4"]
pub type EnblWrGroup1ofTimer4R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfTIMER4` writer - Enable Write Group #1 of TIMER4"]
pub type EnblWrGroup1ofTimer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfTIMER4` reader - Enable Write Group #2 of TIMER4"]
pub type EnblWrGroup2ofTimer4R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfTIMER4` writer - Enable Write Group #2 of TIMER4"]
pub type EnblWrGroup2ofTimer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfTIMER4` reader - Enable Write Group #3 of TIMER4"]
pub type EnblWrGroup3ofTimer4R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfTIMER4` writer - Enable Write Group #3 of TIMER4"]
pub type EnblWrGroup3ofTimer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfTIMER4` reader - Enable Write Group #4 of TIMER4"]
pub type EnblWrGroup4ofTimer4R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfTIMER4` writer - Enable Write Group #4 of TIMER4"]
pub type EnblWrGroup4ofTimer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfTIMER4` reader - Enable Write Group #5 of TIMER4"]
pub type EnblWrGroup5ofTimer4R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfTIMER4` writer - Enable Write Group #5 of TIMER4"]
pub type EnblWrGroup5ofTimer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC128CPRIC1_28C\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric128cpric128c1308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric128cpric128c1308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric128cpric128c1308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC128CPRIC128C1308` reader - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[13:08\\]"]
pub type EnblRstToleranceOfPric128cpric128c1308R =
    crate::BitReader<EnblRstToleranceOfPric128cpric128c1308>;
impl EnblRstToleranceOfPric128cpric128c1308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric128cpric128c1308 {
        match self.bits {
            false => EnblRstToleranceOfPric128cpric128c1308::ResetBySrst,
            true => EnblRstToleranceOfPric128cpric128c1308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric128cpric128c1308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric128cpric128c1308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC128CPRIC128C1308` writer - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[13:08\\]"]
pub type EnblRstToleranceOfPric128cpric128c1308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric128cpric128c1308>;
impl<'a, REG> EnblRstToleranceOfPric128cpric128c1308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric128cpric128c1308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric128cpric128c1308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC128CPRIC128C1408` reader - Enable Write Protection of PRIC128CPRIC1_28C\\[14:08\\]"]
pub type EnblWrProtOfPric128cpric128c1408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC128CPRIC128C1408` writer - Enable Write Protection of PRIC128CPRIC1_28C\\[14:08\\]"]
pub type EnblWrProtOfPric128cpric128c1408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfTIMER5` reader - Enable Write Group #0 of TIMER5"]
pub type EnblWrGroup0ofTimer5R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfTIMER5` writer - Enable Write Group #0 of TIMER5"]
pub type EnblWrGroup0ofTimer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfTIMER5` reader - Enable Write Group #1 of TIMER5"]
pub type EnblWrGroup1ofTimer5R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfTIMER5` writer - Enable Write Group #1 of TIMER5"]
pub type EnblWrGroup1ofTimer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfTIMER5` reader - Enable Write Group #2 of TIMER5"]
pub type EnblWrGroup2ofTimer5R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfTIMER5` writer - Enable Write Group #2 of TIMER5"]
pub type EnblWrGroup2ofTimer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfTIMER5` reader - Enable Write Group #3 of TIMER5"]
pub type EnblWrGroup3ofTimer5R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfTIMER5` writer - Enable Write Group #3 of TIMER5"]
pub type EnblWrGroup3ofTimer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfTIMER5` reader - Enable Write Group #4 of TIMER5"]
pub type EnblWrGroup4ofTimer5R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfTIMER5` writer - Enable Write Group #4 of TIMER5"]
pub type EnblWrGroup4ofTimer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfTIMER5` reader - Enable Write Group #5 of TIMER5"]
pub type EnblWrGroup5ofTimer5R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfTIMER5` writer - Enable Write Group #5 of TIMER5"]
pub type EnblWrGroup5ofTimer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC128CPRIC1_28C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric128cpric128c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric128cpric128c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric128cpric128c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC128CPRIC128C2116` reader - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[21:16\\]"]
pub type EnblRstToleranceOfPric128cpric128c2116R =
    crate::BitReader<EnblRstToleranceOfPric128cpric128c2116>;
impl EnblRstToleranceOfPric128cpric128c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric128cpric128c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric128cpric128c2116::ResetBySrst,
            true => EnblRstToleranceOfPric128cpric128c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric128cpric128c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric128cpric128c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC128CPRIC128C2116` writer - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[21:16\\]"]
pub type EnblRstToleranceOfPric128cpric128c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric128cpric128c2116>;
impl<'a, REG> EnblRstToleranceOfPric128cpric128c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric128cpric128c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric128cpric128c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC128CPRIC128C2216` reader - Enable Write Protection of PRIC128CPRIC1_28C\\[22:16\\]"]
pub type EnblWrProtOfPric128cpric128c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC128CPRIC128C2216` writer - Enable Write Protection of PRIC128CPRIC1_28C\\[22:16\\]"]
pub type EnblWrProtOfPric128cpric128c2216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfTIMER6` reader - Enable Write Group #0 of TIMER6"]
pub type EnblWrGroup0ofTimer6R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfTIMER6` writer - Enable Write Group #0 of TIMER6"]
pub type EnblWrGroup0ofTimer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfTIMER6` reader - Enable Write Group #1 of TIMER6"]
pub type EnblWrGroup1ofTimer6R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfTIMER6` writer - Enable Write Group #1 of TIMER6"]
pub type EnblWrGroup1ofTimer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfTIMER6` reader - Enable Write Group #2 of TIMER6"]
pub type EnblWrGroup2ofTimer6R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfTIMER6` writer - Enable Write Group #2 of TIMER6"]
pub type EnblWrGroup2ofTimer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfTIMER6` reader - Enable Write Group #3 of TIMER6"]
pub type EnblWrGroup3ofTimer6R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfTIMER6` writer - Enable Write Group #3 of TIMER6"]
pub type EnblWrGroup3ofTimer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfTIMER6` reader - Enable Write Group #4 of TIMER6"]
pub type EnblWrGroup4ofTimer6R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfTIMER6` writer - Enable Write Group #4 of TIMER6"]
pub type EnblWrGroup4ofTimer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfTIMER6` reader - Enable Write Group #5 of TIMER6"]
pub type EnblWrGroup5ofTimer6R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfTIMER6` writer - Enable Write Group #5 of TIMER6"]
pub type EnblWrGroup5ofTimer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC128CPRIC1_28C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric128cpric128c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric128cpric128c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric128cpric128c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC128CPRIC128C2924` reader - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[29:24\\]"]
pub type EnblRstToleranceOfPric128cpric128c2924R =
    crate::BitReader<EnblRstToleranceOfPric128cpric128c2924>;
impl EnblRstToleranceOfPric128cpric128c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric128cpric128c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric128cpric128c2924::ResetBySrst,
            true => EnblRstToleranceOfPric128cpric128c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric128cpric128c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric128cpric128c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC128CPRIC128C2924` writer - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[29:24\\]"]
pub type EnblRstToleranceOfPric128cpric128c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric128cpric128c2924>;
impl<'a, REG> EnblRstToleranceOfPric128cpric128c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric128cpric128c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric128cpric128c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC128CPRIC128C3024` reader - Enable Write Protection of PRIC128CPRIC1_28C\\[30:24\\]"]
pub type EnblWrProtOfPric128cpric128c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC128CPRIC128C3024` writer - Enable Write Protection of PRIC128CPRIC1_28C\\[30:24\\]"]
pub type EnblWrProtOfPric128cpric128c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of TIMER3"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer3(&self) -> EnblWrGroup0ofTimer3R {
        EnblWrGroup0ofTimer3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of TIMER3"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer3(&self) -> EnblWrGroup1ofTimer3R {
        EnblWrGroup1ofTimer3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of TIMER3"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer3(&self) -> EnblWrGroup2ofTimer3R {
        EnblWrGroup2ofTimer3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of TIMER3"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer3(&self) -> EnblWrGroup3ofTimer3R {
        EnblWrGroup3ofTimer3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of TIMER3"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer3(&self) -> EnblWrGroup4ofTimer3R {
        EnblWrGroup4ofTimer3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of TIMER3"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer3(&self) -> EnblWrGroup5ofTimer3R {
        EnblWrGroup5ofTimer3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric128cpric128c0500(
        &self,
    ) -> EnblRstToleranceOfPric128cpric128c0500R {
        EnblRstToleranceOfPric128cpric128c0500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC128CPRIC1_28C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric128cpric128c0600(&self) -> EnblWrProtOfPric128cpric128c0600R {
        EnblWrProtOfPric128cpric128c0600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of TIMER4"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer4(&self) -> EnblWrGroup0ofTimer4R {
        EnblWrGroup0ofTimer4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of TIMER4"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer4(&self) -> EnblWrGroup1ofTimer4R {
        EnblWrGroup1ofTimer4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of TIMER4"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer4(&self) -> EnblWrGroup2ofTimer4R {
        EnblWrGroup2ofTimer4R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of TIMER4"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer4(&self) -> EnblWrGroup3ofTimer4R {
        EnblWrGroup3ofTimer4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of TIMER4"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer4(&self) -> EnblWrGroup4ofTimer4R {
        EnblWrGroup4ofTimer4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of TIMER4"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer4(&self) -> EnblWrGroup5ofTimer4R {
        EnblWrGroup5ofTimer4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric128cpric128c1308(
        &self,
    ) -> EnblRstToleranceOfPric128cpric128c1308R {
        EnblRstToleranceOfPric128cpric128c1308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC128CPRIC1_28C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric128cpric128c1408(&self) -> EnblWrProtOfPric128cpric128c1408R {
        EnblWrProtOfPric128cpric128c1408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of TIMER5"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer5(&self) -> EnblWrGroup0ofTimer5R {
        EnblWrGroup0ofTimer5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of TIMER5"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer5(&self) -> EnblWrGroup1ofTimer5R {
        EnblWrGroup1ofTimer5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of TIMER5"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer5(&self) -> EnblWrGroup2ofTimer5R {
        EnblWrGroup2ofTimer5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of TIMER5"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer5(&self) -> EnblWrGroup3ofTimer5R {
        EnblWrGroup3ofTimer5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of TIMER5"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer5(&self) -> EnblWrGroup4ofTimer5R {
        EnblWrGroup4ofTimer5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of TIMER5"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer5(&self) -> EnblWrGroup5ofTimer5R {
        EnblWrGroup5ofTimer5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric128cpric128c2116(
        &self,
    ) -> EnblRstToleranceOfPric128cpric128c2116R {
        EnblRstToleranceOfPric128cpric128c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC128CPRIC1_28C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric128cpric128c2216(&self) -> EnblWrProtOfPric128cpric128c2216R {
        EnblWrProtOfPric128cpric128c2216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of TIMER6"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer6(&self) -> EnblWrGroup0ofTimer6R {
        EnblWrGroup0ofTimer6R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of TIMER6"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer6(&self) -> EnblWrGroup1ofTimer6R {
        EnblWrGroup1ofTimer6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of TIMER6"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer6(&self) -> EnblWrGroup2ofTimer6R {
        EnblWrGroup2ofTimer6R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of TIMER6"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer6(&self) -> EnblWrGroup3ofTimer6R {
        EnblWrGroup3ofTimer6R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of TIMER6"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer6(&self) -> EnblWrGroup4ofTimer6R {
        EnblWrGroup4ofTimer6R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of TIMER6"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer6(&self) -> EnblWrGroup5ofTimer6R {
        EnblWrGroup5ofTimer6R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric128cpric128c2924(
        &self,
    ) -> EnblRstToleranceOfPric128cpric128c2924R {
        EnblRstToleranceOfPric128cpric128c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC128CPRIC1_28C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric128cpric128c3024(&self) -> EnblWrProtOfPric128cpric128c3024R {
        EnblWrProtOfPric128cpric128c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of TIMER3"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer3(&mut self) -> EnblWrGroup0ofTimer3W<PricIo28cSpec> {
        EnblWrGroup0ofTimer3W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of TIMER3"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer3(&mut self) -> EnblWrGroup1ofTimer3W<PricIo28cSpec> {
        EnblWrGroup1ofTimer3W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of TIMER3"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer3(&mut self) -> EnblWrGroup2ofTimer3W<PricIo28cSpec> {
        EnblWrGroup2ofTimer3W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of TIMER3"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer3(&mut self) -> EnblWrGroup3ofTimer3W<PricIo28cSpec> {
        EnblWrGroup3ofTimer3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of TIMER3"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer3(&mut self) -> EnblWrGroup4ofTimer3W<PricIo28cSpec> {
        EnblWrGroup4ofTimer3W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of TIMER3"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer3(&mut self) -> EnblWrGroup5ofTimer3W<PricIo28cSpec> {
        EnblWrGroup5ofTimer3W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric128cpric128c0500(
        &mut self,
    ) -> EnblRstToleranceOfPric128cpric128c0500W<PricIo28cSpec> {
        EnblRstToleranceOfPric128cpric128c0500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC128CPRIC1_28C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric128cpric128c0600(
        &mut self,
    ) -> EnblWrProtOfPric128cpric128c0600W<PricIo28cSpec> {
        EnblWrProtOfPric128cpric128c0600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of TIMER4"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer4(&mut self) -> EnblWrGroup0ofTimer4W<PricIo28cSpec> {
        EnblWrGroup0ofTimer4W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of TIMER4"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer4(&mut self) -> EnblWrGroup1ofTimer4W<PricIo28cSpec> {
        EnblWrGroup1ofTimer4W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of TIMER4"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer4(&mut self) -> EnblWrGroup2ofTimer4W<PricIo28cSpec> {
        EnblWrGroup2ofTimer4W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of TIMER4"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer4(&mut self) -> EnblWrGroup3ofTimer4W<PricIo28cSpec> {
        EnblWrGroup3ofTimer4W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of TIMER4"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer4(&mut self) -> EnblWrGroup4ofTimer4W<PricIo28cSpec> {
        EnblWrGroup4ofTimer4W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of TIMER4"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer4(&mut self) -> EnblWrGroup5ofTimer4W<PricIo28cSpec> {
        EnblWrGroup5ofTimer4W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric128cpric128c1308(
        &mut self,
    ) -> EnblRstToleranceOfPric128cpric128c1308W<PricIo28cSpec> {
        EnblRstToleranceOfPric128cpric128c1308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC128CPRIC1_28C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric128cpric128c1408(
        &mut self,
    ) -> EnblWrProtOfPric128cpric128c1408W<PricIo28cSpec> {
        EnblWrProtOfPric128cpric128c1408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of TIMER5"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer5(&mut self) -> EnblWrGroup0ofTimer5W<PricIo28cSpec> {
        EnblWrGroup0ofTimer5W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of TIMER5"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer5(&mut self) -> EnblWrGroup1ofTimer5W<PricIo28cSpec> {
        EnblWrGroup1ofTimer5W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of TIMER5"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer5(&mut self) -> EnblWrGroup2ofTimer5W<PricIo28cSpec> {
        EnblWrGroup2ofTimer5W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of TIMER5"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer5(&mut self) -> EnblWrGroup3ofTimer5W<PricIo28cSpec> {
        EnblWrGroup3ofTimer5W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of TIMER5"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer5(&mut self) -> EnblWrGroup4ofTimer5W<PricIo28cSpec> {
        EnblWrGroup4ofTimer5W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of TIMER5"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer5(&mut self) -> EnblWrGroup5ofTimer5W<PricIo28cSpec> {
        EnblWrGroup5ofTimer5W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric128cpric128c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric128cpric128c2116W<PricIo28cSpec> {
        EnblRstToleranceOfPric128cpric128c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC128CPRIC1_28C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric128cpric128c2216(
        &mut self,
    ) -> EnblWrProtOfPric128cpric128c2216W<PricIo28cSpec> {
        EnblWrProtOfPric128cpric128c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of TIMER6"]
    #[inline(always)]
    pub fn enbl_wr_group0of_timer6(&mut self) -> EnblWrGroup0ofTimer6W<PricIo28cSpec> {
        EnblWrGroup0ofTimer6W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of TIMER6"]
    #[inline(always)]
    pub fn enbl_wr_group1of_timer6(&mut self) -> EnblWrGroup1ofTimer6W<PricIo28cSpec> {
        EnblWrGroup1ofTimer6W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of TIMER6"]
    #[inline(always)]
    pub fn enbl_wr_group2of_timer6(&mut self) -> EnblWrGroup2ofTimer6W<PricIo28cSpec> {
        EnblWrGroup2ofTimer6W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of TIMER6"]
    #[inline(always)]
    pub fn enbl_wr_group3of_timer6(&mut self) -> EnblWrGroup3ofTimer6W<PricIo28cSpec> {
        EnblWrGroup3ofTimer6W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of TIMER6"]
    #[inline(always)]
    pub fn enbl_wr_group4of_timer6(&mut self) -> EnblWrGroup4ofTimer6W<PricIo28cSpec> {
        EnblWrGroup4ofTimer6W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of TIMER6"]
    #[inline(always)]
    pub fn enbl_wr_group5of_timer6(&mut self) -> EnblWrGroup5ofTimer6W<PricIo28cSpec> {
        EnblWrGroup5ofTimer6W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC128CPRIC1_28C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric128cpric128c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric128cpric128c2924W<PricIo28cSpec> {
        EnblRstToleranceOfPric128cpric128c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC128CPRIC1_28C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric128cpric128c3024(
        &mut self,
    ) -> EnblWrProtOfPric128cpric128c3024W<PricIo28cSpec> {
        EnblWrProtOfPric128cpric128c3024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io28c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io28c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo28cSpec;
impl crate::RegisterSpec for PricIo28cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io28c::R`](R) reader structure"]
impl crate::Readable for PricIo28cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io28c::W`](W) writer structure"]
impl crate::Writable for PricIo28cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO28C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo28cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
