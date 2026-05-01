#[doc = "Register `TIMER010` reader"]
pub type R = crate::R<Timer010Spec>;
#[doc = "Register `TIMER010` writer"]
pub type W = crate::W<Timer010Spec>;
#[doc = "Timer enable for Timer/Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerEnblForTimerCounter {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<TimerEnblForTimerCounter> for bool {
    #[inline(always)]
    fn from(variant: TimerEnblForTimerCounter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimerEnblForTimerCounter` reader - Timer enable for Timer/Counter"]
pub type TimerEnblForTimerCounterR = crate::BitReader<TimerEnblForTimerCounter>;
impl TimerEnblForTimerCounterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerEnblForTimerCounter {
        match self.bits {
            false => TimerEnblForTimerCounter::Disable,
            true => TimerEnblForTimerCounter::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TimerEnblForTimerCounter::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TimerEnblForTimerCounter::Enable
    }
}
#[doc = "Field `TimerEnblForTimerCounter` writer - Timer enable for Timer/Counter"]
pub type TimerEnblForTimerCounterW<'a, REG> = crate::BitWriter<'a, REG, TimerEnblForTimerCounter>;
impl<'a, REG> TimerEnblForTimerCounterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter::Enable)
    }
}
#[doc = "Clock selection for Timer/Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSelForTimerCounter {
    #[doc = "0: APB clock (PCLK)"]
    ApbClockPclk = 0,
    #[doc = "1: 1 MHz clock"]
    _1MhzClock = 1,
}
impl From<ClkSelForTimerCounter> for bool {
    #[inline(always)]
    fn from(variant: ClkSelForTimerCounter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClkSelForTimerCounter` reader - Clock selection for Timer/Counter"]
pub type ClkSelForTimerCounterR = crate::BitReader<ClkSelForTimerCounter>;
impl ClkSelForTimerCounterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSelForTimerCounter {
        match self.bits {
            false => ClkSelForTimerCounter::ApbClockPclk,
            true => ClkSelForTimerCounter::_1MhzClock,
        }
    }
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn is_apb_clock_pclk(&self) -> bool {
        *self == ClkSelForTimerCounter::ApbClockPclk
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn is_1_mhz_clock(&self) -> bool {
        *self == ClkSelForTimerCounter::_1MhzClock
    }
}
#[doc = "Field `ClkSelForTimerCounter` writer - Clock selection for Timer/Counter"]
pub type ClkSelForTimerCounterW<'a, REG> = crate::BitWriter<'a, REG, ClkSelForTimerCounter>;
impl<'a, REG> ClkSelForTimerCounterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn apb_clock_pclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter::ApbClockPclk)
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn _1_mhz_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter::_1MhzClock)
    }
}
#[doc = "Enable Overflow Interrupt for Timer/Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOverflowIntforTimerCounter {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated = 1,
}
impl From<EnblOverflowIntforTimerCounter> for bool {
    #[inline(always)]
    fn from(variant: EnblOverflowIntforTimerCounter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter` reader - Enable Overflow Interrupt for Timer/Counter"]
pub type EnblOverflowIntforTimerCounterR = crate::BitReader<EnblOverflowIntforTimerCounter>;
impl EnblOverflowIntforTimerCounterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOverflowIntforTimerCounter {
        match self . bits { false => EnblOverflowIntforTimerCounter :: Disable , true => EnblOverflowIntforTimerCounter :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated , }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblOverflowIntforTimerCounter::Disable
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn is_enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        &self,
    ) -> bool {
        * self == EnblOverflowIntforTimerCounter :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter` writer - Enable Overflow Interrupt for Timer/Counter"]
pub type EnblOverflowIntforTimerCounterW<'a, REG> =
    crate::BitWriter<'a, REG, EnblOverflowIntforTimerCounter>;
impl<'a, REG> EnblOverflowIntforTimerCounterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOverflowIntforTimerCounter::Disable)
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (EnblOverflowIntforTimerCounter :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated)
    }
}
#[doc = "Field `EnblTimerCanBeRstByWatchdog` reader - Enable Timer can be reset by watchdog"]
pub type EnblTimerCanBeRstByWatchdogR = crate::BitReader;
#[doc = "Field `EnblTimerCanBeRstByWatchdog` writer - Enable Timer can be reset by watchdog"]
pub type EnblTimerCanBeRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `TimerINTEvent` reader - Timer Interrupt Event"]
pub type TimerInteventR = crate::BitReader;
#[doc = "Field `TimerINTEvent` writer - Timer Interrupt Event"]
pub type TimerInteventW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer enable for Timer/Counter"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter(&self) -> TimerEnblForTimerCounterR {
        TimerEnblForTimerCounterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock selection for Timer/Counter"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter(&self) -> ClkSelForTimerCounterR {
        ClkSelForTimerCounterR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Overflow Interrupt for Timer/Counter"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter(&self) -> EnblOverflowIntforTimerCounterR {
        EnblOverflowIntforTimerCounterR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Timer can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer_can_be_rst_by_watchdog(&self) -> EnblTimerCanBeRstByWatchdogR {
        EnblTimerCanBeRstByWatchdogR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Timer Interrupt Event"]
    #[inline(always)]
    pub fn timer_intevent(&self) -> TimerInteventR {
        TimerInteventR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer enable for Timer/Counter"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter(&mut self) -> TimerEnblForTimerCounterW<Timer010Spec> {
        TimerEnblForTimerCounterW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock selection for Timer/Counter"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter(&mut self) -> ClkSelForTimerCounterW<Timer010Spec> {
        ClkSelForTimerCounterW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Overflow Interrupt for Timer/Counter"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter(
        &mut self,
    ) -> EnblOverflowIntforTimerCounterW<Timer010Spec> {
        EnblOverflowIntforTimerCounterW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Timer can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer_can_be_rst_by_watchdog(
        &mut self,
    ) -> EnblTimerCanBeRstByWatchdogW<Timer010Spec> {
        EnblTimerCanBeRstByWatchdogW::new(self, 3)
    }
    #[doc = "Bit 16 - Timer Interrupt Event"]
    #[inline(always)]
    pub fn timer_intevent(&mut self) -> TimerInteventW<Timer010Spec> {
        TimerInteventW::new(self, 16)
    }
}
#[doc = "Counter Control and Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer010Spec;
impl crate::RegisterSpec for Timer010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer010::R`](R) reader structure"]
impl crate::Readable for Timer010Spec {}
#[doc = "`write(|w| ..)` method takes [`timer010::W`](W) writer structure"]
impl crate::Writable for Timer010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER010 to value 0"]
impl crate::Resettable for Timer010Spec {}
