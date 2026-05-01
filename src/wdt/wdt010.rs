#[doc = "Register `WDT010` reader"]
pub type R = crate::R<Wdt010Spec>;
#[doc = "Register `WDT010` writer"]
pub type W = crate::W<Wdt010Spec>;
#[doc = "Indicate timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndicateTimeout {
    #[doc = "0: timeout never occur"]
    TimeoutNeverOccur = 0,
    #[doc = "1: timeout occur"]
    TimeoutOccur = 1,
}
impl From<IndicateTimeout> for bool {
    #[inline(always)]
    fn from(variant: IndicateTimeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IndicateTimeout` reader - Indicate timeout"]
pub type IndicateTimeoutR = crate::BitReader<IndicateTimeout>;
impl IndicateTimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IndicateTimeout {
        match self.bits {
            false => IndicateTimeout::TimeoutNeverOccur,
            true => IndicateTimeout::TimeoutOccur,
        }
    }
    #[doc = "timeout never occur"]
    #[inline(always)]
    pub fn is_timeout_never_occur(&self) -> bool {
        *self == IndicateTimeout::TimeoutNeverOccur
    }
    #[doc = "timeout occur"]
    #[inline(always)]
    pub fn is_timeout_occur(&self) -> bool {
        *self == IndicateTimeout::TimeoutOccur
    }
}
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::BitReader;
#[doc = "Field `IndicateINT` reader - Indicate interrupt"]
pub type IndicateIntR = crate::BitReader;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `WDEventCounterSOCRst` reader - Watchdog event counter : SOC reset"]
pub type WdeventCounterSocrstR = crate::FieldReader;
#[doc = "Field `WDEventCounterFullRst` reader - Watchdog event counter : Full reset"]
pub type WdeventCounterFullRstR = crate::FieldReader;
#[doc = "Field `WDEventCounterARMRst` reader - Watchdog event counter : ARM reset"]
pub type WdeventCounterArmrstR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indicate timeout"]
    #[inline(always)]
    pub fn indicate_timeout(&self) -> IndicateTimeoutR {
        IndicateTimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicate interrupt"]
    #[inline(always)]
    pub fn indicate_int(&self) -> IndicateIntR {
        IndicateIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Watchdog event counter : SOC reset"]
    #[inline(always)]
    pub fn wdevent_counter_socrst(&self) -> WdeventCounterSocrstR {
        WdeventCounterSocrstR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Watchdog event counter : Full reset"]
    #[inline(always)]
    pub fn wdevent_counter_full_rst(&self) -> WdeventCounterFullRstR {
        WdeventCounterFullRstR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Watchdog event counter : ARM reset"]
    #[inline(always)]
    pub fn wdevent_counter_armrst(&self) -> WdeventCounterArmrstR {
        WdeventCounterArmrstR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "WDTn Timeout Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt010Spec;
impl crate::RegisterSpec for Wdt010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt010::R`](R) reader structure"]
impl crate::Readable for Wdt010Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt010::W`](W) writer structure"]
impl crate::Writable for Wdt010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT010 to value 0"]
impl crate::Resettable for Wdt010Spec {}
