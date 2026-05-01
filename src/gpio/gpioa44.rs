#[doc = "Register `GPIOA44` reader"]
pub type R = crate::R<Gpioa44Spec>;
#[doc = "Register `GPIOA44` writer"]
pub type W = crate::W<Gpioa44Spec>;
#[doc = "Field `EnblGPIO052INTToINT13018` reader - Enable GPIO052 Interrupt To INT#130_18"]
pub type EnblGpio052inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO052INTToINT13018` writer - Enable GPIO052 Interrupt To INT#130_18"]
pub type EnblGpio052inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO052INTToINT13019` reader - Enable GPIO052 Interrupt To INT#130_19"]
pub type EnblGpio052inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO052INTToINT13019` writer - Enable GPIO052 Interrupt To INT#130_19"]
pub type EnblGpio052inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO052INTToINT13020` reader - Enable GPIO052 Interrupt To INT#130_20"]
pub type EnblGpio052inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO052INTToINT13020` writer - Enable GPIO052 Interrupt To INT#130_20"]
pub type EnblGpio052inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO052INTToSIO` reader - Enable GPIO052 Interrupt To SIO"]
pub type EnblGpio052inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO052INTToSIO` writer - Enable GPIO052 Interrupt To SIO"]
pub type EnblGpio052inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO052 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio052inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio052inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio052inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO052INTTargetRstTolerance` reader - GPIO052 Interrupt Target Reset Tolerance"]
pub type Gpio052inttargetRstToleranceR = crate::BitReader<Gpio052inttargetRstTolerance>;
impl Gpio052inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio052inttargetRstTolerance {
        match self.bits {
            false => Gpio052inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio052inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio052inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio052inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO052INTTargetRstTolerance` writer - GPIO052 Interrupt Target Reset Tolerance"]
pub type Gpio052inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio052inttargetRstTolerance>;
impl<'a, REG> Gpio052inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio052inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio052inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO052INTTargetWrProt` reader - GPIO052 Interrupt Target Write Protection"]
pub type Gpio052inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO052INTTargetWrProt` writer - GPIO052 Interrupt Target Write Protection"]
pub type Gpio052inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO053INTToINT13018` reader - Enable GPIO053 Interrupt To INT#130_18"]
pub type EnblGpio053inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO053INTToINT13018` writer - Enable GPIO053 Interrupt To INT#130_18"]
pub type EnblGpio053inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO053INTToINT13019` reader - Enable GPIO053 Interrupt To INT#130_19"]
pub type EnblGpio053inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO053INTToINT13019` writer - Enable GPIO053 Interrupt To INT#130_19"]
pub type EnblGpio053inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO053INTToINT13020` reader - Enable GPIO053 Interrupt To INT#130_20"]
pub type EnblGpio053inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO053INTToINT13020` writer - Enable GPIO053 Interrupt To INT#130_20"]
pub type EnblGpio053inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO053INTToSIO` reader - Enable GPIO053 Interrupt To SIO"]
pub type EnblGpio053inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO053INTToSIO` writer - Enable GPIO053 Interrupt To SIO"]
pub type EnblGpio053inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO053 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio053inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio053inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio053inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO053INTTargetRstTolerance` reader - GPIO053 Interrupt Target Reset Tolerance"]
pub type Gpio053inttargetRstToleranceR = crate::BitReader<Gpio053inttargetRstTolerance>;
impl Gpio053inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio053inttargetRstTolerance {
        match self.bits {
            false => Gpio053inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio053inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio053inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio053inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO053INTTargetRstTolerance` writer - GPIO053 Interrupt Target Reset Tolerance"]
pub type Gpio053inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio053inttargetRstTolerance>;
impl<'a, REG> Gpio053inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio053inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio053inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO053INTTargetWrProt` reader - GPIO053 Interrupt Target Write Protection"]
pub type Gpio053inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO053INTTargetWrProt` writer - GPIO053 Interrupt Target Write Protection"]
pub type Gpio053inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO054INTToINT13018` reader - Enable GPIO054 Interrupt To INT#130_18"]
pub type EnblGpio054inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO054INTToINT13018` writer - Enable GPIO054 Interrupt To INT#130_18"]
pub type EnblGpio054inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO054INTToINT13019` reader - Enable GPIO054 Interrupt To INT#130_19"]
pub type EnblGpio054inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO054INTToINT13019` writer - Enable GPIO054 Interrupt To INT#130_19"]
pub type EnblGpio054inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO054INTToINT13020` reader - Enable GPIO054 Interrupt To INT#130_20"]
pub type EnblGpio054inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO054INTToINT13020` writer - Enable GPIO054 Interrupt To INT#130_20"]
pub type EnblGpio054inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO054INTToSIO` reader - Enable GPIO054 Interrupt To SIO"]
pub type EnblGpio054inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO054INTToSIO` writer - Enable GPIO054 Interrupt To SIO"]
pub type EnblGpio054inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO054 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio054inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio054inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio054inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO054INTTargetRstTolerance` reader - GPIO054 Interrupt Target Reset Tolerance"]
pub type Gpio054inttargetRstToleranceR = crate::BitReader<Gpio054inttargetRstTolerance>;
impl Gpio054inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio054inttargetRstTolerance {
        match self.bits {
            false => Gpio054inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio054inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio054inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio054inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO054INTTargetRstTolerance` writer - GPIO054 Interrupt Target Reset Tolerance"]
pub type Gpio054inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio054inttargetRstTolerance>;
impl<'a, REG> Gpio054inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio054inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio054inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO054INTTargetWrProt` reader - GPIO054 Interrupt Target Write Protection"]
pub type Gpio054inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO054INTTargetWrProt` writer - GPIO054 Interrupt Target Write Protection"]
pub type Gpio054inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO055INTToINT13018` reader - Enable GPIO055 Interrupt To INT#130_18"]
pub type EnblGpio055inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO055INTToINT13018` writer - Enable GPIO055 Interrupt To INT#130_18"]
pub type EnblGpio055inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO055INTToINT13019` reader - Enable GPIO055 Interrupt To INT#130_19"]
pub type EnblGpio055inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO055INTToINT13019` writer - Enable GPIO055 Interrupt To INT#130_19"]
pub type EnblGpio055inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO055INTToINT13020` reader - Enable GPIO055 Interrupt To INT#130_20"]
pub type EnblGpio055inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO055INTToINT13020` writer - Enable GPIO055 Interrupt To INT#130_20"]
pub type EnblGpio055inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO055INTToSIO` reader - Enable GPIO055 Interrupt To SIO"]
pub type EnblGpio055inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO055INTToSIO` writer - Enable GPIO055 Interrupt To SIO"]
pub type EnblGpio055inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO055 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio055inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio055inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio055inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO055INTTargetRstTolerance` reader - GPIO055 Interrupt Target Reset Tolerance"]
pub type Gpio055inttargetRstToleranceR = crate::BitReader<Gpio055inttargetRstTolerance>;
impl Gpio055inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio055inttargetRstTolerance {
        match self.bits {
            false => Gpio055inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio055inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio055inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio055inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO055INTTargetRstTolerance` writer - GPIO055 Interrupt Target Reset Tolerance"]
pub type Gpio055inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio055inttargetRstTolerance>;
impl<'a, REG> Gpio055inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio055inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio055inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO055INTTargetWrProt` reader - GPIO055 Interrupt Target Write Protection"]
pub type Gpio055inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO055INTTargetWrProt` writer - GPIO055 Interrupt Target Write Protection"]
pub type Gpio055inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO052 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio052intto_int13018(&self) -> EnblGpio052inttoInt13018R {
        EnblGpio052inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO052 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio052intto_int13019(&self) -> EnblGpio052inttoInt13019R {
        EnblGpio052inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO052 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio052intto_int13020(&self) -> EnblGpio052inttoInt13020R {
        EnblGpio052inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO052 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio052intto_sio(&self) -> EnblGpio052inttoSioR {
        EnblGpio052inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO052 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio052inttarget_rst_tolerance(&self) -> Gpio052inttargetRstToleranceR {
        Gpio052inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO052 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio052inttarget_wr_prot(&self) -> Gpio052inttargetWrProtR {
        Gpio052inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO053 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio053intto_int13018(&self) -> EnblGpio053inttoInt13018R {
        EnblGpio053inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO053 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio053intto_int13019(&self) -> EnblGpio053inttoInt13019R {
        EnblGpio053inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO053 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio053intto_int13020(&self) -> EnblGpio053inttoInt13020R {
        EnblGpio053inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO053 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio053intto_sio(&self) -> EnblGpio053inttoSioR {
        EnblGpio053inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO053 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio053inttarget_rst_tolerance(&self) -> Gpio053inttargetRstToleranceR {
        Gpio053inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO053 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio053inttarget_wr_prot(&self) -> Gpio053inttargetWrProtR {
        Gpio053inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO054 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio054intto_int13018(&self) -> EnblGpio054inttoInt13018R {
        EnblGpio054inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO054 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio054intto_int13019(&self) -> EnblGpio054inttoInt13019R {
        EnblGpio054inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO054 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio054intto_int13020(&self) -> EnblGpio054inttoInt13020R {
        EnblGpio054inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO054 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio054intto_sio(&self) -> EnblGpio054inttoSioR {
        EnblGpio054inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO054 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio054inttarget_rst_tolerance(&self) -> Gpio054inttargetRstToleranceR {
        Gpio054inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO054 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio054inttarget_wr_prot(&self) -> Gpio054inttargetWrProtR {
        Gpio054inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO055 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio055intto_int13018(&self) -> EnblGpio055inttoInt13018R {
        EnblGpio055inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO055 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio055intto_int13019(&self) -> EnblGpio055inttoInt13019R {
        EnblGpio055inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO055 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio055intto_int13020(&self) -> EnblGpio055inttoInt13020R {
        EnblGpio055inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO055 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio055intto_sio(&self) -> EnblGpio055inttoSioR {
        EnblGpio055inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO055 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio055inttarget_rst_tolerance(&self) -> Gpio055inttargetRstToleranceR {
        Gpio055inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO055 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio055inttarget_wr_prot(&self) -> Gpio055inttargetWrProtR {
        Gpio055inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO052 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio052intto_int13018(&mut self) -> EnblGpio052inttoInt13018W<Gpioa44Spec> {
        EnblGpio052inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO052 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio052intto_int13019(&mut self) -> EnblGpio052inttoInt13019W<Gpioa44Spec> {
        EnblGpio052inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO052 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio052intto_int13020(&mut self) -> EnblGpio052inttoInt13020W<Gpioa44Spec> {
        EnblGpio052inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO052 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio052intto_sio(&mut self) -> EnblGpio052inttoSioW<Gpioa44Spec> {
        EnblGpio052inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa44Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa44Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO052 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio052inttarget_rst_tolerance(&mut self) -> Gpio052inttargetRstToleranceW<Gpioa44Spec> {
        Gpio052inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO052 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio052inttarget_wr_prot(&mut self) -> Gpio052inttargetWrProtW<Gpioa44Spec> {
        Gpio052inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO053 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio053intto_int13018(&mut self) -> EnblGpio053inttoInt13018W<Gpioa44Spec> {
        EnblGpio053inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO053 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio053intto_int13019(&mut self) -> EnblGpio053inttoInt13019W<Gpioa44Spec> {
        EnblGpio053inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO053 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio053intto_int13020(&mut self) -> EnblGpio053inttoInt13020W<Gpioa44Spec> {
        EnblGpio053inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO053 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio053intto_sio(&mut self) -> EnblGpio053inttoSioW<Gpioa44Spec> {
        EnblGpio053inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa44Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa44Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO053 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio053inttarget_rst_tolerance(&mut self) -> Gpio053inttargetRstToleranceW<Gpioa44Spec> {
        Gpio053inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO053 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio053inttarget_wr_prot(&mut self) -> Gpio053inttargetWrProtW<Gpioa44Spec> {
        Gpio053inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO054 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio054intto_int13018(&mut self) -> EnblGpio054inttoInt13018W<Gpioa44Spec> {
        EnblGpio054inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO054 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio054intto_int13019(&mut self) -> EnblGpio054inttoInt13019W<Gpioa44Spec> {
        EnblGpio054inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO054 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio054intto_int13020(&mut self) -> EnblGpio054inttoInt13020W<Gpioa44Spec> {
        EnblGpio054inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO054 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio054intto_sio(&mut self) -> EnblGpio054inttoSioW<Gpioa44Spec> {
        EnblGpio054inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa44Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa44Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO054 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio054inttarget_rst_tolerance(&mut self) -> Gpio054inttargetRstToleranceW<Gpioa44Spec> {
        Gpio054inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO054 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio054inttarget_wr_prot(&mut self) -> Gpio054inttargetWrProtW<Gpioa44Spec> {
        Gpio054inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO055 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio055intto_int13018(&mut self) -> EnblGpio055inttoInt13018W<Gpioa44Spec> {
        EnblGpio055inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO055 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio055intto_int13019(&mut self) -> EnblGpio055inttoInt13019W<Gpioa44Spec> {
        EnblGpio055inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO055 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio055intto_int13020(&mut self) -> EnblGpio055inttoInt13020W<Gpioa44Spec> {
        EnblGpio055inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO055 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio055intto_sio(&mut self) -> EnblGpio055inttoSioW<Gpioa44Spec> {
        EnblGpio055inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa44Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO055 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio055inttarget_rst_tolerance(&mut self) -> Gpio055inttargetRstToleranceW<Gpioa44Spec> {
        Gpio055inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO055 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio055inttarget_wr_prot(&mut self) -> Gpio055inttargetWrProtW<Gpioa44Spec> {
        Gpio055inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa44Spec;
impl crate::RegisterSpec for Gpioa44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa44::R`](R) reader structure"]
impl crate::Readable for Gpioa44Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa44::W`](W) writer structure"]
impl crate::Writable for Gpioa44Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA44 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa44Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
