#[doc = "Register `PRIC_IO358` reader"]
pub type R = crate::R<PricIo358Spec>;
#[doc = "Register `PRIC_IO358` writer"]
pub type W = crate::W<PricIo358Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EnblReadGroup0OfPECI` reader - Enable Read Group #0 of PECI"]
pub type EnblReadGroup0ofPeciR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfPECI` writer - Enable Read Group #0 of PECI"]
pub type EnblReadGroup0ofPeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfPECI` reader - Enable Read Group #1 of PECI"]
pub type EnblReadGroup1ofPeciR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfPECI` writer - Enable Read Group #1 of PECI"]
pub type EnblReadGroup1ofPeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfPECI` reader - Enable Read Group #2 of PECI"]
pub type EnblReadGroup2ofPeciR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfPECI` writer - Enable Read Group #2 of PECI"]
pub type EnblReadGroup2ofPeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfPECI` reader - Enable Read Group #3 of PECI"]
pub type EnblReadGroup3ofPeciR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfPECI` writer - Enable Read Group #3 of PECI"]
pub type EnblReadGroup3ofPeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfPECI` reader - Enable Read Group #4 of PECI"]
pub type EnblReadGroup4ofPeciR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfPECI` writer - Enable Read Group #4 of PECI"]
pub type EnblReadGroup4ofPeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfPECI` reader - Enable Read Group #5 of PECI"]
pub type EnblReadGroup5ofPeciR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfPECI` writer - Enable Read Group #5 of PECI"]
pub type EnblReadGroup5ofPeciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1358PRIC1_358\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1358pric13581308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1358pric13581308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1358pric13581308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1358PRIC13581308` reader - Enable Reset Tolerance of PRIC1358PRIC1_358\\[13:08\\]"]
pub type EnblRstToleranceOfPric1358pric13581308R =
    crate::BitReader<EnblRstToleranceOfPric1358pric13581308>;
impl EnblRstToleranceOfPric1358pric13581308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1358pric13581308 {
        match self.bits {
            false => EnblRstToleranceOfPric1358pric13581308::ResetBySrst,
            true => EnblRstToleranceOfPric1358pric13581308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1358pric13581308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1358pric13581308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1358PRIC13581308` writer - Enable Reset Tolerance of PRIC1358PRIC1_358\\[13:08\\]"]
pub type EnblRstToleranceOfPric1358pric13581308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1358pric13581308>;
impl<'a, REG> EnblRstToleranceOfPric1358pric13581308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1358pric13581308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1358pric13581308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1358PRIC13581408` reader - Enable Write Protection of PRIC1358PRIC1_358\\[14:08\\]"]
pub type EnblWrProtOfPric1358pric13581408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1358PRIC13581408` writer - Enable Write Protection of PRIC1358PRIC1_358\\[14:08\\]"]
pub type EnblWrProtOfPric1358pric13581408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI3C0` reader - Enable Read Group #0 of I3C0"]
pub type EnblReadGroup0ofI3c0R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI3C0` writer - Enable Read Group #0 of I3C0"]
pub type EnblReadGroup0ofI3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI3C0` reader - Enable Read Group #1 of I3C0"]
pub type EnblReadGroup1ofI3c0R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI3C0` writer - Enable Read Group #1 of I3C0"]
pub type EnblReadGroup1ofI3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI3C0` reader - Enable Read Group #2 of I3C0"]
pub type EnblReadGroup2ofI3c0R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI3C0` writer - Enable Read Group #2 of I3C0"]
pub type EnblReadGroup2ofI3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI3C0` reader - Enable Read Group #3 of I3C0"]
pub type EnblReadGroup3ofI3c0R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI3C0` writer - Enable Read Group #3 of I3C0"]
pub type EnblReadGroup3ofI3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI3C0` reader - Enable Read Group #4 of I3C0"]
pub type EnblReadGroup4ofI3c0R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI3C0` writer - Enable Read Group #4 of I3C0"]
pub type EnblReadGroup4ofI3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI3C0` reader - Enable Read Group #5 of I3C0"]
pub type EnblReadGroup5ofI3c0R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI3C0` writer - Enable Read Group #5 of I3C0"]
pub type EnblReadGroup5ofI3c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1358PRIC1_358\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1358pric13582116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1358pric13582116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1358pric13582116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1358PRIC13582116` reader - Enable Reset Tolerance of PRIC1358PRIC1_358\\[21:16\\]"]
pub type EnblRstToleranceOfPric1358pric13582116R =
    crate::BitReader<EnblRstToleranceOfPric1358pric13582116>;
