#[doc = "Register `JTAG030` reader"]
pub type R = crate::R<Jtag030Spec>;
#[doc = "Register `JTAG030` writer"]
pub type W = crate::W<Jtag030Spec>;
#[doc = "Field `LowerDataShiftNumber` reader - Lower Data Shift Number."]
pub type LowerDataShiftNumberR = crate::FieldReader;
#[doc = "Field `LowerDataShiftNumber` writer - Lower Data Shift Number."]
pub type LowerDataShiftNumberW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `StartOfShift` reader - Start of shift"]
pub type StartOfShiftR = crate::BitReader;
#[doc = "Field `StartOfShift` writer - Start of shift"]
pub type StartOfShiftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EndOfShift` reader - End of shift"]
pub type EndOfShiftR = crate::BitReader;
#[doc = "Field `EndOfShift` writer - End of shift"]
pub type EndOfShiftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Paddign selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaddignSel {
    #[doc = "0: Use JTAG28 for padding setting."]
    UseJtag28ForPaddingSetting = 0,
    #[doc = "1: Use JTAG2C for padding setting."]
    UseJtag2cForPaddingSetting = 1,
}
impl From<PaddignSel> for bool {
    #[inline(always)]
    fn from(variant: PaddignSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PaddignSel` reader - Paddign selection"]
