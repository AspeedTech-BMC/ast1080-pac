#[doc = "Register `PRIC_IO014` reader"]
pub type R = crate::R<PricIo014Spec>;
#[doc = "Register `PRIC_IO014` writer"]
pub type W = crate::W<PricIo014Spec>;
#[doc = "Field `EnblWrGroup0OfBootMCUIChannelAccess` reader - Enable Write Group #0 of Boot MCU I channel access"]
pub type EnblWrGroup0ofBootMcuichannelAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfBootMCUIChannelAccess` writer - Enable Write Group #0 of Boot MCU I channel access"]
pub type EnblWrGroup0ofBootMcuichannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfBootMCUIChannelAccess` reader - Enable Write Group #1 of Boot MCU I channel access"]
pub type EnblWrGroup1ofBootMcuichannelAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfBootMCUIChannelAccess` writer - Enable Write Group #1 of Boot MCU I channel access"]
pub type EnblWrGroup1ofBootMcuichannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfBootMCUIChannelAccess` reader - Enable Write Group #2 of Boot MCU I channel access"]
pub type EnblWrGroup2ofBootMcuichannelAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfBootMCUIChannelAccess` writer - Enable Write Group #2 of Boot MCU I channel access"]
pub type EnblWrGroup2ofBootMcuichannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfBootMCUIChannelAccess` reader - Enable Write Group #3 of Boot MCU I channel access"]
pub type EnblWrGroup3ofBootMcuichannelAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfBootMCUIChannelAccess` writer - Enable Write Group #3 of Boot MCU I channel access"]
pub type EnblWrGroup3ofBootMcuichannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfBootMCUIChannelAccess` reader - Enable Write Group #4 of Boot MCU I channel access"]
pub type EnblWrGroup4ofBootMcuichannelAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfBootMCUIChannelAccess` writer - Enable Write Group #4 of Boot MCU I channel access"]
pub type EnblWrGroup4ofBootMcuichannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfBootMCUIChannelAccess` reader - Enable Write Group #5 of Boot MCU I channel access"]
pub type EnblWrGroup5ofBootMcuichannelAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfBootMCUIChannelAccess` writer - Enable Write Group #5 of Boot MCU I channel access"]
pub type EnblWrGroup5ofBootMcuichannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1014PRIC1_014\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1014pric10140500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1014pric10140500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1014pric10140500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1014PRIC10140500` reader - Enable Reset Tolerance of PRIC1014PRIC1_014\\[05:00\\]"]
pub type EnblRstToleranceOfPric1014pric10140500R =
    crate::BitReader<EnblRstToleranceOfPric1014pric10140500>;
