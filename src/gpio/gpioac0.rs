#[doc = "Register `GPIOAC0` reader"]
pub type R = crate::R<Gpioac0Spec>;
#[doc = "Register `GPIOAC0` writer"]
pub type W = crate::W<Gpioac0Spec>;
#[doc = "Field `EnblGPIO176INTToINT13018` reader - Enable GPIO176 Interrupt To INT#130_18"]
pub type EnblGpio176inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO176INTToINT13018` writer - Enable GPIO176 Interrupt To INT#130_18"]
pub type EnblGpio176inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO176INTToINT13019` reader - Enable GPIO176 Interrupt To INT#130_19"]
pub type EnblGpio176inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO176INTToINT13019` writer - Enable GPIO176 Interrupt To INT#130_19"]
pub type EnblGpio176inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO176INTToINT13020` reader - Enable GPIO176 Interrupt To INT#130_20"]
pub type EnblGpio176inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO176INTToINT13020` writer - Enable GPIO176 Interrupt To INT#130_20"]
pub type EnblGpio176inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO176INTToSIO` reader - Enable GPIO176 Interrupt To SIO"]
pub type EnblGpio176inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO176INTToSIO` writer - Enable GPIO176 Interrupt To SIO"]
pub type EnblGpio176inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO176 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio176inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio176inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio176inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO176INTTargetRstTolerance` reader - GPIO176 Interrupt Target Reset Tolerance"]
pub type Gpio176inttargetRstToleranceR = crate::BitReader<Gpio176inttargetRstTolerance>;
impl Gpio176inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio176inttargetRstTolerance {
        match self.bits {
            false => Gpio176inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio176inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio176inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio176inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO176INTTargetRstTolerance` writer - GPIO176 Interrupt Target Reset Tolerance"]
pub type Gpio176inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio176inttargetRstTolerance>;
impl<'a, REG> Gpio176inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio176inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio176inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO176INTTargetWrProt` reader - GPIO176 Interrupt Target Write Protection"]
pub type Gpio176inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO176INTTargetWrProt` writer - GPIO176 Interrupt Target Write Protection"]
pub type Gpio176inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO177INTToINT13018` reader - Enable GPIO177 Interrupt To INT#130_18"]
pub type EnblGpio177inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO177INTToINT13018` writer - Enable GPIO177 Interrupt To INT#130_18"]
pub type EnblGpio177inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO177INTToINT13019` reader - Enable GPIO177 Interrupt To INT#130_19"]
pub type EnblGpio177inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO177INTToINT13019` writer - Enable GPIO177 Interrupt To INT#130_19"]
pub type EnblGpio177inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO177INTToINT13020` reader - Enable GPIO177 Interrupt To INT#130_20"]
pub type EnblGpio177inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO177INTToINT13020` writer - Enable GPIO177 Interrupt To INT#130_20"]
pub type EnblGpio177inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO177INTToSIO` reader - Enable GPIO177 Interrupt To SIO"]
pub type EnblGpio177inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO177INTToSIO` writer - Enable GPIO177 Interrupt To SIO"]
pub type EnblGpio177inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO177 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio177inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio177inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio177inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO177INTTargetRstTolerance` reader - GPIO177 Interrupt Target Reset Tolerance"]
pub type Gpio177inttargetRstToleranceR = crate::BitReader<Gpio177inttargetRstTolerance>;
impl Gpio177inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio177inttargetRstTolerance {
        match self.bits {
            false => Gpio177inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio177inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio177inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio177inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO177INTTargetRstTolerance` writer - GPIO177 Interrupt Target Reset Tolerance"]
pub type Gpio177inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio177inttargetRstTolerance>;
impl<'a, REG> Gpio177inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio177inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio177inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO177INTTargetWrProt` reader - GPIO177 Interrupt Target Write Protection"]
pub type Gpio177inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO177INTTargetWrProt` writer - GPIO177 Interrupt Target Write Protection"]
pub type Gpio177inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO178INTToINT13018` reader - Enable GPIO178 Interrupt To INT#130_18"]
pub type EnblGpio178inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO178INTToINT13018` writer - Enable GPIO178 Interrupt To INT#130_18"]
pub type EnblGpio178inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO178INTToINT13019` reader - Enable GPIO178 Interrupt To INT#130_19"]
pub type EnblGpio178inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO178INTToINT13019` writer - Enable GPIO178 Interrupt To INT#130_19"]
pub type EnblGpio178inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO178INTToINT13020` reader - Enable GPIO178 Interrupt To INT#130_20"]
pub type EnblGpio178inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO178INTToINT13020` writer - Enable GPIO178 Interrupt To INT#130_20"]
pub type EnblGpio178inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO178INTToSIO` reader - Enable GPIO178 Interrupt To SIO"]
pub type EnblGpio178inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO178INTToSIO` writer - Enable GPIO178 Interrupt To SIO"]
pub type EnblGpio178inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO178 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio178inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio178inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio178inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO178INTTargetRstTolerance` reader - GPIO178 Interrupt Target Reset Tolerance"]
pub type Gpio178inttargetRstToleranceR = crate::BitReader<Gpio178inttargetRstTolerance>;
impl Gpio178inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio178inttargetRstTolerance {
        match self.bits {
            false => Gpio178inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio178inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio178inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio178inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO178INTTargetRstTolerance` writer - GPIO178 Interrupt Target Reset Tolerance"]
pub type Gpio178inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio178inttargetRstTolerance>;
impl<'a, REG> Gpio178inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio178inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio178inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO178INTTargetWrProt` reader - GPIO178 Interrupt Target Write Protection"]
pub type Gpio178inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO178INTTargetWrProt` writer - GPIO178 Interrupt Target Write Protection"]
pub type Gpio178inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO179INTToINT13018` reader - Enable GPIO179 Interrupt To INT#130_18"]
pub type EnblGpio179inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO179INTToINT13018` writer - Enable GPIO179 Interrupt To INT#130_18"]
pub type EnblGpio179inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO179INTToINT13019` reader - Enable GPIO179 Interrupt To INT#130_19"]
pub type EnblGpio179inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO179INTToINT13019` writer - Enable GPIO179 Interrupt To INT#130_19"]
pub type EnblGpio179inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO179INTToINT13020` reader - Enable GPIO179 Interrupt To INT#130_20"]
pub type EnblGpio179inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO179INTToINT13020` writer - Enable GPIO179 Interrupt To INT#130_20"]
pub type EnblGpio179inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO179INTToSIO` reader - Enable GPIO179 Interrupt To SIO"]
pub type EnblGpio179inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO179INTToSIO` writer - Enable GPIO179 Interrupt To SIO"]
pub type EnblGpio179inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO179 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio179inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio179inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio179inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO179INTTargetRstTolerance` reader - GPIO179 Interrupt Target Reset Tolerance"]
pub type Gpio179inttargetRstToleranceR = crate::BitReader<Gpio179inttargetRstTolerance>;
impl Gpio179inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio179inttargetRstTolerance {
        match self.bits {
            false => Gpio179inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio179inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio179inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio179inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO179INTTargetRstTolerance` writer - GPIO179 Interrupt Target Reset Tolerance"]
pub type Gpio179inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio179inttargetRstTolerance>;
impl<'a, REG> Gpio179inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio179inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio179inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO179INTTargetWrProt` reader - GPIO179 Interrupt Target Write Protection"]
pub type Gpio179inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO179INTTargetWrProt` writer - GPIO179 Interrupt Target Write Protection"]
pub type Gpio179inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO176 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio176intto_int13018(&self) -> EnblGpio176inttoInt13018R {
        EnblGpio176inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO176 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio176intto_int13019(&self) -> EnblGpio176inttoInt13019R {
        EnblGpio176inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO176 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio176intto_int13020(&self) -> EnblGpio176inttoInt13020R {
        EnblGpio176inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO176 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio176intto_sio(&self) -> EnblGpio176inttoSioR {
        EnblGpio176inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO176 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio176inttarget_rst_tolerance(&self) -> Gpio176inttargetRstToleranceR {
        Gpio176inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO176 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio176inttarget_wr_prot(&self) -> Gpio176inttargetWrProtR {
        Gpio176inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO177 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio177intto_int13018(&self) -> EnblGpio177inttoInt13018R {
        EnblGpio177inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO177 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio177intto_int13019(&self) -> EnblGpio177inttoInt13019R {
        EnblGpio177inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO177 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio177intto_int13020(&self) -> EnblGpio177inttoInt13020R {
        EnblGpio177inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO177 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio177intto_sio(&self) -> EnblGpio177inttoSioR {
        EnblGpio177inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO177 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio177inttarget_rst_tolerance(&self) -> Gpio177inttargetRstToleranceR {
        Gpio177inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO177 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio177inttarget_wr_prot(&self) -> Gpio177inttargetWrProtR {
        Gpio177inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO178 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio178intto_int13018(&self) -> EnblGpio178inttoInt13018R {
        EnblGpio178inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO178 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio178intto_int13019(&self) -> EnblGpio178inttoInt13019R {
        EnblGpio178inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO178 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio178intto_int13020(&self) -> EnblGpio178inttoInt13020R {
        EnblGpio178inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO178 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio178intto_sio(&self) -> EnblGpio178inttoSioR {
        EnblGpio178inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO178 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio178inttarget_rst_tolerance(&self) -> Gpio178inttargetRstToleranceR {
        Gpio178inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO178 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio178inttarget_wr_prot(&self) -> Gpio178inttargetWrProtR {
        Gpio178inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO179 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio179intto_int13018(&self) -> EnblGpio179inttoInt13018R {
        EnblGpio179inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO179 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio179intto_int13019(&self) -> EnblGpio179inttoInt13019R {
        EnblGpio179inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO179 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio179intto_int13020(&self) -> EnblGpio179inttoInt13020R {
        EnblGpio179inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO179 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio179intto_sio(&self) -> EnblGpio179inttoSioR {
        EnblGpio179inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO179 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio179inttarget_rst_tolerance(&self) -> Gpio179inttargetRstToleranceR {
        Gpio179inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO179 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio179inttarget_wr_prot(&self) -> Gpio179inttargetWrProtR {
        Gpio179inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO176 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio176intto_int13018(&mut self) -> EnblGpio176inttoInt13018W<Gpioac0Spec> {
        EnblGpio176inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO176 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio176intto_int13019(&mut self) -> EnblGpio176inttoInt13019W<Gpioac0Spec> {
        EnblGpio176inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO176 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio176intto_int13020(&mut self) -> EnblGpio176inttoInt13020W<Gpioac0Spec> {
        EnblGpio176inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO176 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio176intto_sio(&mut self) -> EnblGpio176inttoSioW<Gpioac0Spec> {
        EnblGpio176inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioac0Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioac0Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO176 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio176inttarget_rst_tolerance(&mut self) -> Gpio176inttargetRstToleranceW<Gpioac0Spec> {
        Gpio176inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO176 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio176inttarget_wr_prot(&mut self) -> Gpio176inttargetWrProtW<Gpioac0Spec> {
        Gpio176inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO177 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio177intto_int13018(&mut self) -> EnblGpio177inttoInt13018W<Gpioac0Spec> {
        EnblGpio177inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO177 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio177intto_int13019(&mut self) -> EnblGpio177inttoInt13019W<Gpioac0Spec> {
        EnblGpio177inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO177 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio177intto_int13020(&mut self) -> EnblGpio177inttoInt13020W<Gpioac0Spec> {
        EnblGpio177inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO177 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio177intto_sio(&mut self) -> EnblGpio177inttoSioW<Gpioac0Spec> {
        EnblGpio177inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioac0Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioac0Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO177 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio177inttarget_rst_tolerance(&mut self) -> Gpio177inttargetRstToleranceW<Gpioac0Spec> {
        Gpio177inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO177 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio177inttarget_wr_prot(&mut self) -> Gpio177inttargetWrProtW<Gpioac0Spec> {
        Gpio177inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO178 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio178intto_int13018(&mut self) -> EnblGpio178inttoInt13018W<Gpioac0Spec> {
        EnblGpio178inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO178 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio178intto_int13019(&mut self) -> EnblGpio178inttoInt13019W<Gpioac0Spec> {
        EnblGpio178inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO178 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio178intto_int13020(&mut self) -> EnblGpio178inttoInt13020W<Gpioac0Spec> {
        EnblGpio178inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO178 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio178intto_sio(&mut self) -> EnblGpio178inttoSioW<Gpioac0Spec> {
        EnblGpio178inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioac0Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioac0Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO178 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio178inttarget_rst_tolerance(&mut self) -> Gpio178inttargetRstToleranceW<Gpioac0Spec> {
        Gpio178inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO178 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio178inttarget_wr_prot(&mut self) -> Gpio178inttargetWrProtW<Gpioac0Spec> {
        Gpio178inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO179 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio179intto_int13018(&mut self) -> EnblGpio179inttoInt13018W<Gpioac0Spec> {
        EnblGpio179inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO179 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio179intto_int13019(&mut self) -> EnblGpio179inttoInt13019W<Gpioac0Spec> {
        EnblGpio179inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO179 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio179intto_int13020(&mut self) -> EnblGpio179inttoInt13020W<Gpioac0Spec> {
        EnblGpio179inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO179 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio179intto_sio(&mut self) -> EnblGpio179inttoSioW<Gpioac0Spec> {
        EnblGpio179inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioac0Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO179 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio179inttarget_rst_tolerance(&mut self) -> Gpio179inttargetRstToleranceW<Gpioac0Spec> {
        Gpio179inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO179 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio179inttarget_wr_prot(&mut self) -> Gpio179inttargetWrProtW<Gpioac0Spec> {
        Gpio179inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#44\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioac0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioac0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioac0Spec;
impl crate::RegisterSpec for Gpioac0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioac0::R`](R) reader structure"]
impl crate::Readable for Gpioac0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioac0::W`](W) writer structure"]
impl crate::Writable for Gpioac0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOAC0 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioac0Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
