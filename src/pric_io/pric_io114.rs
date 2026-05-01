#[doc = "Register `PRIC_IO114` reader"]
pub type R = crate::R<PricIo114Spec>;
#[doc = "Register `PRIC_IO114` writer"]
pub type W = crate::W<PricIo114Spec>;
#[doc = "Field `EnblReadGroup0OfBootMCUIChannelAccess` reader - Enable Read Group #0 of Boot MCU I channel access"]
pub type EnblReadGroup0ofBootMcuichannelAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfBootMCUIChannelAccess` writer - Enable Read Group #0 of Boot MCU I channel access"]
pub type EnblReadGroup0ofBootMcuichannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfBootMCUIChannelAccess` reader - Enable Read Group #1 of Boot MCU I channel access"]
pub type EnblReadGroup1ofBootMcuichannelAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfBootMCUIChannelAccess` writer - Enable Read Group #1 of Boot MCU I channel access"]
pub type EnblReadGroup1ofBootMcuichannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfBootMCUIChannelAccess` reader - Enable Read Group #2 of Boot MCU I channel access"]
pub type EnblReadGroup2ofBootMcuichannelAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfBootMCUIChannelAccess` writer - Enable Read Group #2 of Boot MCU I channel access"]
pub type EnblReadGroup2ofBootMcuichannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfBootMCUIChannelAccess` reader - Enable Read Group #3 of Boot MCU I channel access"]
pub type EnblReadGroup3ofBootMcuichannelAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfBootMCUIChannelAccess` writer - Enable Read Group #3 of Boot MCU I channel access"]
pub type EnblReadGroup3ofBootMcuichannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfBootMCUIChannelAccess` reader - Enable Read Group #4 of Boot MCU I channel access"]
pub type EnblReadGroup4ofBootMcuichannelAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfBootMCUIChannelAccess` writer - Enable Read Group #4 of Boot MCU I channel access"]
pub type EnblReadGroup4ofBootMcuichannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfBootMCUIChannelAccess` reader - Enable Read Group #5 of Boot MCU I channel access"]
pub type EnblReadGroup5ofBootMcuichannelAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfBootMCUIChannelAccess` writer - Enable Read Group #5 of Boot MCU I channel access"]
pub type EnblReadGroup5ofBootMcuichannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1114PRIC1_114\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1114pric11140500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1114pric11140500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1114pric11140500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1114PRIC11140500` reader - Enable Reset Tolerance of PRIC1114PRIC1_114\\[05:00\\]"]
pub type EnblRstToleranceOfPric1114pric11140500R =
    crate::BitReader<EnblRstToleranceOfPric1114pric11140500>;
