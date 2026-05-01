#[doc = "Register `RTC008` reader"]
pub type R = crate::R<Rtc008Spec>;
#[doc = "Register `RTC008` writer"]
pub type W = crate::W<Rtc008Spec>;
#[doc = "Field `SecondAlarm` reader - Second alarm"]
pub type SecondAlarmR = crate::FieldReader;
#[doc = "Field `SecondAlarm` writer - Second alarm"]
pub type SecondAlarmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MinuteAlarm` reader - Minute alarm"]
pub type MinuteAlarmR = crate::FieldReader;
#[doc = "Field `MinuteAlarm` writer - Minute alarm"]
pub type MinuteAlarmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HourAlarm` reader - Hour alarm"]
pub type HourAlarmR = crate::FieldReader;
#[doc = "Field `HourAlarm` writer - Hour alarm"]
pub type HourAlarmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DayAlarm` reader - Day alarm"]
pub type DayAlarmR = crate::FieldReader;
#[doc = "Field `DayAlarm` writer - Day alarm"]
pub type DayAlarmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - Second alarm"]
    #[inline(always)]
    pub fn second_alarm(&self) -> SecondAlarmR {
        SecondAlarmR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minute alarm"]
    #[inline(always)]
    pub fn minute_alarm(&self) -> MinuteAlarmR {
        MinuteAlarmR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hour alarm"]
    #[inline(always)]
    pub fn hour_alarm(&self) -> HourAlarmR {
        HourAlarmR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Day alarm"]
    #[inline(always)]
    pub fn day_alarm(&self) -> DayAlarmR {
        DayAlarmR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Second alarm"]
    #[inline(always)]
    pub fn second_alarm(&mut self) -> SecondAlarmW<Rtc008Spec> {
        SecondAlarmW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Minute alarm"]
    #[inline(always)]
    pub fn minute_alarm(&mut self) -> MinuteAlarmW<Rtc008Spec> {
        MinuteAlarmW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Hour alarm"]
    #[inline(always)]
    pub fn hour_alarm(&mut self) -> HourAlarmW<Rtc008Spec> {
        HourAlarmW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Day alarm"]
    #[inline(always)]
    pub fn day_alarm(&mut self) -> DayAlarmW<Rtc008Spec> {
        DayAlarmW::new(self, 24)
    }
}
#[doc = "Clock Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtc008Spec;
impl crate::RegisterSpec for Rtc008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc008::R`](R) reader structure"]
impl crate::Readable for Rtc008Spec {}
#[doc = "`write(|w| ..)` method takes [`rtc008::W`](W) writer structure"]
impl crate::Writable for Rtc008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC008 to value 0"]
impl crate::Resettable for Rtc008Spec {}
