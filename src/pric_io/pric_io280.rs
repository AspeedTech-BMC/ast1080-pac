#[doc = "Register `PRIC_IO280` reader"]
pub type R = crate::R<PricIo280Spec>;
#[doc = "Register `PRIC_IO280` writer"]
pub type W = crate::W<PricIo280Spec>;
#[doc = "Field `EnblWrGroup0OfWDT2` reader - Enable Write Group #0 of WDT 2"]
pub type EnblWrGroup0ofWdt2R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfWDT2` writer - Enable Write Group #0 of WDT 2"]
pub type EnblWrGroup0ofWdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfWDT2` reader - Enable Write Group #1 of WDT 2"]
pub type EnblWrGroup1ofWdt2R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfWDT2` writer - Enable Write Group #1 of WDT 2"]
pub type EnblWrGroup1ofWdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfWDT2` reader - Enable Write Group #2 of WDT 2"]
pub type EnblWrGroup2ofWdt2R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfWDT2` writer - Enable Write Group #2 of WDT 2"]
pub type EnblWrGroup2ofWdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfWDT2` reader - Enable Write Group #3 of WDT 2"]
pub type EnblWrGroup3ofWdt2R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfWDT2` writer - Enable Write Group #3 of WDT 2"]
pub type EnblWrGroup3ofWdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfWDT2` reader - Enable Write Group #4 of WDT 2"]
pub type EnblWrGroup4ofWdt2R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfWDT2` writer - Enable Write Group #4 of WDT 2"]
pub type EnblWrGroup4ofWdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfWDT2` reader - Enable Write Group #5 of WDT 2"]
pub type EnblWrGroup5ofWdt2R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfWDT2` writer - Enable Write Group #5 of WDT 2"]
pub type EnblWrGroup5ofWdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1280PRIC1_280\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1280pric12800500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1280pric12800500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1280pric12800500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1280PRIC12800500` reader - Enable Reset Tolerance of PRIC1280PRIC1_280\\[05:00\\]"]
pub type EnblRstToleranceOfPric1280pric12800500R =
    crate::BitReader<EnblRstToleranceOfPric1280pric12800500>;
