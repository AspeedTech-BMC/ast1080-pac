#[doc = "Register `PRIC_IO284` reader"]
pub type R = crate::R<PricIo284Spec>;
#[doc = "Register `PRIC_IO284` writer"]
pub type W = crate::W<PricIo284Spec>;
#[doc = "Field `EnblWrGroup0OfWDT6` reader - Enable Write Group #0 of WDT 6"]
pub type EnblWrGroup0ofWdt6R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfWDT6` writer - Enable Write Group #0 of WDT 6"]
pub type EnblWrGroup0ofWdt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfWDT6` reader - Enable Write Group #1 of WDT 6"]
pub type EnblWrGroup1ofWdt6R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfWDT6` writer - Enable Write Group #1 of WDT 6"]
pub type EnblWrGroup1ofWdt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfWDT6` reader - Enable Write Group #2 of WDT 6"]
pub type EnblWrGroup2ofWdt6R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfWDT6` writer - Enable Write Group #2 of WDT 6"]
pub type EnblWrGroup2ofWdt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfWDT6` reader - Enable Write Group #3 of WDT 6"]
pub type EnblWrGroup3ofWdt6R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfWDT6` writer - Enable Write Group #3 of WDT 6"]
pub type EnblWrGroup3ofWdt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfWDT6` reader - Enable Write Group #4 of WDT 6"]
pub type EnblWrGroup4ofWdt6R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfWDT6` writer - Enable Write Group #4 of WDT 6"]
pub type EnblWrGroup4ofWdt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfWDT6` reader - Enable Write Group #5 of WDT 6"]
pub type EnblWrGroup5ofWdt6R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfWDT6` writer - Enable Write Group #5 of WDT 6"]
pub type EnblWrGroup5ofWdt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1284PRIC1_284\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1284pric12840500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1284pric12840500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1284pric12840500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1284PRIC12840500` reader - Enable Reset Tolerance of PRIC1284PRIC1_284\\[05:00\\]"]
pub type EnblRstToleranceOfPric1284pric12840500R =
    crate::BitReader<EnblRstToleranceOfPric1284pric12840500>;