impl EnblRstToleranceOfPric1114pric11140500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1114pric11140500 {
        match self.bits {
            false => EnblRstToleranceOfPric1114pric11140500::ResetBySrst,
            true => EnblRstToleranceOfPric1114pric11140500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1114pric11140500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1114pric11140500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1114PRIC11140500` writer - Enable Reset Tolerance of PRIC1114PRIC1_114\\[05:00\\]"]
pub type EnblRstToleranceOfPric1114pric11140500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1114pric11140500>;
impl<'a, REG> EnblRstToleranceOfPric1114pric11140500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1114pric11140500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1114pric11140500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1114PRIC11140600` reader - Enable Write Protection of PRIC1114PRIC1_114\\[06:00\\]"]
pub type EnblWrProtOfPric1114pric11140600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1114PRIC11140600` writer - Enable Write Protection of PRIC1114PRIC1_114\\[06:00\\]"]
pub type EnblWrProtOfPric1114pric11140600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfBootMCUDChannelAccess` reader - Enable Read Group #0 of Boot MCU D channel access"]
pub type EnblReadGroup0ofBootMcudchannelAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfBootMCUDChannelAccess` writer - Enable Read Group #0 of Boot MCU D channel access"]
pub type EnblReadGroup0ofBootMcudchannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfBootMCUDChannelAccess` reader - Enable Read Group #1 of Boot MCU D channel access"]
pub type EnblReadGroup1ofBootMcudchannelAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfBootMCUDChannelAccess` writer - Enable Read Group #1 of Boot MCU D channel access"]
pub type EnblReadGroup1ofBootMcudchannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfBootMCUDChannelAccess` reader - Enable Read Group #2 of Boot MCU D channel access"]
pub type EnblReadGroup2ofBootMcudchannelAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfBootMCUDChannelAccess` writer - Enable Read Group #2 of Boot MCU D channel access"]
pub type EnblReadGroup2ofBootMcudchannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfBootMCUDChannelAccess` reader - Enable Read Group #3 of Boot MCU D channel access"]
pub type EnblReadGroup3ofBootMcudchannelAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfBootMCUDChannelAccess` writer - Enable Read Group #3 of Boot MCU D channel access"]
pub type EnblReadGroup3ofBootMcudchannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfBootMCUDChannelAccess` reader - Enable Read Group #4 of Boot MCU D channel access"]
pub type EnblReadGroup4ofBootMcudchannelAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfBootMCUDChannelAccess` writer - Enable Read Group #4 of Boot MCU D channel access"]
pub type EnblReadGroup4ofBootMcudchannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfBootMCUDChannelAccess` reader - Enable Read Group #5 of Boot MCU D channel access"]
pub type EnblReadGroup5ofBootMcudchannelAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfBootMCUDChannelAccess` writer - Enable Read Group #5 of Boot MCU D channel access"]
pub type EnblReadGroup5ofBootMcudchannelAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1114PRIC1_114\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1114pric11141308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1114pric11141308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1114pric11141308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1114PRIC11141308` reader - Enable Reset Tolerance of PRIC1114PRIC1_114\\[13:08\\]"]
pub type EnblRstToleranceOfPric1114pric11141308R =
    crate::BitReader<EnblRstToleranceOfPric1114pric11141308>;
impl EnblRstToleranceOfPric1114pric11141308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1114pric11141308 {
        match self.bits {
            false => EnblRstToleranceOfPric1114pric11141308::ResetBySrst,
            true => EnblRstToleranceOfPric1114pric11141308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1114pric11141308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1114pric11141308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1114PRIC11141308` writer - Enable Reset Tolerance of PRIC1114PRIC1_114\\[13:08\\]"]
pub type EnblRstToleranceOfPric1114pric11141308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1114pric11141308>;
impl<'a, REG> EnblRstToleranceOfPric1114pric11141308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1114pric11141308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1114pric11141308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1114PRIC11141408` reader - Enable Write Protection of PRIC1114PRIC1_114\\[14:08\\]"]
pub type EnblWrProtOfPric1114pric11141408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1114PRIC11141408` writer - Enable Write Protection of PRIC1114PRIC1_114\\[14:08\\]"]
pub type EnblWrProtOfPric1114pric11141408W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - Enable Read Group #0 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_read_group0of_boot_mcuichannel_access(
        &self,
    ) -> EnblReadGroup0ofBootMcuichannelAccessR {
        EnblReadGroup0ofBootMcuichannelAccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_read_group1of_boot_mcuichannel_access(
        &self,
    ) -> EnblReadGroup1ofBootMcuichannelAccessR {
        EnblReadGroup1ofBootMcuichannelAccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_read_group2of_boot_mcuichannel_access(
        &self,
    ) -> EnblReadGroup2ofBootMcuichannelAccessR {
        EnblReadGroup2ofBootMcuichannelAccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_read_group3of_boot_mcuichannel_access(
        &self,
    ) -> EnblReadGroup3ofBootMcuichannelAccessR {
        EnblReadGroup3ofBootMcuichannelAccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_read_group4of_boot_mcuichannel_access(
        &self,
    ) -> EnblReadGroup4ofBootMcuichannelAccessR {
        EnblReadGroup4ofBootMcuichannelAccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_read_group5of_boot_mcuichannel_access(
        &self,
    ) -> EnblReadGroup5ofBootMcuichannelAccessR {
        EnblReadGroup5ofBootMcuichannelAccessR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1114PRIC1_114\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1114pric11140500(
        &self,
    ) -> EnblRstToleranceOfPric1114pric11140500R {
        EnblRstToleranceOfPric1114pric11140500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1114PRIC1_114\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1114pric11140600(&self) -> EnblWrProtOfPric1114pric11140600R {
        EnblWrProtOfPric1114pric11140600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_read_group0of_boot_mcudchannel_access(
        &self,
    ) -> EnblReadGroup0ofBootMcudchannelAccessR {
        EnblReadGroup0ofBootMcudchannelAccessR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_read_group1of_boot_mcudchannel_access(
        &self,
    ) -> EnblReadGroup1ofBootMcudchannelAccessR {
        EnblReadGroup1ofBootMcudchannelAccessR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_read_group2of_boot_mcudchannel_access(
        &self,
    ) -> EnblReadGroup2ofBootMcudchannelAccessR {
        EnblReadGroup2ofBootMcudchannelAccessR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_read_group3of_boot_mcudchannel_access(
        &self,
    ) -> EnblReadGroup3ofBootMcudchannelAccessR {
        EnblReadGroup3ofBootMcudchannelAccessR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_read_group4of_boot_mcudchannel_access(
        &self,
    ) -> EnblReadGroup4ofBootMcudchannelAccessR {
        EnblReadGroup4ofBootMcudchannelAccessR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_read_group5of_boot_mcudchannel_access(
        &self,
    ) -> EnblReadGroup5ofBootMcudchannelAccessR {
        EnblReadGroup5ofBootMcudchannelAccessR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1114PRIC1_114\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1114pric11141308(
        &self,
    ) -> EnblRstToleranceOfPric1114pric11141308R {
        EnblRstToleranceOfPric1114pric11141308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1114PRIC1_114\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1114pric11141408(&self) -> EnblWrProtOfPric1114pric11141408R {
        EnblWrProtOfPric1114pric11141408R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 0 - Enable Read Group #0 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_read_group0of_boot_mcuichannel_access(
        &mut self,
    ) -> EnblReadGroup0ofBootMcuichannelAccessW<PricIo114Spec> {
        EnblReadGroup0ofBootMcuichannelAccessW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_read_group1of_boot_mcuichannel_access(
        &mut self,
    ) -> EnblReadGroup1ofBootMcuichannelAccessW<PricIo114Spec> {
        EnblReadGroup1ofBootMcuichannelAccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_read_group2of_boot_mcuichannel_access(
        &mut self,
    ) -> EnblReadGroup2ofBootMcuichannelAccessW<PricIo114Spec> {
        EnblReadGroup2ofBootMcuichannelAccessW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_read_group3of_boot_mcuichannel_access(
        &mut self,
    ) -> EnblReadGroup3ofBootMcuichannelAccessW<PricIo114Spec> {
        EnblReadGroup3ofBootMcuichannelAccessW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_read_group4of_boot_mcuichannel_access(
        &mut self,
    ) -> EnblReadGroup4ofBootMcuichannelAccessW<PricIo114Spec> {
        EnblReadGroup4ofBootMcuichannelAccessW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of Boot MCU I channel access"]
    #[inline(always)]
    pub fn enbl_read_group5of_boot_mcuichannel_access(
        &mut self,
    ) -> EnblReadGroup5ofBootMcuichannelAccessW<PricIo114Spec> {
        EnblReadGroup5ofBootMcuichannelAccessW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1114PRIC1_114\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1114pric11140500(
        &mut self,
    ) -> EnblRstToleranceOfPric1114pric11140500W<PricIo114Spec> {
        EnblRstToleranceOfPric1114pric11140500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1114PRIC1_114\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1114pric11140600(
        &mut self,
    ) -> EnblWrProtOfPric1114pric11140600W<PricIo114Spec> {
        EnblWrProtOfPric1114pric11140600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_read_group0of_boot_mcudchannel_access(
        &mut self,
    ) -> EnblReadGroup0ofBootMcudchannelAccessW<PricIo114Spec> {
        EnblReadGroup0ofBootMcudchannelAccessW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_read_group1of_boot_mcudchannel_access(
        &mut self,
    ) -> EnblReadGroup1ofBootMcudchannelAccessW<PricIo114Spec> {
        EnblReadGroup1ofBootMcudchannelAccessW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_read_group2of_boot_mcudchannel_access(
        &mut self,
    ) -> EnblReadGroup2ofBootMcudchannelAccessW<PricIo114Spec> {
        EnblReadGroup2ofBootMcudchannelAccessW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_read_group3of_boot_mcudchannel_access(
        &mut self,
    ) -> EnblReadGroup3ofBootMcudchannelAccessW<PricIo114Spec> {
        EnblReadGroup3ofBootMcudchannelAccessW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_read_group4of_boot_mcudchannel_access(
        &mut self,
    ) -> EnblReadGroup4ofBootMcudchannelAccessW<PricIo114Spec> {
        EnblReadGroup4ofBootMcudchannelAccessW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of Boot MCU D channel access"]
    #[inline(always)]
    pub fn enbl_read_group5of_boot_mcudchannel_access(
        &mut self,
    ) -> EnblReadGroup5ofBootMcudchannelAccessW<PricIo114Spec> {
        EnblReadGroup5ofBootMcudchannelAccessW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1114PRIC1_114\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1114pric11141308(
        &mut self,
    ) -> EnblRstToleranceOfPric1114pric11141308W<PricIo114Spec> {
        EnblRstToleranceOfPric1114pric11141308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1114PRIC1_114\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1114pric11141408(
        &mut self,
    ) -> EnblWrProtOfPric1114pric11141408W<PricIo114Spec> {
        EnblWrProtOfPric1114pric11141408W::new(self, 15)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<PricIo114Spec> {
        Reserved9W::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<PricIo114Spec> {
        Reserved8W::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo114Spec> {
        Reserved7W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo114Spec> {
        Reserved6W::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo114Spec> {
        Reserved5W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo114Spec> {
        Reserved4W::new(self, 21)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo114Spec> {
        Reserved3W::new(self, 22)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo114Spec> {
        Reserved2W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo114Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Master Read Group Setting Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io114::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io114::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo114Spec;
impl crate::RegisterSpec for PricIo114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io114::R`](R) reader structure"]
impl crate::Readable for PricIo114Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io114::W`](W) writer structure"]
impl crate::Writable for PricIo114Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO114 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo114Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
