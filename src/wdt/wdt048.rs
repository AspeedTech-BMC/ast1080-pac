#[doc = "Register `WDT048` reader"]
pub type R = crate::R<Wdt048Spec>;
#[doc = "Register `WDT048` writer"]
pub type W = crate::W<Wdt048Spec>;
#[doc = "Field `DisFullWatchdogFn` reader - Disable full watchdog function"]
pub type DisFullWatchdogFnR = crate::BitReader;
#[doc = "Field `DisFullWatchdogFn` writer - Disable full watchdog function"]
pub type DisFullWatchdogFnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWatchdogSOCRstMode` reader - Disable watchdog SOC reset mode"]
pub type DisWatchdogSocrstModeR = crate::BitReader;
#[doc = "Field `DisWatchdogSOCRstMode` writer - Disable watchdog SOC reset mode"]
pub type DisWatchdogSocrstModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWatchdogFullRstMode` reader - Disable watchdog Full reset mode"]
pub type DisWatchdogFullRstModeR = crate::BitReader;
#[doc = "Field `DisWatchdogFullRstMode` writer - Disable watchdog Full reset mode"]
pub type DisWatchdogFullRstModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWatchdogCPUFMCRstMode` reader - Disable watchdog CPU/FMC reset mode"]
pub type DisWatchdogCpufmcrstModeR = crate::BitReader;
#[doc = "Field `DisWatchdogCPUFMCRstMode` writer - Disable watchdog CPU/FMC reset mode"]
pub type DisWatchdogCpufmcrstModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWatchdogINTGeneration` reader - Disable watchdog interrupt generation"]
pub type DisWatchdogIntgenerationR = crate::BitReader;
#[doc = "Field `DisWatchdogINTGeneration` writer - Disable watchdog interrupt generation"]
pub type DisWatchdogIntgenerationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWatchdogSwModeRst` reader - Disable watchdog Software mode reset"]
pub type DisWatchdogSwModeRstR = crate::BitReader;
#[doc = "Field `DisWatchdogSwModeRst` writer - Disable watchdog Software mode reset"]
pub type DisWatchdogSwModeRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable full watchdog function"]
    #[inline(always)]
    pub fn dis_full_watchdog_fn(&self) -> DisFullWatchdogFnR {
        DisFullWatchdogFnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable watchdog SOC reset mode"]
    #[inline(always)]
    pub fn dis_watchdog_socrst_mode(&self) -> DisWatchdogSocrstModeR {
        DisWatchdogSocrstModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable watchdog Full reset mode"]
    #[inline(always)]
    pub fn dis_watchdog_full_rst_mode(&self) -> DisWatchdogFullRstModeR {
        DisWatchdogFullRstModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable watchdog CPU/FMC reset mode"]
    #[inline(always)]
    pub fn dis_watchdog_cpufmcrst_mode(&self) -> DisWatchdogCpufmcrstModeR {
        DisWatchdogCpufmcrstModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable watchdog interrupt generation"]
    #[inline(always)]
    pub fn dis_watchdog_intgeneration(&self) -> DisWatchdogIntgenerationR {
        DisWatchdogIntgenerationR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable watchdog Software mode reset"]
    #[inline(always)]
    pub fn dis_watchdog_sw_mode_rst(&self) -> DisWatchdogSwModeRstR {
        DisWatchdogSwModeRstR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable full watchdog function"]
    #[inline(always)]
    pub fn dis_full_watchdog_fn(&mut self) -> DisFullWatchdogFnW<Wdt048Spec> {
        DisFullWatchdogFnW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable watchdog SOC reset mode"]
    #[inline(always)]
    pub fn dis_watchdog_socrst_mode(&mut self) -> DisWatchdogSocrstModeW<Wdt048Spec> {
        DisWatchdogSocrstModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable watchdog Full reset mode"]
    #[inline(always)]
    pub fn dis_watchdog_full_rst_mode(&mut self) -> DisWatchdogFullRstModeW<Wdt048Spec> {
        DisWatchdogFullRstModeW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable watchdog CPU/FMC reset mode"]
    #[inline(always)]
    pub fn dis_watchdog_cpufmcrst_mode(&mut self) -> DisWatchdogCpufmcrstModeW<Wdt048Spec> {
        DisWatchdogCpufmcrstModeW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable watchdog interrupt generation"]
    #[inline(always)]
    pub fn dis_watchdog_intgeneration(&mut self) -> DisWatchdogIntgenerationW<Wdt048Spec> {
        DisWatchdogIntgenerationW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable watchdog Software mode reset"]
    #[inline(always)]
    pub fn dis_watchdog_sw_mode_rst(&mut self) -> DisWatchdogSwModeRstW<Wdt048Spec> {
        DisWatchdogSwModeRstW::new(self, 5)
    }
}
#[doc = "WDTn Funtion Disable Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt048Spec;
impl crate::RegisterSpec for Wdt048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt048::R`](R) reader structure"]
impl crate::Readable for Wdt048Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt048::W`](W) writer structure"]
impl crate::Writable for Wdt048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT048 to value 0"]
impl crate::Resettable for Wdt048Spec {}
