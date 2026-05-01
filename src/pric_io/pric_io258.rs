#[doc = "Register `PRIC_IO258` reader"]
pub type R = crate::R<PricIo258Spec>;
#[doc = "Register `PRIC_IO258` writer"]
pub type W = crate::W<PricIo258Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EnblWrGroup0OfPECI` reader - Enable Write Group #0 of PECI"]
pub type EnblWrGroup0ofPeciR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfPECI` writer - Enable Write Group #0 of PECI"]
pub type EnblWrGroup0ofPeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfPECI` reader - Enable Write Group #1 of PECI"]
pub type EnblWrGroup1ofPeciR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfPECI` writer - Enable Write Group #1 of PECI"]
pub type EnblWrGroup1ofPeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfPECI` reader - Enable Write Group #2 of PECI"]
pub type EnblWrGroup2ofPeciR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfPECI` writer - Enable Write Group #2 of PECI"]
pub type EnblWrGroup2ofPeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfPECI` reader - Enable Write Group #3 of PECI"]
pub type EnblWrGroup3ofPeciR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfPECI` writer - Enable Write Group #3 of PECI"]
pub type EnblWrGroup3ofPeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfPECI` reader - Enable Write Group #4 of PECI"]
pub type EnblWrGroup4ofPeciR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfPECI` writer - Enable Write Group #4 of PECI"]
pub type EnblWrGroup4ofPeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfPECI` reader - Enable Write Group #5 of PECI"]
pub type EnblWrGroup5ofPeciR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfPECI` writer - Enable Write Group #5 of PECI"]
pub type EnblWrGroup5ofPeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1258PRIC1_258\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1258pric12581308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1258pric12581308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1258pric12581308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1258PRIC12581308` reader - Enable Reset Tolerance of PRIC1258PRIC1_258\\[13:08\\]"]
pub type EnblRstToleranceOfPric1258pric12581308R =
    crate::BitReader<EnblRstToleranceOfPric1258pric12581308>;
impl EnblRstToleranceOfPric1258pric12581308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1258pric12581308 {
        match self.bits {
            false => EnblRstToleranceOfPric1258pric12581308::ResetBySrst,
            true => EnblRstToleranceOfPric1258pric12581308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1258pric12581308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1258pric12581308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1258PRIC12581308` writer - Enable Reset Tolerance of PRIC1258PRIC1_258\\[13:08\\]"]
pub type EnblRstToleranceOfPric1258pric12581308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1258pric12581308>;
impl<'a, REG> EnblRstToleranceOfPric1258pric12581308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1258pric12581308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1258pric12581308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1258PRIC12581408` reader - Enable Write Protection of PRIC1258PRIC1_258\\[14:08\\]"]
pub type EnblWrProtOfPric1258pric12581408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1258PRIC12581408` writer - Enable Write Protection of PRIC1258PRIC1_258\\[14:08\\]"]
pub type EnblWrProtOfPric1258pric12581408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfI3C0` reader - Enable Write Group #0 of I3C0"]
pub type EnblWrGroup0ofI3c0R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI3C0` writer - Enable Write Group #0 of I3C0"]
pub type EnblWrGroup0ofI3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI3C0` reader - Enable Write Group #1 of I3C0"]
pub type EnblWrGroup1ofI3c0R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI3C0` writer - Enable Write Group #1 of I3C0"]
pub type EnblWrGroup1ofI3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI3C0` reader - Enable Write Group #2 of I3C0"]
pub type EnblWrGroup2ofI3c0R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI3C0` writer - Enable Write Group #2 of I3C0"]
pub type EnblWrGroup2ofI3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI3C0` reader - Enable Write Group #3 of I3C0"]
pub type EnblWrGroup3ofI3c0R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI3C0` writer - Enable Write Group #3 of I3C0"]
pub type EnblWrGroup3ofI3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI3C0` reader - Enable Write Group #4 of I3C0"]
pub type EnblWrGroup4ofI3c0R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI3C0` writer - Enable Write Group #4 of I3C0"]
pub type EnblWrGroup4ofI3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI3C0` reader - Enable Write Group #5 of I3C0"]
pub type EnblWrGroup5ofI3c0R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI3C0` writer - Enable Write Group #5 of I3C0"]
pub type EnblWrGroup5ofI3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1258PRIC1_258\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1258pric12582116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1258pric12582116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1258pric12582116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1258PRIC12582116` reader - Enable Reset Tolerance of PRIC1258PRIC1_258\\[21:16\\]"]
pub type EnblRstToleranceOfPric1258pric12582116R =
    crate::BitReader<EnblRstToleranceOfPric1258pric12582116>;