pub type PaddignSelR = crate::BitReader<PaddignSel>;
impl PaddignSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PaddignSel {
        match self.bits {
            false => PaddignSel::UseJtag28ForPaddingSetting,
            true => PaddignSel::UseJtag2cForPaddingSetting,
        }
    }
    #[doc = "Use JTAG28 for padding setting."]
    #[inline(always)]
    pub fn is_use_jtag28_for_padding_setting(&self) -> bool {
        *self == PaddignSel::UseJtag28ForPaddingSetting
    }
    #[doc = "Use JTAG2C for padding setting."]
    #[inline(always)]
    pub fn is_use_jtag2c_for_padding_setting(&self) -> bool {
        *self == PaddignSel::UseJtag2cForPaddingSetting
    }
}
#[doc = "Field `PaddignSel` writer - Paddign selection"]
pub type PaddignSelW<'a, REG> = crate::BitWriter<'a, REG, PaddignSel>;
impl<'a, REG> PaddignSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use JTAG28 for padding setting."]
    #[inline(always)]
    pub fn use_jtag28_for_padding_setting(self) -> &'a mut crate::W<REG> {
        self.variant(PaddignSel::UseJtag28ForPaddingSetting)
    }
    #[doc = "Use JTAG2C for padding setting."]
    #[inline(always)]
    pub fn use_jtag2c_for_padding_setting(self) -> &'a mut crate::W<REG> {
        self.variant(PaddignSel::UseJtag2cForPaddingSetting)
    }
}
#[doc = "Field `PreTMSShiftNumber` reader - Pre TMS Shift Number."]
pub type PreTmsshiftNumberR = crate::FieldReader;
#[doc = "Field `PreTMSShiftNumber` writer - Pre TMS Shift Number."]
pub type PreTmsshiftNumberW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PostTMSShiftNumber` reader - Post TMS Shift Number."]
pub type PostTmsshiftNumberR = crate::FieldReader;
#[doc = "Field `PostTMSShiftNumber` writer - Post TMS Shift Number."]
pub type PostTmsshiftNumberW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TMSValue` reader - TMS Value."]
pub type TmsvalueR = crate::FieldReader<u16>;
#[doc = "Field `TMSValue` writer - TMS Value."]
pub type TmsvalueW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `EnblStaticShift` reader - Enable Static Shift"]
pub type EnblStaticShiftR = crate::BitReader;
#[doc = "Field `EnblStaticShift` writer - Enable Static Shift"]
pub type EnblStaticShiftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Free Run TCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblFreeRunTck {
    #[doc = "1: TCK is free running."]
    TckIsFreeRunning = 1,
    #[doc = "0: TCK toggles only when necessary."]
    TckTogglesOnlyWhenNecessary = 0,
}
impl From<EnblFreeRunTck> for bool {
    #[inline(always)]
    fn from(variant: EnblFreeRunTck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblFreeRunTCK` reader - Enable Free Run TCK"]
pub type EnblFreeRunTckR = crate::BitReader<EnblFreeRunTck>;
impl EnblFreeRunTckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblFreeRunTck {
        match self.bits {
            true => EnblFreeRunTck::TckIsFreeRunning,
            false => EnblFreeRunTck::TckTogglesOnlyWhenNecessary,
        }
    }
    #[doc = "TCK is free running."]
    #[inline(always)]
    pub fn is_tck_is_free_running(&self) -> bool {
        *self == EnblFreeRunTck::TckIsFreeRunning
    }
    #[doc = "TCK toggles only when necessary."]
    #[inline(always)]
    pub fn is_tck_toggles_only_when_necessary(&self) -> bool {
        *self == EnblFreeRunTck::TckTogglesOnlyWhenNecessary
    }
}
#[doc = "Field `EnblFreeRunTCK` writer - Enable Free Run TCK"]
pub type EnblFreeRunTckW<'a, REG> = crate::BitWriter<'a, REG, EnblFreeRunTck>;
impl<'a, REG> EnblFreeRunTckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TCK is free running."]
    #[inline(always)]
    pub fn tck_is_free_running(self) -> &'a mut crate::W<REG> {
        self.variant(EnblFreeRunTck::TckIsFreeRunning)
    }
    #[doc = "TCK toggles only when necessary."]
    #[inline(always)]
    pub fn tck_toggles_only_when_necessary(self) -> &'a mut crate::W<REG> {
        self.variant(EnblFreeRunTck::TckTogglesOnlyWhenNecessary)
    }
}
impl R {
    #[doc = "Bits 0:6 - Lower Data Shift Number."]
    #[inline(always)]
    pub fn lower_data_shift_number(&self) -> LowerDataShiftNumberR {
        LowerDataShiftNumberR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Start of shift"]
    #[inline(always)]
    pub fn start_of_shift(&self) -> StartOfShiftR {
        StartOfShiftR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - End of shift"]
    #[inline(always)]
    pub fn end_of_shift(&self) -> EndOfShiftR {
        EndOfShiftR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Paddign selection"]
    #[inline(always)]
    pub fn paddign_sel(&self) -> PaddignSelR {
        PaddignSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Pre TMS Shift Number."]
    #[inline(always)]
    pub fn pre_tmsshift_number(&self) -> PreTmsshiftNumberR {
        PreTmsshiftNumberR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Post TMS Shift Number."]
    #[inline(always)]
    pub fn post_tmsshift_number(&self) -> PostTmsshiftNumberR {
        PostTmsshiftNumberR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:29 - TMS Value."]
    #[inline(always)]
    pub fn tmsvalue(&self) -> TmsvalueR {
        TmsvalueR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - Enable Static Shift"]
    #[inline(always)]
    pub fn enbl_static_shift(&self) -> EnblStaticShiftR {
        EnblStaticShiftR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Free Run TCK"]
    #[inline(always)]
    pub fn enbl_free_run_tck(&self) -> EnblFreeRunTckR {
        EnblFreeRunTckR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Lower Data Shift Number."]
    #[inline(always)]
    pub fn lower_data_shift_number(&mut self) -> LowerDataShiftNumberW<Jtag030Spec> {
        LowerDataShiftNumberW::new(self, 0)
    }
    #[doc = "Bit 7 - Start of shift"]
    #[inline(always)]
    pub fn start_of_shift(&mut self) -> StartOfShiftW<Jtag030Spec> {
        StartOfShiftW::new(self, 7)
    }
    #[doc = "Bit 8 - End of shift"]
    #[inline(always)]
    pub fn end_of_shift(&mut self) -> EndOfShiftW<Jtag030Spec> {
        EndOfShiftW::new(self, 8)
    }
    #[doc = "Bit 9 - Paddign selection"]
    #[inline(always)]
    pub fn paddign_sel(&mut self) -> PaddignSelW<Jtag030Spec> {
        PaddignSelW::new(self, 9)
    }
    #[doc = "Bits 10:12 - Pre TMS Shift Number."]
    #[inline(always)]
    pub fn pre_tmsshift_number(&mut self) -> PreTmsshiftNumberW<Jtag030Spec> {
        PreTmsshiftNumberW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Post TMS Shift Number."]
    #[inline(always)]
    pub fn post_tmsshift_number(&mut self) -> PostTmsshiftNumberW<Jtag030Spec> {
        PostTmsshiftNumberW::new(self, 13)
    }
    #[doc = "Bits 16:29 - TMS Value."]
    #[inline(always)]
    pub fn tmsvalue(&mut self) -> TmsvalueW<Jtag030Spec> {
        TmsvalueW::new(self, 16)
    }
    #[doc = "Bit 30 - Enable Static Shift"]
    #[inline(always)]
    pub fn enbl_static_shift(&mut self) -> EnblStaticShiftW<Jtag030Spec> {
        EnblStaticShiftW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Free Run TCK"]
    #[inline(always)]
    pub fn enbl_free_run_tck(&mut self) -> EnblFreeRunTckW<Jtag030Spec> {
        EnblFreeRunTckW::new(self, 31)
    }
}
#[doc = "Shift control\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag030Spec;
impl crate::RegisterSpec for Jtag030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag030::R`](R) reader structure"]
impl crate::Readable for Jtag030Spec {}
#[doc = "`write(|w| ..)` method takes [`jtag030::W`](W) writer structure"]
impl crate::Writable for Jtag030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG030 to value 0"]
impl crate::Resettable for Jtag030Spec {}
