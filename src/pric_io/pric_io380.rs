#[doc = "Register `PRIC_IO380` reader"]
pub type R = crate::R<PricIo380Spec>;
#[doc = "Register `PRIC_IO380` writer"]
pub type W = crate::W<PricIo380Spec>;
#[doc = "Field `EnblReadGroup0OfWDT2` reader - Enable Read Group #0 of WDT 2"]
pub type EnblReadGroup0ofWdt2R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfWDT2` writer - Enable Read Group #0 of WDT 2"]
pub type EnblReadGroup0ofWdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfWDT2` reader - Enable Read Group #1 of WDT 2"]
pub type EnblReadGroup1ofWdt2R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfWDT2` writer - Enable Read Group #1 of WDT 2"]
pub type EnblReadGroup1ofWdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfWDT2` reader - Enable Read Group #2 of WDT 2"]
pub type EnblReadGroup2ofWdt2R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfWDT2` writer - Enable Read Group #2 of WDT 2"]
pub type EnblReadGroup2ofWdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfWDT2` reader - Enable Read Group #3 of WDT 2"]
pub type EnblReadGroup3ofWdt2R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfWDT2` writer - Enable Read Group #3 of WDT 2"]
pub type EnblReadGroup3ofWdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfWDT2` reader - Enable Read Group #4 of WDT 2"]
pub type EnblReadGroup4ofWdt2R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfWDT2` writer - Enable Read Group #4 of WDT 2"]
pub type EnblReadGroup4ofWdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfWDT2` reader - Enable Read Group #5 of WDT 2"]
pub type EnblReadGroup5ofWdt2R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfWDT2` writer - Enable Read Group #5 of WDT 2"]
pub type EnblReadGroup5ofWdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1380PRIC1_380\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1380pric13800500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1380pric13800500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1380pric13800500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1380PRIC13800500` reader - Enable Reset Tolerance of PRIC1380PRIC1_380\\[05:00\\]"]
pub type EnblRstToleranceOfPric1380pric13800500R =
    crate::BitReader<EnblRstToleranceOfPric1380pric13800500>;
