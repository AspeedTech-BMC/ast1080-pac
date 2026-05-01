#[doc = "Register `PRIC_IO35C` reader"]
pub type R = crate::R<PricIo35cSpec>;
#[doc = "Register `PRIC_IO35C` writer"]
pub type W = crate::W<PricIo35cSpec>;
#[doc = "Field `EnblReadGroup0OfI3C2` reader - Enable Read Group #0 of I3C2"]
pub type EnblReadGroup0ofI3c2R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI3C2` writer - Enable Read Group #0 of I3C2"]
pub type EnblReadGroup0ofI3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI3C2` reader - Enable Read Group #1 of I3C2"]
pub type EnblReadGroup1ofI3c2R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI3C2` writer - Enable Read Group #1 of I3C2"]
pub type EnblReadGroup1ofI3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI3C2` reader - Enable Read Group #2 of I3C2"]
pub type EnblReadGroup2ofI3c2R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI3C2` writer - Enable Read Group #2 of I3C2"]
pub type EnblReadGroup2ofI3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI3C2` reader - Enable Read Group #3 of I3C2"]
pub type EnblReadGroup3ofI3c2R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI3C2` writer - Enable Read Group #3 of I3C2"]
pub type EnblReadGroup3ofI3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI3C2` reader - Enable Read Group #4 of I3C2"]
pub type EnblReadGroup4ofI3c2R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI3C2` writer - Enable Read Group #4 of I3C2"]
pub type EnblReadGroup4ofI3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI3C2` reader - Enable Read Group #5 of I3C2"]
pub type EnblReadGroup5ofI3c2R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI3C2` writer - Enable Read Group #5 of I3C2"]
pub type EnblReadGroup5ofI3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC135CPRIC1_35C\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric135cpric135c0500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric135cpric135c0500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric135cpric135c0500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC135CPRIC135C0500` reader - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[05:00\\]"]
pub type EnblRstToleranceOfPric135cpric135c0500R =
    crate::BitReader<EnblRstToleranceOfPric135cpric135c0500>;
