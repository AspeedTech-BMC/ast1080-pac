#[doc = "Register `PRIC_IO25C` reader"]
pub type R = crate::R<PricIo25cSpec>;
#[doc = "Register `PRIC_IO25C` writer"]
pub type W = crate::W<PricIo25cSpec>;
#[doc = "Field `EnblWrGroup0OfI3C2` reader - Enable Write Group #0 of I3C2"]
pub type EnblWrGroup0ofI3c2R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI3C2` writer - Enable Write Group #0 of I3C2"]
pub type EnblWrGroup0ofI3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI3C2` reader - Enable Write Group #1 of I3C2"]
pub type EnblWrGroup1ofI3c2R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI3C2` writer - Enable Write Group #1 of I3C2"]
pub type EnblWrGroup1ofI3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI3C2` reader - Enable Write Group #2 of I3C2"]
pub type EnblWrGroup2ofI3c2R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI3C2` writer - Enable Write Group #2 of I3C2"]
pub type EnblWrGroup2ofI3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI3C2` reader - Enable Write Group #3 of I3C2"]
pub type EnblWrGroup3ofI3c2R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI3C2` writer - Enable Write Group #3 of I3C2"]
pub type EnblWrGroup3ofI3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI3C2` reader - Enable Write Group #4 of I3C2"]
pub type EnblWrGroup4ofI3c2R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI3C2` writer - Enable Write Group #4 of I3C2"]
pub type EnblWrGroup4ofI3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI3C2` reader - Enable Write Group #5 of I3C2"]
pub type EnblWrGroup5ofI3c2R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI3C2` writer - Enable Write Group #5 of I3C2"]
pub type EnblWrGroup5ofI3c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC125CPRIC1_25C\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric125cpric125c0500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric125cpric125c0500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric125cpric125c0500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC125CPRIC125C0500` reader - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[05:00\\]"]
pub type EnblRstToleranceOfPric125cpric125c0500R =
    crate::BitReader<EnblRstToleranceOfPric125cpric125c0500>;