impl EnblRstToleranceOfPric1358pric13582116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1358pric13582116 {
        match self.bits {
            false => EnblRstToleranceOfPric1358pric13582116::ResetBySrst,
            true => EnblRstToleranceOfPric1358pric13582116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1358pric13582116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1358pric13582116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1358PRIC13582116` writer - Enable Reset Tolerance of PRIC1358PRIC1_358\\[21:16\\]"]
pub type EnblRstToleranceOfPric1358pric13582116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1358pric13582116>;
impl<'a, REG> EnblRstToleranceOfPric1358pric13582116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1358pric13582116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1358pric13582116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1358PRIC13582216` reader - Enable Write Protection of PRIC1358PRIC1_358\\[22:16\\]"]
pub type EnblWrProtOfPric1358pric13582216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1358PRIC13582216` writer - Enable Write Protection of PRIC1358PRIC1_358\\[22:16\\]"]
pub type EnblWrProtOfPric1358pric13582216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfI3C1` reader - Enable Read Group #0 of I3C1"]
pub type EnblReadGroup0ofI3c1R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI3C1` writer - Enable Read Group #0 of I3C1"]
pub type EnblReadGroup0ofI3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI3C1` reader - Enable Read Group #1 of I3C1"]
pub type EnblReadGroup1ofI3c1R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI3C1` writer - Enable Read Group #1 of I3C1"]
pub type EnblReadGroup1ofI3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI3C1` reader - Enable Read Group #2 of I3C1"]
pub type EnblReadGroup2ofI3c1R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI3C1` writer - Enable Read Group #2 of I3C1"]
pub type EnblReadGroup2ofI3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI3C1` reader - Enable Read Group #3 of I3C1"]
pub type EnblReadGroup3ofI3c1R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI3C1` writer - Enable Read Group #3 of I3C1"]
pub type EnblReadGroup3ofI3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI3C1` reader - Enable Read Group #4 of I3C1"]
pub type EnblReadGroup4ofI3c1R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI3C1` writer - Enable Read Group #4 of I3C1"]
pub type EnblReadGroup4ofI3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI3C1` reader - Enable Read Group #5 of I3C1"]
pub type EnblReadGroup5ofI3c1R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI3C1` writer - Enable Read Group #5 of I3C1"]
pub type EnblReadGroup5ofI3c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1358PRIC1_358\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1358pric13582924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1358pric13582924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1358pric13582924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1358PRIC13582924` reader - Enable Reset Tolerance of PRIC1358PRIC1_358\\[29:24\\]"]
pub type EnblRstToleranceOfPric1358pric13582924R =
    crate::BitReader<EnblRstToleranceOfPric1358pric13582924>;
impl EnblRstToleranceOfPric1358pric13582924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1358pric13582924 {
        match self.bits {
            false => EnblRstToleranceOfPric1358pric13582924::ResetBySrst,
            true => EnblRstToleranceOfPric1358pric13582924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1358pric13582924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1358pric13582924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1358PRIC13582924` writer - Enable Reset Tolerance of PRIC1358PRIC1_358\\[29:24\\]"]
pub type EnblRstToleranceOfPric1358pric13582924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1358pric13582924>;
impl<'a, REG> EnblRstToleranceOfPric1358pric13582924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1358pric13582924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1358pric13582924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1358PRIC13583024` reader - Enable Write Protection of PRIC1358PRIC1_358\\[30:24\\]"]
pub type EnblWrProtOfPric1358pric13583024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1358PRIC13583024` writer - Enable Write Protection of PRIC1358PRIC1_358\\[30:24\\]"]
pub type EnblWrProtOfPric1358pric13583024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of PECI"]
    #[inline(always)]
    pub fn enbl_read_group0of_peci(&self) -> EnblReadGroup0ofPeciR {
        EnblReadGroup0ofPeciR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of PECI"]
    #[inline(always)]
    pub fn enbl_read_group1of_peci(&self) -> EnblReadGroup1ofPeciR {
        EnblReadGroup1ofPeciR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of PECI"]
    #[inline(always)]
    pub fn enbl_read_group2of_peci(&self) -> EnblReadGroup2ofPeciR {
        EnblReadGroup2ofPeciR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of PECI"]
    #[inline(always)]
    pub fn enbl_read_group3of_peci(&self) -> EnblReadGroup3ofPeciR {
        EnblReadGroup3ofPeciR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of PECI"]
    #[inline(always)]
    pub fn enbl_read_group4of_peci(&self) -> EnblReadGroup4ofPeciR {
        EnblReadGroup4ofPeciR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of PECI"]
    #[inline(always)]
    pub fn enbl_read_group5of_peci(&self) -> EnblReadGroup5ofPeciR {
        EnblReadGroup5ofPeciR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1358PRIC1_358\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1358pric13581308(
        &self,
    ) -> EnblRstToleranceOfPric1358pric13581308R {
        EnblRstToleranceOfPric1358pric13581308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1358PRIC1_358\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1358pric13581408(&self) -> EnblWrProtOfPric1358pric13581408R {
        EnblWrProtOfPric1358pric13581408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of I3C0"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c0(&self) -> EnblReadGroup0ofI3c0R {
        EnblReadGroup0ofI3c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of I3C0"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c0(&self) -> EnblReadGroup1ofI3c0R {
        EnblReadGroup1ofI3c0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of I3C0"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c0(&self) -> EnblReadGroup2ofI3c0R {
        EnblReadGroup2ofI3c0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of I3C0"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c0(&self) -> EnblReadGroup3ofI3c0R {
        EnblReadGroup3ofI3c0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of I3C0"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c0(&self) -> EnblReadGroup4ofI3c0R {
        EnblReadGroup4ofI3c0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of I3C0"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c0(&self) -> EnblReadGroup5ofI3c0R {
        EnblReadGroup5ofI3c0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1358PRIC1_358\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1358pric13582116(
        &self,
    ) -> EnblRstToleranceOfPric1358pric13582116R {
        EnblRstToleranceOfPric1358pric13582116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1358PRIC1_358\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1358pric13582216(&self) -> EnblWrProtOfPric1358pric13582216R {
        EnblWrProtOfPric1358pric13582216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of I3C1"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c1(&self) -> EnblReadGroup0ofI3c1R {
        EnblReadGroup0ofI3c1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of I3C1"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c1(&self) -> EnblReadGroup1ofI3c1R {
        EnblReadGroup1ofI3c1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of I3C1"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c1(&self) -> EnblReadGroup2ofI3c1R {
        EnblReadGroup2ofI3c1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of I3C1"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c1(&self) -> EnblReadGroup3ofI3c1R {
        EnblReadGroup3ofI3c1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of I3C1"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c1(&self) -> EnblReadGroup4ofI3c1R {
        EnblReadGroup4ofI3c1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of I3C1"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c1(&self) -> EnblReadGroup5ofI3c1R {
        EnblReadGroup5ofI3c1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1358PRIC1_358\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1358pric13582924(
        &self,
    ) -> EnblRstToleranceOfPric1358pric13582924R {
        EnblRstToleranceOfPric1358pric13582924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1358PRIC1_358\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1358pric13583024(&self) -> EnblWrProtOfPric1358pric13583024R {
        EnblWrProtOfPric1358pric13583024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo358Spec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of PECI"]
    #[inline(always)]
    pub fn enbl_read_group0of_peci(&mut self) -> EnblReadGroup0ofPeciW<PricIo358Spec> {
        EnblReadGroup0ofPeciW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of PECI"]
    #[inline(always)]
    pub fn enbl_read_group1of_peci(&mut self) -> EnblReadGroup1ofPeciW<PricIo358Spec> {
        EnblReadGroup1ofPeciW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of PECI"]
    #[inline(always)]
    pub fn enbl_read_group2of_peci(&mut self) -> EnblReadGroup2ofPeciW<PricIo358Spec> {
        EnblReadGroup2ofPeciW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of PECI"]
    #[inline(always)]
    pub fn enbl_read_group3of_peci(&mut self) -> EnblReadGroup3ofPeciW<PricIo358Spec> {
        EnblReadGroup3ofPeciW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of PECI"]
    #[inline(always)]
    pub fn enbl_read_group4of_peci(&mut self) -> EnblReadGroup4ofPeciW<PricIo358Spec> {
        EnblReadGroup4ofPeciW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of PECI"]
    #[inline(always)]
    pub fn enbl_read_group5of_peci(&mut self) -> EnblReadGroup5ofPeciW<PricIo358Spec> {
        EnblReadGroup5ofPeciW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1358PRIC1_358\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1358pric13581308(
        &mut self,
    ) -> EnblRstToleranceOfPric1358pric13581308W<PricIo358Spec> {
        EnblRstToleranceOfPric1358pric13581308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1358PRIC1_358\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1358pric13581408(
        &mut self,
    ) -> EnblWrProtOfPric1358pric13581408W<PricIo358Spec> {
        EnblWrProtOfPric1358pric13581408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of I3C0"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c0(&mut self) -> EnblReadGroup0ofI3c0W<PricIo358Spec> {
        EnblReadGroup0ofI3c0W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of I3C0"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c0(&mut self) -> EnblReadGroup1ofI3c0W<PricIo358Spec> {
        EnblReadGroup1ofI3c0W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of I3C0"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c0(&mut self) -> EnblReadGroup2ofI3c0W<PricIo358Spec> {
        EnblReadGroup2ofI3c0W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of I3C0"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c0(&mut self) -> EnblReadGroup3ofI3c0W<PricIo358Spec> {
        EnblReadGroup3ofI3c0W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of I3C0"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c0(&mut self) -> EnblReadGroup4ofI3c0W<PricIo358Spec> {
        EnblReadGroup4ofI3c0W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of I3C0"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c0(&mut self) -> EnblReadGroup5ofI3c0W<PricIo358Spec> {
        EnblReadGroup5ofI3c0W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1358PRIC1_358\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1358pric13582116(
        &mut self,
    ) -> EnblRstToleranceOfPric1358pric13582116W<PricIo358Spec> {
        EnblRstToleranceOfPric1358pric13582116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1358PRIC1_358\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1358pric13582216(
        &mut self,
    ) -> EnblWrProtOfPric1358pric13582216W<PricIo358Spec> {
        EnblWrProtOfPric1358pric13582216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of I3C1"]
    #[inline(always)]
    pub fn enbl_read_group0of_i3c1(&mut self) -> EnblReadGroup0ofI3c1W<PricIo358Spec> {
        EnblReadGroup0ofI3c1W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of I3C1"]
    #[inline(always)]
    pub fn enbl_read_group1of_i3c1(&mut self) -> EnblReadGroup1ofI3c1W<PricIo358Spec> {
        EnblReadGroup1ofI3c1W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of I3C1"]
    #[inline(always)]
    pub fn enbl_read_group2of_i3c1(&mut self) -> EnblReadGroup2ofI3c1W<PricIo358Spec> {
        EnblReadGroup2ofI3c1W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of I3C1"]
    #[inline(always)]
    pub fn enbl_read_group3of_i3c1(&mut self) -> EnblReadGroup3ofI3c1W<PricIo358Spec> {
        EnblReadGroup3ofI3c1W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of I3C1"]
    #[inline(always)]
    pub fn enbl_read_group4of_i3c1(&mut self) -> EnblReadGroup4ofI3c1W<PricIo358Spec> {
        EnblReadGroup4ofI3c1W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of I3C1"]
    #[inline(always)]
    pub fn enbl_read_group5of_i3c1(&mut self) -> EnblReadGroup5ofI3c1W<PricIo358Spec> {
        EnblReadGroup5ofI3c1W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1358PRIC1_358\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1358pric13582924(
        &mut self,
    ) -> EnblRstToleranceOfPric1358pric13582924W<PricIo358Spec> {
        EnblRstToleranceOfPric1358pric13582924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1358PRIC1_358\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1358pric13583024(
        &mut self,
    ) -> EnblWrProtOfPric1358pric13583024W<PricIo358Spec> {
        EnblWrProtOfPric1358pric13583024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io358::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io358::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo358Spec;
impl crate::RegisterSpec for PricIo358Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io358::R`](R) reader structure"]
impl crate::Readable for PricIo358Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io358::W`](W) writer structure"]
impl crate::Writable for PricIo358Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO358 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo358Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
