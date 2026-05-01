#[doc = "Register `WDT00C` reader"]
pub type R = crate::R<Wdt00cSpec>;
#[doc = "Register `WDT00C` writer"]
pub type W = crate::W<Wdt00cSpec>;
#[doc = "WDT enable signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtenblSig {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<WdtenblSig> for bool {
    #[inline(always)]
    fn from(variant: WdtenblSig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTEnblSig` reader - WDT enable signal"]
pub type WdtenblSigR = crate::BitReader<WdtenblSig>;
impl WdtenblSigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtenblSig {
        match self.bits {
            false => WdtenblSig::Disable,
            true => WdtenblSig::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WdtenblSig::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WdtenblSig::Enable
    }
}
#[doc = "Field `WDTEnblSig` writer - WDT enable signal"]
pub type WdtenblSigW<'a, REG> = crate::BitWriter<'a, REG, WdtenblSig>;
impl<'a, REG> WdtenblSigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtenblSig::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtenblSig::Enable)
    }
}
#[doc = "Reset system after timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstSysAfterTimeout {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<RstSysAfterTimeout> for bool {
    #[inline(always)]
    fn from(variant: RstSysAfterTimeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstSysAfterTimeout` reader - Reset system after timeout"]
pub type RstSysAfterTimeoutR = crate::BitReader<RstSysAfterTimeout>;
impl RstSysAfterTimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstSysAfterTimeout {
        match self.bits {
            false => RstSysAfterTimeout::Disable,
            true => RstSysAfterTimeout::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RstSysAfterTimeout::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RstSysAfterTimeout::Enable
    }
}
#[doc = "Field `RstSysAfterTimeout` writer - Reset system after timeout"]
pub type RstSysAfterTimeoutW<'a, REG> = crate::BitWriter<'a, REG, RstSysAfterTimeout>;
impl<'a, REG> RstSysAfterTimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RstSysAfterTimeout::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RstSysAfterTimeout::Enable)
    }
}
#[doc = "wdt_intr: Generate interrupt when timer count matches to the pre-timeout setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSetting {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSetting> for bool {
    #[inline(always)]
    fn from(variant: WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSetting) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WdtintrGenerateINTWhenTimerCountMatchesToThePretimeoutSetting` reader - wdt_intr: Generate interrupt when timer count matches to the pre-timeout setting"]
pub type WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSettingR =
    crate::BitReader<WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSetting>;
impl WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSettingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSetting {
        match self.bits {
            false => WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSetting::Disable,
            true => WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSetting::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSetting::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSetting::Enable
    }
}
#[doc = "Field `WdtintrGenerateINTWhenTimerCountMatchesToThePretimeoutSetting` writer - wdt_intr: Generate interrupt when timer count matches to the pre-timeout setting"]
pub type WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSettingW<'a, REG> =
    crate::BitWriter<'a, REG, WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSetting>;
impl<'a, REG> WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSettingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSetting::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSetting::Enable)
    }
}
#[doc = "wdt_ext: External signal enable after timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtextExtSigEnblAfterTimeout {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<WdtextExtSigEnblAfterTimeout> for bool {
    #[inline(always)]
    fn from(variant: WdtextExtSigEnblAfterTimeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WdtextExtSigEnblAfterTimeout` reader - wdt_ext: External signal enable after timeout"]
pub type WdtextExtSigEnblAfterTimeoutR = crate::BitReader<WdtextExtSigEnblAfterTimeout>;
impl WdtextExtSigEnblAfterTimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtextExtSigEnblAfterTimeout {
        match self.bits {
            false => WdtextExtSigEnblAfterTimeout::Disable,
            true => WdtextExtSigEnblAfterTimeout::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WdtextExtSigEnblAfterTimeout::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WdtextExtSigEnblAfterTimeout::Enable
    }
}
#[doc = "Field `WdtextExtSigEnblAfterTimeout` writer - wdt_ext: External signal enable after timeout"]
pub type WdtextExtSigEnblAfterTimeoutW<'a, REG> =
    crate::BitWriter<'a, REG, WdtextExtSigEnblAfterTimeout>;