impl EnblRstToleranceOfPric125cpric125c0500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric125cpric125c0500 {
        match self.bits {
            false => EnblRstToleranceOfPric125cpric125c0500::ResetBySrst,
            true => EnblRstToleranceOfPric125cpric125c0500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric125cpric125c0500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric125cpric125c0500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC125CPRIC125C0500` writer - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[05:00\\]"]
pub type EnblRstToleranceOfPric125cpric125c0500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric125cpric125c0500>;
impl<'a, REG> EnblRstToleranceOfPric125cpric125c0500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric125cpric125c0500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric125cpric125c0500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC125CPRIC125C0600` reader - Enable Write Protection of PRIC125CPRIC1_25C\\[06:00\\]"]
pub type EnblWrProtOfPric125cpric125c0600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC125CPRIC125C0600` writer - Enable Write Protection of PRIC125CPRIC1_25C\\[06:00\\]"]
pub type EnblWrProtOfPric125cpric125c0600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfI3C3` reader - Enable Write Group #0 of I3C3"]
pub type EnblWrGroup0ofI3c3R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI3C3` writer - Enable Write Group #0 of I3C3"]
pub type EnblWrGroup0ofI3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI3C3` reader - Enable Write Group #1 of I3C3"]
pub type EnblWrGroup1ofI3c3R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI3C3` writer - Enable Write Group #1 of I3C3"]
pub type EnblWrGroup1ofI3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI3C3` reader - Enable Write Group #2 of I3C3"]
pub type EnblWrGroup2ofI3c3R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI3C3` writer - Enable Write Group #2 of I3C3"]
pub type EnblWrGroup2ofI3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI3C3` reader - Enable Write Group #3 of I3C3"]
pub type EnblWrGroup3ofI3c3R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI3C3` writer - Enable Write Group #3 of I3C3"]
pub type EnblWrGroup3ofI3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI3C3` reader - Enable Write Group #4 of I3C3"]
pub type EnblWrGroup4ofI3c3R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI3C3` writer - Enable Write Group #4 of I3C3"]
pub type EnblWrGroup4ofI3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI3C3` reader - Enable Write Group #5 of I3C3"]
pub type EnblWrGroup5ofI3c3R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI3C3` writer - Enable Write Group #5 of I3C3"]
pub type EnblWrGroup5ofI3c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC125CPRIC1_25C\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric125cpric125c1308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric125cpric125c1308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric125cpric125c1308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC125CPRIC125C1308` reader - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[13:08\\]"]
pub type EnblRstToleranceOfPric125cpric125c1308R =
    crate::BitReader<EnblRstToleranceOfPric125cpric125c1308>;
impl EnblRstToleranceOfPric125cpric125c1308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric125cpric125c1308 {
        match self.bits {
            false => EnblRstToleranceOfPric125cpric125c1308::ResetBySrst,
            true => EnblRstToleranceOfPric125cpric125c1308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric125cpric125c1308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric125cpric125c1308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC125CPRIC125C1308` writer - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[13:08\\]"]
pub type EnblRstToleranceOfPric125cpric125c1308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric125cpric125c1308>;
impl<'a, REG> EnblRstToleranceOfPric125cpric125c1308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric125cpric125c1308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric125cpric125c1308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC125CPRIC125C1408` reader - Enable Write Protection of PRIC125CPRIC1_25C\\[14:08\\]"]
pub type EnblWrProtOfPric125cpric125c1408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC125CPRIC125C1408` writer - Enable Write Protection of PRIC125CPRIC1_25C\\[14:08\\]"]
pub type EnblWrProtOfPric125cpric125c1408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfI3C4` reader - Enable Write Group #0 of I3C4"]
pub type EnblWrGroup0ofI3c4R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI3C4` writer - Enable Write Group #0 of I3C4"]
pub type EnblWrGroup0ofI3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI3C4` reader - Enable Write Group #1 of I3C4"]
pub type EnblWrGroup1ofI3c4R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI3C4` writer - Enable Write Group #1 of I3C4"]
pub type EnblWrGroup1ofI3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI3C4` reader - Enable Write Group #2 of I3C4"]
pub type EnblWrGroup2ofI3c4R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI3C4` writer - Enable Write Group #2 of I3C4"]
pub type EnblWrGroup2ofI3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI3C4` reader - Enable Write Group #3 of I3C4"]
pub type EnblWrGroup3ofI3c4R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI3C4` writer - Enable Write Group #3 of I3C4"]
pub type EnblWrGroup3ofI3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI3C4` reader - Enable Write Group #4 of I3C4"]
pub type EnblWrGroup4ofI3c4R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI3C4` writer - Enable Write Group #4 of I3C4"]
pub type EnblWrGroup4ofI3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI3C4` reader - Enable Write Group #5 of I3C4"]
pub type EnblWrGroup5ofI3c4R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI3C4` writer - Enable Write Group #5 of I3C4"]
pub type EnblWrGroup5ofI3c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC125CPRIC1_25C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric125cpric125c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric125cpric125c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric125cpric125c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC125CPRIC125C2116` reader - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[21:16\\]"]
pub type EnblRstToleranceOfPric125cpric125c2116R =
    crate::BitReader<EnblRstToleranceOfPric125cpric125c2116>;
impl EnblRstToleranceOfPric125cpric125c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric125cpric125c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric125cpric125c2116::ResetBySrst,
            true => EnblRstToleranceOfPric125cpric125c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric125cpric125c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric125cpric125c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC125CPRIC125C2116` writer - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[21:16\\]"]
pub type EnblRstToleranceOfPric125cpric125c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric125cpric125c2116>;
impl<'a, REG> EnblRstToleranceOfPric125cpric125c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric125cpric125c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric125cpric125c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC125CPRIC125C2216` reader - Enable Write Protection of PRIC125CPRIC1_25C\\[22:16\\]"]
pub type EnblWrProtOfPric125cpric125c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC125CPRIC125C2216` writer - Enable Write Protection of PRIC125CPRIC1_25C\\[22:16\\]"]
pub type EnblWrProtOfPric125cpric125c2216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfI3C5` reader - Enable Write Group #0 of I3C5"]
pub type EnblWrGroup0ofI3c5R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI3C5` writer - Enable Write Group #0 of I3C5"]
pub type EnblWrGroup0ofI3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI3C5` reader - Enable Write Group #1 of I3C5"]
pub type EnblWrGroup1ofI3c5R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI3C5` writer - Enable Write Group #1 of I3C5"]
pub type EnblWrGroup1ofI3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI3C5` reader - Enable Write Group #2 of I3C5"]
pub type EnblWrGroup2ofI3c5R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI3C5` writer - Enable Write Group #2 of I3C5"]
pub type EnblWrGroup2ofI3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI3C5` reader - Enable Write Group #3 of I3C5"]
pub type EnblWrGroup3ofI3c5R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI3C5` writer - Enable Write Group #3 of I3C5"]
pub type EnblWrGroup3ofI3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI3C5` reader - Enable Write Group #4 of I3C5"]
pub type EnblWrGroup4ofI3c5R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI3C5` writer - Enable Write Group #4 of I3C5"]
pub type EnblWrGroup4ofI3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI3C5` reader - Enable Write Group #5 of I3C5"]
pub type EnblWrGroup5ofI3c5R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI3C5` writer - Enable Write Group #5 of I3C5"]
pub type EnblWrGroup5ofI3c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC125CPRIC1_25C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric125cpric125c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric125cpric125c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric125cpric125c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC125CPRIC125C2924` reader - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[29:24\\]"]
pub type EnblRstToleranceOfPric125cpric125c2924R =
    crate::BitReader<EnblRstToleranceOfPric125cpric125c2924>;
impl EnblRstToleranceOfPric125cpric125c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric125cpric125c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric125cpric125c2924::ResetBySrst,
            true => EnblRstToleranceOfPric125cpric125c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric125cpric125c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric125cpric125c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC125CPRIC125C2924` writer - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[29:24\\]"]
pub type EnblRstToleranceOfPric125cpric125c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric125cpric125c2924>;
impl<'a, REG> EnblRstToleranceOfPric125cpric125c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric125cpric125c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric125cpric125c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC125CPRIC125C3024` reader - Enable Write Protection of PRIC125CPRIC1_25C\\[30:24\\]"]
pub type EnblWrProtOfPric125cpric125c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC125CPRIC125C3024` writer - Enable Write Protection of PRIC125CPRIC1_25C\\[30:24\\]"]
pub type EnblWrProtOfPric125cpric125c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of I3C2"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i3c2(&self) -> EnblWrGroup0ofI3c2R {
        EnblWrGroup0ofI3c2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of I3C2"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i3c2(&self) -> EnblWrGroup1ofI3c2R {
        EnblWrGroup1ofI3c2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of I3C2"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i3c2(&self) -> EnblWrGroup2ofI3c2R {
        EnblWrGroup2ofI3c2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of I3C2"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i3c2(&self) -> EnblWrGroup3ofI3c2R {
        EnblWrGroup3ofI3c2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of I3C2"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i3c2(&self) -> EnblWrGroup4ofI3c2R {
        EnblWrGroup4ofI3c2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of I3C2"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i3c2(&self) -> EnblWrGroup5ofI3c2R {
        EnblWrGroup5ofI3c2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric125cpric125c0500(
        &self,
    ) -> EnblRstToleranceOfPric125cpric125c0500R {
        EnblRstToleranceOfPric125cpric125c0500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC125CPRIC1_25C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric125cpric125c0600(&self) -> EnblWrProtOfPric125cpric125c0600R {
        EnblWrProtOfPric125cpric125c0600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of I3C3"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i3c3(&self) -> EnblWrGroup0ofI3c3R {
        EnblWrGroup0ofI3c3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of I3C3"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i3c3(&self) -> EnblWrGroup1ofI3c3R {
        EnblWrGroup1ofI3c3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of I3C3"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i3c3(&self) -> EnblWrGroup2ofI3c3R {
        EnblWrGroup2ofI3c3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of I3C3"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i3c3(&self) -> EnblWrGroup3ofI3c3R {
        EnblWrGroup3ofI3c3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of I3C3"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i3c3(&self) -> EnblWrGroup4ofI3c3R {
        EnblWrGroup4ofI3c3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of I3C3"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i3c3(&self) -> EnblWrGroup5ofI3c3R {
        EnblWrGroup5ofI3c3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric125cpric125c1308(
        &self,
    ) -> EnblRstToleranceOfPric125cpric125c1308R {
        EnblRstToleranceOfPric125cpric125c1308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC125CPRIC1_25C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric125cpric125c1408(&self) -> EnblWrProtOfPric125cpric125c1408R {
        EnblWrProtOfPric125cpric125c1408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of I3C4"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i3c4(&self) -> EnblWrGroup0ofI3c4R {
        EnblWrGroup0ofI3c4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of I3C4"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i3c4(&self) -> EnblWrGroup1ofI3c4R {
        EnblWrGroup1ofI3c4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of I3C4"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i3c4(&self) -> EnblWrGroup2ofI3c4R {
        EnblWrGroup2ofI3c4R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of I3C4"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i3c4(&self) -> EnblWrGroup3ofI3c4R {
        EnblWrGroup3ofI3c4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of I3C4"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i3c4(&self) -> EnblWrGroup4ofI3c4R {
        EnblWrGroup4ofI3c4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of I3C4"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i3c4(&self) -> EnblWrGroup5ofI3c4R {
        EnblWrGroup5ofI3c4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric125cpric125c2116(
        &self,
    ) -> EnblRstToleranceOfPric125cpric125c2116R {
        EnblRstToleranceOfPric125cpric125c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC125CPRIC1_25C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric125cpric125c2216(&self) -> EnblWrProtOfPric125cpric125c2216R {
        EnblWrProtOfPric125cpric125c2216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of I3C5"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i3c5(&self) -> EnblWrGroup0ofI3c5R {
        EnblWrGroup0ofI3c5R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of I3C5"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i3c5(&self) -> EnblWrGroup1ofI3c5R {
        EnblWrGroup1ofI3c5R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of I3C5"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i3c5(&self) -> EnblWrGroup2ofI3c5R {
        EnblWrGroup2ofI3c5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of I3C5"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i3c5(&self) -> EnblWrGroup3ofI3c5R {
        EnblWrGroup3ofI3c5R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of I3C5"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i3c5(&self) -> EnblWrGroup4ofI3c5R {
        EnblWrGroup4ofI3c5R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of I3C5"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i3c5(&self) -> EnblWrGroup5ofI3c5R {
        EnblWrGroup5ofI3c5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric125cpric125c2924(
        &self,
    ) -> EnblRstToleranceOfPric125cpric125c2924R {
        EnblRstToleranceOfPric125cpric125c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC125CPRIC1_25C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric125cpric125c3024(&self) -> EnblWrProtOfPric125cpric125c3024R {
        EnblWrProtOfPric125cpric125c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of I3C2"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i3c2(&mut self) -> EnblWrGroup0ofI3c2W<PricIo25cSpec> {
        EnblWrGroup0ofI3c2W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of I3C2"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i3c2(&mut self) -> EnblWrGroup1ofI3c2W<PricIo25cSpec> {
        EnblWrGroup1ofI3c2W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of I3C2"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i3c2(&mut self) -> EnblWrGroup2ofI3c2W<PricIo25cSpec> {
        EnblWrGroup2ofI3c2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of I3C2"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i3c2(&mut self) -> EnblWrGroup3ofI3c2W<PricIo25cSpec> {
        EnblWrGroup3ofI3c2W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of I3C2"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i3c2(&mut self) -> EnblWrGroup4ofI3c2W<PricIo25cSpec> {
        EnblWrGroup4ofI3c2W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of I3C2"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i3c2(&mut self) -> EnblWrGroup5ofI3c2W<PricIo25cSpec> {
        EnblWrGroup5ofI3c2W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric125cpric125c0500(
        &mut self,
    ) -> EnblRstToleranceOfPric125cpric125c0500W<PricIo25cSpec> {
        EnblRstToleranceOfPric125cpric125c0500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC125CPRIC1_25C\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric125cpric125c0600(
        &mut self,
    ) -> EnblWrProtOfPric125cpric125c0600W<PricIo25cSpec> {
        EnblWrProtOfPric125cpric125c0600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of I3C3"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i3c3(&mut self) -> EnblWrGroup0ofI3c3W<PricIo25cSpec> {
        EnblWrGroup0ofI3c3W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of I3C3"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i3c3(&mut self) -> EnblWrGroup1ofI3c3W<PricIo25cSpec> {
        EnblWrGroup1ofI3c3W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of I3C3"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i3c3(&mut self) -> EnblWrGroup2ofI3c3W<PricIo25cSpec> {
        EnblWrGroup2ofI3c3W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of I3C3"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i3c3(&mut self) -> EnblWrGroup3ofI3c3W<PricIo25cSpec> {
        EnblWrGroup3ofI3c3W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of I3C3"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i3c3(&mut self) -> EnblWrGroup4ofI3c3W<PricIo25cSpec> {
        EnblWrGroup4ofI3c3W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of I3C3"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i3c3(&mut self) -> EnblWrGroup5ofI3c3W<PricIo25cSpec> {
        EnblWrGroup5ofI3c3W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric125cpric125c1308(
        &mut self,
    ) -> EnblRstToleranceOfPric125cpric125c1308W<PricIo25cSpec> {
        EnblRstToleranceOfPric125cpric125c1308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC125CPRIC1_25C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric125cpric125c1408(
        &mut self,
    ) -> EnblWrProtOfPric125cpric125c1408W<PricIo25cSpec> {
        EnblWrProtOfPric125cpric125c1408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of I3C4"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i3c4(&mut self) -> EnblWrGroup0ofI3c4W<PricIo25cSpec> {
        EnblWrGroup0ofI3c4W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of I3C4"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i3c4(&mut self) -> EnblWrGroup1ofI3c4W<PricIo25cSpec> {
        EnblWrGroup1ofI3c4W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of I3C4"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i3c4(&mut self) -> EnblWrGroup2ofI3c4W<PricIo25cSpec> {
        EnblWrGroup2ofI3c4W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of I3C4"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i3c4(&mut self) -> EnblWrGroup3ofI3c4W<PricIo25cSpec> {
        EnblWrGroup3ofI3c4W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of I3C4"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i3c4(&mut self) -> EnblWrGroup4ofI3c4W<PricIo25cSpec> {
        EnblWrGroup4ofI3c4W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of I3C4"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i3c4(&mut self) -> EnblWrGroup5ofI3c4W<PricIo25cSpec> {
        EnblWrGroup5ofI3c4W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric125cpric125c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric125cpric125c2116W<PricIo25cSpec> {
        EnblRstToleranceOfPric125cpric125c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC125CPRIC1_25C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric125cpric125c2216(
        &mut self,
    ) -> EnblWrProtOfPric125cpric125c2216W<PricIo25cSpec> {
        EnblWrProtOfPric125cpric125c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of I3C5"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i3c5(&mut self) -> EnblWrGroup0ofI3c5W<PricIo25cSpec> {
        EnblWrGroup0ofI3c5W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of I3C5"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i3c5(&mut self) -> EnblWrGroup1ofI3c5W<PricIo25cSpec> {
        EnblWrGroup1ofI3c5W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of I3C5"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i3c5(&mut self) -> EnblWrGroup2ofI3c5W<PricIo25cSpec> {
        EnblWrGroup2ofI3c5W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of I3C5"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i3c5(&mut self) -> EnblWrGroup3ofI3c5W<PricIo25cSpec> {
        EnblWrGroup3ofI3c5W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of I3C5"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i3c5(&mut self) -> EnblWrGroup4ofI3c5W<PricIo25cSpec> {
        EnblWrGroup4ofI3c5W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of I3C5"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i3c5(&mut self) -> EnblWrGroup5ofI3c5W<PricIo25cSpec> {
        EnblWrGroup5ofI3c5W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC125CPRIC1_25C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric125cpric125c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric125cpric125c2924W<PricIo25cSpec> {
        EnblRstToleranceOfPric125cpric125c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC125CPRIC1_25C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric125cpric125c3024(
        &mut self,
    ) -> EnblWrProtOfPric125cpric125c3024W<PricIo25cSpec> {
        EnblWrProtOfPric125cpric125c3024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io25c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io25c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo25cSpec;
impl crate::RegisterSpec for PricIo25cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io25c::R`](R) reader structure"]
impl crate::Readable for PricIo25cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io25c::W`](W) writer structure"]
impl crate::Writable for PricIo25cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO25C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo25cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