impl EnblRstToleranceOfPric1014pric10140500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1014pric10140500 {
        match self.bits {
            false => EnblRstToleranceOfPric1014pric10140500::ResetBySrst,
            true => EnblRstToleranceOfPric1014pric10140500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1014pric10140500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1014pric10140500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1014PRIC10140500` writer - Enable Reset Tolerance of PRIC1014PRIC1_014\\[05:00\\]"]
pub type EnblRstToleranceOfPric1014pric10140500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1014pric10140500>;
impl<'a, REG> EnblRstToleranceOfPric1014pric10140500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1014pric10140500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1014pric10140500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1014PRIC10140600` reader - Enable Write Protection of PRIC1014PRIC1_014\\[06:00\\]"]
pub type EnblWrProtOfPric1014pric10140600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1014PRIC10140600` writer - Enable Write Protection of PRIC1014PRIC1_014\\[06:00\\]"]
pub type EnblWrProtOfPric1014pric10140600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfBootMCUDChannelAccess` reader - Enable Write Group #0 of Boot MCU D channel access"]
pub type EnblWrGroup0ofBootMcudchannelAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfBootMCUDChannelAccess` writer - Enable Write Group #0 of Boot MCU D channel access"]
pub type EnblWrGroup0ofBootMcudchannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfBootMCUDChannelAccess` reader - Enable Write Group #1 of Boot MCU D channel access"]
pub type EnblWrGroup1ofBootMcudchannelAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfBootMCUDChannelAccess` writer - Enable Write Group #1 of Boot MCU D channel access"]
pub type EnblWrGroup1ofBootMcudchannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfBootMCUDChannelAccess` reader - Enable Write Group #2 of Boot MCU D channel access"]
pub type EnblWrGroup2ofBootMcudchannelAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfBootMCUDChannelAccess` writer - Enable Write Group #2 of Boot MCU D channel access"]
pub type EnblWrGroup2ofBootMcudchannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfBootMCUDChannelAccess` reader - Enable Write Group #3 of Boot MCU D channel access"]
pub type EnblWrGroup3ofBootMcudchannelAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfBootMCUDChannelAccess` writer - Enable Write Group #3 of Boot MCU D channel access"]
pub type EnblWrGroup3ofBootMcudchannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfBootMCUDChannelAccess` reader - Enable Write Group #4 of Boot MCU D channel access"]
pub type EnblWrGroup4ofBootMcudchannelAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfBootMCUDChannelAccess` writer - Enable Write Group #4 of Boot MCU D channel access"]
pub type EnblWrGroup4ofBootMcudchannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfBootMCUDChannelAccess` reader - Enable Write Group #5 of Boot MCU D channel access"]
pub type EnblWrGroup5ofBootMcudchannelAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfBootMCUDChannelAccess` writer - Enable Write Group #5 of Boot MCU D channel access"]
pub type EnblWrGroup5ofBootMcudchannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1014PRIC1_014\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1014pric10141308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1014pric10141308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1014pric10141308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1014PRIC10141308` reader - Enable Reset Tolerance of PRIC1014PRIC1_014\\[13:08\\]"]
pub type EnblRstToleranceOfPric1014pric10141308R =
    crate::BitReader<EnblRstToleranceOfPric1014pric10141308>;
impl EnblRstToleranceOfPric1014pric10141308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1014pric10141308 {
        match self.bits {
            false => EnblRstToleranceOfPric1014pric10141308::ResetBySrst,
            true => EnblRstToleranceOfPric1014pric10141308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1014pric10141308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1014pric10141308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1014PRIC10141308` writer - Enable Reset Tolerance of PRIC1014PRIC1_014\\[13:08\\]"]
pub type EnblRstToleranceOfPric1014pric10141308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1014pric10141308>;
impl<'a, REG> EnblRstToleranceOfPric1014pric10141308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1014pric10141308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1014pric10141308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1014PRIC10141408` reader - Enable Write Protection of PRIC1014PRIC1_014\\[14:08\\]"]
pub type EnblWrProtOfPric1014pric10141408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1014PRIC10141408` writer - Enable Write Protection of PRIC1014PRIC1_014\\[14:08\\]"]
pub type EnblWrProtOfPric1014pric10141408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `Reserved9` writer - Reserved"]
pub type Reserved9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `Reserved8` writer - Reserved"]
pub type Reserved8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_boot_mcuichannel_access(&self) -> EnblWrGroup0ofBootMcuichannelAccessR {
        EnblWrGroup0ofBootMcuichannelAccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_boot_mcuichannel_access(&self) -> EnblWrGroup1ofBootMcuichannelAccessR {
        EnblWrGroup1ofBootMcuichannelAccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_boot_mcuichannel_access(&self) -> EnblWrGroup2ofBootMcuichannelAccessR {
        EnblWrGroup2ofBootMcuichannelAccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_boot_mcuichannel_access(&self) -> EnblWrGroup3ofBootMcuichannelAccessR {
        EnblWrGroup3ofBootMcuichannelAccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_boot_mcuichannel_access(&self) -> EnblWrGroup4ofBootMcuichannelAccessR {
        EnblWrGroup4ofBootMcuichannelAccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_boot_mcuichannel_access(&self) -> EnblWrGroup5ofBootMcuichannelAccessR {
        EnblWrGroup5ofBootMcuichannelAccessR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1014PRIC1_014\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1014pric10140500(
        &self,
    ) -> EnblRstToleranceOfPric1014pric10140500R {
        EnblRstToleranceOfPric1014pric10140500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1014PRIC1_014\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1014pric10140600(&self) -> EnblWrProtOfPric1014pric10140600R {
        EnblWrProtOfPric1014pric10140600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_boot_mcudchannel_access(&self) -> EnblWrGroup0ofBootMcudchannelAccessR {
        EnblWrGroup0ofBootMcudchannelAccessR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_boot_mcudchannel_access(&self) -> EnblWrGroup1ofBootMcudchannelAccessR {
        EnblWrGroup1ofBootMcudchannelAccessR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_boot_mcudchannel_access(&self) -> EnblWrGroup2ofBootMcudchannelAccessR {
        EnblWrGroup2ofBootMcudchannelAccessR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_boot_mcudchannel_access(&self) -> EnblWrGroup3ofBootMcudchannelAccessR {
        EnblWrGroup3ofBootMcudchannelAccessR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_boot_mcudchannel_access(&self) -> EnblWrGroup4ofBootMcudchannelAccessR {
        EnblWrGroup4ofBootMcudchannelAccessR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_boot_mcudchannel_access(&self) -> EnblWrGroup5ofBootMcudchannelAccessR {
        EnblWrGroup5ofBootMcudchannelAccessR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1014PRIC1_014\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1014pric10141308(
        &self,
    ) -> EnblRstToleranceOfPric1014pric10141308R {
        EnblRstToleranceOfPric1014pric10141308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1014PRIC1_014\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1014pric10141408(&self) -> EnblWrProtOfPric1014pric10141408R {
        EnblWrProtOfPric1014pric10141408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_boot_mcuichannel_access(
        &mut self,
    ) -> EnblWrGroup0ofBootMcuichannelAccessW<PricIo014Spec> {
        EnblWrGroup0ofBootMcuichannelAccessW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_boot_mcuichannel_access(
        &mut self,
    ) -> EnblWrGroup1ofBootMcuichannelAccessW<PricIo014Spec> {
        EnblWrGroup1ofBootMcuichannelAccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_boot_mcuichannel_access(
        &mut self,
    ) -> EnblWrGroup2ofBootMcuichannelAccessW<PricIo014Spec> {
        EnblWrGroup2ofBootMcuichannelAccessW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_boot_mcuichannel_access(
        &mut self,
    ) -> EnblWrGroup3ofBootMcuichannelAccessW<PricIo014Spec> {
        EnblWrGroup3ofBootMcuichannelAccessW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_boot_mcuichannel_access(
        &mut self,
    ) -> EnblWrGroup4ofBootMcuichannelAccessW<PricIo014Spec> {
        EnblWrGroup4ofBootMcuichannelAccessW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_boot_mcuichannel_access(
        &mut self,
    ) -> EnblWrGroup5ofBootMcuichannelAccessW<PricIo014Spec> {
        EnblWrGroup5ofBootMcuichannelAccessW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1014PRIC1_014\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1014pric10140500(
        &mut self,
    ) -> EnblRstToleranceOfPric1014pric10140500W<PricIo014Spec> {
        EnblRstToleranceOfPric1014pric10140500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1014PRIC1_014\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1014pric10140600(
        &mut self,
    ) -> EnblWrProtOfPric1014pric10140600W<PricIo014Spec> {
        EnblWrProtOfPric1014pric10140600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_boot_mcudchannel_access(
        &mut self,
    ) -> EnblWrGroup0ofBootMcudchannelAccessW<PricIo014Spec> {
        EnblWrGroup0ofBootMcudchannelAccessW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_boot_mcudchannel_access(
        &mut self,
    ) -> EnblWrGroup1ofBootMcudchannelAccessW<PricIo014Spec> {
        EnblWrGroup1ofBootMcudchannelAccessW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_boot_mcudchannel_access(
        &mut self,
    ) -> EnblWrGroup2ofBootMcudchannelAccessW<PricIo014Spec> {
        EnblWrGroup2ofBootMcudchannelAccessW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_boot_mcudchannel_access(
        &mut self,
    ) -> EnblWrGroup3ofBootMcudchannelAccessW<PricIo014Spec> {
        EnblWrGroup3ofBootMcudchannelAccessW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_boot_mcudchannel_access(
        &mut self,
    ) -> EnblWrGroup4ofBootMcudchannelAccessW<PricIo014Spec> {
        EnblWrGroup4ofBootMcudchannelAccessW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_boot_mcudchannel_access(
        &mut self,
    ) -> EnblWrGroup5ofBootMcudchannelAccessW<PricIo014Spec> {
        EnblWrGroup5ofBootMcudchannelAccessW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1014PRIC1_014\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1014pric10141308(
        &mut self,
    ) -> EnblRstToleranceOfPric1014pric10141308W<PricIo014Spec> {
        EnblRstToleranceOfPric1014pric10141308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1014PRIC1_014\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1014pric10141408(
        &mut self,
    ) -> EnblWrProtOfPric1014pric10141408W<PricIo014Spec> {
        EnblWrProtOfPric1014pric10141408W::new(self, 15)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<PricIo014Spec> {
        Reserved9W::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<PricIo014Spec> {
        Reserved8W::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo014Spec> {
        Reserved7W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo014Spec> {
        Reserved6W::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo014Spec> {
        Reserved5W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo014Spec> {
        Reserved4W::new(self, 21)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo014Spec> {
        Reserved3W::new(self, 22)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo014Spec> {
        Reserved2W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo014Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Master Write Group Setting Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo014Spec;
impl crate::RegisterSpec for PricIo014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io014::R`](R) reader structure"]
impl crate::Readable for PricIo014Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io014::W`](W) writer structure"]
impl crate::Writable for PricIo014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO014 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo014Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
