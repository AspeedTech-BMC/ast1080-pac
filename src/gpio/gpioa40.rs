#[doc = "Register `GPIOA40` reader"]
pub type R = crate::R<Gpioa40Spec>;
#[doc = "Register `GPIOA40` writer"]
pub type W = crate::W<Gpioa40Spec>;
#[doc = "Field `EnblGPIO048INTToINT13018` reader - Enable GPIO048 Interrupt To INT#130_18"]
pub type EnblGpio048inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO048INTToINT13018` writer - Enable GPIO048 Interrupt To INT#130_18"]
pub type EnblGpio048inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO048INTToINT13019` reader - Enable GPIO048 Interrupt To INT#130_19"]
pub type EnblGpio048inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO048INTToINT13019` writer - Enable GPIO048 Interrupt To INT#130_19"]
pub type EnblGpio048inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO048INTToINT13020` reader - Enable GPIO048 Interrupt To INT#130_20"]
pub type EnblGpio048inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO048INTToINT13020` writer - Enable GPIO048 Interrupt To INT#130_20"]
pub type EnblGpio048inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO048INTToSIO` reader - Enable GPIO048 Interrupt To SIO"]
pub type EnblGpio048inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO048INTToSIO` writer - Enable GPIO048 Interrupt To SIO"]
pub type EnblGpio048inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO048 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio048inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio048inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio048inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO048INTTargetRstTolerance` reader - GPIO048 Interrupt Target Reset Tolerance"]
pub type Gpio048inttargetRstToleranceR = crate::BitReader<Gpio048inttargetRstTolerance>;
impl Gpio048inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio048inttargetRstTolerance {
        match self.bits {
            false => Gpio048inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio048inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio048inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio048inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO048INTTargetRstTolerance` writer - GPIO048 Interrupt Target Reset Tolerance"]
pub type Gpio048inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio048inttargetRstTolerance>;
impl<'a, REG> Gpio048inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio048inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio048inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO048INTTargetWrProt` reader - GPIO048 Interrupt Target Write Protection"]
pub type Gpio048inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO048INTTargetWrProt` writer - GPIO048 Interrupt Target Write Protection"]
pub type Gpio048inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO049INTToINT13018` reader - Enable GPIO049 Interrupt To INT#130_18"]
pub type EnblGpio049inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO049INTToINT13018` writer - Enable GPIO049 Interrupt To INT#130_18"]
pub type EnblGpio049inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO049INTToINT13019` reader - Enable GPIO049 Interrupt To INT#130_19"]
pub type EnblGpio049inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO049INTToINT13019` writer - Enable GPIO049 Interrupt To INT#130_19"]
pub type EnblGpio049inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO049INTToINT13020` reader - Enable GPIO049 Interrupt To INT#130_20"]
pub type EnblGpio049inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO049INTToINT13020` writer - Enable GPIO049 Interrupt To INT#130_20"]
pub type EnblGpio049inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO049INTToSIO` reader - Enable GPIO049 Interrupt To SIO"]
pub type EnblGpio049inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO049INTToSIO` writer - Enable GPIO049 Interrupt To SIO"]
pub type EnblGpio049inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO049 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio049inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio049inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio049inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO049INTTargetRstTolerance` reader - GPIO049 Interrupt Target Reset Tolerance"]
pub type Gpio049inttargetRstToleranceR = crate::BitReader<Gpio049inttargetRstTolerance>;
impl Gpio049inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio049inttargetRstTolerance {
        match self.bits {
            false => Gpio049inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio049inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio049inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio049inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO049INTTargetRstTolerance` writer - GPIO049 Interrupt Target Reset Tolerance"]
pub type Gpio049inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio049inttargetRstTolerance>;
impl<'a, REG> Gpio049inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio049inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio049inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO049INTTargetWrProt` reader - GPIO049 Interrupt Target Write Protection"]
pub type Gpio049inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO049INTTargetWrProt` writer - GPIO049 Interrupt Target Write Protection"]
pub type Gpio049inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO050INTToINT13018` reader - Enable GPIO050 Interrupt To INT#130_18"]
pub type EnblGpio050inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO050INTToINT13018` writer - Enable GPIO050 Interrupt To INT#130_18"]
pub type EnblGpio050inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO050INTToINT13019` reader - Enable GPIO050 Interrupt To INT#130_19"]
pub type EnblGpio050inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO050INTToINT13019` writer - Enable GPIO050 Interrupt To INT#130_19"]
pub type EnblGpio050inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO050INTToINT13020` reader - Enable GPIO050 Interrupt To INT#130_20"]
pub type EnblGpio050inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO050INTToINT13020` writer - Enable GPIO050 Interrupt To INT#130_20"]
pub type EnblGpio050inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO050INTToSIO` reader - Enable GPIO050 Interrupt To SIO"]
pub type EnblGpio050inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO050INTToSIO` writer - Enable GPIO050 Interrupt To SIO"]
pub type EnblGpio050inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO050 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio050inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio050inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio050inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO050INTTargetRstTolerance` reader - GPIO050 Interrupt Target Reset Tolerance"]
pub type Gpio050inttargetRstToleranceR = crate::BitReader<Gpio050inttargetRstTolerance>;
impl Gpio050inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio050inttargetRstTolerance {
        match self.bits {
            false => Gpio050inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio050inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio050inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio050inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO050INTTargetRstTolerance` writer - GPIO050 Interrupt Target Reset Tolerance"]
pub type Gpio050inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio050inttargetRstTolerance>;
impl<'a, REG> Gpio050inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio050inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio050inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO050INTTargetWrProt` reader - GPIO050 Interrupt Target Write Protection"]
pub type Gpio050inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO050INTTargetWrProt` writer - GPIO050 Interrupt Target Write Protection"]
pub type Gpio050inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO051INTToINT13018` reader - Enable GPIO051 Interrupt To INT#130_18"]
pub type EnblGpio051inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO051INTToINT13018` writer - Enable GPIO051 Interrupt To INT#130_18"]
pub type EnblGpio051inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO051INTToINT13019` reader - Enable GPIO051 Interrupt To INT#130_19"]
pub type EnblGpio051inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO051INTToINT13019` writer - Enable GPIO051 Interrupt To INT#130_19"]
pub type EnblGpio051inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO051INTToINT13020` reader - Enable GPIO051 Interrupt To INT#130_20"]
pub type EnblGpio051inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO051INTToINT13020` writer - Enable GPIO051 Interrupt To INT#130_20"]
pub type EnblGpio051inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO051INTToSIO` reader - Enable GPIO051 Interrupt To SIO"]
pub type EnblGpio051inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO051INTToSIO` writer - Enable GPIO051 Interrupt To SIO"]
pub type EnblGpio051inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO051 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio051inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio051inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio051inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO051INTTargetRstTolerance` reader - GPIO051 Interrupt Target Reset Tolerance"]
pub type Gpio051inttargetRstToleranceR = crate::BitReader<Gpio051inttargetRstTolerance>;
impl Gpio051inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio051inttargetRstTolerance {
        match self.bits {
            false => Gpio051inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio051inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio051inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio051inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO051INTTargetRstTolerance` writer - GPIO051 Interrupt Target Reset Tolerance"]
pub type Gpio051inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio051inttargetRstTolerance>;
impl<'a, REG> Gpio051inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio051inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio051inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO051INTTargetWrProt` reader - GPIO051 Interrupt Target Write Protection"]
pub type Gpio051inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO051INTTargetWrProt` writer - GPIO051 Interrupt Target Write Protection"]
pub type Gpio051inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO048 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio048intto_int13018(&self) -> EnblGpio048inttoInt13018R {
        EnblGpio048inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO048 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio048intto_int13019(&self) -> EnblGpio048inttoInt13019R {
        EnblGpio048inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO048 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio048intto_int13020(&self) -> EnblGpio048inttoInt13020R {
        EnblGpio048inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO048 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio048intto_sio(&self) -> EnblGpio048inttoSioR {
        EnblGpio048inttoSioR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO048 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio048inttarget_rst_tolerance(&self) -> Gpio048inttargetRstToleranceR {
        Gpio048inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO048 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio048inttarget_wr_prot(&self) -> Gpio048inttargetWrProtR {
        Gpio048inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO049 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio049intto_int13018(&self) -> EnblGpio049inttoInt13018R {
        EnblGpio049inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO049 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio049intto_int13019(&self) -> EnblGpio049inttoInt13019R {
        EnblGpio049inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO049 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio049intto_int13020(&self) -> EnblGpio049inttoInt13020R {
        EnblGpio049inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO049 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio049intto_sio(&self) -> EnblGpio049inttoSioR {
        EnblGpio049inttoSioR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO049 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio049inttarget_rst_tolerance(&self) -> Gpio049inttargetRstToleranceR {
        Gpio049inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO049 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio049inttarget_wr_prot(&self) -> Gpio049inttargetWrProtR {
        Gpio049inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO050 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio050intto_int13018(&self) -> EnblGpio050inttoInt13018R {
        EnblGpio050inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO050 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio050intto_int13019(&self) -> EnblGpio050inttoInt13019R {
        EnblGpio050inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO050 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio050intto_int13020(&self) -> EnblGpio050inttoInt13020R {
        EnblGpio050inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO050 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio050intto_sio(&self) -> EnblGpio050inttoSioR {
        EnblGpio050inttoSioR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO050 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio050inttarget_rst_tolerance(&self) -> Gpio050inttargetRstToleranceR {
        Gpio050inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO050 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio050inttarget_wr_prot(&self) -> Gpio050inttargetWrProtR {
        Gpio050inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO051 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio051intto_int13018(&self) -> EnblGpio051inttoInt13018R {
        EnblGpio051inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO051 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio051intto_int13019(&self) -> EnblGpio051inttoInt13019R {
        EnblGpio051inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO051 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio051intto_int13020(&self) -> EnblGpio051inttoInt13020R {
        EnblGpio051inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO051 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio051intto_sio(&self) -> EnblGpio051inttoSioR {
        EnblGpio051inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO051 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio051inttarget_rst_tolerance(&self) -> Gpio051inttargetRstToleranceR {
        Gpio051inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO051 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio051inttarget_wr_prot(&self) -> Gpio051inttargetWrProtR {
        Gpio051inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO048 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio048intto_int13018(&mut self) -> EnblGpio048inttoInt13018W<Gpioa40Spec> {
        EnblGpio048inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO048 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio048intto_int13019(&mut self) -> EnblGpio048inttoInt13019W<Gpioa40Spec> {
        EnblGpio048inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO048 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio048intto_int13020(&mut self) -> EnblGpio048inttoInt13020W<Gpioa40Spec> {
        EnblGpio048inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO048 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio048intto_sio(&mut self) -> EnblGpio048inttoSioW<Gpioa40Spec> {
        EnblGpio048inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa40Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa40Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO048 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio048inttarget_rst_tolerance(&mut self) -> Gpio048inttargetRstToleranceW<Gpioa40Spec> {
        Gpio048inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO048 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio048inttarget_wr_prot(&mut self) -> Gpio048inttargetWrProtW<Gpioa40Spec> {
        Gpio048inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO049 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio049intto_int13018(&mut self) -> EnblGpio049inttoInt13018W<Gpioa40Spec> {
        EnblGpio049inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO049 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio049intto_int13019(&mut self) -> EnblGpio049inttoInt13019W<Gpioa40Spec> {
        EnblGpio049inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO049 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio049intto_int13020(&mut self) -> EnblGpio049inttoInt13020W<Gpioa40Spec> {
        EnblGpio049inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO049 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio049intto_sio(&mut self) -> EnblGpio049inttoSioW<Gpioa40Spec> {
        EnblGpio049inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa40Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa40Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO049 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio049inttarget_rst_tolerance(&mut self) -> Gpio049inttargetRstToleranceW<Gpioa40Spec> {
        Gpio049inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO049 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio049inttarget_wr_prot(&mut self) -> Gpio049inttargetWrProtW<Gpioa40Spec> {
        Gpio049inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO050 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio050intto_int13018(&mut self) -> EnblGpio050inttoInt13018W<Gpioa40Spec> {
        EnblGpio050inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO050 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio050intto_int13019(&mut self) -> EnblGpio050inttoInt13019W<Gpioa40Spec> {
        EnblGpio050inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO050 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio050intto_int13020(&mut self) -> EnblGpio050inttoInt13020W<Gpioa40Spec> {
        EnblGpio050inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO050 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio050intto_sio(&mut self) -> EnblGpio050inttoSioW<Gpioa40Spec> {
        EnblGpio050inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa40Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa40Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO050 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio050inttarget_rst_tolerance(&mut self) -> Gpio050inttargetRstToleranceW<Gpioa40Spec> {
        Gpio050inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO050 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio050inttarget_wr_prot(&mut self) -> Gpio050inttargetWrProtW<Gpioa40Spec> {
        Gpio050inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO051 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio051intto_int13018(&mut self) -> EnblGpio051inttoInt13018W<Gpioa40Spec> {
        EnblGpio051inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO051 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio051intto_int13019(&mut self) -> EnblGpio051inttoInt13019W<Gpioa40Spec> {
        EnblGpio051inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO051 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio051intto_int13020(&mut self) -> EnblGpio051inttoInt13020W<Gpioa40Spec> {
        EnblGpio051inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO051 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio051intto_sio(&mut self) -> EnblGpio051inttoSioW<Gpioa40Spec> {
        EnblGpio051inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa40Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO051 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio051inttarget_rst_tolerance(&mut self) -> Gpio051inttargetRstToleranceW<Gpioa40Spec> {
        Gpio051inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO051 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio051inttarget_wr_prot(&mut self) -> Gpio051inttargetWrProtW<Gpioa40Spec> {
        Gpio051inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa40Spec;
impl crate::RegisterSpec for Gpioa40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa40::R`](R) reader structure"]
impl crate::Readable for Gpioa40Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa40::W`](W) writer structure"]
impl crate::Writable for Gpioa40Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA40 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa40Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
