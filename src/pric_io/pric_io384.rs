#[doc = "Register `PRIC_IO384` reader"]
pub type R = crate::R<PricIo384Spec>;
#[doc = "Register `PRIC_IO384` writer"]
pub type W = crate::W<PricIo384Spec>;
#[doc = "Field `EnblReadGroup0OfWDT6` reader - Enable Read Group #0 of WDT 6"]
pub type EnblReadGroup0ofWdt6R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfWDT6` writer - Enable Read Group #0 of WDT 6"]
pub type EnblReadGroup0ofWdt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfWDT6` reader - Enable Read Group #1 of WDT 6"]
pub type EnblReadGroup1ofWdt6R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfWDT6` writer - Enable Read Group #1 of WDT 6"]
pub type EnblReadGroup1ofWdt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfWDT6` reader - Enable Read Group #2 of WDT 6"]
pub type EnblReadGroup2ofWdt6R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfWDT6` writer - Enable Read Group #2 of WDT 6"]
pub type EnblReadGroup2ofWdt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfWDT6` reader - Enable Read Group #3 of WDT 6"]
pub type EnblReadGroup3ofWdt6R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfWDT6` writer - Enable Read Group #3 of WDT 6"]
pub type EnblReadGroup3ofWdt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfWDT6` reader - Enable Read Group #4 of WDT 6"]
pub type EnblReadGroup4ofWdt6R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfWDT6` writer - Enable Read Group #4 of WDT 6"]
pub type EnblReadGroup4ofWdt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfWDT6` reader - Enable Read Group #5 of WDT 6"]
pub type EnblReadGroup5ofWdt6R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfWDT6` writer - Enable Read Group #5 of WDT 6"]
pub type EnblReadGroup5ofWdt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1384PRIC1_384\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1384pric13840500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1384pric13840500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1384pric13840500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1384PRIC13840500` reader - Enable Reset Tolerance of PRIC1384PRIC1_384\\[05:00\\]"]
pub type EnblRstToleranceOfPric1384pric13840500R =
    crate::BitReader<EnblRstToleranceOfPric1384pric13840500>;
