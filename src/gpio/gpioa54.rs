#[doc = "Register `GPIOA54` reader"]
pub type R = crate::R<Gpioa54Spec>;
#[doc = "Register `GPIOA54` writer"]
pub type W = crate::W<Gpioa54Spec>;
#[doc = "Field `EnblGPIO068INTToINT13018` reader - Enable GPIO068 Interrupt To INT#130_18"]
pub type EnblGpio068inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO068INTToINT13018` writer - Enable GPIO068 Interrupt To INT#130_18"]
pub type EnblGpio068inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO068INTToINT13019` reader - Enable GPIO068 Interrupt To INT#130_19"]
pub type EnblGpio068inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO068INTToINT13019` writer - Enable GPIO068 Interrupt To INT#130_19"]
pub type EnblGpio068inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO068INTToINT13020` reader - Enable GPIO068 Interrupt To INT#130_20"]
pub type EnblGpio068inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO068INTToINT13020` writer - Enable GPIO068 Interrupt To INT#130_20"]
pub type EnblGpio068inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO068INTToSIO` reader - Enable GPIO068 Interrupt To SIO"]
pub type EnblGpio068inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO068INTToSIO` writer - Enable GPIO068 Interrupt To SIO"]
pub type EnblGpio068inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO068 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio068inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio068inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio068inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO068INTTargetRstTolerance` reader - GPIO068 Interrupt Target Reset Tolerance"]
pub type Gpio068inttargetRstToleranceR = crate::BitReader<Gpio068inttargetRstTolerance>;
impl Gpio068inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio068inttargetRstTolerance {
        match self.bits {
            false => Gpio068inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio068inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio068inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio068inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO068INTTargetRstTolerance` writer - GPIO068 Interrupt Target Reset Tolerance"]
pub type Gpio068inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio068inttargetRstTolerance>;
impl<'a, REG> Gpio068inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio068inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio068inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO068INTTargetWrProt` reader - GPIO068 Interrupt Target Write Protection"]
pub type Gpio068inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO068INTTargetWrProt` writer - GPIO068 Interrupt Target Write Protection"]
pub type Gpio068inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO069INTToINT13018` reader - Enable GPIO069 Interrupt To INT#130_18"]
pub type EnblGpio069inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO069INTToINT13018` writer - Enable GPIO069 Interrupt To INT#130_18"]
pub type EnblGpio069inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO069INTToINT13019` reader - Enable GPIO069 Interrupt To INT#130_19"]
pub type EnblGpio069inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO069INTToINT13019` writer - Enable GPIO069 Interrupt To INT#130_19"]
pub type EnblGpio069inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO069INTToINT13020` reader - Enable GPIO069 Interrupt To INT#130_20"]
pub type EnblGpio069inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO069INTToINT13020` writer - Enable GPIO069 Interrupt To INT#130_20"]
pub type EnblGpio069inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO069INTToSIO` reader - Enable GPIO069 Interrupt To SIO"]
pub type EnblGpio069inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO069INTToSIO` writer - Enable GPIO069 Interrupt To SIO"]
pub type EnblGpio069inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO069 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio069inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio069inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio069inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO069INTTargetRstTolerance` reader - GPIO069 Interrupt Target Reset Tolerance"]
pub type Gpio069inttargetRstToleranceR = crate::BitReader<Gpio069inttargetRstTolerance>;
impl Gpio069inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio069inttargetRstTolerance {
        match self.bits {
            false => Gpio069inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio069inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio069inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio069inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO069INTTargetRstTolerance` writer - GPIO069 Interrupt Target Reset Tolerance"]
pub type Gpio069inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio069inttargetRstTolerance>;
impl<'a, REG> Gpio069inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio069inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio069inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO069INTTargetWrProt` reader - GPIO069 Interrupt Target Write Protection"]
pub type Gpio069inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO069INTTargetWrProt` writer - GPIO069 Interrupt Target Write Protection"]
pub type Gpio069inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO070INTToINT13018` reader - Enable GPIO070 Interrupt To INT#130_18"]
pub type EnblGpio070inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO070INTToINT13018` writer - Enable GPIO070 Interrupt To INT#130_18"]
pub type EnblGpio070inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO070INTToINT13019` reader - Enable GPIO070 Interrupt To INT#130_19"]
pub type EnblGpio070inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO070INTToINT13019` writer - Enable GPIO070 Interrupt To INT#130_19"]
pub type EnblGpio070inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO070INTToINT13020` reader - Enable GPIO070 Interrupt To INT#130_20"]
pub type EnblGpio070inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO070INTToINT13020` writer - Enable GPIO070 Interrupt To INT#130_20"]
pub type EnblGpio070inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO070INTToSIO` reader - Enable GPIO070 Interrupt To SIO"]
pub type EnblGpio070inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO070INTToSIO` writer - Enable GPIO070 Interrupt To SIO"]
pub type EnblGpio070inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO070 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio070inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio070inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio070inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO070INTTargetRstTolerance` reader - GPIO070 Interrupt Target Reset Tolerance"]
pub type Gpio070inttargetRstToleranceR = crate::BitReader<Gpio070inttargetRstTolerance>;
impl Gpio070inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio070inttargetRstTolerance {
        match self.bits {
            false => Gpio070inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio070inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio070inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio070inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO070INTTargetRstTolerance` writer - GPIO070 Interrupt Target Reset Tolerance"]
pub type Gpio070inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio070inttargetRstTolerance>;
impl<'a, REG> Gpio070inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio070inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio070inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO070INTTargetWrProt` reader - GPIO070 Interrupt Target Write Protection"]
pub type Gpio070inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO070INTTargetWrProt` writer - GPIO070 Interrupt Target Write Protection"]
pub type Gpio070inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO071INTToINT13018` reader - Enable GPIO071 Interrupt To INT#130_18"]
pub type EnblGpio071inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO071INTToINT13018` writer - Enable GPIO071 Interrupt To INT#130_18"]
pub type EnblGpio071inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO071INTToINT13019` reader - Enable GPIO071 Interrupt To INT#130_19"]
pub type EnblGpio071inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO071INTToINT13019` writer - Enable GPIO071 Interrupt To INT#130_19"]
pub type EnblGpio071inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO071INTToINT13020` reader - Enable GPIO071 Interrupt To INT#130_20"]
pub type EnblGpio071inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO071INTToINT13020` writer - Enable GPIO071 Interrupt To INT#130_20"]
pub type EnblGpio071inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO071INTToSIO` reader - Enable GPIO071 Interrupt To SIO"]
pub type EnblGpio071inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO071INTToSIO` writer - Enable GPIO071 Interrupt To SIO"]
pub type EnblGpio071inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO071 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio071inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio071inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio071inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO071INTTargetRstTolerance` reader - GPIO071 Interrupt Target Reset Tolerance"]
pub type Gpio071inttargetRstToleranceR = crate::BitReader<Gpio071inttargetRstTolerance>;
impl Gpio071inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio071inttargetRstTolerance {
        match self.bits {
            false => Gpio071inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio071inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio071inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio071inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO071INTTargetRstTolerance` writer - GPIO071 Interrupt Target Reset Tolerance"]
pub type Gpio071inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio071inttargetRstTolerance>;
impl<'a, REG> Gpio071inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio071inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio071inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO071INTTargetWrProt` reader - GPIO071 Interrupt Target Write Protection"]
pub type Gpio071inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO071INTTargetWrProt` writer - GPIO071 Interrupt Target Write Protection"]
pub type Gpio071inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO068 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio068intto_int13018(&self) -> EnblGpio068inttoInt13018R {
        EnblGpio068inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO068 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio068intto_int13019(&self) -> EnblGpio068inttoInt13019R {
        EnblGpio068inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO068 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio068intto_int13020(&self) -> EnblGpio068inttoInt13020R {
        EnblGpio068inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO068 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio068intto_sio(&self) -> EnblGpio068inttoSioR {
        EnblGpio068inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO068 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio068inttarget_rst_tolerance(&self) -> Gpio068inttargetRstToleranceR {
        Gpio068inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO068 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio068inttarget_wr_prot(&self) -> Gpio068inttargetWrProtR {
        Gpio068inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO069 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio069intto_int13018(&self) -> EnblGpio069inttoInt13018R {
        EnblGpio069inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO069 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio069intto_int13019(&self) -> EnblGpio069inttoInt13019R {
        EnblGpio069inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO069 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio069intto_int13020(&self) -> EnblGpio069inttoInt13020R {
        EnblGpio069inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO069 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio069intto_sio(&self) -> EnblGpio069inttoSioR {
        EnblGpio069inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO069 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio069inttarget_rst_tolerance(&self) -> Gpio069inttargetRstToleranceR {
        Gpio069inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO069 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio069inttarget_wr_prot(&self) -> Gpio069inttargetWrProtR {
        Gpio069inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO070 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio070intto_int13018(&self) -> EnblGpio070inttoInt13018R {
        EnblGpio070inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO070 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio070intto_int13019(&self) -> EnblGpio070inttoInt13019R {
        EnblGpio070inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO070 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio070intto_int13020(&self) -> EnblGpio070inttoInt13020R {
        EnblGpio070inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO070 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio070intto_sio(&self) -> EnblGpio070inttoSioR {
        EnblGpio070inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO070 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio070inttarget_rst_tolerance(&self) -> Gpio070inttargetRstToleranceR {
        Gpio070inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO070 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio070inttarget_wr_prot(&self) -> Gpio070inttargetWrProtR {
        Gpio070inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO071 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio071intto_int13018(&self) -> EnblGpio071inttoInt13018R {
        EnblGpio071inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO071 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio071intto_int13019(&self) -> EnblGpio071inttoInt13019R {
        EnblGpio071inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO071 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio071intto_int13020(&self) -> EnblGpio071inttoInt13020R {
        EnblGpio071inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO071 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio071intto_sio(&self) -> EnblGpio071inttoSioR {
        EnblGpio071inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO071 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio071inttarget_rst_tolerance(&self) -> Gpio071inttargetRstToleranceR {
        Gpio071inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO071 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio071inttarget_wr_prot(&self) -> Gpio071inttargetWrProtR {
        Gpio071inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO068 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio068intto_int13018(&mut self) -> EnblGpio068inttoInt13018W<Gpioa54Spec> {
        EnblGpio068inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO068 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio068intto_int13019(&mut self) -> EnblGpio068inttoInt13019W<Gpioa54Spec> {
        EnblGpio068inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO068 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio068intto_int13020(&mut self) -> EnblGpio068inttoInt13020W<Gpioa54Spec> {
        EnblGpio068inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO068 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio068intto_sio(&mut self) -> EnblGpio068inttoSioW<Gpioa54Spec> {
        EnblGpio068inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa54Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa54Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO068 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio068inttarget_rst_tolerance(&mut self) -> Gpio068inttargetRstToleranceW<Gpioa54Spec> {
        Gpio068inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO068 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio068inttarget_wr_prot(&mut self) -> Gpio068inttargetWrProtW<Gpioa54Spec> {
        Gpio068inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO069 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio069intto_int13018(&mut self) -> EnblGpio069inttoInt13018W<Gpioa54Spec> {
        EnblGpio069inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO069 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio069intto_int13019(&mut self) -> EnblGpio069inttoInt13019W<Gpioa54Spec> {
        EnblGpio069inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO069 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio069intto_int13020(&mut self) -> EnblGpio069inttoInt13020W<Gpioa54Spec> {
        EnblGpio069inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO069 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio069intto_sio(&mut self) -> EnblGpio069inttoSioW<Gpioa54Spec> {
        EnblGpio069inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa54Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa54Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO069 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio069inttarget_rst_tolerance(&mut self) -> Gpio069inttargetRstToleranceW<Gpioa54Spec> {
        Gpio069inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO069 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio069inttarget_wr_prot(&mut self) -> Gpio069inttargetWrProtW<Gpioa54Spec> {
        Gpio069inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO070 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio070intto_int13018(&mut self) -> EnblGpio070inttoInt13018W<Gpioa54Spec> {
        EnblGpio070inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO070 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio070intto_int13019(&mut self) -> EnblGpio070inttoInt13019W<Gpioa54Spec> {
        EnblGpio070inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO070 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio070intto_int13020(&mut self) -> EnblGpio070inttoInt13020W<Gpioa54Spec> {
        EnblGpio070inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO070 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio070intto_sio(&mut self) -> EnblGpio070inttoSioW<Gpioa54Spec> {
        EnblGpio070inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa54Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa54Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO070 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio070inttarget_rst_tolerance(&mut self) -> Gpio070inttargetRstToleranceW<Gpioa54Spec> {
        Gpio070inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO070 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio070inttarget_wr_prot(&mut self) -> Gpio070inttargetWrProtW<Gpioa54Spec> {
        Gpio070inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO071 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio071intto_int13018(&mut self) -> EnblGpio071inttoInt13018W<Gpioa54Spec> {
        EnblGpio071inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO071 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio071intto_int13019(&mut self) -> EnblGpio071inttoInt13019W<Gpioa54Spec> {
        EnblGpio071inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO071 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio071intto_int13020(&mut self) -> EnblGpio071inttoInt13020W<Gpioa54Spec> {
        EnblGpio071inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO071 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio071intto_sio(&mut self) -> EnblGpio071inttoSioW<Gpioa54Spec> {
        EnblGpio071inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa54Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO071 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio071inttarget_rst_tolerance(&mut self) -> Gpio071inttargetRstToleranceW<Gpioa54Spec> {
        Gpio071inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO071 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio071inttarget_wr_prot(&mut self) -> Gpio071inttargetWrProtW<Gpioa54Spec> {
        Gpio071inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa54Spec;
impl crate::RegisterSpec for Gpioa54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa54::R`](R) reader structure"]
impl crate::Readable for Gpioa54Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa54::W`](W) writer structure"]
impl crate::Writable for Gpioa54Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA54 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa54Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