impl EnblRstToleranceOfPric1258pric12582116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1258pric12582116 {
        match self.bits {
            false => EnblRstToleranceOfPric1258pric12582116::ResetBySrst,
            true => EnblRstToleranceOfPric1258pric12582116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1258pric12582116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1258pric12582116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1258PRIC12582116` writer - Enable Reset Tolerance of PRIC1258PRIC1_258\\[21:16\\]"]
pub type EnblRstToleranceOfPric1258pric12582116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1258pric12582116>;
impl<'a, REG> EnblRstToleranceOfPric1258pric12582116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1258pric12582116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1258pric12582116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1258PRIC12582216` reader - Enable Write Protection of PRIC1258PRIC1_258\\[22:16\\]"]
pub type EnblWrProtOfPric1258pric12582216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1258PRIC12582216` writer - Enable Write Protection of PRIC1258PRIC1_258\\[22:16\\]"]
pub type EnblWrProtOfPric1258pric12582216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfI3C1` reader - Enable Write Group #0 of I3C1"]
pub type EnblWrGroup0ofI3c1R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI3C1` writer - Enable Write Group #0 of I3C1"]
pub type EnblWrGroup0ofI3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI3C1` reader - Enable Write Group #1 of I3C1"]
pub type EnblWrGroup1ofI3c1R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI3C1` writer - Enable Write Group #1 of I3C1"]
pub type EnblWrGroup1ofI3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI3C1` reader - Enable Write Group #2 of I3C1"]
pub type EnblWrGroup2ofI3c1R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI3C1` writer - Enable Write Group #2 of I3C1"]
pub type EnblWrGroup2ofI3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI3C1` reader - Enable Write Group #3 of I3C1"]
pub type EnblWrGroup3ofI3c1R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI3C1` writer - Enable Write Group #3 of I3C1"]
pub type EnblWrGroup3ofI3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI3C1` reader - Enable Write Group #4 of I3C1"]
pub type EnblWrGroup4ofI3c1R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI3C1` writer - Enable Write Group #4 of I3C1"]
pub type EnblWrGroup4ofI3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI3C1` reader - Enable Write Group #5 of I3C1"]
pub type EnblWrGroup5ofI3c1R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI3C1` writer - Enable Write Group #5 of I3C1"]
pub type EnblWrGroup5ofI3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1258PRIC1_258\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1258pric12582924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1258pric12582924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1258pric12582924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1258PRIC12582924` reader - Enable Reset Tolerance of PRIC1258PRIC1_258\\[29:24\\]"]
pub type EnblRstToleranceOfPric1258pric12582924R =
    crate::BitReader<EnblRstToleranceOfPric1258pric12582924>;
impl EnblRstToleranceOfPric1258pric12582924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1258pric12582924 {
        match self.bits {
            false => EnblRstToleranceOfPric1258pric12582924::ResetBySrst,
            true => EnblRstToleranceOfPric1258pric12582924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1258pric12582924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1258pric12582924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1258PRIC12582924` writer - Enable Reset Tolerance of PRIC1258PRIC1_258\\[29:24\\]"]
pub type EnblRstToleranceOfPric1258pric12582924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1258pric12582924>;
impl<'a, REG> EnblRstToleranceOfPric1258pric12582924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1258pric12582924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1258pric12582924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1258PRIC12583024` reader - Enable Write Protection of PRIC1258PRIC1_258\\[30:24\\]"]
pub type EnblWrProtOfPric1258pric12583024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1258PRIC12583024` writer - Enable Write Protection of PRIC1258PRIC1_258\\[30:24\\]"]
pub type EnblWrProtOfPric1258pric12583024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of PECI"]
    #[inline(always)]
    pub fn enbl_wr_group0of_peci(&self) -> EnblWrGroup0ofPeciR {
        EnblWrGroup0ofPeciR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of PECI"]
    #[inline(always)]
    pub fn enbl_wr_group1of_peci(&self) -> EnblWrGroup1ofPeciR {
        EnblWrGroup1ofPeciR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of PECI"]
    #[inline(always)]
    pub fn enbl_wr_group2of_peci(&self) -> EnblWrGroup2ofPeciR {
        EnblWrGroup2ofPeciR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of PECI"]
    #[inline(always)]
    pub fn enbl_wr_group3of_peci(&self) -> EnblWrGroup3ofPeciR {
        EnblWrGroup3ofPeciR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of PECI"]
    #[inline(always)]
    pub fn enbl_wr_group4of_peci(&self) -> EnblWrGroup4ofPeciR {
        EnblWrGroup4ofPeciR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of PECI"]
    #[inline(always)]
    pub fn enbl_wr_group5of_peci(&self) -> EnblWrGroup5ofPeciR {
        EnblWrGroup5ofPeciR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1258PRIC1_258\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1258pric12581308(
        &self,
    ) -> EnblRstToleranceOfPric1258pric12581308R {
        EnblRstToleranceOfPric1258pric12581308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1258PRIC1_258\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1258pric12581408(&self) -> EnblWrProtOfPric1258pric12581408R {
        EnblWrProtOfPric1258pric12581408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of I3C0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i3c0(&self) -> EnblWrGroup0ofI3c0R {
        EnblWrGroup0ofI3c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of I3C0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i3c0(&self) -> EnblWrGroup1ofI3c0R {
        EnblWrGroup1ofI3c0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of I3C0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i3c0(&self) -> EnblWrGroup2ofI3c0R {
        EnblWrGroup2ofI3c0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of I3C0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i3c0(&self) -> EnblWrGroup3ofI3c0R {
        EnblWrGroup3ofI3c0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of I3C0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i3c0(&self) -> EnblWrGroup4ofI3c0R {
        EnblWrGroup4ofI3c0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of I3C0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i3c0(&self) -> EnblWrGroup5ofI3c0R {
        EnblWrGroup5ofI3c0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1258PRIC1_258\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1258pric12582116(
        &self,
    ) -> EnblRstToleranceOfPric1258pric12582116R {
        EnblRstToleranceOfPric1258pric12582116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1258PRIC1_258\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1258pric12582216(&self) -> EnblWrProtOfPric1258pric12582216R {
        EnblWrProtOfPric1258pric12582216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of I3C1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i3c1(&self) -> EnblWrGroup0ofI3c1R {
        EnblWrGroup0ofI3c1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of I3C1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i3c1(&self) -> EnblWrGroup1ofI3c1R {
        EnblWrGroup1ofI3c1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of I3C1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i3c1(&self) -> EnblWrGroup2ofI3c1R {
        EnblWrGroup2ofI3c1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of I3C1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i3c1(&self) -> EnblWrGroup3ofI3c1R {
        EnblWrGroup3ofI3c1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of I3C1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i3c1(&self) -> EnblWrGroup4ofI3c1R {
        EnblWrGroup4ofI3c1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of I3C1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i3c1(&self) -> EnblWrGroup5ofI3c1R {
        EnblWrGroup5ofI3c1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1258PRIC1_258\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1258pric12582924(
        &self,
    ) -> EnblRstToleranceOfPric1258pric12582924R {
        EnblRstToleranceOfPric1258pric12582924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1258PRIC1_258\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1258pric12583024(&self) -> EnblWrProtOfPric1258pric12583024R {
        EnblWrProtOfPric1258pric12583024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo258Spec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of PECI"]
    #[inline(always)]
    pub fn enbl_wr_group0of_peci(&mut self) -> EnblWrGroup0ofPeciW<PricIo258Spec> {
        EnblWrGroup0ofPeciW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of PECI"]
    #[inline(always)]
    pub fn enbl_wr_group1of_peci(&mut self) -> EnblWrGroup1ofPeciW<PricIo258Spec> {
        EnblWrGroup1ofPeciW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of PECI"]
    #[inline(always)]
    pub fn enbl_wr_group2of_peci(&mut self) -> EnblWrGroup2ofPeciW<PricIo258Spec> {
        EnblWrGroup2ofPeciW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of PECI"]
    #[inline(always)]
    pub fn enbl_wr_group3of_peci(&mut self) -> EnblWrGroup3ofPeciW<PricIo258Spec> {
        EnblWrGroup3ofPeciW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of PECI"]
    #[inline(always)]
    pub fn enbl_wr_group4of_peci(&mut self) -> EnblWrGroup4ofPeciW<PricIo258Spec> {
        EnblWrGroup4ofPeciW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of PECI"]
    #[inline(always)]
    pub fn enbl_wr_group5of_peci(&mut self) -> EnblWrGroup5ofPeciW<PricIo258Spec> {
        EnblWrGroup5ofPeciW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1258PRIC1_258\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1258pric12581308(
        &mut self,
    ) -> EnblRstToleranceOfPric1258pric12581308W<PricIo258Spec> {
        EnblRstToleranceOfPric1258pric12581308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1258PRIC1_258\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1258pric12581408(
        &mut self,
    ) -> EnblWrProtOfPric1258pric12581408W<PricIo258Spec> {
        EnblWrProtOfPric1258pric12581408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of I3C0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i3c0(&mut self) -> EnblWrGroup0ofI3c0W<PricIo258Spec> {
        EnblWrGroup0ofI3c0W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of I3C0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i3c0(&mut self) -> EnblWrGroup1ofI3c0W<PricIo258Spec> {
        EnblWrGroup1ofI3c0W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of I3C0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i3c0(&mut self) -> EnblWrGroup2ofI3c0W<PricIo258Spec> {
        EnblWrGroup2ofI3c0W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of I3C0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i3c0(&mut self) -> EnblWrGroup3ofI3c0W<PricIo258Spec> {
        EnblWrGroup3ofI3c0W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of I3C0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i3c0(&mut self) -> EnblWrGroup4ofI3c0W<PricIo258Spec> {
        EnblWrGroup4ofI3c0W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of I3C0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i3c0(&mut self) -> EnblWrGroup5ofI3c0W<PricIo258Spec> {
        EnblWrGroup5ofI3c0W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1258PRIC1_258\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1258pric12582116(
        &mut self,
    ) -> EnblRstToleranceOfPric1258pric12582116W<PricIo258Spec> {
        EnblRstToleranceOfPric1258pric12582116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1258PRIC1_258\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1258pric12582216(
        &mut self,
    ) -> EnblWrProtOfPric1258pric12582216W<PricIo258Spec> {
        EnblWrProtOfPric1258pric12582216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of I3C1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i3c1(&mut self) -> EnblWrGroup0ofI3c1W<PricIo258Spec> {
        EnblWrGroup0ofI3c1W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of I3C1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i3c1(&mut self) -> EnblWrGroup1ofI3c1W<PricIo258Spec> {
        EnblWrGroup1ofI3c1W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of I3C1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i3c1(&mut self) -> EnblWrGroup2ofI3c1W<PricIo258Spec> {
        EnblWrGroup2ofI3c1W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of I3C1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i3c1(&mut self) -> EnblWrGroup3ofI3c1W<PricIo258Spec> {
        EnblWrGroup3ofI3c1W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of I3C1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i3c1(&mut self) -> EnblWrGroup4ofI3c1W<PricIo258Spec> {
        EnblWrGroup4ofI3c1W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of I3C1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i3c1(&mut self) -> EnblWrGroup5ofI3c1W<PricIo258Spec> {
        EnblWrGroup5ofI3c1W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1258PRIC1_258\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1258pric12582924(
        &mut self,
    ) -> EnblRstToleranceOfPric1258pric12582924W<PricIo258Spec> {
        EnblRstToleranceOfPric1258pric12582924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1258PRIC1_258\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1258pric12583024(
        &mut self,
    ) -> EnblWrProtOfPric1258pric12583024W<PricIo258Spec> {
        EnblWrProtOfPric1258pric12583024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io258::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io258::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo258Spec;
impl crate::RegisterSpec for PricIo258Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io258::R`](R) reader structure"]
impl crate::Readable for PricIo258Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io258::W`](W) writer structure"]
impl crate::Writable for PricIo258Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO258 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo258Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