impl EnblRstToleranceOfPric1284pric12840500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1284pric12840500 {
        match self.bits {
            false => EnblRstToleranceOfPric1284pric12840500::ResetBySrst,
            true => EnblRstToleranceOfPric1284pric12840500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1284pric12840500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1284pric12840500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1284PRIC12840500` writer - Enable Reset Tolerance of PRIC1284PRIC1_284\\[05:00\\]"]
pub type EnblRstToleranceOfPric1284pric12840500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1284pric12840500>;
impl<'a, REG> EnblRstToleranceOfPric1284pric12840500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1284pric12840500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1284pric12840500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1284PRIC12840600` reader - Enable Write Protection of PRIC1284PRIC1_284\\[06:00\\]"]
pub type EnblWrProtOfPric1284pric12840600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1284PRIC12840600` writer - Enable Write Protection of PRIC1284PRIC1_284\\[06:00\\]"]
pub type EnblWrProtOfPric1284pric12840600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfWDT7` reader - Enable Write Group #0 of WDT 7"]
pub type EnblWrGroup0ofWdt7R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfWDT7` writer - Enable Write Group #0 of WDT 7"]
pub type EnblWrGroup0ofWdt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfWDT7` reader - Enable Write Group #1 of WDT 7"]
pub type EnblWrGroup1ofWdt7R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfWDT7` writer - Enable Write Group #1 of WDT 7"]
pub type EnblWrGroup1ofWdt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfWDT7` reader - Enable Write Group #2 of WDT 7"]
pub type EnblWrGroup2ofWdt7R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfWDT7` writer - Enable Write Group #2 of WDT 7"]
pub type EnblWrGroup2ofWdt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfWDT7` reader - Enable Write Group #3 of WDT 7"]
pub type EnblWrGroup3ofWdt7R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfWDT7` writer - Enable Write Group #3 of WDT 7"]
pub type EnblWrGroup3ofWdt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfWDT7` reader - Enable Write Group #4 of WDT 7"]
pub type EnblWrGroup4ofWdt7R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfWDT7` writer - Enable Write Group #4 of WDT 7"]
pub type EnblWrGroup4ofWdt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfWDT7` reader - Enable Write Group #5 of WDT 7"]
pub type EnblWrGroup5ofWdt7R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfWDT7` writer - Enable Write Group #5 of WDT 7"]
pub type EnblWrGroup5ofWdt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1284PRIC1_284\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1284pric12841308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1284pric12841308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1284pric12841308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1284PRIC12841308` reader - Enable Reset Tolerance of PRIC1284PRIC1_284\\[13:08\\]"]
pub type EnblRstToleranceOfPric1284pric12841308R =
    crate::BitReader<EnblRstToleranceOfPric1284pric12841308>;
impl EnblRstToleranceOfPric1284pric12841308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1284pric12841308 {
        match self.bits {
            false => EnblRstToleranceOfPric1284pric12841308::ResetBySrst,
            true => EnblRstToleranceOfPric1284pric12841308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1284pric12841308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1284pric12841308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1284PRIC12841308` writer - Enable Reset Tolerance of PRIC1284PRIC1_284\\[13:08\\]"]
pub type EnblRstToleranceOfPric1284pric12841308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1284pric12841308>;
impl<'a, REG> EnblRstToleranceOfPric1284pric12841308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1284pric12841308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1284pric12841308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1284PRIC12841408` reader - Enable Write Protection of PRIC1284PRIC1_284\\[14:08\\]"]
pub type EnblWrProtOfPric1284pric12841408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1284PRIC12841408` writer - Enable Write Protection of PRIC1284PRIC1_284\\[14:08\\]"]
pub type EnblWrProtOfPric1284pric12841408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfWDT8` reader - Enable Write Group #0 of WDT 8"]
pub type EnblWrGroup0ofWdt8R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfWDT8` writer - Enable Write Group #0 of WDT 8"]
pub type EnblWrGroup0ofWdt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfWDT8` reader - Enable Write Group #1 of WDT 8"]
pub type EnblWrGroup1ofWdt8R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfWDT8` writer - Enable Write Group #1 of WDT 8"]
pub type EnblWrGroup1ofWdt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfWDT8` reader - Enable Write Group #2 of WDT 8"]
pub type EnblWrGroup2ofWdt8R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfWDT8` writer - Enable Write Group #2 of WDT 8"]
pub type EnblWrGroup2ofWdt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfWDT8` reader - Enable Write Group #3 of WDT 8"]
pub type EnblWrGroup3ofWdt8R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfWDT8` writer - Enable Write Group #3 of WDT 8"]
pub type EnblWrGroup3ofWdt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfWDT8` reader - Enable Write Group #4 of WDT 8"]
pub type EnblWrGroup4ofWdt8R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfWDT8` writer - Enable Write Group #4 of WDT 8"]
pub type EnblWrGroup4ofWdt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfWDT8` reader - Enable Write Group #5 of WDT 8"]
pub type EnblWrGroup5ofWdt8R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfWDT8` writer - Enable Write Group #5 of WDT 8"]
pub type EnblWrGroup5ofWdt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1284PRIC1_284\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1284pric12842116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1284pric12842116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1284pric12842116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1284PRIC12842116` reader - Enable Reset Tolerance of PRIC1284PRIC1_284\\[21:16\\]"]
pub type EnblRstToleranceOfPric1284pric12842116R =
    crate::BitReader<EnblRstToleranceOfPric1284pric12842116>;
impl EnblRstToleranceOfPric1284pric12842116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1284pric12842116 {
        match self.bits {
            false => EnblRstToleranceOfPric1284pric12842116::ResetBySrst,
            true => EnblRstToleranceOfPric1284pric12842116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1284pric12842116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1284pric12842116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1284PRIC12842116` writer - Enable Reset Tolerance of PRIC1284PRIC1_284\\[21:16\\]"]
pub type EnblRstToleranceOfPric1284pric12842116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1284pric12842116>;
impl<'a, REG> EnblRstToleranceOfPric1284pric12842116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1284pric12842116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1284pric12842116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1284PRIC12842216` reader - Enable Write Protection of PRIC1284PRIC1_284\\[22:16\\]"]
pub type EnblWrProtOfPric1284pric12842216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1284PRIC12842216` writer - Enable Write Protection of PRIC1284PRIC1_284\\[22:16\\]"]
pub type EnblWrProtOfPric1284pric12842216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfESPITAFSBridge` reader - Enable Write Group #0 of eSPI TAFS Bridge"]
pub type EnblWrGroup0ofEspitafsbridgeR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfESPITAFSBridge` writer - Enable Write Group #0 of eSPI TAFS Bridge"]
pub type EnblWrGroup0ofEspitafsbridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfESPITAFSBridge` reader - Enable Write Group #1 of eSPI TAFS Bridge"]
pub type EnblWrGroup1ofEspitafsbridgeR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfESPITAFSBridge` writer - Enable Write Group #1 of eSPI TAFS Bridge"]
pub type EnblWrGroup1ofEspitafsbridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfESPITAFSBridge` reader - Enable Write Group #2 of eSPI TAFS Bridge"]
pub type EnblWrGroup2ofEspitafsbridgeR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfESPITAFSBridge` writer - Enable Write Group #2 of eSPI TAFS Bridge"]
pub type EnblWrGroup2ofEspitafsbridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfESPITAFSBridge` reader - Enable Write Group #3 of eSPI TAFS Bridge"]
pub type EnblWrGroup3ofEspitafsbridgeR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfESPITAFSBridge` writer - Enable Write Group #3 of eSPI TAFS Bridge"]
pub type EnblWrGroup3ofEspitafsbridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfESPITAFSBridge` reader - Enable Write Group #4 of eSPI TAFS Bridge"]
pub type EnblWrGroup4ofEspitafsbridgeR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfESPITAFSBridge` writer - Enable Write Group #4 of eSPI TAFS Bridge"]
pub type EnblWrGroup4ofEspitafsbridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfESPITAFSBridge` reader - Enable Write Group #5 of eSPI TAFS Bridge"]
pub type EnblWrGroup5ofEspitafsbridgeR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfESPITAFSBridge` writer - Enable Write Group #5 of eSPI TAFS Bridge"]
pub type EnblWrGroup5ofEspitafsbridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1284PRIC1_284\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1284pric12842924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1284pric12842924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1284pric12842924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1284PRIC12842924` reader - Enable Reset Tolerance of PRIC1284PRIC1_284\\[29:24\\]"]
pub type EnblRstToleranceOfPric1284pric12842924R =
    crate::BitReader<EnblRstToleranceOfPric1284pric12842924>;
impl EnblRstToleranceOfPric1284pric12842924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1284pric12842924 {
        match self.bits {
            false => EnblRstToleranceOfPric1284pric12842924::ResetBySrst,
            true => EnblRstToleranceOfPric1284pric12842924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1284pric12842924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1284pric12842924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1284PRIC12842924` writer - Enable Reset Tolerance of PRIC1284PRIC1_284\\[29:24\\]"]
pub type EnblRstToleranceOfPric1284pric12842924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1284pric12842924>;
impl<'a, REG> EnblRstToleranceOfPric1284pric12842924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1284pric12842924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1284pric12842924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1284PRIC12843024` reader - Enable Write Protection of PRIC1284PRIC1_284\\[30:24\\]"]
pub type EnblWrProtOfPric1284pric12843024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1284PRIC12843024` writer - Enable Write Protection of PRIC1284PRIC1_284\\[30:24\\]"]
pub type EnblWrProtOfPric1284pric12843024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of WDT 6"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt6(&self) -> EnblWrGroup0ofWdt6R {
        EnblWrGroup0ofWdt6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of WDT 6"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt6(&self) -> EnblWrGroup1ofWdt6R {
        EnblWrGroup1ofWdt6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of WDT 6"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt6(&self) -> EnblWrGroup2ofWdt6R {
        EnblWrGroup2ofWdt6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of WDT 6"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt6(&self) -> EnblWrGroup3ofWdt6R {
        EnblWrGroup3ofWdt6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of WDT 6"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt6(&self) -> EnblWrGroup4ofWdt6R {
        EnblWrGroup4ofWdt6R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of WDT 6"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt6(&self) -> EnblWrGroup5ofWdt6R {
        EnblWrGroup5ofWdt6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1284PRIC1_284\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1284pric12840500(
        &self,
    ) -> EnblRstToleranceOfPric1284pric12840500R {
        EnblRstToleranceOfPric1284pric12840500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1284PRIC1_284\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1284pric12840600(&self) -> EnblWrProtOfPric1284pric12840600R {
        EnblWrProtOfPric1284pric12840600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of WDT 7"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt7(&self) -> EnblWrGroup0ofWdt7R {
        EnblWrGroup0ofWdt7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of WDT 7"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt7(&self) -> EnblWrGroup1ofWdt7R {
        EnblWrGroup1ofWdt7R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of WDT 7"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt7(&self) -> EnblWrGroup2ofWdt7R {
        EnblWrGroup2ofWdt7R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of WDT 7"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt7(&self) -> EnblWrGroup3ofWdt7R {
        EnblWrGroup3ofWdt7R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of WDT 7"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt7(&self) -> EnblWrGroup4ofWdt7R {
        EnblWrGroup4ofWdt7R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of WDT 7"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt7(&self) -> EnblWrGroup5ofWdt7R {
        EnblWrGroup5ofWdt7R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1284PRIC1_284\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1284pric12841308(
        &self,
    ) -> EnblRstToleranceOfPric1284pric12841308R {
        EnblRstToleranceOfPric1284pric12841308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1284PRIC1_284\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1284pric12841408(&self) -> EnblWrProtOfPric1284pric12841408R {
        EnblWrProtOfPric1284pric12841408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of WDT 8"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt8(&self) -> EnblWrGroup0ofWdt8R {
        EnblWrGroup0ofWdt8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of WDT 8"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt8(&self) -> EnblWrGroup1ofWdt8R {
        EnblWrGroup1ofWdt8R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of WDT 8"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt8(&self) -> EnblWrGroup2ofWdt8R {
        EnblWrGroup2ofWdt8R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of WDT 8"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt8(&self) -> EnblWrGroup3ofWdt8R {
        EnblWrGroup3ofWdt8R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of WDT 8"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt8(&self) -> EnblWrGroup4ofWdt8R {
        EnblWrGroup4ofWdt8R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of WDT 8"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt8(&self) -> EnblWrGroup5ofWdt8R {
        EnblWrGroup5ofWdt8R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1284PRIC1_284\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1284pric12842116(
        &self,
    ) -> EnblRstToleranceOfPric1284pric12842116R {
        EnblRstToleranceOfPric1284pric12842116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1284PRIC1_284\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1284pric12842216(&self) -> EnblWrProtOfPric1284pric12842216R {
        EnblWrProtOfPric1284pric12842216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espitafsbridge(&self) -> EnblWrGroup0ofEspitafsbridgeR {
        EnblWrGroup0ofEspitafsbridgeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espitafsbridge(&self) -> EnblWrGroup1ofEspitafsbridgeR {
        EnblWrGroup1ofEspitafsbridgeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espitafsbridge(&self) -> EnblWrGroup2ofEspitafsbridgeR {
        EnblWrGroup2ofEspitafsbridgeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espitafsbridge(&self) -> EnblWrGroup3ofEspitafsbridgeR {
        EnblWrGroup3ofEspitafsbridgeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espitafsbridge(&self) -> EnblWrGroup4ofEspitafsbridgeR {
        EnblWrGroup4ofEspitafsbridgeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espitafsbridge(&self) -> EnblWrGroup5ofEspitafsbridgeR {
        EnblWrGroup5ofEspitafsbridgeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1284PRIC1_284\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1284pric12842924(
        &self,
    ) -> EnblRstToleranceOfPric1284pric12842924R {
        EnblRstToleranceOfPric1284pric12842924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1284PRIC1_284\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1284pric12843024(&self) -> EnblWrProtOfPric1284pric12843024R {
        EnblWrProtOfPric1284pric12843024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of WDT 6"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt6(&mut self) -> EnblWrGroup0ofWdt6W<PricIo284Spec> {
        EnblWrGroup0ofWdt6W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of WDT 6"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt6(&mut self) -> EnblWrGroup1ofWdt6W<PricIo284Spec> {
        EnblWrGroup1ofWdt6W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of WDT 6"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt6(&mut self) -> EnblWrGroup2ofWdt6W<PricIo284Spec> {
        EnblWrGroup2ofWdt6W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of WDT 6"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt6(&mut self) -> EnblWrGroup3ofWdt6W<PricIo284Spec> {
        EnblWrGroup3ofWdt6W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of WDT 6"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt6(&mut self) -> EnblWrGroup4ofWdt6W<PricIo284Spec> {
        EnblWrGroup4ofWdt6W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of WDT 6"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt6(&mut self) -> EnblWrGroup5ofWdt6W<PricIo284Spec> {
        EnblWrGroup5ofWdt6W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1284PRIC1_284\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1284pric12840500(
        &mut self,
    ) -> EnblRstToleranceOfPric1284pric12840500W<PricIo284Spec> {
        EnblRstToleranceOfPric1284pric12840500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1284PRIC1_284\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1284pric12840600(
        &mut self,
    ) -> EnblWrProtOfPric1284pric12840600W<PricIo284Spec> {
        EnblWrProtOfPric1284pric12840600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of WDT 7"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt7(&mut self) -> EnblWrGroup0ofWdt7W<PricIo284Spec> {
        EnblWrGroup0ofWdt7W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of WDT 7"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt7(&mut self) -> EnblWrGroup1ofWdt7W<PricIo284Spec> {
        EnblWrGroup1ofWdt7W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of WDT 7"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt7(&mut self) -> EnblWrGroup2ofWdt7W<PricIo284Spec> {
        EnblWrGroup2ofWdt7W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of WDT 7"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt7(&mut self) -> EnblWrGroup3ofWdt7W<PricIo284Spec> {
        EnblWrGroup3ofWdt7W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of WDT 7"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt7(&mut self) -> EnblWrGroup4ofWdt7W<PricIo284Spec> {
        EnblWrGroup4ofWdt7W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of WDT 7"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt7(&mut self) -> EnblWrGroup5ofWdt7W<PricIo284Spec> {
        EnblWrGroup5ofWdt7W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1284PRIC1_284\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1284pric12841308(
        &mut self,
    ) -> EnblRstToleranceOfPric1284pric12841308W<PricIo284Spec> {
        EnblRstToleranceOfPric1284pric12841308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1284PRIC1_284\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1284pric12841408(
        &mut self,
    ) -> EnblWrProtOfPric1284pric12841408W<PricIo284Spec> {
        EnblWrProtOfPric1284pric12841408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of WDT 8"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt8(&mut self) -> EnblWrGroup0ofWdt8W<PricIo284Spec> {
        EnblWrGroup0ofWdt8W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of WDT 8"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt8(&mut self) -> EnblWrGroup1ofWdt8W<PricIo284Spec> {
        EnblWrGroup1ofWdt8W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of WDT 8"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt8(&mut self) -> EnblWrGroup2ofWdt8W<PricIo284Spec> {
        EnblWrGroup2ofWdt8W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of WDT 8"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt8(&mut self) -> EnblWrGroup3ofWdt8W<PricIo284Spec> {
        EnblWrGroup3ofWdt8W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of WDT 8"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt8(&mut self) -> EnblWrGroup4ofWdt8W<PricIo284Spec> {
        EnblWrGroup4ofWdt8W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of WDT 8"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt8(&mut self) -> EnblWrGroup5ofWdt8W<PricIo284Spec> {
        EnblWrGroup5ofWdt8W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1284PRIC1_284\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1284pric12842116(
        &mut self,
    ) -> EnblRstToleranceOfPric1284pric12842116W<PricIo284Spec> {
        EnblRstToleranceOfPric1284pric12842116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1284PRIC1_284\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1284pric12842216(
        &mut self,
    ) -> EnblWrProtOfPric1284pric12842216W<PricIo284Spec> {
        EnblWrProtOfPric1284pric12842216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espitafsbridge(
        &mut self,
    ) -> EnblWrGroup0ofEspitafsbridgeW<PricIo284Spec> {
        EnblWrGroup0ofEspitafsbridgeW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espitafsbridge(
        &mut self,
    ) -> EnblWrGroup1ofEspitafsbridgeW<PricIo284Spec> {
        EnblWrGroup1ofEspitafsbridgeW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espitafsbridge(
        &mut self,
    ) -> EnblWrGroup2ofEspitafsbridgeW<PricIo284Spec> {
        EnblWrGroup2ofEspitafsbridgeW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espitafsbridge(
        &mut self,
    ) -> EnblWrGroup3ofEspitafsbridgeW<PricIo284Spec> {
        EnblWrGroup3ofEspitafsbridgeW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espitafsbridge(
        &mut self,
    ) -> EnblWrGroup4ofEspitafsbridgeW<PricIo284Spec> {
        EnblWrGroup4ofEspitafsbridgeW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espitafsbridge(
        &mut self,
    ) -> EnblWrGroup5ofEspitafsbridgeW<PricIo284Spec> {
        EnblWrGroup5ofEspitafsbridgeW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1284PRIC1_284\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1284pric12842924(
        &mut self,
    ) -> EnblRstToleranceOfPric1284pric12842924W<PricIo284Spec> {
        EnblRstToleranceOfPric1284pric12842924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1284PRIC1_284\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1284pric12843024(
        &mut self,
    ) -> EnblWrProtOfPric1284pric12843024W<PricIo284Spec> {
        EnblWrProtOfPric1284pric12843024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io284::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io284::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo284Spec;
impl crate::RegisterSpec for PricIo284Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io284::R`](R) reader structure"]
impl crate::Readable for PricIo284Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io284::W`](W) writer structure"]
impl crate::Writable for PricIo284Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO284 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo284Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