impl EnblRstToleranceOfPric135cpric135c0500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric135cpric135c0500 {
        match self.bits {
            false => EnblRstToleranceOfPric135cpric135c0500::ResetBySrst,
            true => EnblRstToleranceOfPric135cpric135c0500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric135cpric135c0500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric135cpric135c0500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC135CPRIC135C0500` writer - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[05:00\\]"]
pub type EnblRstToleranceOfPric135cpric135c0500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric135cpric135c0500>;
impl<'a, REG> EnblRstToleranceOfPric135cpric135c0500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric135cpric135c0500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric135cpric135c0500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC135CPRIC135C0600` reader - Enable Write Protection of PRIC135CPRIC1_35C\\[06:00\\]"]
pub type EnblWrProtOfPric135cpric135c0600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC135CPRIC135C0600` writer - Enable Write Protection of PRIC135CPRIC1_35C\\[06:00\\]"]
pub type EnblWrProtOfPric135cpric135c0600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI3C3` reader - Enable Read Group #0 of I3C3"]
pub type EnblReadGroup0ofI3c3R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI3C3` writer - Enable Read Group #0 of I3C3"]
pub type EnblReadGroup0ofI3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI3C3` reader - Enable Read Group #1 of I3C3"]
pub type EnblReadGroup1ofI3c3R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI3C3` writer - Enable Read Group #1 of I3C3"]
pub type EnblReadGroup1ofI3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI3C3` reader - Enable Read Group #2 of I3C3"]
pub type EnblReadGroup2ofI3c3R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI3C3` writer - Enable Read Group #2 of I3C3"]
pub type EnblReadGroup2ofI3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI3C3` reader - Enable Read Group #3 of I3C3"]
pub type EnblReadGroup3ofI3c3R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI3C3` writer - Enable Read Group #3 of I3C3"]
pub type EnblReadGroup3ofI3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI3C3` reader - Enable Read Group #4 of I3C3"]
pub type EnblReadGroup4ofI3c3R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI3C3` writer - Enable Read Group #4 of I3C3"]
pub type EnblReadGroup4ofI3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI3C3` reader - Enable Read Group #5 of I3C3"]
pub type EnblReadGroup5ofI3c3R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI3C3` writer - Enable Read Group #5 of I3C3"]
pub type EnblReadGroup5ofI3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC135CPRIC1_35C\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric135cpric135c1308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric135cpric135c1308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric135cpric135c1308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC135CPRIC135C1308` reader - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[13:08\\]"]
pub type EnblRstToleranceOfPric135cpric135c1308R =
    crate::BitReader<EnblRstToleranceOfPric135cpric135c1308>;
impl EnblRstToleranceOfPric135cpric135c1308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric135cpric135c1308 {
        match self.bits {
            false => EnblRstToleranceOfPric135cpric135c1308::ResetBySrst,
            true => EnblRstToleranceOfPric135cpric135c1308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric135cpric135c1308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric135cpric135c1308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC135CPRIC135C1308` writer - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[13:08\\]"]
pub type EnblRstToleranceOfPric135cpric135c1308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric135cpric135c1308>;
impl<'a, REG> EnblRstToleranceOfPric135cpric135c1308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric135cpric135c1308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric135cpric135c1308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC135CPRIC135C1408` reader - Enable Write Protection of PRIC135CPRIC1_35C\\[14:08\\]"]
pub type EnblWrProtOfPric135cpric135c1408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC135CPRIC135C1408` writer - Enable Write Protection of PRIC135CPRIC1_35C\\[14:08\\]"]
pub type EnblWrProtOfPric135cpric135c1408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI3C4` reader - Enable Read Group #0 of I3C4"]
pub type EnblReadGroup0ofI3c4R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI3C4` writer - Enable Read Group #0 of I3C4"]
pub type EnblReadGroup0ofI3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI3C4` reader - Enable Read Group #1 of I3C4"]
pub type EnblReadGroup1ofI3c4R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI3C4` writer - Enable Read Group #1 of I3C4"]
pub type EnblReadGroup1ofI3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI3C4` reader - Enable Read Group #2 of I3C4"]
pub type EnblReadGroup2ofI3c4R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI3C4` writer - Enable Read Group #2 of I3C4"]
pub type EnblReadGroup2ofI3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI3C4` reader - Enable Read Group #3 of I3C4"]
pub type EnblReadGroup3ofI3c4R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI3C4` writer - Enable Read Group #3 of I3C4"]
pub type EnblReadGroup3ofI3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI3C4` reader - Enable Read Group #4 of I3C4"]
pub type EnblReadGroup4ofI3c4R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI3C4` writer - Enable Read Group #4 of I3C4"]
pub type EnblReadGroup4ofI3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI3C4` reader - Enable Read Group #5 of I3C4"]
pub type EnblReadGroup5ofI3c4R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI3C4` writer - Enable Read Group #5 of I3C4"]
pub type EnblReadGroup5ofI3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC135CPRIC1_35C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric135cpric135c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric135cpric135c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric135cpric135c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC135CPRIC135C2116` reader - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[21:16\\]"]
pub type EnblRstToleranceOfPric135cpric135c2116R =
    crate::BitReader<EnblRstToleranceOfPric135cpric135c2116>;
impl EnblRstToleranceOfPric135cpric135c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric135cpric135c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric135cpric135c2116::ResetBySrst,
            true => EnblRstToleranceOfPric135cpric135c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric135cpric135c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric135cpric135c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC135CPRIC135C2116` writer - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[21:16\\]"]
pub type EnblRstToleranceOfPric135cpric135c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric135cpric135c2116>;
impl<'a, REG> EnblRstToleranceOfPric135cpric135c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric135cpric135c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric135cpric135c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC135CPRIC135C2216` reader - Enable Write Protection of PRIC135CPRIC1_35C\\[22:16\\]"]
pub type EnblWrProtOfPric135cpric135c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC135CPRIC135C2216` writer - Enable Write Protection of PRIC135CPRIC1_35C\\[22:16\\]"]
pub type EnblWrProtOfPric135cpric135c2216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI3C5` reader - Enable Read Group #0 of I3C5"]
pub type EnblReadGroup0ofI3c5R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI3C5` writer - Enable Read Group #0 of I3C5"]
pub type EnblReadGroup0ofI3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI3C5` reader - Enable Read Group #1 of I3C5"]
pub type EnblReadGroup1ofI3c5R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI3C5` writer - Enable Read Group #1 of I3C5"]
pub type EnblReadGroup1ofI3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI3C5` reader - Enable Read Group #2 of I3C5"]
pub type EnblReadGroup2ofI3c5R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI3C5` writer - Enable Read Group #2 of I3C5"]
pub type EnblReadGroup2ofI3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI3C5` reader - Enable Read Group #3 of I3C5"]
pub type EnblReadGroup3ofI3c5R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI3C5` writer - Enable Read Group #3 of I3C5"]
pub type EnblReadGroup3ofI3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI3C5` reader - Enable Read Group #4 of I3C5"]
pub type EnblReadGroup4ofI3c5R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI3C5` writer - Enable Read Group #4 of I3C5"]
pub type EnblReadGroup4ofI3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI3C5` reader - Enable Read Group #5 of I3C5"]
pub type EnblReadGroup5ofI3c5R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI3C5` writer - Enable Read Group #5 of I3C5"]
pub type EnblReadGroup5ofI3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC135CPRIC1_35C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric135cpric135c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric135cpric135c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric135cpric135c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC135CPRIC135C2924` reader - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[29:24\\]"]
pub type EnblRstToleranceOfPric135cpric135c2924R =
    crate::BitReader<EnblRstToleranceOfPric135cpric135c2924>;
impl EnblRstToleranceOfPric135cpric135c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric135cpric135c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric135cpric135c2924::ResetBySrst,
            true => EnblRstToleranceOfPric135cpric135c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric135cpric135c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric135cpric135c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC135CPRIC135C2924` writer - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[29:24\\]"]
pub type EnblRstToleranceOfPric135cpric135c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric135cpric135c2924>;
impl<'a, REG> EnblRstToleranceOfPric135cpric135c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric135cpric135c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric135cpric135c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC135CPRIC135C3024` reader - Enable Write Protection of PRIC135CPRIC1_35C\\[30:24\\]"]
pub type EnblWrProtOfPric135cpric135c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC135CPRIC135C3024` writer - Enable Write Protection of PRIC135CPRIC1_35C\\[30:24\\]"]
pub type EnblWrProtOfPric135cpric135c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of I3C2"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c2(&self) -> EnblReadGroup0ofI3c2R {
        EnblReadGroup0ofI3c2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I3C2"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c2(&self) -> EnblReadGroup1ofI3c2R {
        EnblReadGroup1ofI3c2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I3C2"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c2(&self) -> EnblReadGroup2ofI3c2R {
        EnblReadGroup2ofI3c2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I3C2"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c2(&self) -> EnblReadGroup3ofI3c2R {
        EnblReadGroup3ofI3c2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I3C2"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c2(&self) -> EnblReadGroup4ofI3c2R {
        EnblReadGroup4ofI3c2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I3C2"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c2(&self) -> EnblReadGroup5ofI3c2R {
        EnblReadGroup5ofI3c2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric135cpric135c0500(
        &self,
    ) -> EnblRstToleranceOfPric135cpric135c0500R {
        EnblRstToleranceOfPric135cpric135c0500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC135CPRIC1_35C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric135cpric135c0600(&self) -> EnblWrProtOfPric135cpric135c0600R {
        EnblWrProtOfPric135cpric135c0600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of I3C3"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c3(&self) -> EnblReadGroup0ofI3c3R {
        EnblReadGroup0ofI3c3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of I3C3"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c3(&self) -> EnblReadGroup1ofI3c3R {
        EnblReadGroup1ofI3c3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of I3C3"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c3(&self) -> EnblReadGroup2ofI3c3R {
        EnblReadGroup2ofI3c3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of I3C3"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c3(&self) -> EnblReadGroup3ofI3c3R {
        EnblReadGroup3ofI3c3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of I3C3"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c3(&self) -> EnblReadGroup4ofI3c3R {
        EnblReadGroup4ofI3c3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of I3C3"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c3(&self) -> EnblReadGroup5ofI3c3R {
        EnblReadGroup5ofI3c3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric135cpric135c1308(
        &self,
    ) -> EnblRstToleranceOfPric135cpric135c1308R {
        EnblRstToleranceOfPric135cpric135c1308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC135CPRIC1_35C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric135cpric135c1408(&self) -> EnblWrProtOfPric135cpric135c1408R {
        EnblWrProtOfPric135cpric135c1408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of I3C4"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c4(&self) -> EnblReadGroup0ofI3c4R {
        EnblReadGroup0ofI3c4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of I3C4"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c4(&self) -> EnblReadGroup1ofI3c4R {
        EnblReadGroup1ofI3c4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of I3C4"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c4(&self) -> EnblReadGroup2ofI3c4R {
        EnblReadGroup2ofI3c4R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of I3C4"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c4(&self) -> EnblReadGroup3ofI3c4R {
        EnblReadGroup3ofI3c4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of I3C4"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c4(&self) -> EnblReadGroup4ofI3c4R {
        EnblReadGroup4ofI3c4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of I3C4"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c4(&self) -> EnblReadGroup5ofI3c4R {
        EnblReadGroup5ofI3c4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric135cpric135c2116(
        &self,
    ) -> EnblRstToleranceOfPric135cpric135c2116R {
        EnblRstToleranceOfPric135cpric135c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC135CPRIC1_35C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric135cpric135c2216(&self) -> EnblWrProtOfPric135cpric135c2216R {
        EnblWrProtOfPric135cpric135c2216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of I3C5"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c5(&self) -> EnblReadGroup0ofI3c5R {
        EnblReadGroup0ofI3c5R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of I3C5"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c5(&self) -> EnblReadGroup1ofI3c5R {
        EnblReadGroup1ofI3c5R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of I3C5"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c5(&self) -> EnblReadGroup2ofI3c5R {
        EnblReadGroup2ofI3c5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of I3C5"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c5(&self) -> EnblReadGroup3ofI3c5R {
        EnblReadGroup3ofI3c5R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of I3C5"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c5(&self) -> EnblReadGroup4ofI3c5R {
        EnblReadGroup4ofI3c5R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of I3C5"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c5(&self) -> EnblReadGroup5ofI3c5R {
        EnblReadGroup5ofI3c5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric135cpric135c2924(
        &self,
    ) -> EnblRstToleranceOfPric135cpric135c2924R {
        EnblRstToleranceOfPric135cpric135c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC135CPRIC1_35C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric135cpric135c3024(&self) -> EnblWrProtOfPric135cpric135c3024R {
        EnblWrProtOfPric135cpric135c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of I3C2"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c2(&mut self) -> EnblReadGroup0ofI3c2W<PricIo35cSpec> {
        EnblReadGroup0ofI3c2W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of I3C2"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c2(&mut self) -> EnblReadGroup1ofI3c2W<PricIo35cSpec> {
        EnblReadGroup1ofI3c2W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of I3C2"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c2(&mut self) -> EnblReadGroup2ofI3c2W<PricIo35cSpec> {
        EnblReadGroup2ofI3c2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of I3C2"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c2(&mut self) -> EnblReadGroup3ofI3c2W<PricIo35cSpec> {
        EnblReadGroup3ofI3c2W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of I3C2"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c2(&mut self) -> EnblReadGroup4ofI3c2W<PricIo35cSpec> {
        EnblReadGroup4ofI3c2W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of I3C2"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c2(&mut self) -> EnblReadGroup5ofI3c2W<PricIo35cSpec> {
        EnblReadGroup5ofI3c2W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric135cpric135c0500(
        &mut self,
    ) -> EnblRstToleranceOfPric135cpric135c0500W<PricIo35cSpec> {
        EnblRstToleranceOfPric135cpric135c0500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC135CPRIC1_35C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric135cpric135c0600(
        &mut self,
    ) -> EnblWrProtOfPric135cpric135c0600W<PricIo35cSpec> {
        EnblWrProtOfPric135cpric135c0600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of I3C3"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c3(&mut self) -> EnblReadGroup0ofI3c3W<PricIo35cSpec> {
        EnblReadGroup0ofI3c3W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of I3C3"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c3(&mut self) -> EnblReadGroup1ofI3c3W<PricIo35cSpec> {
        EnblReadGroup1ofI3c3W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of I3C3"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c3(&mut self) -> EnblReadGroup2ofI3c3W<PricIo35cSpec> {
        EnblReadGroup2ofI3c3W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of I3C3"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c3(&mut self) -> EnblReadGroup3ofI3c3W<PricIo35cSpec> {
        EnblReadGroup3ofI3c3W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of I3C3"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c3(&mut self) -> EnblReadGroup4ofI3c3W<PricIo35cSpec> {
        EnblReadGroup4ofI3c3W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of I3C3"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c3(&mut self) -> EnblReadGroup5ofI3c3W<PricIo35cSpec> {
        EnblReadGroup5ofI3c3W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric135cpric135c1308(
        &mut self,
    ) -> EnblRstToleranceOfPric135cpric135c1308W<PricIo35cSpec> {
        EnblRstToleranceOfPric135cpric135c1308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC135CPRIC1_35C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric135cpric135c1408(
        &mut self,
    ) -> EnblWrProtOfPric135cpric135c1408W<PricIo35cSpec> {
        EnblWrProtOfPric135cpric135c1408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of I3C4"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c4(&mut self) -> EnblReadGroup0ofI3c4W<PricIo35cSpec> {
        EnblReadGroup0ofI3c4W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of I3C4"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c4(&mut self) -> EnblReadGroup1ofI3c4W<PricIo35cSpec> {
        EnblReadGroup1ofI3c4W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of I3C4"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c4(&mut self) -> EnblReadGroup2ofI3c4W<PricIo35cSpec> {
        EnblReadGroup2ofI3c4W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of I3C4"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c4(&mut self) -> EnblReadGroup3ofI3c4W<PricIo35cSpec> {
        EnblReadGroup3ofI3c4W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of I3C4"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c4(&mut self) -> EnblReadGroup4ofI3c4W<PricIo35cSpec> {
        EnblReadGroup4ofI3c4W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of I3C4"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c4(&mut self) -> EnblReadGroup5ofI3c4W<PricIo35cSpec> {
        EnblReadGroup5ofI3c4W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric135cpric135c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric135cpric135c2116W<PricIo35cSpec> {
        EnblRstToleranceOfPric135cpric135c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC135CPRIC1_35C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric135cpric135c2216(
        &mut self,
    ) -> EnblWrProtOfPric135cpric135c2216W<PricIo35cSpec> {
        EnblWrProtOfPric135cpric135c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of I3C5"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c5(&mut self) -> EnblReadGroup0ofI3c5W<PricIo35cSpec> {
        EnblReadGroup0ofI3c5W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of I3C5"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c5(&mut self) -> EnblReadGroup1ofI3c5W<PricIo35cSpec> {
        EnblReadGroup1ofI3c5W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of I3C5"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c5(&mut self) -> EnblReadGroup2ofI3c5W<PricIo35cSpec> {
        EnblReadGroup2ofI3c5W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of I3C5"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c5(&mut self) -> EnblReadGroup3ofI3c5W<PricIo35cSpec> {
        EnblReadGroup3ofI3c5W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of I3C5"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c5(&mut self) -> EnblReadGroup4ofI3c5W<PricIo35cSpec> {
        EnblReadGroup4ofI3c5W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of I3C5"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c5(&mut self) -> EnblReadGroup5ofI3c5W<PricIo35cSpec> {
        EnblReadGroup5ofI3c5W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC135CPRIC1_35C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric135cpric135c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric135cpric135c2924W<PricIo35cSpec> {
        EnblRstToleranceOfPric135cpric135c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC135CPRIC1_35C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric135cpric135c3024(
        &mut self,
    ) -> EnblWrProtOfPric135cpric135c3024W<PricIo35cSpec> {
        EnblWrProtOfPric135cpric135c3024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io35c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io35c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo35cSpec;
impl crate::RegisterSpec for PricIo35cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io35c::R`](R) reader structure"]
impl crate::Readable for PricIo35cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io35c::W`](W) writer structure"]
impl crate::Writable for PricIo35cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO35C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo35cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