impl EnblRstToleranceOfPric1380pric13800500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1380pric13800500 {
        match self.bits {
            false => EnblRstToleranceOfPric1380pric13800500::ResetBySrst,
            true => EnblRstToleranceOfPric1380pric13800500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1380pric13800500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1380pric13800500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1380PRIC13800500` writer - Enable Reset Tolerance of PRIC1380PRIC1_380\\[05:00\\]"]
pub type EnblRstToleranceOfPric1380pric13800500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1380pric13800500>;
impl<'a, REG> EnblRstToleranceOfPric1380pric13800500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1380pric13800500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1380pric13800500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1380PRIC13800600` reader - Enable Write Protection of PRIC1380PRIC1_380\\[06:00\\]"]
pub type EnblWrProtOfPric1380pric13800600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1380PRIC13800600` writer - Enable Write Protection of PRIC1380PRIC1_380\\[06:00\\]"]
pub type EnblWrProtOfPric1380pric13800600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfWDT3` reader - Enable Read Group #0 of WDT 3"]
pub type EnblReadGroup0ofWdt3R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfWDT3` writer - Enable Read Group #0 of WDT 3"]
pub type EnblReadGroup0ofWdt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfWDT3` reader - Enable Read Group #1 of WDT 3"]
pub type EnblReadGroup1ofWdt3R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfWDT3` writer - Enable Read Group #1 of WDT 3"]
pub type EnblReadGroup1ofWdt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfWDT3` reader - Enable Read Group #2 of WDT 3"]
pub type EnblReadGroup2ofWdt3R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfWDT3` writer - Enable Read Group #2 of WDT 3"]
pub type EnblReadGroup2ofWdt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfWDT3` reader - Enable Read Group #3 of WDT 3"]
pub type EnblReadGroup3ofWdt3R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfWDT3` writer - Enable Read Group #3 of WDT 3"]
pub type EnblReadGroup3ofWdt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfWDT3` reader - Enable Read Group #4 of WDT 3"]
pub type EnblReadGroup4ofWdt3R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfWDT3` writer - Enable Read Group #4 of WDT 3"]
pub type EnblReadGroup4ofWdt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfWDT3` reader - Enable Read Group #5 of WDT 3"]
pub type EnblReadGroup5ofWdt3R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfWDT3` writer - Enable Read Group #5 of WDT 3"]
pub type EnblReadGroup5ofWdt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1380PRIC1_380\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1380pric13801308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1380pric13801308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1380pric13801308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1380PRIC13801308` reader - Enable Reset Tolerance of PRIC1380PRIC1_380\\[13:08\\]"]
pub type EnblRstToleranceOfPric1380pric13801308R =
    crate::BitReader<EnblRstToleranceOfPric1380pric13801308>;
impl EnblRstToleranceOfPric1380pric13801308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1380pric13801308 {
        match self.bits {
            false => EnblRstToleranceOfPric1380pric13801308::ResetBySrst,
            true => EnblRstToleranceOfPric1380pric13801308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1380pric13801308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1380pric13801308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1380PRIC13801308` writer - Enable Reset Tolerance of PRIC1380PRIC1_380\\[13:08\\]"]
pub type EnblRstToleranceOfPric1380pric13801308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1380pric13801308>;
impl<'a, REG> EnblRstToleranceOfPric1380pric13801308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1380pric13801308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1380pric13801308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1380PRIC13801408` reader - Enable Write Protection of PRIC1380PRIC1_380\\[14:08\\]"]
pub type EnblWrProtOfPric1380pric13801408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1380PRIC13801408` writer - Enable Write Protection of PRIC1380PRIC1_380\\[14:08\\]"]
pub type EnblWrProtOfPric1380pric13801408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfWDT4` reader - Enable Read Group #0 of WDT 4"]
pub type EnblReadGroup0ofWdt4R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfWDT4` writer - Enable Read Group #0 of WDT 4"]
pub type EnblReadGroup0ofWdt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfWDT4` reader - Enable Read Group #1 of WDT 4"]
pub type EnblReadGroup1ofWdt4R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfWDT4` writer - Enable Read Group #1 of WDT 4"]
pub type EnblReadGroup1ofWdt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfWDT4` reader - Enable Read Group #2 of WDT 4"]
pub type EnblReadGroup2ofWdt4R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfWDT4` writer - Enable Read Group #2 of WDT 4"]
pub type EnblReadGroup2ofWdt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfWDT4` reader - Enable Read Group #3 of WDT 4"]
pub type EnblReadGroup3ofWdt4R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfWDT4` writer - Enable Read Group #3 of WDT 4"]
pub type EnblReadGroup3ofWdt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfWDT4` reader - Enable Read Group #4 of WDT 4"]
pub type EnblReadGroup4ofWdt4R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfWDT4` writer - Enable Read Group #4 of WDT 4"]
pub type EnblReadGroup4ofWdt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfWDT4` reader - Enable Read Group #5 of WDT 4"]
pub type EnblReadGroup5ofWdt4R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfWDT4` writer - Enable Read Group #5 of WDT 4"]
pub type EnblReadGroup5ofWdt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1380PRIC1_380\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1380pric13802116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1380pric13802116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1380pric13802116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1380PRIC13802116` reader - Enable Reset Tolerance of PRIC1380PRIC1_380\\[21:16\\]"]
pub type EnblRstToleranceOfPric1380pric13802116R =
    crate::BitReader<EnblRstToleranceOfPric1380pric13802116>;
impl EnblRstToleranceOfPric1380pric13802116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1380pric13802116 {
        match self.bits {
            false => EnblRstToleranceOfPric1380pric13802116::ResetBySrst,
            true => EnblRstToleranceOfPric1380pric13802116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1380pric13802116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1380pric13802116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1380PRIC13802116` writer - Enable Reset Tolerance of PRIC1380PRIC1_380\\[21:16\\]"]
pub type EnblRstToleranceOfPric1380pric13802116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1380pric13802116>;
impl<'a, REG> EnblRstToleranceOfPric1380pric13802116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1380pric13802116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1380pric13802116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1380PRIC13802216` reader - Enable Write Protection of PRIC1380PRIC1_380\\[22:16\\]"]
pub type EnblWrProtOfPric1380pric13802216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1380PRIC13802216` writer - Enable Write Protection of PRIC1380PRIC1_380\\[22:16\\]"]
pub type EnblWrProtOfPric1380pric13802216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfWDT5` reader - Enable Read Group #0 of WDT 5"]
pub type EnblReadGroup0ofWdt5R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfWDT5` writer - Enable Read Group #0 of WDT 5"]
pub type EnblReadGroup0ofWdt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfWDT5` reader - Enable Read Group #1 of WDT 5"]
pub type EnblReadGroup1ofWdt5R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfWDT5` writer - Enable Read Group #1 of WDT 5"]
pub type EnblReadGroup1ofWdt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfWDT5` reader - Enable Read Group #2 of WDT 5"]
pub type EnblReadGroup2ofWdt5R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfWDT5` writer - Enable Read Group #2 of WDT 5"]
pub type EnblReadGroup2ofWdt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfWDT5` reader - Enable Read Group #3 of WDT 5"]
pub type EnblReadGroup3ofWdt5R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfWDT5` writer - Enable Read Group #3 of WDT 5"]
pub type EnblReadGroup3ofWdt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfWDT5` reader - Enable Read Group #4 of WDT 5"]
pub type EnblReadGroup4ofWdt5R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfWDT5` writer - Enable Read Group #4 of WDT 5"]
pub type EnblReadGroup4ofWdt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfWDT5` reader - Enable Read Group #5 of WDT 5"]
pub type EnblReadGroup5ofWdt5R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfWDT5` writer - Enable Read Group #5 of WDT 5"]
pub type EnblReadGroup5ofWdt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1380PRIC1_380\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1380pric13802924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1380pric13802924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1380pric13802924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1380PRIC13802924` reader - Enable Reset Tolerance of PRIC1380PRIC1_380\\[29:24\\]"]
pub type EnblRstToleranceOfPric1380pric13802924R =
    crate::BitReader<EnblRstToleranceOfPric1380pric13802924>;
impl EnblRstToleranceOfPric1380pric13802924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1380pric13802924 {
        match self.bits {
            false => EnblRstToleranceOfPric1380pric13802924::ResetBySrst,
            true => EnblRstToleranceOfPric1380pric13802924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1380pric13802924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1380pric13802924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1380PRIC13802924` writer - Enable Reset Tolerance of PRIC1380PRIC1_380\\[29:24\\]"]
pub type EnblRstToleranceOfPric1380pric13802924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1380pric13802924>;
impl<'a, REG> EnblRstToleranceOfPric1380pric13802924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1380pric13802924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1380pric13802924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1380PRIC13803024` reader - Enable Write Protection of PRIC1380PRIC1_380\\[30:24\\]"]
pub type EnblWrProtOfPric1380pric13803024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1380PRIC13803024` writer - Enable Write Protection of PRIC1380PRIC1_380\\[30:24\\]"]
pub type EnblWrProtOfPric1380pric13803024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of WDT 2"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt2(&self) -> EnblReadGroup0ofWdt2R {
        EnblReadGroup0ofWdt2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of WDT 2"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt2(&self) -> EnblReadGroup1ofWdt2R {
        EnblReadGroup1ofWdt2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of WDT 2"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt2(&self) -> EnblReadGroup2ofWdt2R {
        EnblReadGroup2ofWdt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of WDT 2"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt2(&self) -> EnblReadGroup3ofWdt2R {
        EnblReadGroup3ofWdt2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of WDT 2"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt2(&self) -> EnblReadGroup4ofWdt2R {
        EnblReadGroup4ofWdt2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of WDT 2"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt2(&self) -> EnblReadGroup5ofWdt2R {
        EnblReadGroup5ofWdt2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1380PRIC1_380\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1380pric13800500(
        &self,
    ) -> EnblRstToleranceOfPric1380pric13800500R {
        EnblRstToleranceOfPric1380pric13800500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1380PRIC1_380\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1380pric13800600(&self) -> EnblWrProtOfPric1380pric13800600R {
        EnblWrProtOfPric1380pric13800600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of WDT 3"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt3(&self) -> EnblReadGroup0ofWdt3R {
        EnblReadGroup0ofWdt3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of WDT 3"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt3(&self) -> EnblReadGroup1ofWdt3R {
        EnblReadGroup1ofWdt3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of WDT 3"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt3(&self) -> EnblReadGroup2ofWdt3R {
        EnblReadGroup2ofWdt3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of WDT 3"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt3(&self) -> EnblReadGroup3ofWdt3R {
        EnblReadGroup3ofWdt3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of WDT 3"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt3(&self) -> EnblReadGroup4ofWdt3R {
        EnblReadGroup4ofWdt3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of WDT 3"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt3(&self) -> EnblReadGroup5ofWdt3R {
        EnblReadGroup5ofWdt3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1380PRIC1_380\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1380pric13801308(
        &self,
    ) -> EnblRstToleranceOfPric1380pric13801308R {
        EnblRstToleranceOfPric1380pric13801308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1380PRIC1_380\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1380pric13801408(&self) -> EnblWrProtOfPric1380pric13801408R {
        EnblWrProtOfPric1380pric13801408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of WDT 4"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt4(&self) -> EnblReadGroup0ofWdt4R {
        EnblReadGroup0ofWdt4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of WDT 4"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt4(&self) -> EnblReadGroup1ofWdt4R {
        EnblReadGroup1ofWdt4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of WDT 4"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt4(&self) -> EnblReadGroup2ofWdt4R {
        EnblReadGroup2ofWdt4R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of WDT 4"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt4(&self) -> EnblReadGroup3ofWdt4R {
        EnblReadGroup3ofWdt4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of WDT 4"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt4(&self) -> EnblReadGroup4ofWdt4R {
        EnblReadGroup4ofWdt4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of WDT 4"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt4(&self) -> EnblReadGroup5ofWdt4R {
        EnblReadGroup5ofWdt4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1380PRIC1_380\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1380pric13802116(
        &self,
    ) -> EnblRstToleranceOfPric1380pric13802116R {
        EnblRstToleranceOfPric1380pric13802116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1380PRIC1_380\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1380pric13802216(&self) -> EnblWrProtOfPric1380pric13802216R {
        EnblWrProtOfPric1380pric13802216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of WDT 5"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt5(&self) -> EnblReadGroup0ofWdt5R {
        EnblReadGroup0ofWdt5R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of WDT 5"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt5(&self) -> EnblReadGroup1ofWdt5R {
        EnblReadGroup1ofWdt5R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of WDT 5"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt5(&self) -> EnblReadGroup2ofWdt5R {
        EnblReadGroup2ofWdt5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of WDT 5"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt5(&self) -> EnblReadGroup3ofWdt5R {
        EnblReadGroup3ofWdt5R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of WDT 5"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt5(&self) -> EnblReadGroup4ofWdt5R {
        EnblReadGroup4ofWdt5R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of WDT 5"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt5(&self) -> EnblReadGroup5ofWdt5R {
        EnblReadGroup5ofWdt5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1380PRIC1_380\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1380pric13802924(
        &self,
    ) -> EnblRstToleranceOfPric1380pric13802924R {
        EnblRstToleranceOfPric1380pric13802924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1380PRIC1_380\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1380pric13803024(&self) -> EnblWrProtOfPric1380pric13803024R {
        EnblWrProtOfPric1380pric13803024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of WDT 2"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt2(&mut self) -> EnblReadGroup0ofWdt2W<PricIo380Spec> {
        EnblReadGroup0ofWdt2W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of WDT 2"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt2(&mut self) -> EnblReadGroup1ofWdt2W<PricIo380Spec> {
        EnblReadGroup1ofWdt2W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of WDT 2"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt2(&mut self) -> EnblReadGroup2ofWdt2W<PricIo380Spec> {
        EnblReadGroup2ofWdt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of WDT 2"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt2(&mut self) -> EnblReadGroup3ofWdt2W<PricIo380Spec> {
        EnblReadGroup3ofWdt2W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of WDT 2"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt2(&mut self) -> EnblReadGroup4ofWdt2W<PricIo380Spec> {
        EnblReadGroup4ofWdt2W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of WDT 2"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt2(&mut self) -> EnblReadGroup5ofWdt2W<PricIo380Spec> {
        EnblReadGroup5ofWdt2W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1380PRIC1_380\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1380pric13800500(
        &mut self,
    ) -> EnblRstToleranceOfPric1380pric13800500W<PricIo380Spec> {
        EnblRstToleranceOfPric1380pric13800500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1380PRIC1_380\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1380pric13800600(
        &mut self,
    ) -> EnblWrProtOfPric1380pric13800600W<PricIo380Spec> {
        EnblWrProtOfPric1380pric13800600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of WDT 3"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt3(&mut self) -> EnblReadGroup0ofWdt3W<PricIo380Spec> {
        EnblReadGroup0ofWdt3W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of WDT 3"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt3(&mut self) -> EnblReadGroup1ofWdt3W<PricIo380Spec> {
        EnblReadGroup1ofWdt3W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of WDT 3"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt3(&mut self) -> EnblReadGroup2ofWdt3W<PricIo380Spec> {
        EnblReadGroup2ofWdt3W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of WDT 3"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt3(&mut self) -> EnblReadGroup3ofWdt3W<PricIo380Spec> {
        EnblReadGroup3ofWdt3W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of WDT 3"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt3(&mut self) -> EnblReadGroup4ofWdt3W<PricIo380Spec> {
        EnblReadGroup4ofWdt3W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of WDT 3"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt3(&mut self) -> EnblReadGroup5ofWdt3W<PricIo380Spec> {
        EnblReadGroup5ofWdt3W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1380PRIC1_380\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1380pric13801308(
        &mut self,
    ) -> EnblRstToleranceOfPric1380pric13801308W<PricIo380Spec> {
        EnblRstToleranceOfPric1380pric13801308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1380PRIC1_380\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1380pric13801408(
        &mut self,
    ) -> EnblWrProtOfPric1380pric13801408W<PricIo380Spec> {
        EnblWrProtOfPric1380pric13801408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of WDT 4"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt4(&mut self) -> EnblReadGroup0ofWdt4W<PricIo380Spec> {
        EnblReadGroup0ofWdt4W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of WDT 4"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt4(&mut self) -> EnblReadGroup1ofWdt4W<PricIo380Spec> {
        EnblReadGroup1ofWdt4W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of WDT 4"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt4(&mut self) -> EnblReadGroup2ofWdt4W<PricIo380Spec> {
        EnblReadGroup2ofWdt4W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of WDT 4"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt4(&mut self) -> EnblReadGroup3ofWdt4W<PricIo380Spec> {
        EnblReadGroup3ofWdt4W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of WDT 4"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt4(&mut self) -> EnblReadGroup4ofWdt4W<PricIo380Spec> {
        EnblReadGroup4ofWdt4W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of WDT 4"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt4(&mut self) -> EnblReadGroup5ofWdt4W<PricIo380Spec> {
        EnblReadGroup5ofWdt4W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1380PRIC1_380\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1380pric13802116(
        &mut self,
    ) -> EnblRstToleranceOfPric1380pric13802116W<PricIo380Spec> {
        EnblRstToleranceOfPric1380pric13802116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1380PRIC1_380\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1380pric13802216(
        &mut self,
    ) -> EnblWrProtOfPric1380pric13802216W<PricIo380Spec> {
        EnblWrProtOfPric1380pric13802216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of WDT 5"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt5(&mut self) -> EnblReadGroup0ofWdt5W<PricIo380Spec> {
        EnblReadGroup0ofWdt5W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of WDT 5"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt5(&mut self) -> EnblReadGroup1ofWdt5W<PricIo380Spec> {
        EnblReadGroup1ofWdt5W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of WDT 5"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt5(&mut self) -> EnblReadGroup2ofWdt5W<PricIo380Spec> {
        EnblReadGroup2ofWdt5W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of WDT 5"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt5(&mut self) -> EnblReadGroup3ofWdt5W<PricIo380Spec> {
        EnblReadGroup3ofWdt5W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of WDT 5"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt5(&mut self) -> EnblReadGroup4ofWdt5W<PricIo380Spec> {
        EnblReadGroup4ofWdt5W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of WDT 5"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt5(&mut self) -> EnblReadGroup5ofWdt5W<PricIo380Spec> {
        EnblReadGroup5ofWdt5W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1380PRIC1_380\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1380pric13802924(
        &mut self,
    ) -> EnblRstToleranceOfPric1380pric13802924W<PricIo380Spec> {
        EnblRstToleranceOfPric1380pric13802924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1380PRIC1_380\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1380pric13803024(
        &mut self,
    ) -> EnblWrProtOfPric1380pric13803024W<PricIo380Spec> {
        EnblWrProtOfPric1380pric13803024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io380::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io380::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo380Spec;
impl crate::RegisterSpec for PricIo380Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io380::R`](R) reader structure"]
impl crate::Readable for PricIo380Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io380::W`](W) writer structure"]
impl crate::Writable for PricIo380Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO380 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo380Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
