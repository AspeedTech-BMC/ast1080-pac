#[doc = "Register `RTC010` reader"]
pub type R = crate::R<Rtc010Spec>;
#[doc = "Register `RTC010` writer"]
pub type W = crate::W<Rtc010Spec>;
#[doc = "RTC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcenbl {
    #[doc = "1: enable"]
    Enable = 1,
    #[doc = "0: disable"]
    Disable = 0,
}
impl From<Rtcenbl> for bool {
    #[inline(always)]
    fn from(variant: Rtcenbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEnbl` reader - RTC enable"]
pub type RtcenblR = crate::BitReader<Rtcenbl>;
impl RtcenblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcenbl {
        match self.bits {
            true => Rtcenbl::Enable,
            false => Rtcenbl::Disable,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtcenbl::Enable
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rtcenbl::Disable
    }
}
#[doc = "Field `RTCEnbl` writer - RTC enable"]
pub type RtcenblW<'a, REG> = crate::BitWriter<'a, REG, Rtcenbl>;
impl<'a, REG> RtcenblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcenbl::Enable)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcenbl::Disable)
    }
}
#[doc = "RTC Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtclock {
    #[doc = "1: enable update"]
    EnableUpdate = 1,
    #[doc = "0: disable update"]
    DisableUpdate = 0,
}
impl From<Rtclock> for bool {
    #[inline(always)]
    fn from(variant: Rtclock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCLock` reader - RTC Lock"]
pub type RtclockR = crate::BitReader<Rtclock>;
impl RtclockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtclock {
        match self.bits {
            true => Rtclock::EnableUpdate,
            false => Rtclock::DisableUpdate,
        }
    }
    #[doc = "enable update"]
    #[inline(always)]
    pub fn is_enable_update(&self) -> bool {
        *self == Rtclock::EnableUpdate
    }
    #[doc = "disable update"]
    #[inline(always)]
    pub fn is_disable_update(&self) -> bool {
        *self == Rtclock::DisableUpdate
    }
}
#[doc = "Field `RTCLock` writer - RTC Lock"]
pub type RtclockW<'a, REG> = crate::BitWriter<'a, REG, Rtclock>;
impl<'a, REG> RtclockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable update"]
    #[inline(always)]
    pub fn enable_update(self) -> &'a mut crate::W<REG> {
        self.variant(Rtclock::EnableUpdate)
    }
    #[doc = "disable update"]
    #[inline(always)]
    pub fn disable_update(self) -> &'a mut crate::W<REG> {
        self.variant(Rtclock::DisableUpdate)
    }
}
#[doc = "Alarm mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlarmModeSel {
    #[doc = "1: Combination mode, alarm is issued when all enabled alarm were met."]
    CombinationModeAlarmIsIssuedWhenAllEnabledAlarmWereMet = 1,
    #[doc = "0: Individual mode, alarm is issued depending on which alarm was set."]
    IndividualModeAlarmIsIssuedDependingOnWhichAlarmWasSet = 0,
}
impl From<AlarmModeSel> for bool {
    #[inline(always)]
    fn from(variant: AlarmModeSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AlarmModeSel` reader - Alarm mode selection"]
pub type AlarmModeSelR = crate::BitReader<AlarmModeSel>;
impl AlarmModeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlarmModeSel {
        match self.bits {
            true => AlarmModeSel::CombinationModeAlarmIsIssuedWhenAllEnabledAlarmWereMet,
            false => AlarmModeSel::IndividualModeAlarmIsIssuedDependingOnWhichAlarmWasSet,
        }
    }
    #[doc = "Combination mode, alarm is issued when all enabled alarm were met."]
    #[inline(always)]
    pub fn is_combination_mode_alarm_is_issued_when_all_enabled_alarm_were_met(&self) -> bool {
        *self == AlarmModeSel::CombinationModeAlarmIsIssuedWhenAllEnabledAlarmWereMet
    }
    #[doc = "Individual mode, alarm is issued depending on which alarm was set."]
    #[inline(always)]
    pub fn is_individual_mode_alarm_is_issued_depending_on_which_alarm_was_set(&self) -> bool {
        *self == AlarmModeSel::IndividualModeAlarmIsIssuedDependingOnWhichAlarmWasSet
    }
}
#[doc = "Field `AlarmModeSel` writer - Alarm mode selection"]
pub type AlarmModeSelW<'a, REG> = crate::BitWriter<'a, REG, AlarmModeSel>;
impl<'a, REG> AlarmModeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Combination mode, alarm is issued when all enabled alarm were met."]
    #[inline(always)]
    pub fn combination_mode_alarm_is_issued_when_all_enabled_alarm_were_met(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(AlarmModeSel::CombinationModeAlarmIsIssuedWhenAllEnabledAlarmWereMet)
    }
    #[doc = "Individual mode, alarm is issued depending on which alarm was set."]
    #[inline(always)]
    pub fn individual_mode_alarm_is_issued_depending_on_which_alarm_was_set(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(AlarmModeSel::IndividualModeAlarmIsIssuedDependingOnWhichAlarmWasSet)
    }
}
#[doc = "Enable second alarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblSecondAlarm {
    #[doc = "1: enable"]
    Enable = 1,
    #[doc = "0: disable"]
    Disable = 0,
}
impl From<EnblSecondAlarm> for bool {
    #[inline(always)]
    fn from(variant: EnblSecondAlarm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblSecondAlarm` reader - Enable second alarm"]
pub type EnblSecondAlarmR = crate::BitReader<EnblSecondAlarm>;
impl EnblSecondAlarmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblSecondAlarm {
        match self.bits {
            true => EnblSecondAlarm::Enable,
            false => EnblSecondAlarm::Disable,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnblSecondAlarm::Enable
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblSecondAlarm::Disable
    }
}
#[doc = "Field `EnblSecondAlarm` writer - Enable second alarm"]
pub type EnblSecondAlarmW<'a, REG> = crate::BitWriter<'a, REG, EnblSecondAlarm>;
impl<'a, REG> EnblSecondAlarmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblSecondAlarm::Enable)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblSecondAlarm::Disable)
    }
}
#[doc = "Enable minute alarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblMinuteAlarm {
    #[doc = "1: enable"]
    Enable = 1,
    #[doc = "0: disable"]
    Disable = 0,
}
impl From<EnblMinuteAlarm> for bool {
    #[inline(always)]
    fn from(variant: EnblMinuteAlarm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblMinuteAlarm` reader - Enable minute alarm"]
pub type EnblMinuteAlarmR = crate::BitReader<EnblMinuteAlarm>;
impl EnblMinuteAlarmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblMinuteAlarm {
        match self.bits {
            true => EnblMinuteAlarm::Enable,
            false => EnblMinuteAlarm::Disable,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnblMinuteAlarm::Enable
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblMinuteAlarm::Disable
    }
}
#[doc = "Field `EnblMinuteAlarm` writer - Enable minute alarm"]
pub type EnblMinuteAlarmW<'a, REG> = crate::BitWriter<'a, REG, EnblMinuteAlarm>;
impl<'a, REG> EnblMinuteAlarmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblMinuteAlarm::Enable)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblMinuteAlarm::Disable)
    }
}
#[doc = "Enable hour alarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblHourAlarm {
    #[doc = "1: enable"]
    Enable = 1,
    #[doc = "0: disable"]
    Disable = 0,
}
impl From<EnblHourAlarm> for bool {
    #[inline(always)]
    fn from(variant: EnblHourAlarm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblHourAlarm` reader - Enable hour alarm"]
pub type EnblHourAlarmR = crate::BitReader<EnblHourAlarm>;
impl EnblHourAlarmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblHourAlarm {
        match self.bits {
            true => EnblHourAlarm::Enable,
            false => EnblHourAlarm::Disable,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnblHourAlarm::Enable
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblHourAlarm::Disable
    }
}
#[doc = "Field `EnblHourAlarm` writer - Enable hour alarm"]
pub type EnblHourAlarmW<'a, REG> = crate::BitWriter<'a, REG, EnblHourAlarm>;
impl<'a, REG> EnblHourAlarmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblHourAlarm::Enable)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblHourAlarm::Disable)
    }
}
#[doc = "Enable day alarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblDayAlarm {
    #[doc = "1: enable"]
    Enable = 1,
    #[doc = "0: disable"]
    Disable = 0,
}
impl From<EnblDayAlarm> for bool {
    #[inline(always)]
    fn from(variant: EnblDayAlarm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblDayAlarm` reader - Enable day alarm"]
pub type EnblDayAlarmR = crate::BitReader<EnblDayAlarm>;
impl EnblDayAlarmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblDayAlarm {
        match self.bits {
            true => EnblDayAlarm::Enable,
            false => EnblDayAlarm::Disable,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnblDayAlarm::Enable
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblDayAlarm::Disable
    }
}
#[doc = "Field `EnblDayAlarm` writer - Enable day alarm"]
pub type EnblDayAlarmW<'a, REG> = crate::BitWriter<'a, REG, EnblDayAlarm>;
impl<'a, REG> EnblDayAlarmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblDayAlarm::Enable)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblDayAlarm::Disable)
    }
}
#[doc = "Enable second interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblSecondInt {
    #[doc = "1: enable"]
    Enable = 1,
    #[doc = "0: disable"]
    Disable = 0,
}
impl From<EnblSecondInt> for bool {
    #[inline(always)]
    fn from(variant: EnblSecondInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblSecondINT` reader - Enable second interrupt"]
pub type EnblSecondIntR = crate::BitReader<EnblSecondInt>;
impl EnblSecondIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblSecondInt {
        match self.bits {
            true => EnblSecondInt::Enable,
            false => EnblSecondInt::Disable,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnblSecondInt::Enable
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblSecondInt::Disable
    }
}
#[doc = "Field `EnblSecondINT` writer - Enable second interrupt"]
pub type EnblSecondIntW<'a, REG> = crate::BitWriter<'a, REG, EnblSecondInt>;
impl<'a, REG> EnblSecondIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblSecondInt::Enable)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblSecondInt::Disable)
    }
}
#[doc = "Enable wakeup alarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblWakeupAlarm {
    #[doc = "1: enable"]
    Enable = 1,
    #[doc = "0: disable"]
    Disable = 0,
}
impl From<EnblWakeupAlarm> for bool {
    #[inline(always)]
    fn from(variant: EnblWakeupAlarm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblWakeupAlarm` reader - Enable wakeup alarm"]
pub type EnblWakeupAlarmR = crate::BitReader<EnblWakeupAlarm>;
impl EnblWakeupAlarmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblWakeupAlarm {
        match self.bits {
            true => EnblWakeupAlarm::Enable,
            false => EnblWakeupAlarm::Disable,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnblWakeupAlarm::Enable
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblWakeupAlarm::Disable
    }
}
#[doc = "Field `EnblWakeupAlarm` writer - Enable wakeup alarm"]
pub type EnblWakeupAlarmW<'a, REG> = crate::BitWriter<'a, REG, EnblWakeupAlarm>;
impl<'a, REG> EnblWakeupAlarmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblWakeupAlarm::Enable)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblWakeupAlarm::Disable)
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - RTC enable"]
    #[inline(always)]
    pub fn rtcenbl(&self) -> RtcenblR {
        RtcenblR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Lock"]
    #[inline(always)]
    pub fn rtclock(&self) -> RtclockR {
        RtclockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alarm mode selection"]
    #[inline(always)]
    pub fn alarm_mode_sel(&self) -> AlarmModeSelR {
        AlarmModeSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable second alarm"]
    #[inline(always)]
    pub fn enbl_second_alarm(&self) -> EnblSecondAlarmR {
        EnblSecondAlarmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable minute alarm"]
    #[inline(always)]
    pub fn enbl_minute_alarm(&self) -> EnblMinuteAlarmR {
        EnblMinuteAlarmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable hour alarm"]
    #[inline(always)]
    pub fn enbl_hour_alarm(&self) -> EnblHourAlarmR {
        EnblHourAlarmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable day alarm"]
    #[inline(always)]
    pub fn enbl_day_alarm(&self) -> EnblDayAlarmR {
        EnblDayAlarmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable second interrupt"]
    #[inline(always)]
    pub fn enbl_second_int(&self) -> EnblSecondIntR {
        EnblSecondIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable wakeup alarm"]
    #[inline(always)]
    pub fn enbl_wakeup_alarm(&self) -> EnblWakeupAlarmR {
        EnblWakeupAlarmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - RTC enable"]
    #[inline(always)]
    pub fn rtcenbl(&mut self) -> RtcenblW<Rtc010Spec> {
        RtcenblW::new(self, 0)
    }
    #[doc = "Bit 1 - RTC Lock"]
    #[inline(always)]
    pub fn rtclock(&mut self) -> RtclockW<Rtc010Spec> {
        RtclockW::new(self, 1)
    }
    #[doc = "Bit 2 - Alarm mode selection"]
    #[inline(always)]
    pub fn alarm_mode_sel(&mut self) -> AlarmModeSelW<Rtc010Spec> {
        AlarmModeSelW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable second alarm"]
    #[inline(always)]
    pub fn enbl_second_alarm(&mut self) -> EnblSecondAlarmW<Rtc010Spec> {
        EnblSecondAlarmW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable minute alarm"]
    #[inline(always)]
    pub fn enbl_minute_alarm(&mut self) -> EnblMinuteAlarmW<Rtc010Spec> {
        EnblMinuteAlarmW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable hour alarm"]
    #[inline(always)]
    pub fn enbl_hour_alarm(&mut self) -> EnblHourAlarmW<Rtc010Spec> {
        EnblHourAlarmW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable day alarm"]
    #[inline(always)]
    pub fn enbl_day_alarm(&mut self) -> EnblDayAlarmW<Rtc010Spec> {
        EnblDayAlarmW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable second interrupt"]
    #[inline(always)]
    pub fn enbl_second_int(&mut self) -> EnblSecondIntW<Rtc010Spec> {
        EnblSecondIntW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable wakeup alarm"]
    #[inline(always)]
    pub fn enbl_wakeup_alarm(&mut self) -> EnblWakeupAlarmW<Rtc010Spec> {
        EnblWakeupAlarmW::new(self, 8)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtc010Spec;
impl crate::RegisterSpec for Rtc010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc010::R`](R) reader structure"]
impl crate::Readable for Rtc010Spec {}
#[doc = "`write(|w| ..)` method takes [`rtc010::W`](W) writer structure"]
impl crate::Writable for Rtc010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC010 to value 0"]
impl crate::Resettable for Rtc010Spec {}