impl EnblRstToleranceOfPric1280pric12800500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1280pric12800500 {
        match self.bits {
            false => EnblRstToleranceOfPric1280pric12800500::ResetBySrst,
            true => EnblRstToleranceOfPric1280pric12800500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1280pric12800500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1280pric12800500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1280PRIC12800500` writer - Enable Reset Tolerance of PRIC1280PRIC1_280\\[05:00\\]"]
pub type EnblRstToleranceOfPric1280pric12800500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1280pric12800500>;
impl<'a, REG> EnblRstToleranceOfPric1280pric12800500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1280pric12800500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1280pric12800500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1280PRIC12800600` reader - Enable Write Protection of PRIC1280PRIC1_280\\[06:00\\]"]
pub type EnblWrProtOfPric1280pric12800600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1280PRIC12800600` writer - Enable Write Protection of PRIC1280PRIC1_280\\[06:00\\]"]
pub type EnblWrProtOfPric1280pric12800600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfWDT3` reader - Enable Write Group #0 of WDT 3"]
pub type EnblWrGroup0ofWdt3R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfWDT3` writer - Enable Write Group #0 of WDT 3"]
pub type EnblWrGroup0ofWdt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfWDT3` reader - Enable Write Group #1 of WDT 3"]
pub type EnblWrGroup1ofWdt3R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfWDT3` writer - Enable Write Group #1 of WDT 3"]
pub type EnblWrGroup1ofWdt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfWDT3` reader - Enable Write Group #2 of WDT 3"]
pub type EnblWrGroup2ofWdt3R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfWDT3` writer - Enable Write Group #2 of WDT 3"]
pub type EnblWrGroup2ofWdt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfWDT3` reader - Enable Write Group #3 of WDT 3"]
pub type EnblWrGroup3ofWdt3R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfWDT3` writer - Enable Write Group #3 of WDT 3"]
pub type EnblWrGroup3ofWdt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfWDT3` reader - Enable Write Group #4 of WDT 3"]
pub type EnblWrGroup4ofWdt3R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfWDT3` writer - Enable Write Group #4 of WDT 3"]
pub type EnblWrGroup4ofWdt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfWDT3` reader - Enable Write Group #5 of WDT 3"]
pub type EnblWrGroup5ofWdt3R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfWDT3` writer - Enable Write Group #5 of WDT 3"]
pub type EnblWrGroup5ofWdt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1280PRIC1_280\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1280pric12801308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1280pric12801308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1280pric12801308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1280PRIC12801308` reader - Enable Reset Tolerance of PRIC1280PRIC1_280\\[13:08\\]"]
pub type EnblRstToleranceOfPric1280pric12801308R =
    crate::BitReader<EnblRstToleranceOfPric1280pric12801308>;
impl EnblRstToleranceOfPric1280pric12801308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1280pric12801308 {
        match self.bits {
            false => EnblRstToleranceOfPric1280pric12801308::ResetBySrst,
            true => EnblRstToleranceOfPric1280pric12801308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1280pric12801308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1280pric12801308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1280PRIC12801308` writer - Enable Reset Tolerance of PRIC1280PRIC1_280\\[13:08\\]"]
pub type EnblRstToleranceOfPric1280pric12801308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1280pric12801308>;
impl<'a, REG> EnblRstToleranceOfPric1280pric12801308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1280pric12801308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1280pric12801308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1280PRIC12801408` reader - Enable Write Protection of PRIC1280PRIC1_280\\[14:08\\]"]
pub type EnblWrProtOfPric1280pric12801408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1280PRIC12801408` writer - Enable Write Protection of PRIC1280PRIC1_280\\[14:08\\]"]
pub type EnblWrProtOfPric1280pric12801408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfWDT4` reader - Enable Write Group #0 of WDT 4"]
pub type EnblWrGroup0ofWdt4R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfWDT4` writer - Enable Write Group #0 of WDT 4"]
pub type EnblWrGroup0ofWdt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfWDT4` reader - Enable Write Group #1 of WDT 4"]
pub type EnblWrGroup1ofWdt4R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfWDT4` writer - Enable Write Group #1 of WDT 4"]
pub type EnblWrGroup1ofWdt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfWDT4` reader - Enable Write Group #2 of WDT 4"]
pub type EnblWrGroup2ofWdt4R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfWDT4` writer - Enable Write Group #2 of WDT 4"]
pub type EnblWrGroup2ofWdt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfWDT4` reader - Enable Write Group #3 of WDT 4"]
pub type EnblWrGroup3ofWdt4R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfWDT4` writer - Enable Write Group #3 of WDT 4"]
pub type EnblWrGroup3ofWdt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfWDT4` reader - Enable Write Group #4 of WDT 4"]
pub type EnblWrGroup4ofWdt4R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfWDT4` writer - Enable Write Group #4 of WDT 4"]
pub type EnblWrGroup4ofWdt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfWDT4` reader - Enable Write Group #5 of WDT 4"]
pub type EnblWrGroup5ofWdt4R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfWDT4` writer - Enable Write Group #5 of WDT 4"]
pub type EnblWrGroup5ofWdt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1280PRIC1_280\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1280pric12802116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1280pric12802116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1280pric12802116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1280PRIC12802116` reader - Enable Reset Tolerance of PRIC1280PRIC1_280\\[21:16\\]"]
pub type EnblRstToleranceOfPric1280pric12802116R =
    crate::BitReader<EnblRstToleranceOfPric1280pric12802116>;
impl EnblRstToleranceOfPric1280pric12802116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1280pric12802116 {
        match self.bits {
            false => EnblRstToleranceOfPric1280pric12802116::ResetBySrst,
            true => EnblRstToleranceOfPric1280pric12802116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1280pric12802116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1280pric12802116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1280PRIC12802116` writer - Enable Reset Tolerance of PRIC1280PRIC1_280\\[21:16\\]"]
pub type EnblRstToleranceOfPric1280pric12802116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1280pric12802116>;
impl<'a, REG> EnblRstToleranceOfPric1280pric12802116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1280pric12802116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1280pric12802116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1280PRIC12802216` reader - Enable Write Protection of PRIC1280PRIC1_280\\[22:16\\]"]
pub type EnblWrProtOfPric1280pric12802216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1280PRIC12802216` writer - Enable Write Protection of PRIC1280PRIC1_280\\[22:16\\]"]
pub type EnblWrProtOfPric1280pric12802216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfWDT5` reader - Enable Write Group #0 of WDT 5"]
pub type EnblWrGroup0ofWdt5R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfWDT5` writer - Enable Write Group #0 of WDT 5"]
pub type EnblWrGroup0ofWdt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfWDT5` reader - Enable Write Group #1 of WDT 5"]
pub type EnblWrGroup1ofWdt5R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfWDT5` writer - Enable Write Group #1 of WDT 5"]
pub type EnblWrGroup1ofWdt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfWDT5` reader - Enable Write Group #2 of WDT 5"]
pub type EnblWrGroup2ofWdt5R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfWDT5` writer - Enable Write Group #2 of WDT 5"]
pub type EnblWrGroup2ofWdt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfWDT5` reader - Enable Write Group #3 of WDT 5"]
pub type EnblWrGroup3ofWdt5R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfWDT5` writer - Enable Write Group #3 of WDT 5"]
pub type EnblWrGroup3ofWdt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfWDT5` reader - Enable Write Group #4 of WDT 5"]
pub type EnblWrGroup4ofWdt5R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfWDT5` writer - Enable Write Group #4 of WDT 5"]
pub type EnblWrGroup4ofWdt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfWDT5` reader - Enable Write Group #5 of WDT 5"]
pub type EnblWrGroup5ofWdt5R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfWDT5` writer - Enable Write Group #5 of WDT 5"]
pub type EnblWrGroup5ofWdt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1280PRIC1_280\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1280pric12802924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1280pric12802924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1280pric12802924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1280PRIC12802924` reader - Enable Reset Tolerance of PRIC1280PRIC1_280\\[29:24\\]"]
pub type EnblRstToleranceOfPric1280pric12802924R =
    crate::BitReader<EnblRstToleranceOfPric1280pric12802924>;
impl EnblRstToleranceOfPric1280pric12802924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1280pric12802924 {
        match self.bits {
            false => EnblRstToleranceOfPric1280pric12802924::ResetBySrst,
            true => EnblRstToleranceOfPric1280pric12802924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1280pric12802924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1280pric12802924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1280PRIC12802924` writer - Enable Reset Tolerance of PRIC1280PRIC1_280\\[29:24\\]"]
pub type EnblRstToleranceOfPric1280pric12802924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1280pric12802924>;
impl<'a, REG> EnblRstToleranceOfPric1280pric12802924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1280pric12802924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1280pric12802924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1280PRIC12803024` reader - Enable Write Protection of PRIC1280PRIC1_280\\[30:24\\]"]
pub type EnblWrProtOfPric1280pric12803024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1280PRIC12803024` writer - Enable Write Protection of PRIC1280PRIC1_280\\[30:24\\]"]
pub type EnblWrProtOfPric1280pric12803024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of WDT 2"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt2(&self) -> EnblWrGroup0ofWdt2R {
        EnblWrGroup0ofWdt2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of WDT 2"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt2(&self) -> EnblWrGroup1ofWdt2R {
        EnblWrGroup1ofWdt2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of WDT 2"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt2(&self) -> EnblWrGroup2ofWdt2R {
        EnblWrGroup2ofWdt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of WDT 2"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt2(&self) -> EnblWrGroup3ofWdt2R {
        EnblWrGroup3ofWdt2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of WDT 2"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt2(&self) -> EnblWrGroup4ofWdt2R {
        EnblWrGroup4ofWdt2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of WDT 2"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt2(&self) -> EnblWrGroup5ofWdt2R {
        EnblWrGroup5ofWdt2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1280PRIC1_280\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1280pric12800500(
        &self,
    ) -> EnblRstToleranceOfPric1280pric12800500R {
        EnblRstToleranceOfPric1280pric12800500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1280PRIC1_280\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1280pric12800600(&self) -> EnblWrProtOfPric1280pric12800600R {
        EnblWrProtOfPric1280pric12800600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of WDT 3"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt3(&self) -> EnblWrGroup0ofWdt3R {
        EnblWrGroup0ofWdt3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of WDT 3"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt3(&self) -> EnblWrGroup1ofWdt3R {
        EnblWrGroup1ofWdt3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of WDT 3"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt3(&self) -> EnblWrGroup2ofWdt3R {
        EnblWrGroup2ofWdt3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of WDT 3"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt3(&self) -> EnblWrGroup3ofWdt3R {
        EnblWrGroup3ofWdt3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of WDT 3"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt3(&self) -> EnblWrGroup4ofWdt3R {
        EnblWrGroup4ofWdt3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of WDT 3"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt3(&self) -> EnblWrGroup5ofWdt3R {
        EnblWrGroup5ofWdt3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1280PRIC1_280\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1280pric12801308(
        &self,
    ) -> EnblRstToleranceOfPric1280pric12801308R {
        EnblRstToleranceOfPric1280pric12801308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1280PRIC1_280\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1280pric12801408(&self) -> EnblWrProtOfPric1280pric12801408R {
        EnblWrProtOfPric1280pric12801408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of WDT 4"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt4(&self) -> EnblWrGroup0ofWdt4R {
        EnblWrGroup0ofWdt4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of WDT 4"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt4(&self) -> EnblWrGroup1ofWdt4R {
        EnblWrGroup1ofWdt4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of WDT 4"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt4(&self) -> EnblWrGroup2ofWdt4R {
        EnblWrGroup2ofWdt4R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of WDT 4"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt4(&self) -> EnblWrGroup3ofWdt4R {
        EnblWrGroup3ofWdt4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of WDT 4"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt4(&self) -> EnblWrGroup4ofWdt4R {
        EnblWrGroup4ofWdt4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of WDT 4"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt4(&self) -> EnblWrGroup5ofWdt4R {
        EnblWrGroup5ofWdt4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1280PRIC1_280\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1280pric12802116(
        &self,
    ) -> EnblRstToleranceOfPric1280pric12802116R {
        EnblRstToleranceOfPric1280pric12802116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1280PRIC1_280\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1280pric12802216(&self) -> EnblWrProtOfPric1280pric12802216R {
        EnblWrProtOfPric1280pric12802216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of WDT 5"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt5(&self) -> EnblWrGroup0ofWdt5R {
        EnblWrGroup0ofWdt5R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of WDT 5"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt5(&self) -> EnblWrGroup1ofWdt5R {
        EnblWrGroup1ofWdt5R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of WDT 5"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt5(&self) -> EnblWrGroup2ofWdt5R {
        EnblWrGroup2ofWdt5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of WDT 5"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt5(&self) -> EnblWrGroup3ofWdt5R {
        EnblWrGroup3ofWdt5R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of WDT 5"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt5(&self) -> EnblWrGroup4ofWdt5R {
        EnblWrGroup4ofWdt5R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of WDT 5"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt5(&self) -> EnblWrGroup5ofWdt5R {
        EnblWrGroup5ofWdt5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1280PRIC1_280\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1280pric12802924(
        &self,
    ) -> EnblRstToleranceOfPric1280pric12802924R {
        EnblRstToleranceOfPric1280pric12802924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1280PRIC1_280\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1280pric12803024(&self) -> EnblWrProtOfPric1280pric12803024R {
        EnblWrProtOfPric1280pric12803024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of WDT 2"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt2(&mut self) -> EnblWrGroup0ofWdt2W<PricIo280Spec> {
        EnblWrGroup0ofWdt2W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of WDT 2"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt2(&mut self) -> EnblWrGroup1ofWdt2W<PricIo280Spec> {
        EnblWrGroup1ofWdt2W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of WDT 2"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt2(&mut self) -> EnblWrGroup2ofWdt2W<PricIo280Spec> {
        EnblWrGroup2ofWdt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of WDT 2"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt2(&mut self) -> EnblWrGroup3ofWdt2W<PricIo280Spec> {
        EnblWrGroup3ofWdt2W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of WDT 2"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt2(&mut self) -> EnblWrGroup4ofWdt2W<PricIo280Spec> {
        EnblWrGroup4ofWdt2W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of WDT 2"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt2(&mut self) -> EnblWrGroup5ofWdt2W<PricIo280Spec> {
        EnblWrGroup5ofWdt2W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1280PRIC1_280\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1280pric12800500(
        &mut self,
    ) -> EnblRstToleranceOfPric1280pric12800500W<PricIo280Spec> {
        EnblRstToleranceOfPric1280pric12800500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1280PRIC1_280\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1280pric12800600(
        &mut self,
    ) -> EnblWrProtOfPric1280pric12800600W<PricIo280Spec> {
        EnblWrProtOfPric1280pric12800600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of WDT 3"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt3(&mut self) -> EnblWrGroup0ofWdt3W<PricIo280Spec> {
        EnblWrGroup0ofWdt3W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of WDT 3"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt3(&mut self) -> EnblWrGroup1ofWdt3W<PricIo280Spec> {
        EnblWrGroup1ofWdt3W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of WDT 3"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt3(&mut self) -> EnblWrGroup2ofWdt3W<PricIo280Spec> {
        EnblWrGroup2ofWdt3W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of WDT 3"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt3(&mut self) -> EnblWrGroup3ofWdt3W<PricIo280Spec> {
        EnblWrGroup3ofWdt3W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of WDT 3"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt3(&mut self) -> EnblWrGroup4ofWdt3W<PricIo280Spec> {
        EnblWrGroup4ofWdt3W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of WDT 3"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt3(&mut self) -> EnblWrGroup5ofWdt3W<PricIo280Spec> {
        EnblWrGroup5ofWdt3W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1280PRIC1_280\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1280pric12801308(
        &mut self,
    ) -> EnblRstToleranceOfPric1280pric12801308W<PricIo280Spec> {
        EnblRstToleranceOfPric1280pric12801308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1280PRIC1_280\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1280pric12801408(
        &mut self,
    ) -> EnblWrProtOfPric1280pric12801408W<PricIo280Spec> {
        EnblWrProtOfPric1280pric12801408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of WDT 4"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt4(&mut self) -> EnblWrGroup0ofWdt4W<PricIo280Spec> {
        EnblWrGroup0ofWdt4W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of WDT 4"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt4(&mut self) -> EnblWrGroup1ofWdt4W<PricIo280Spec> {
        EnblWrGroup1ofWdt4W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of WDT 4"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt4(&mut self) -> EnblWrGroup2ofWdt4W<PricIo280Spec> {
        EnblWrGroup2ofWdt4W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of WDT 4"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt4(&mut self) -> EnblWrGroup3ofWdt4W<PricIo280Spec> {
        EnblWrGroup3ofWdt4W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of WDT 4"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt4(&mut self) -> EnblWrGroup4ofWdt4W<PricIo280Spec> {
        EnblWrGroup4ofWdt4W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of WDT 4"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt4(&mut self) -> EnblWrGroup5ofWdt4W<PricIo280Spec> {
        EnblWrGroup5ofWdt4W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1280PRIC1_280\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1280pric12802116(
        &mut self,
    ) -> EnblRstToleranceOfPric1280pric12802116W<PricIo280Spec> {
        EnblRstToleranceOfPric1280pric12802116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1280PRIC1_280\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1280pric12802216(
        &mut self,
    ) -> EnblWrProtOfPric1280pric12802216W<PricIo280Spec> {
        EnblWrProtOfPric1280pric12802216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of WDT 5"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt5(&mut self) -> EnblWrGroup0ofWdt5W<PricIo280Spec> {
        EnblWrGroup0ofWdt5W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of WDT 5"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt5(&mut self) -> EnblWrGroup1ofWdt5W<PricIo280Spec> {
        EnblWrGroup1ofWdt5W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of WDT 5"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt5(&mut self) -> EnblWrGroup2ofWdt5W<PricIo280Spec> {
        EnblWrGroup2ofWdt5W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of WDT 5"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt5(&mut self) -> EnblWrGroup3ofWdt5W<PricIo280Spec> {
        EnblWrGroup3ofWdt5W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of WDT 5"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt5(&mut self) -> EnblWrGroup4ofWdt5W<PricIo280Spec> {
        EnblWrGroup4ofWdt5W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of WDT 5"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt5(&mut self) -> EnblWrGroup5ofWdt5W<PricIo280Spec> {
        EnblWrGroup5ofWdt5W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1280PRIC1_280\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1280pric12802924(
        &mut self,
    ) -> EnblRstToleranceOfPric1280pric12802924W<PricIo280Spec> {
        EnblRstToleranceOfPric1280pric12802924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1280PRIC1_280\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1280pric12803024(
        &mut self,
    ) -> EnblWrProtOfPric1280pric12803024W<PricIo280Spec> {
        EnblWrProtOfPric1280pric12803024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io280::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io280::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo280Spec;
impl crate::RegisterSpec for PricIo280Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io280::R`](R) reader structure"]
impl crate::Readable for PricIo280Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io280::W`](W) writer structure"]
impl crate::Writable for PricIo280Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO280 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo280Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
