#[doc = "Register `RTC014` reader"]
pub type R = crate::R<Rtc014Spec>;
#[doc = "Register `RTC014` writer"]
pub type W = crate::W<Rtc014Spec>;
#[doc = "Second or Combination alarm status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecondOrCombinationAlarmSts {
    #[doc = "1: Alarmed"]
    Alarmed = 1,
    #[doc = "0: Idle"]
    Idle = 0,
}
impl From<SecondOrCombinationAlarmSts> for bool {
    #[inline(always)]
    fn from(variant: SecondOrCombinationAlarmSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecondOrCombinationAlarmSts` reader - Second or Combination alarm status"]
pub type SecondOrCombinationAlarmStsR = crate::BitReader<SecondOrCombinationAlarmSts>;
impl SecondOrCombinationAlarmStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecondOrCombinationAlarmSts {
        match self.bits {
            true => SecondOrCombinationAlarmSts::Alarmed,
            false => SecondOrCombinationAlarmSts::Idle,
        }
    }
    #[doc = "Alarmed"]
    #[inline(always)]
    pub fn is_alarmed(&self) -> bool {
        *self == SecondOrCombinationAlarmSts::Alarmed
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SecondOrCombinationAlarmSts::Idle
    }
}
#[doc = "Field `SecondOrCombinationAlarmSts` writer - Second or Combination alarm status"]
pub type SecondOrCombinationAlarmStsW<'a, REG> =
    crate::BitWriter<'a, REG, SecondOrCombinationAlarmSts>;
impl<'a, REG> SecondOrCombinationAlarmStsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarmed"]
    #[inline(always)]
    pub fn alarmed(self) -> &'a mut crate::W<REG> {
        self.variant(SecondOrCombinationAlarmSts::Alarmed)
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(SecondOrCombinationAlarmSts::Idle)
    }
}
#[doc = "Minute alarm status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MinuteAlarmSts {
    #[doc = "1: Alarmed"]
    Alarmed = 1,
    #[doc = "0: Idle"]
    Idle = 0,
}
impl From<MinuteAlarmSts> for bool {
    #[inline(always)]
    fn from(variant: MinuteAlarmSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MinuteAlarmSts` reader - Minute alarm status"]
pub type MinuteAlarmStsR = crate::BitReader<MinuteAlarmSts>;
impl MinuteAlarmStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MinuteAlarmSts {
        match self.bits {
            true => MinuteAlarmSts::Alarmed,
            false => MinuteAlarmSts::Idle,
        }
    }
    #[doc = "Alarmed"]
    #[inline(always)]
    pub fn is_alarmed(&self) -> bool {
        *self == MinuteAlarmSts::Alarmed
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == MinuteAlarmSts::Idle
    }
}
#[doc = "Field `MinuteAlarmSts` writer - Minute alarm status"]
pub type MinuteAlarmStsW<'a, REG> = crate::BitWriter<'a, REG, MinuteAlarmSts>;
impl<'a, REG> MinuteAlarmStsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarmed"]
    #[inline(always)]
    pub fn alarmed(self) -> &'a mut crate::W<REG> {
        self.variant(MinuteAlarmSts::Alarmed)
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(MinuteAlarmSts::Idle)
    }
}
#[doc = "Hour alarm status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HourAlarmSts {
    #[doc = "1: Alarmed"]
    Alarmed = 1,
    #[doc = "0: Idle"]
    Idle = 0,
}
impl From<HourAlarmSts> for bool {
    #[inline(always)]
    fn from(variant: HourAlarmSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HourAlarmSts` reader - Hour alarm status"]
pub type HourAlarmStsR = crate::BitReader<HourAlarmSts>;
impl HourAlarmStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HourAlarmSts {
        match self.bits {
            true => HourAlarmSts::Alarmed,
            false => HourAlarmSts::Idle,
        }
    }
    #[doc = "Alarmed"]
    #[inline(always)]
    pub fn is_alarmed(&self) -> bool {
        *self == HourAlarmSts::Alarmed
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == HourAlarmSts::Idle
    }
}
#[doc = "Field `HourAlarmSts` writer - Hour alarm status"]
pub type HourAlarmStsW<'a, REG> = crate::BitWriter<'a, REG, HourAlarmSts>;
impl<'a, REG> HourAlarmStsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarmed"]
    #[inline(always)]
    pub fn alarmed(self) -> &'a mut crate::W<REG> {
        self.variant(HourAlarmSts::Alarmed)
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(HourAlarmSts::Idle)
    }
}
#[doc = "Day alarm status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DayAlarmSts {
    #[doc = "1: Alarmed"]
    Alarmed = 1,
    #[doc = "0: Idle"]
    Idle = 0,
}
impl From<DayAlarmSts> for bool {
    #[inline(always)]
    fn from(variant: DayAlarmSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DayAlarmSts` reader - Day alarm status"]
pub type DayAlarmStsR = crate::BitReader<DayAlarmSts>;
impl DayAlarmStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DayAlarmSts {
        match self.bits {
            true => DayAlarmSts::Alarmed,
            false => DayAlarmSts::Idle,
        }
    }
    #[doc = "Alarmed"]
    #[inline(always)]
    pub fn is_alarmed(&self) -> bool {
        *self == DayAlarmSts::Alarmed
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == DayAlarmSts::Idle
    }
}
#[doc = "Field `DayAlarmSts` writer - Day alarm status"]
pub type DayAlarmStsW<'a, REG> = crate::BitWriter<'a, REG, DayAlarmSts>;
impl<'a, REG> DayAlarmStsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarmed"]
    #[inline(always)]
    pub fn alarmed(self) -> &'a mut crate::W<REG> {
        self.variant(DayAlarmSts::Alarmed)
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(DayAlarmSts::Idle)
    }
}
#[doc = "Second interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecondIntsts {
    #[doc = "1: Alarmed"]
    Alarmed = 1,
    #[doc = "0: Idle"]
    Idle = 0,
}
impl From<SecondIntsts> for bool {
    #[inline(always)]
    fn from(variant: SecondIntsts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecondINTSts` reader - Second interrupt status"]
pub type SecondIntstsR = crate::BitReader<SecondIntsts>;
impl SecondIntstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecondIntsts {
        match self.bits {
            true => SecondIntsts::Alarmed,
            false => SecondIntsts::Idle,
        }
    }
    #[doc = "Alarmed"]
    #[inline(always)]
    pub fn is_alarmed(&self) -> bool {
        *self == SecondIntsts::Alarmed
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SecondIntsts::Idle
    }
}
#[doc = "Field `SecondINTSts` writer - Second interrupt status"]
pub type SecondIntstsW<'a, REG> = crate::BitWriter<'a, REG, SecondIntsts>;
impl<'a, REG> SecondIntstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarmed"]
    #[inline(always)]
    pub fn alarmed(self) -> &'a mut crate::W<REG> {
        self.variant(SecondIntsts::Alarmed)
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(SecondIntsts::Idle)
    }
}
#[doc = "Wakeup alarm status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupAlarmSts {
    #[doc = "1: Alarmed"]
    Alarmed = 1,
    #[doc = "0: Idle"]
    Idle = 0,
}
impl From<WakeupAlarmSts> for bool {
    #[inline(always)]
    fn from(variant: WakeupAlarmSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WakeupAlarmSts` reader - Wakeup alarm status"]
pub type WakeupAlarmStsR = crate::BitReader<WakeupAlarmSts>;
impl WakeupAlarmStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupAlarmSts {
        match self.bits {
            true => WakeupAlarmSts::Alarmed,
            false => WakeupAlarmSts::Idle,
        }
    }
    #[doc = "Alarmed"]
    #[inline(always)]
    pub fn is_alarmed(&self) -> bool {
        *self == WakeupAlarmSts::Alarmed
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WakeupAlarmSts::Idle
    }
}
#[doc = "Field `WakeupAlarmSts` writer - Wakeup alarm status"]
pub type WakeupAlarmStsW<'a, REG> = crate::BitWriter<'a, REG, WakeupAlarmSts>;
impl<'a, REG> WakeupAlarmStsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarmed"]
    #[inline(always)]
    pub fn alarmed(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupAlarmSts::Alarmed)
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupAlarmSts::Idle)
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Second or Combination alarm status"]
    #[inline(always)]
    pub fn second_or_combination_alarm_sts(&self) -> SecondOrCombinationAlarmStsR {
        SecondOrCombinationAlarmStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Minute alarm status"]
    #[inline(always)]
    pub fn minute_alarm_sts(&self) -> MinuteAlarmStsR {
        MinuteAlarmStsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hour alarm status"]
    #[inline(always)]
    pub fn hour_alarm_sts(&self) -> HourAlarmStsR {
        HourAlarmStsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Day alarm status"]
    #[inline(always)]
    pub fn day_alarm_sts(&self) -> DayAlarmStsR {
        DayAlarmStsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Second interrupt status"]
    #[inline(always)]
    pub fn second_intsts(&self) -> SecondIntstsR {
        SecondIntstsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup alarm status"]
    #[inline(always)]
    pub fn wakeup_alarm_sts(&self) -> WakeupAlarmStsR {
        WakeupAlarmStsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Second or Combination alarm status"]
    #[inline(always)]
    pub fn second_or_combination_alarm_sts(&mut self) -> SecondOrCombinationAlarmStsW<Rtc014Spec> {
        SecondOrCombinationAlarmStsW::new(self, 0)
    }
    #[doc = "Bit 1 - Minute alarm status"]
    #[inline(always)]
    pub fn minute_alarm_sts(&mut self) -> MinuteAlarmStsW<Rtc014Spec> {
        MinuteAlarmStsW::new(self, 1)
    }
    #[doc = "Bit 2 - Hour alarm status"]
    #[inline(always)]
    pub fn hour_alarm_sts(&mut self) -> HourAlarmStsW<Rtc014Spec> {
        HourAlarmStsW::new(self, 2)
    }
    #[doc = "Bit 3 - Day alarm status"]
    #[inline(always)]
    pub fn day_alarm_sts(&mut self) -> DayAlarmStsW<Rtc014Spec> {
        DayAlarmStsW::new(self, 3)
    }
    #[doc = "Bit 4 - Second interrupt status"]
    #[inline(always)]
    pub fn second_intsts(&mut self) -> SecondIntstsW<Rtc014Spec> {
        SecondIntstsW::new(self, 4)
    }
    #[doc = "Bit 5 - Wakeup alarm status"]
    #[inline(always)]
    pub fn wakeup_alarm_sts(&mut self) -> WakeupAlarmStsW<Rtc014Spec> {
        WakeupAlarmStsW::new(self, 5)
    }
}
#[doc = "Alarm Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtc014Spec;
impl crate::RegisterSpec for Rtc014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc014::R`](R) reader structure"]
impl crate::Readable for Rtc014Spec {}
#[doc = "`write(|w| ..)` method takes [`rtc014::W`](W) writer structure"]
impl crate::Writable for Rtc014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC014 to value 0"]
impl crate::Resettable for Rtc014Spec {}
