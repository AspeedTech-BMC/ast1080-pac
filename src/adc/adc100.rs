#[doc = "Register `ADC100` reader"]
pub type R = crate::R<Adc100Spec>;
#[doc = "Register `ADC100` writer"]
pub type W = crate::W<Adc100Spec>;
#[doc = "Field `EngEnbl` reader - Engine enable."]
pub type EngEnblR = crate::BitReader;
#[doc = "Field `EngEnbl` writer - Engine enable."]
pub type EngEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC Operation mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcopMode {
    #[doc = "0: Power down mode."]
    PowerDownMode = 0,
    #[doc = "1: Standby mode."]
    StandbyMode = 1,
    #[doc = "7: Normal mode."]
    NormalMode = 7,
}
impl From<AdcopMode> for u8 {
    #[inline(always)]
    fn from(variant: AdcopMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AdcopMode {
    type Ux = u8;
}
impl crate::IsEnum for AdcopMode {}
#[doc = "Field `ADCOpMode` reader - ADC Operation mode."]
pub type AdcopModeR = crate::FieldReader<AdcopMode>;
impl AdcopModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AdcopMode> {
        match self.bits {
            0 => Some(AdcopMode::PowerDownMode),
            1 => Some(AdcopMode::StandbyMode),
            7 => Some(AdcopMode::NormalMode),
            _ => None,
        }
    }
    #[doc = "Power down mode."]
    #[inline(always)]
    pub fn is_power_down_mode(&self) -> bool {
        *self == AdcopMode::PowerDownMode
    }
    #[doc = "Standby mode."]
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        *self == AdcopMode::StandbyMode
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == AdcopMode::NormalMode
    }
}
#[doc = "Field `ADCOpMode` writer - ADC Operation mode."]
pub type AdcopModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, AdcopMode>;
impl<'a, REG> AdcopModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Power down mode."]
    #[inline(always)]
    pub fn power_down_mode(self) -> &'a mut crate::W<REG> {
        self.variant(AdcopMode::PowerDownMode)
    }
    #[doc = "Standby mode."]
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut crate::W<REG> {
        self.variant(AdcopMode::StandbyMode)
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(AdcopMode::NormalMode)
    }
}
#[doc = "Field `CompensatingSensingMode` reader - Compensating sensing mode."]
pub type CompensatingSensingModeR = crate::BitReader;
#[doc = "Field `CompensatingSensingMode` writer - Compensating sensing mode."]
pub type CompensatingSensingModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Auto Compensating sensing mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoCompensatingSensingMode {
    #[doc = "1: Trigger compensating method."]
    TriggerCompensatingMethod = 1,
    #[doc = "0: Compensating method is done."]
    CompensatingMethodIsDone = 0,
}
impl From<AutoCompensatingSensingMode> for bool {
    #[inline(always)]
    fn from(variant: AutoCompensatingSensingMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AutoCompensatingSensingMode` reader - Auto Compensating sensing mode."]
pub type AutoCompensatingSensingModeR = crate::BitReader<AutoCompensatingSensingMode>;
impl AutoCompensatingSensingModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoCompensatingSensingMode {
        match self.bits {
            true => AutoCompensatingSensingMode::TriggerCompensatingMethod,
            false => AutoCompensatingSensingMode::CompensatingMethodIsDone,
        }
    }
    #[doc = "Trigger compensating method."]
    #[inline(always)]
    pub fn is_trigger_compensating_method(&self) -> bool {
        *self == AutoCompensatingSensingMode::TriggerCompensatingMethod
    }
    #[doc = "Compensating method is done."]
    #[inline(always)]
    pub fn is_compensating_method_is_done(&self) -> bool {
        *self == AutoCompensatingSensingMode::CompensatingMethodIsDone
    }
}
#[doc = "Field `AutoCompensatingSensingMode` writer - Auto Compensating sensing mode."]
pub type AutoCompensatingSensingModeW<'a, REG> =
    crate::BitWriter<'a, REG, AutoCompensatingSensingMode>;
impl<'a, REG> AutoCompensatingSensingModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger compensating method."]
    #[inline(always)]
    pub fn trigger_compensating_method(self) -> &'a mut crate::W<REG> {
        self.variant(AutoCompensatingSensingMode::TriggerCompensatingMethod)
    }
    #[doc = "Compensating method is done."]
    #[inline(always)]
    pub fn compensating_method_is_done(self) -> &'a mut crate::W<REG> {
        self.variant(AutoCompensatingSensingMode::CompensatingMethodIsDone)
    }
}
#[doc = "Reference Voltage Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ReferenceVoltageSel {
    #[doc = "0: 2.5v"]
    _25v = 0,
    #[doc = "1: 1.2v"]
    _12v = 1,
    #[doc = "2: External Voltage (1.55v to 2.7v)"]
    ExternalVoltage155vTo27v = 2,
    #[doc = "3: External Voltage (0.9v to 1.65v)"]
    ExternalVoltage09v_To165v = 3,
}
impl From<ReferenceVoltageSel> for u8 {
    #[inline(always)]
    fn from(variant: ReferenceVoltageSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ReferenceVoltageSel {
    type Ux = u8;
}
impl crate::IsEnum for ReferenceVoltageSel {}
#[doc = "Field `ReferenceVoltageSel` reader - Reference Voltage Selection"]
pub type ReferenceVoltageSelR = crate::FieldReader<ReferenceVoltageSel>;
impl ReferenceVoltageSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReferenceVoltageSel {
        match self.bits {
            0 => ReferenceVoltageSel::_25v,
            1 => ReferenceVoltageSel::_12v,
            2 => ReferenceVoltageSel::ExternalVoltage155vTo27v,
            3 => ReferenceVoltageSel::ExternalVoltage09v_To165v,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5v"]
    #[inline(always)]
    pub fn is_25v(&self) -> bool {
        *self == ReferenceVoltageSel::_25v
    }
    #[doc = "1.2v"]
    #[inline(always)]
    pub fn is_12v(&self) -> bool {
        *self == ReferenceVoltageSel::_12v
    }
    #[doc = "External Voltage (1.55v to 2.7v)"]
    #[inline(always)]
    pub fn is_external_voltage_155v_to_27v(&self) -> bool {
        *self == ReferenceVoltageSel::ExternalVoltage155vTo27v
    }
    #[doc = "External Voltage (0.9v to 1.65v)"]
    #[inline(always)]
    pub fn is_external_voltage_09v__to_165v(&self) -> bool {
        *self == ReferenceVoltageSel::ExternalVoltage09v_To165v
    }
}
#[doc = "Field `ReferenceVoltageSel` writer - Reference Voltage Selection"]
pub type ReferenceVoltageSelW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, ReferenceVoltageSel, crate::Safe>;
impl<'a, REG> ReferenceVoltageSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5v"]
    #[inline(always)]
    pub fn _25v(self) -> &'a mut crate::W<REG> {
        self.variant(ReferenceVoltageSel::_25v)
    }
    #[doc = "1.2v"]
    #[inline(always)]
    pub fn _12v(self) -> &'a mut crate::W<REG> {
        self.variant(ReferenceVoltageSel::_12v)
    }
    #[doc = "External Voltage (1.55v to 2.7v)"]
    #[inline(always)]
    pub fn external_voltage_155v_to_27v(self) -> &'a mut crate::W<REG> {
        self.variant(ReferenceVoltageSel::ExternalVoltage155vTo27v)
    }
    #[doc = "External Voltage (0.9v to 1.65v)"]
    #[inline(always)]
    pub fn external_voltage_09v__to_165v(self) -> &'a mut crate::W<REG> {
        self.variant(ReferenceVoltageSel::ExternalVoltage09v_To165v)
    }
}
#[doc = "Field `InitialSequenceComplete` reader - Initial sequence complete"]
pub type InitialSequenceCompleteR = crate::BitReader;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Channel 15 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel15sel {
    #[doc = "0: Normal Voltage"]
    NormalVoltage = 0,
    #[doc = "1: Battery"]
    Battery = 1,
}
impl From<Channel15sel> for bool {
    #[inline(always)]
    fn from(variant: Channel15sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Channel15Sel` reader - Channel 15 Selection"]
pub type Channel15selR = crate::BitReader<Channel15sel>;
impl Channel15selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel15sel {
        match self.bits {
            false => Channel15sel::NormalVoltage,
            true => Channel15sel::Battery,
        }
    }
    #[doc = "Normal Voltage"]
    #[inline(always)]
    pub fn is_normal_voltage(&self) -> bool {
        *self == Channel15sel::NormalVoltage
    }
    #[doc = "Battery"]
    #[inline(always)]
    pub fn is_battery(&self) -> bool {
        *self == Channel15sel::Battery
    }
}
#[doc = "Field `Channel15Sel` writer - Channel 15 Selection"]
pub type Channel15selW<'a, REG> = crate::BitWriter<'a, REG, Channel15sel>;
impl<'a, REG> Channel15selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Voltage"]
    #[inline(always)]
    pub fn normal_voltage(self) -> &'a mut crate::W<REG> {
        self.variant(Channel15sel::NormalVoltage)
    }
    #[doc = "Battery"]
    #[inline(always)]
    pub fn battery(self) -> &'a mut crate::W<REG> {
        self.variant(Channel15sel::Battery)
    }
}
#[doc = "Field `EnblBatterySensing` reader - Enable Battery Sensing"]
pub type EnblBatterySensingR = crate::BitReader;
#[doc = "Field `EnblBatterySensing` writer - Enable Battery Sensing"]
pub type EnblBatterySensingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `ChannelEnbl` reader - Channel enable."]
pub type ChannelEnblR = crate::FieldReader;
#[doc = "Field `ChannelEnbl` writer - Channel enable."]
pub type ChannelEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Engine enable."]
    #[inline(always)]
    pub fn eng_enbl(&self) -> EngEnblR {
        EngEnblR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - ADC Operation mode."]
    #[inline(always)]
    pub fn adcop_mode(&self) -> AdcopModeR {
        AdcopModeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Compensating sensing mode."]
    #[inline(always)]
    pub fn compensating_sensing_mode(&self) -> CompensatingSensingModeR {
        CompensatingSensingModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto Compensating sensing mode."]
    #[inline(always)]
    pub fn auto_compensating_sensing_mode(&self) -> AutoCompensatingSensingModeR {
        AutoCompensatingSensingModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reference Voltage Selection"]
    #[inline(always)]
    pub fn reference_voltage_sel(&self) -> ReferenceVoltageSelR {
        ReferenceVoltageSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Initial sequence complete"]
    #[inline(always)]
    pub fn initial_sequence_complete(&self) -> InitialSequenceCompleteR {
        InitialSequenceCompleteR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Channel 15 Selection"]
    #[inline(always)]
    pub fn channel15sel(&self) -> Channel15selR {
        Channel15selR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Battery Sensing"]
    #[inline(always)]
    pub fn enbl_battery_sensing(&self) -> EnblBatterySensingR {
        EnblBatterySensingR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Channel enable."]
    #[inline(always)]
    pub fn channel_enbl(&self) -> ChannelEnblR {
        ChannelEnblR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Engine enable."]
    #[inline(always)]
    pub fn eng_enbl(&mut self) -> EngEnblW<Adc100Spec> {
        EngEnblW::new(self, 0)
    }
    #[doc = "Bits 1:3 - ADC Operation mode."]
    #[inline(always)]
    pub fn adcop_mode(&mut self) -> AdcopModeW<Adc100Spec> {
        AdcopModeW::new(self, 1)
    }
    #[doc = "Bit 4 - Compensating sensing mode."]
    #[inline(always)]
    pub fn compensating_sensing_mode(&mut self) -> CompensatingSensingModeW<Adc100Spec> {
        CompensatingSensingModeW::new(self, 4)
    }
    #[doc = "Bit 5 - Auto Compensating sensing mode."]
    #[inline(always)]
    pub fn auto_compensating_sensing_mode(&mut self) -> AutoCompensatingSensingModeW<Adc100Spec> {
        AutoCompensatingSensingModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Reference Voltage Selection"]
    #[inline(always)]
    pub fn reference_voltage_sel(&mut self) -> ReferenceVoltageSelW<Adc100Spec> {
        ReferenceVoltageSelW::new(self, 6)
    }
    #[doc = "Bit 12 - Channel 15 Selection"]
    #[inline(always)]
    pub fn channel15sel(&mut self) -> Channel15selW<Adc100Spec> {
        Channel15selW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Battery Sensing"]
    #[inline(always)]
    pub fn enbl_battery_sensing(&mut self) -> EnblBatterySensingW<Adc100Spec> {
        EnblBatterySensingW::new(self, 13)
    }
    #[doc = "Bits 16:23 - Channel enable."]
    #[inline(always)]
    pub fn channel_enbl(&mut self) -> ChannelEnblW<Adc100Spec> {
        ChannelEnblW::new(self, 16)
    }
}
#[doc = "Engine Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc100::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc100::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc100Spec;
impl crate::RegisterSpec for Adc100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc100::R`](R) reader structure"]
impl crate::Readable for Adc100Spec {}
#[doc = "`write(|w| ..)` method takes [`adc100::W`](W) writer structure"]
impl crate::Writable for Adc100Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC100 to value 0"]
impl crate::Resettable for Adc100Spec {}
