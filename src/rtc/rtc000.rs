#[doc = "Register `RTC000` reader"]
pub type R = crate::R<Rtc000Spec>;
#[doc = "Register `RTC000` writer"]
pub type W = crate::W<Rtc000Spec>;
#[doc = "Field `SecCntStatusOfSecondCounter` reader - SecCnt: Status of Second Counter."]
pub type SecCntStatusOfSecondCounterR = crate::FieldReader;
#[doc = "Field `SecCntStatusOfSecondCounter` writer - SecCnt: Status of Second Counter."]
pub type SecCntStatusOfSecondCounterW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved03` reader - Reserved (0)"]
pub type Reserved03R = crate::FieldReader;
#[doc = "Field `MinuCntStatusOfMinuteCounter` reader - MinuCnt: Status of Minute Counter."]
pub type MinuCntStatusOfMinuteCounterR = crate::FieldReader;
#[doc = "Field `MinuCntStatusOfMinuteCounter` writer - MinuCnt: Status of Minute Counter."]
pub type MinuCntStatusOfMinuteCounterW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "Field `HourCntStatusOfHourCounter` reader - HourCnt: Status of Hour Counter."]
pub type HourCntStatusOfHourCounterR = crate::FieldReader;
#[doc = "Field `HourCntStatusOfHourCounter` writer - HourCnt: Status of Hour Counter."]
pub type HourCntStatusOfHourCounterW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `DayCntStatusOfDayCounter` reader - DayCnt: Status of Day Counter."]
pub type DayCntStatusOfDayCounterR = crate::FieldReader;
#[doc = "Field `DayCntStatusOfDayCounter` writer - DayCnt: Status of Day Counter."]
pub type DayCntStatusOfDayCounterW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - SecCnt: Status of Second Counter."]
    #[inline(always)]
    pub fn sec_cnt_status_of_second_counter(&self) -> SecCntStatusOfSecondCounterR {
        SecCntStatusOfSecondCounterR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved03(&self) -> Reserved03R {
        Reserved03R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - MinuCnt: Status of Minute Counter."]
    #[inline(always)]
    pub fn minu_cnt_status_of_minute_counter(&self) -> MinuCntStatusOfMinuteCounterR {
        MinuCntStatusOfMinuteCounterR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20 - HourCnt: Status of Hour Counter."]
    #[inline(always)]
    pub fn hour_cnt_status_of_hour_counter(&self) -> HourCntStatusOfHourCounterR {
        HourCntStatusOfHourCounterR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - DayCnt: Status of Day Counter."]
    #[inline(always)]
    pub fn day_cnt_status_of_day_counter(&self) -> DayCntStatusOfDayCounterR {
        DayCntStatusOfDayCounterR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - SecCnt: Status of Second Counter."]
    #[inline(always)]
    pub fn sec_cnt_status_of_second_counter(&mut self) -> SecCntStatusOfSecondCounterW<Rtc000Spec> {
        SecCntStatusOfSecondCounterW::new(self, 0)
    }
    #[doc = "Bits 8:13 - MinuCnt: Status of Minute Counter."]
    #[inline(always)]
    pub fn minu_cnt_status_of_minute_counter(
        &mut self,
    ) -> MinuCntStatusOfMinuteCounterW<Rtc000Spec> {
        MinuCntStatusOfMinuteCounterW::new(self, 8)
    }
    #[doc = "Bits 16:20 - HourCnt: Status of Hour Counter."]
    #[inline(always)]
    pub fn hour_cnt_status_of_hour_counter(&mut self) -> HourCntStatusOfHourCounterW<Rtc000Spec> {
        HourCntStatusOfHourCounterW::new(self, 16)
    }
    #[doc = "Bits 24:28 - DayCnt: Status of Day Counter."]
    #[inline(always)]
    pub fn day_cnt_status_of_day_counter(&mut self) -> DayCntStatusOfDayCounterW<Rtc000Spec> {
        DayCntStatusOfDayCounterW::new(self, 24)
    }
}
#[doc = "Counter Status Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtc000Spec;
impl crate::RegisterSpec for Rtc000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc000::R`](R) reader structure"]
impl crate::Readable for Rtc000Spec {}
#[doc = "`write(|w| ..)` method takes [`rtc000::W`](W) writer structure"]
impl crate::Writable for Rtc000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC000 to value 0"]
impl crate::Resettable for Rtc000Spec {}