impl<'a, REG> WdtextExtSigEnblAfterTimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtextExtSigEnblAfterTimeout::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtextExtSigEnblAfterTimeout::Enable)
    }
}
#[doc = "Enable WDT(n) to be reset by SOC reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblWdtnToBeRstBySocreset {
    #[doc = "0: WDT(n) can not be reset by SOC reset, only SRST\\# or Full chip reset can reset it."]
    WdtnCanNotBeResetBySocResetOnlySrstOrFullChipResetCanResetIt = 0,
    #[doc = "1: WDT(n) can be reset by SOC reset."]
    WdtnCanBeResetBySocReset = 1,
}
impl From<EnblWdtnToBeRstBySocreset> for bool {
    #[inline(always)]
    fn from(variant: EnblWdtnToBeRstBySocreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblWDTnToBeRstBySOCReset` reader - Enable WDT(n) to be reset by SOC reset"]
pub type EnblWdtnToBeRstBySocresetR = crate::BitReader<EnblWdtnToBeRstBySocreset>;
impl EnblWdtnToBeRstBySocresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblWdtnToBeRstBySocreset {
        match self . bits { false => EnblWdtnToBeRstBySocreset :: WdtnCanNotBeResetBySocResetOnlySrstOrFullChipResetCanResetIt , true => EnblWdtnToBeRstBySocreset :: WdtnCanBeResetBySocReset , }
    }
    #[doc = "WDT(n) can not be reset by SOC reset, only SRST\\# or Full chip reset can reset it."]
    #[inline(always)]
    pub fn is_wdtn_can_not_be_reset_by_soc_reset_only_srst_or_full_chip_reset_can_reset_it(
        &self,
    ) -> bool {
        * self == EnblWdtnToBeRstBySocreset :: WdtnCanNotBeResetBySocResetOnlySrstOrFullChipResetCanResetIt
    }
    #[doc = "WDT(n) can be reset by SOC reset."]
    #[inline(always)]
    pub fn is_wdtn_can_be_reset_by_soc_reset(&self) -> bool {
        *self == EnblWdtnToBeRstBySocreset::WdtnCanBeResetBySocReset
    }
}
#[doc = "Field `EnblWDTnToBeRstBySOCReset` writer - Enable WDT(n) to be reset by SOC reset"]
pub type EnblWdtnToBeRstBySocresetW<'a, REG> = crate::BitWriter<'a, REG, EnblWdtnToBeRstBySocreset>;
impl<'a, REG> EnblWdtnToBeRstBySocresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDT(n) can not be reset by SOC reset, only SRST\\# or Full chip reset can reset it."]
    #[inline(always)]
    pub fn wdtn_can_not_be_reset_by_soc_reset_only_srst_or_full_chip_reset_can_reset_it(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(
            EnblWdtnToBeRstBySocreset::WdtnCanNotBeResetBySocResetOnlySrstOrFullChipResetCanResetIt,
        )
    }
    #[doc = "WDT(n) can be reset by SOC reset."]
    #[inline(always)]
    pub fn wdtn_can_be_reset_by_soc_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblWdtnToBeRstBySocreset::WdtnCanBeResetBySocReset)
    }
}
#[doc = "Reset system mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RstSysMode {
    #[doc = "0: SOC system ewver{(gated by reset mask registers)"]
    SocSystemEwvergatedByResetMaskRegisters = 0,
    #[doc = "1: Full chip"]
    FullChip = 1,
    #[doc = "2: CPU/FMC only, just reboot firmware, no any other IPs will be reset."]
    CpufmcOnlyJustRebootFirmwareNoAnyOtherIpsWillBeReset = 2,
}
impl From<RstSysMode> for u8 {
    #[inline(always)]
    fn from(variant: RstSysMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RstSysMode {
    type Ux = u8;
}
impl crate::IsEnum for RstSysMode {}
#[doc = "Field `RstSysMode` reader - Reset system mode"]
pub type RstSysModeR = crate::FieldReader<RstSysMode>;
impl RstSysModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RstSysMode> {
        match self.bits {
            0 => Some(RstSysMode::SocSystemEwvergatedByResetMaskRegisters),
            1 => Some(RstSysMode::FullChip),
            2 => Some(RstSysMode::CpufmcOnlyJustRebootFirmwareNoAnyOtherIpsWillBeReset),
            _ => None,
        }
    }
    #[doc = "SOC system ewver{(gated by reset mask registers)"]
    #[inline(always)]
    pub fn is_soc_system_ewvergated_by_reset_mask_registers(&self) -> bool {
        *self == RstSysMode::SocSystemEwvergatedByResetMaskRegisters
    }
    #[doc = "Full chip"]
    #[inline(always)]
    pub fn is_full_chip(&self) -> bool {
        *self == RstSysMode::FullChip
    }
    #[doc = "CPU/FMC only, just reboot firmware, no any other IPs will be reset."]
    #[inline(always)]
    pub fn is_cpufmc_only_just_reboot_firmware_no_any_other_ips_will_be_reset(&self) -> bool {
        *self == RstSysMode::CpufmcOnlyJustRebootFirmwareNoAnyOtherIpsWillBeReset
    }
}
#[doc = "Field `RstSysMode` writer - Reset system mode"]
pub type RstSysModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, RstSysMode>;
impl<'a, REG> RstSysModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SOC system ewver{(gated by reset mask registers)"]
    #[inline(always)]
    pub fn soc_system_ewvergated_by_reset_mask_registers(self) -> &'a mut crate::W<REG> {
        self.variant(RstSysMode::SocSystemEwvergatedByResetMaskRegisters)
    }
    #[doc = "Full chip"]
    #[inline(always)]
    pub fn full_chip(self) -> &'a mut crate::W<REG> {
        self.variant(RstSysMode::FullChip)
    }
    #[doc = "CPU/FMC only, just reboot firmware, no any other IPs will be reset."]
    #[inline(always)]
    pub fn cpufmc_only_just_reboot_firmware_no_any_other_ips_will_be_reset(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(RstSysMode::CpufmcOnlyJustRebootFirmwareNoAnyOtherIpsWillBeReset)
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `PretimeoutValueForINTGeneration` reader - Pre-timeout value for interrupt generation"]
pub type PretimeoutValueForIntgenerationR = crate::FieldReader<u32>;
#[doc = "Field `PretimeoutValueForINTGeneration` writer - Pre-timeout value for interrupt generation"]
pub type PretimeoutValueForIntgenerationW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - WDT enable signal"]
    #[inline(always)]
    pub fn wdtenbl_sig(&self) -> WdtenblSigR {
        WdtenblSigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset system after timeout"]
    #[inline(always)]
    pub fn rst_sys_after_timeout(&self) -> RstSysAfterTimeoutR {
        RstSysAfterTimeoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - wdt_intr: Generate interrupt when timer count matches to the pre-timeout setting"]
    #[inline(always)]
    pub fn wdtintr_generate_intwhen_timer_count_matches_to_the_pretimeout_setting(
        &self,
    ) -> WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSettingR {
        WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSettingR::new(
            ((self.bits >> 2) & 1) != 0,
        )
    }
    #[doc = "Bit 3 - wdt_ext: External signal enable after timeout"]
    #[inline(always)]
    pub fn wdtext_ext_sig_enbl_after_timeout(&self) -> WdtextExtSigEnblAfterTimeoutR {
        WdtextExtSigEnblAfterTimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable WDT(n) to be reset by SOC reset"]
    #[inline(always)]
    pub fn enbl_wdtn_to_be_rst_by_socreset(&self) -> EnblWdtnToBeRstBySocresetR {
        EnblWdtnToBeRstBySocresetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Reset system mode"]
    #[inline(always)]
    pub fn rst_sys_mode(&self) -> RstSysModeR {
        RstSysModeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:9 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:31 - Pre-timeout value for interrupt generation"]
    #[inline(always)]
    pub fn pretimeout_value_for_intgeneration(&self) -> PretimeoutValueForIntgenerationR {
        PretimeoutValueForIntgenerationR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - WDT enable signal"]
    #[inline(always)]
    pub fn wdtenbl_sig(&mut self) -> WdtenblSigW<Wdt00cSpec> {
        WdtenblSigW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset system after timeout"]
    #[inline(always)]
    pub fn rst_sys_after_timeout(&mut self) -> RstSysAfterTimeoutW<Wdt00cSpec> {
        RstSysAfterTimeoutW::new(self, 1)
    }
    #[doc = "Bit 2 - wdt_intr: Generate interrupt when timer count matches to the pre-timeout setting"]
    #[inline(always)]
    pub fn wdtintr_generate_intwhen_timer_count_matches_to_the_pretimeout_setting(
        &mut self,
    ) -> WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSettingW<Wdt00cSpec> {
        WdtintrGenerateIntwhenTimerCountMatchesToThePretimeoutSettingW::new(self, 2)
    }
    #[doc = "Bit 3 - wdt_ext: External signal enable after timeout"]
    #[inline(always)]
    pub fn wdtext_ext_sig_enbl_after_timeout(
        &mut self,
    ) -> WdtextExtSigEnblAfterTimeoutW<Wdt00cSpec> {
        WdtextExtSigEnblAfterTimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable WDT(n) to be reset by SOC reset"]
    #[inline(always)]
    pub fn enbl_wdtn_to_be_rst_by_socreset(&mut self) -> EnblWdtnToBeRstBySocresetW<Wdt00cSpec> {
        EnblWdtnToBeRstBySocresetW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Reset system mode"]
    #[inline(always)]
    pub fn rst_sys_mode(&mut self) -> RstSysModeW<Wdt00cSpec> {
        RstSysModeW::new(self, 5)
    }
    #[doc = "Bits 10:31 - Pre-timeout value for interrupt generation"]
    #[inline(always)]
    pub fn pretimeout_value_for_intgeneration(
        &mut self,
    ) -> PretimeoutValueForIntgenerationW<Wdt00cSpec> {
        PretimeoutValueForIntgenerationW::new(self, 10)
    }
}
#[doc = "WDTn Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt00cSpec;
impl crate::RegisterSpec for Wdt00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt00c::R`](R) reader structure"]
impl crate::Readable for Wdt00cSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt00c::W`](W) writer structure"]
impl crate::Writable for Wdt00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT00C to value 0x10"]
impl crate::Resettable for Wdt00cSpec {
    const RESET_VALUE: u32 = 0x10;
}