impl EnblRstToleranceOfPric1384pric13840500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1384pric13840500 {
        match self.bits {
            false => EnblRstToleranceOfPric1384pric13840500::ResetBySrst,
            true => EnblRstToleranceOfPric1384pric13840500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1384pric13840500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1384pric13840500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1384PRIC13840500` writer - Enable Reset Tolerance of PRIC1384PRIC1_384\\[05:00\\]"]
pub type EnblRstToleranceOfPric1384pric13840500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1384pric13840500>;
impl<'a, REG> EnblRstToleranceOfPric1384pric13840500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1384pric13840500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1384pric13840500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1384PRIC13840600` reader - Enable Write Protection of PRIC1384PRIC1_384\\[06:00\\]"]
pub type EnblWrProtOfPric1384pric13840600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1384PRIC13840600` writer - Enable Write Protection of PRIC1384PRIC1_384\\[06:00\\]"]
pub type EnblWrProtOfPric1384pric13840600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfWDT7` reader - Enable Read Group #0 of WDT 7"]
pub type EnblReadGroup0ofWdt7R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfWDT7` writer - Enable Read Group #0 of WDT 7"]
pub type EnblReadGroup0ofWdt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfWDT7` reader - Enable Read Group #1 of WDT 7"]
pub type EnblReadGroup1ofWdt7R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfWDT7` writer - Enable Read Group #1 of WDT 7"]
pub type EnblReadGroup1ofWdt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfWDT7` reader - Enable Read Group #2 of WDT 7"]
pub type EnblReadGroup2ofWdt7R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfWDT7` writer - Enable Read Group #2 of WDT 7"]
pub type EnblReadGroup2ofWdt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfWDT7` reader - Enable Read Group #3 of WDT 7"]
pub type EnblReadGroup3ofWdt7R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfWDT7` writer - Enable Read Group #3 of WDT 7"]
pub type EnblReadGroup3ofWdt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfWDT7` reader - Enable Read Group #4 of WDT 7"]
pub type EnblReadGroup4ofWdt7R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfWDT7` writer - Enable Read Group #4 of WDT 7"]
pub type EnblReadGroup4ofWdt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfWDT7` reader - Enable Read Group #5 of WDT 7"]
pub type EnblReadGroup5ofWdt7R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfWDT7` writer - Enable Read Group #5 of WDT 7"]
pub type EnblReadGroup5ofWdt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1384PRIC1_384\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1384pric13841308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1384pric13841308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1384pric13841308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1384PRIC13841308` reader - Enable Reset Tolerance of PRIC1384PRIC1_384\\[13:08\\]"]
pub type EnblRstToleranceOfPric1384pric13841308R =
    crate::BitReader<EnblRstToleranceOfPric1384pric13841308>;
impl EnblRstToleranceOfPric1384pric13841308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1384pric13841308 {
        match self.bits {
            false => EnblRstToleranceOfPric1384pric13841308::ResetBySrst,
            true => EnblRstToleranceOfPric1384pric13841308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1384pric13841308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1384pric13841308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1384PRIC13841308` writer - Enable Reset Tolerance of PRIC1384PRIC1_384\\[13:08\\]"]
pub type EnblRstToleranceOfPric1384pric13841308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1384pric13841308>;
impl<'a, REG> EnblRstToleranceOfPric1384pric13841308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1384pric13841308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1384pric13841308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1384PRIC13841408` reader - Enable Write Protection of PRIC1384PRIC1_384\\[14:08\\]"]
pub type EnblWrProtOfPric1384pric13841408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1384PRIC13841408` writer - Enable Write Protection of PRIC1384PRIC1_384\\[14:08\\]"]
pub type EnblWrProtOfPric1384pric13841408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfWDT8` reader - Enable Read Group #0 of WDT 8"]
pub type EnblReadGroup0ofWdt8R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfWDT8` writer - Enable Read Group #0 of WDT 8"]
pub type EnblReadGroup0ofWdt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfWDT8` reader - Enable Read Group #1 of WDT 8"]
pub type EnblReadGroup1ofWdt8R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfWDT8` writer - Enable Read Group #1 of WDT 8"]
pub type EnblReadGroup1ofWdt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfWDT8` reader - Enable Read Group #2 of WDT 8"]
pub type EnblReadGroup2ofWdt8R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfWDT8` writer - Enable Read Group #2 of WDT 8"]
pub type EnblReadGroup2ofWdt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfWDT8` reader - Enable Read Group #3 of WDT 8"]
pub type EnblReadGroup3ofWdt8R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfWDT8` writer - Enable Read Group #3 of WDT 8"]
pub type EnblReadGroup3ofWdt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfWDT8` reader - Enable Read Group #4 of WDT 8"]
pub type EnblReadGroup4ofWdt8R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfWDT8` writer - Enable Read Group #4 of WDT 8"]
pub type EnblReadGroup4ofWdt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfWDT8` reader - Enable Read Group #5 of WDT 8"]
pub type EnblReadGroup5ofWdt8R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfWDT8` writer - Enable Read Group #5 of WDT 8"]
pub type EnblReadGroup5ofWdt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1384PRIC1_384\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1384pric13842116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1384pric13842116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1384pric13842116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1384PRIC13842116` reader - Enable Reset Tolerance of PRIC1384PRIC1_384\\[21:16\\]"]
pub type EnblRstToleranceOfPric1384pric13842116R =
    crate::BitReader<EnblRstToleranceOfPric1384pric13842116>;
impl EnblRstToleranceOfPric1384pric13842116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1384pric13842116 {
        match self.bits {
            false => EnblRstToleranceOfPric1384pric13842116::ResetBySrst,
            true => EnblRstToleranceOfPric1384pric13842116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1384pric13842116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1384pric13842116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1384PRIC13842116` writer - Enable Reset Tolerance of PRIC1384PRIC1_384\\[21:16\\]"]
pub type EnblRstToleranceOfPric1384pric13842116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1384pric13842116>;
impl<'a, REG> EnblRstToleranceOfPric1384pric13842116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1384pric13842116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1384pric13842116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1384PRIC13842216` reader - Enable Write Protection of PRIC1384PRIC1_384\\[22:16\\]"]
pub type EnblWrProtOfPric1384pric13842216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1384PRIC13842216` writer - Enable Write Protection of PRIC1384PRIC1_384\\[22:16\\]"]
pub type EnblWrProtOfPric1384pric13842216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfESPITAFSBridge` reader - Enable Read Group #0 of eSPI TAFS Bridge"]
pub type EnblReadGroup0ofEspitafsbridgeR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfESPITAFSBridge` writer - Enable Read Group #0 of eSPI TAFS Bridge"]
pub type EnblReadGroup0ofEspitafsbridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfESPITAFSBridge` reader - Enable Read Group #1 of eSPI TAFS Bridge"]
pub type EnblReadGroup1ofEspitafsbridgeR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfESPITAFSBridge` writer - Enable Read Group #1 of eSPI TAFS Bridge"]
pub type EnblReadGroup1ofEspitafsbridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfESPITAFSBridge` reader - Enable Read Group #2 of eSPI TAFS Bridge"]
pub type EnblReadGroup2ofEspitafsbridgeR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfESPITAFSBridge` writer - Enable Read Group #2 of eSPI TAFS Bridge"]
pub type EnblReadGroup2ofEspitafsbridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfESPITAFSBridge` reader - Enable Read Group #3 of eSPI TAFS Bridge"]
pub type EnblReadGroup3ofEspitafsbridgeR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfESPITAFSBridge` writer - Enable Read Group #3 of eSPI TAFS Bridge"]
pub type EnblReadGroup3ofEspitafsbridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfESPITAFSBridge` reader - Enable Read Group #4 of eSPI TAFS Bridge"]
pub type EnblReadGroup4ofEspitafsbridgeR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfESPITAFSBridge` writer - Enable Read Group #4 of eSPI TAFS Bridge"]
pub type EnblReadGroup4ofEspitafsbridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfESPITAFSBridge` reader - Enable Read Group #5 of eSPI TAFS Bridge"]
pub type EnblReadGroup5ofEspitafsbridgeR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfESPITAFSBridge` writer - Enable Read Group #5 of eSPI TAFS Bridge"]
pub type EnblReadGroup5ofEspitafsbridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1384PRIC1_384\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1384pric13842924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1384pric13842924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1384pric13842924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1384PRIC13842924` reader - Enable Reset Tolerance of PRIC1384PRIC1_384\\[29:24\\]"]
pub type EnblRstToleranceOfPric1384pric13842924R =
    crate::BitReader<EnblRstToleranceOfPric1384pric13842924>;
impl EnblRstToleranceOfPric1384pric13842924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1384pric13842924 {
        match self.bits {
            false => EnblRstToleranceOfPric1384pric13842924::ResetBySrst,
            true => EnblRstToleranceOfPric1384pric13842924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1384pric13842924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1384pric13842924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1384PRIC13842924` writer - Enable Reset Tolerance of PRIC1384PRIC1_384\\[29:24\\]"]
pub type EnblRstToleranceOfPric1384pric13842924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1384pric13842924>;
impl<'a, REG> EnblRstToleranceOfPric1384pric13842924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1384pric13842924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1384pric13842924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1384PRIC13843024` reader - Enable Write Protection of PRIC1384PRIC1_384\\[30:24\\]"]
pub type EnblWrProtOfPric1384pric13843024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1384PRIC13843024` writer - Enable Write Protection of PRIC1384PRIC1_384\\[30:24\\]"]
pub type EnblWrProtOfPric1384pric13843024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of WDT 6"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt6(&self) -> EnblReadGroup0ofWdt6R {
        EnblReadGroup0ofWdt6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of WDT 6"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt6(&self) -> EnblReadGroup1ofWdt6R {
        EnblReadGroup1ofWdt6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of WDT 6"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt6(&self) -> EnblReadGroup2ofWdt6R {
        EnblReadGroup2ofWdt6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of WDT 6"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt6(&self) -> EnblReadGroup3ofWdt6R {
        EnblReadGroup3ofWdt6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of WDT 6"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt6(&self) -> EnblReadGroup4ofWdt6R {
        EnblReadGroup4ofWdt6R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of WDT 6"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt6(&self) -> EnblReadGroup5ofWdt6R {
        EnblReadGroup5ofWdt6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1384PRIC1_384\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1384pric13840500(
        &self,
    ) -> EnblRstToleranceOfPric1384pric13840500R {
        EnblRstToleranceOfPric1384pric13840500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1384PRIC1_384\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1384pric13840600(&self) -> EnblWrProtOfPric1384pric13840600R {
        EnblWrProtOfPric1384pric13840600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of WDT 7"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt7(&self) -> EnblReadGroup0ofWdt7R {
        EnblReadGroup0ofWdt7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of WDT 7"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt7(&self) -> EnblReadGroup1ofWdt7R {
        EnblReadGroup1ofWdt7R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of WDT 7"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt7(&self) -> EnblReadGroup2ofWdt7R {
        EnblReadGroup2ofWdt7R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of WDT 7"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt7(&self) -> EnblReadGroup3ofWdt7R {
        EnblReadGroup3ofWdt7R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of WDT 7"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt7(&self) -> EnblReadGroup4ofWdt7R {
        EnblReadGroup4ofWdt7R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of WDT 7"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt7(&self) -> EnblReadGroup5ofWdt7R {
        EnblReadGroup5ofWdt7R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1384PRIC1_384\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1384pric13841308(
        &self,
    ) -> EnblRstToleranceOfPric1384pric13841308R {
        EnblRstToleranceOfPric1384pric13841308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1384PRIC1_384\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1384pric13841408(&self) -> EnblWrProtOfPric1384pric13841408R {
        EnblWrProtOfPric1384pric13841408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of WDT 8"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt8(&self) -> EnblReadGroup0ofWdt8R {
        EnblReadGroup0ofWdt8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of WDT 8"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt8(&self) -> EnblReadGroup1ofWdt8R {
        EnblReadGroup1ofWdt8R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of WDT 8"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt8(&self) -> EnblReadGroup2ofWdt8R {
        EnblReadGroup2ofWdt8R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of WDT 8"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt8(&self) -> EnblReadGroup3ofWdt8R {
        EnblReadGroup3ofWdt8R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of WDT 8"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt8(&self) -> EnblReadGroup4ofWdt8R {
        EnblReadGroup4ofWdt8R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of WDT 8"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt8(&self) -> EnblReadGroup5ofWdt8R {
        EnblReadGroup5ofWdt8R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1384PRIC1_384\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1384pric13842116(
        &self,
    ) -> EnblRstToleranceOfPric1384pric13842116R {
        EnblRstToleranceOfPric1384pric13842116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1384PRIC1_384\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1384pric13842216(&self) -> EnblWrProtOfPric1384pric13842216R {
        EnblWrProtOfPric1384pric13842216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_read_group0of_espitafsbridge(&self) -> EnblReadGroup0ofEspitafsbridgeR {
        EnblReadGroup0ofEspitafsbridgeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_read_group1of_espitafsbridge(&self) -> EnblReadGroup1ofEspitafsbridgeR {
        EnblReadGroup1ofEspitafsbridgeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_read_group2of_espitafsbridge(&self) -> EnblReadGroup2ofEspitafsbridgeR {
        EnblReadGroup2ofEspitafsbridgeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_read_group3of_espitafsbridge(&self) -> EnblReadGroup3ofEspitafsbridgeR {
        EnblReadGroup3ofEspitafsbridgeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_read_group4of_espitafsbridge(&self) -> EnblReadGroup4ofEspitafsbridgeR {
        EnblReadGroup4ofEspitafsbridgeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_read_group5of_espitafsbridge(&self) -> EnblReadGroup5ofEspitafsbridgeR {
        EnblReadGroup5ofEspitafsbridgeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1384PRIC1_384\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1384pric13842924(
        &self,
    ) -> EnblRstToleranceOfPric1384pric13842924R {
        EnblRstToleranceOfPric1384pric13842924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1384PRIC1_384\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1384pric13843024(&self) -> EnblWrProtOfPric1384pric13843024R {
        EnblWrProtOfPric1384pric13843024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of WDT 6"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt6(&mut self) -> EnblReadGroup0ofWdt6W<PricIo384Spec> {
        EnblReadGroup0ofWdt6W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of WDT 6"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt6(&mut self) -> EnblReadGroup1ofWdt6W<PricIo384Spec> {
        EnblReadGroup1ofWdt6W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of WDT 6"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt6(&mut self) -> EnblReadGroup2ofWdt6W<PricIo384Spec> {
        EnblReadGroup2ofWdt6W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of WDT 6"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt6(&mut self) -> EnblReadGroup3ofWdt6W<PricIo384Spec> {
        EnblReadGroup3ofWdt6W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of WDT 6"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt6(&mut self) -> EnblReadGroup4ofWdt6W<PricIo384Spec> {
        EnblReadGroup4ofWdt6W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of WDT 6"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt6(&mut self) -> EnblReadGroup5ofWdt6W<PricIo384Spec> {
        EnblReadGroup5ofWdt6W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1384PRIC1_384\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1384pric13840500(
        &mut self,
    ) -> EnblRstToleranceOfPric1384pric13840500W<PricIo384Spec> {
        EnblRstToleranceOfPric1384pric13840500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1384PRIC1_384\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1384pric13840600(
        &mut self,
    ) -> EnblWrProtOfPric1384pric13840600W<PricIo384Spec> {
        EnblWrProtOfPric1384pric13840600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of WDT 7"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt7(&mut self) -> EnblReadGroup0ofWdt7W<PricIo384Spec> {
        EnblReadGroup0ofWdt7W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of WDT 7"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt7(&mut self) -> EnblReadGroup1ofWdt7W<PricIo384Spec> {
        EnblReadGroup1ofWdt7W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of WDT 7"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt7(&mut self) -> EnblReadGroup2ofWdt7W<PricIo384Spec> {
        EnblReadGroup2ofWdt7W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of WDT 7"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt7(&mut self) -> EnblReadGroup3ofWdt7W<PricIo384Spec> {
        EnblReadGroup3ofWdt7W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of WDT 7"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt7(&mut self) -> EnblReadGroup4ofWdt7W<PricIo384Spec> {
        EnblReadGroup4ofWdt7W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of WDT 7"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt7(&mut self) -> EnblReadGroup5ofWdt7W<PricIo384Spec> {
        EnblReadGroup5ofWdt7W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1384PRIC1_384\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1384pric13841308(
        &mut self,
    ) -> EnblRstToleranceOfPric1384pric13841308W<PricIo384Spec> {
        EnblRstToleranceOfPric1384pric13841308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1384PRIC1_384\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1384pric13841408(
        &mut self,
    ) -> EnblWrProtOfPric1384pric13841408W<PricIo384Spec> {
        EnblWrProtOfPric1384pric13841408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of WDT 8"]
    #[inline(always)]
    pub fn enbl_read_group0of_wdt8(&mut self) -> EnblReadGroup0ofWdt8W<PricIo384Spec> {
        EnblReadGroup0ofWdt8W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of WDT 8"]
    #[inline(always)]
    pub fn enbl_read_group1of_wdt8(&mut self) -> EnblReadGroup1ofWdt8W<PricIo384Spec> {
        EnblReadGroup1ofWdt8W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of WDT 8"]
    #[inline(always)]
    pub fn enbl_read_group2of_wdt8(&mut self) -> EnblReadGroup2ofWdt8W<PricIo384Spec> {
        EnblReadGroup2ofWdt8W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of WDT 8"]
    #[inline(always)]
    pub fn enbl_read_group3of_wdt8(&mut self) -> EnblReadGroup3ofWdt8W<PricIo384Spec> {
        EnblReadGroup3ofWdt8W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of WDT 8"]
    #[inline(always)]
    pub fn enbl_read_group4of_wdt8(&mut self) -> EnblReadGroup4ofWdt8W<PricIo384Spec> {
        EnblReadGroup4ofWdt8W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of WDT 8"]
    #[inline(always)]
    pub fn enbl_read_group5of_wdt8(&mut self) -> EnblReadGroup5ofWdt8W<PricIo384Spec> {
        EnblReadGroup5ofWdt8W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1384PRIC1_384\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1384pric13842116(
        &mut self,
    ) -> EnblRstToleranceOfPric1384pric13842116W<PricIo384Spec> {
        EnblRstToleranceOfPric1384pric13842116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1384PRIC1_384\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1384pric13842216(
        &mut self,
    ) -> EnblWrProtOfPric1384pric13842216W<PricIo384Spec> {
        EnblWrProtOfPric1384pric13842216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_read_group0of_espitafsbridge(
        &mut self,
    ) -> EnblReadGroup0ofEspitafsbridgeW<PricIo384Spec> {
        EnblReadGroup0ofEspitafsbridgeW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_read_group1of_espitafsbridge(
        &mut self,
    ) -> EnblReadGroup1ofEspitafsbridgeW<PricIo384Spec> {
        EnblReadGroup1ofEspitafsbridgeW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_read_group2of_espitafsbridge(
        &mut self,
    ) -> EnblReadGroup2ofEspitafsbridgeW<PricIo384Spec> {
        EnblReadGroup2ofEspitafsbridgeW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_read_group3of_espitafsbridge(
        &mut self,
    ) -> EnblReadGroup3ofEspitafsbridgeW<PricIo384Spec> {
        EnblReadGroup3ofEspitafsbridgeW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_read_group4of_espitafsbridge(
        &mut self,
    ) -> EnblReadGroup4ofEspitafsbridgeW<PricIo384Spec> {
        EnblReadGroup4ofEspitafsbridgeW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of eSPI TAFS Bridge"]
    #[inline(always)]
    pub fn enbl_read_group5of_espitafsbridge(
        &mut self,
    ) -> EnblReadGroup5ofEspitafsbridgeW<PricIo384Spec> {
        EnblReadGroup5ofEspitafsbridgeW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1384PRIC1_384\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1384pric13842924(
        &mut self,
    ) -> EnblRstToleranceOfPric1384pric13842924W<PricIo384Spec> {
        EnblRstToleranceOfPric1384pric13842924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1384PRIC1_384\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1384pric13843024(
        &mut self,
    ) -> EnblWrProtOfPric1384pric13843024W<PricIo384Spec> {
        EnblWrProtOfPric1384pric13843024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io384::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io384::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo384Spec;
impl crate::RegisterSpec for PricIo384Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io384::R`](R) reader structure"]
impl crate::Readable for PricIo384Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io384::W`](W) writer structure"]
impl crate::Writable for PricIo384Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO384 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo384Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
