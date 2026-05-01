#[doc = "Register `RTC004` reader"]
pub type R = crate::R<Rtc004Spec>;
#[doc = "Register `RTC004` writer"]
pub type W = crate::W<Rtc004Spec>;
#[doc = "Field `MonCntStatusOfMonthCounter` reader - MonCnt: Status of Month Counter."]
pub type MonCntStatusOfMonthCounterR = crate::FieldReader;
#[doc = "Field `MonCntStatusOfMonthCounter` writer - MonCnt: Status of Month Counter."]
pub type MonCntStatusOfMonthCounterW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "Field `YearCntStatusOfYearCounter` reader - YearCnt: Status of Year Counter."]
pub type YearCntStatusOfYearCounterR = crate::FieldReader;
#[doc = "Field `YearCntStatusOfYearCounter` writer - YearCnt: Status of Year Counter."]
pub type YearCntStatusOfYearCounterW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::BitReader;
#[doc = "Field `CentCntStatusOfCenturyCounter` reader - CentCnt: Status of Century Counter."]
pub type CentCntStatusOfCenturyCounterR = crate::FieldReader;
#[doc = "Field `CentCntStatusOfCenturyCounter` writer - CentCnt: Status of Century Counter."]
pub type CentCntStatusOfCenturyCounterW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - MonCnt: Status of Month Counter."]
    #[inline(always)]
    pub fn mon_cnt_status_of_month_counter(&self) -> MonCntStatusOfMonthCounterR {
        MonCntStatusOfMonthCounterR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - YearCnt: Status of Year Counter."]
    #[inline(always)]
    pub fn year_cnt_status_of_year_counter(&self) -> YearCntStatusOfYearCounterR {
        YearCntStatusOfYearCounterR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - CentCnt: Status of Century Counter."]
    #[inline(always)]
    pub fn cent_cnt_status_of_century_counter(&self) -> CentCntStatusOfCenturyCounterR {
        CentCntStatusOfCenturyCounterR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - MonCnt: Status of Month Counter."]
    #[inline(always)]
    pub fn mon_cnt_status_of_month_counter(&mut self) -> MonCntStatusOfMonthCounterW<Rtc004Spec> {
        MonCntStatusOfMonthCounterW::new(self, 0)
    }
    #[doc = "Bits 8:14 - YearCnt: Status of Year Counter."]
    #[inline(always)]
    pub fn year_cnt_status_of_year_counter(&mut self) -> YearCntStatusOfYearCounterW<Rtc004Spec> {
        YearCntStatusOfYearCounterW::new(self, 8)
    }
    #[doc = "Bits 16:20 - CentCnt: Status of Century Counter."]
    #[inline(always)]
    pub fn cent_cnt_status_of_century_counter(
        &mut self,
    ) -> CentCntStatusOfCenturyCounterW<Rtc004Spec> {
        CentCntStatusOfCenturyCounterW::new(self, 16)
    }
}
#[doc = "Counter Status Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtc004Spec;
impl crate::RegisterSpec for Rtc004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc004::R`](R) reader structure"]
impl crate::Readable for Rtc004Spec {}
#[doc = "`write(|w| ..)` method takes [`rtc004::W`](W) writer structure"]
impl crate::Writable for Rtc004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC004 to value 0"]
impl crate::Resettable for Rtc004Spec {}
