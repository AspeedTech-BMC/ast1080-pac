#[doc = "Register `GPIOA80` reader"]
pub type R = crate::R<Gpioa80Spec>;
#[doc = "Register `GPIOA80` writer"]
pub type W = crate::W<Gpioa80Spec>;
#[doc = "Field `EnblGPIO112INTToINT13018` reader - Enable GPIO112 Interrupt To INT#130_18"]
pub type EnblGpio112inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO112INTToINT13018` writer - Enable GPIO112 Interrupt To INT#130_18"]
pub type EnblGpio112inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO112INTToINT13019` reader - Enable GPIO112 Interrupt To INT#130_19"]
pub type EnblGpio112inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO112INTToINT13019` writer - Enable GPIO112 Interrupt To INT#130_19"]
pub type EnblGpio112inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO112INTToINT13020` reader - Enable GPIO112 Interrupt To INT#130_20"]
pub type EnblGpio112inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO112INTToINT13020` writer - Enable GPIO112 Interrupt To INT#130_20"]
pub type EnblGpio112inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO112INTToSIO` reader - Enable GPIO112 Interrupt To SIO"]
pub type EnblGpio112inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO112INTToSIO` writer - Enable GPIO112 Interrupt To SIO"]
pub type EnblGpio112inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO112 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio112inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio112inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio112inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO112INTTargetRstTolerance` reader - GPIO112 Interrupt Target Reset Tolerance"]
pub type Gpio112inttargetRstToleranceR = crate::BitReader<Gpio112inttargetRstTolerance>;
impl Gpio112inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio112inttargetRstTolerance {
        match self.bits {
            false => Gpio112inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio112inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio112inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio112inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO112INTTargetRstTolerance` writer - GPIO112 Interrupt Target Reset Tolerance"]
pub type Gpio112inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio112inttargetRstTolerance>;
impl<'a, REG> Gpio112inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio112inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio112inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO112INTTargetWrProt` reader - GPIO112 Interrupt Target Write Protection"]
pub type Gpio112inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO112INTTargetWrProt` writer - GPIO112 Interrupt Target Write Protection"]
pub type Gpio112inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO113INTToINT13018` reader - Enable GPIO113 Interrupt To INT#130_18"]
pub type EnblGpio113inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO113INTToINT13018` writer - Enable GPIO113 Interrupt To INT#130_18"]
pub type EnblGpio113inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO113INTToINT13019` reader - Enable GPIO113 Interrupt To INT#130_19"]
pub type EnblGpio113inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO113INTToINT13019` writer - Enable GPIO113 Interrupt To INT#130_19"]
pub type EnblGpio113inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO113INTToINT13020` reader - Enable GPIO113 Interrupt To INT#130_20"]
pub type EnblGpio113inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO113INTToINT13020` writer - Enable GPIO113 Interrupt To INT#130_20"]
pub type EnblGpio113inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO113INTToSIO` reader - Enable GPIO113 Interrupt To SIO"]
pub type EnblGpio113inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO113INTToSIO` writer - Enable GPIO113 Interrupt To SIO"]
pub type EnblGpio113inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO113 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio113inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio113inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio113inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO113INTTargetRstTolerance` reader - GPIO113 Interrupt Target Reset Tolerance"]
pub type Gpio113inttargetRstToleranceR = crate::BitReader<Gpio113inttargetRstTolerance>;
impl Gpio113inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio113inttargetRstTolerance {
        match self.bits {
            false => Gpio113inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio113inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio113inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio113inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO113INTTargetRstTolerance` writer - GPIO113 Interrupt Target Reset Tolerance"]
pub type Gpio113inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio113inttargetRstTolerance>;
impl<'a, REG> Gpio113inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio113inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio113inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO113INTTargetWrProt` reader - GPIO113 Interrupt Target Write Protection"]
pub type Gpio113inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO113INTTargetWrProt` writer - GPIO113 Interrupt Target Write Protection"]
pub type Gpio113inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO114INTToINT13018` reader - Enable GPIO114 Interrupt To INT#130_18"]
pub type EnblGpio114inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO114INTToINT13018` writer - Enable GPIO114 Interrupt To INT#130_18"]
pub type EnblGpio114inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO114INTToINT13019` reader - Enable GPIO114 Interrupt To INT#130_19"]
pub type EnblGpio114inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO114INTToINT13019` writer - Enable GPIO114 Interrupt To INT#130_19"]
pub type EnblGpio114inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO114INTToINT13020` reader - Enable GPIO114 Interrupt To INT#130_20"]
pub type EnblGpio114inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO114INTToINT13020` writer - Enable GPIO114 Interrupt To INT#130_20"]
pub type EnblGpio114inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO114INTToSIO` reader - Enable GPIO114 Interrupt To SIO"]
pub type EnblGpio114inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO114INTToSIO` writer - Enable GPIO114 Interrupt To SIO"]
pub type EnblGpio114inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO114 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio114inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio114inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio114inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO114INTTargetRstTolerance` reader - GPIO114 Interrupt Target Reset Tolerance"]
pub type Gpio114inttargetRstToleranceR = crate::BitReader<Gpio114inttargetRstTolerance>;
impl Gpio114inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio114inttargetRstTolerance {
        match self.bits {
            false => Gpio114inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio114inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio114inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio114inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO114INTTargetRstTolerance` writer - GPIO114 Interrupt Target Reset Tolerance"]
pub type Gpio114inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio114inttargetRstTolerance>;
impl<'a, REG> Gpio114inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio114inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio114inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO114INTTargetWrProt` reader - GPIO114 Interrupt Target Write Protection"]
pub type Gpio114inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO114INTTargetWrProt` writer - GPIO114 Interrupt Target Write Protection"]
pub type Gpio114inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO115INTToINT13018` reader - Enable GPIO115 Interrupt To INT#130_18"]
pub type EnblGpio115inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO115INTToINT13018` writer - Enable GPIO115 Interrupt To INT#130_18"]
pub type EnblGpio115inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO115INTToINT13019` reader - Enable GPIO115 Interrupt To INT#130_19"]
pub type EnblGpio115inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO115INTToINT13019` writer - Enable GPIO115 Interrupt To INT#130_19"]
pub type EnblGpio115inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO115INTToINT13020` reader - Enable GPIO115 Interrupt To INT#130_20"]
pub type EnblGpio115inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO115INTToINT13020` writer - Enable GPIO115 Interrupt To INT#130_20"]
pub type EnblGpio115inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO115INTToSIO` reader - Enable GPIO115 Interrupt To SIO"]
pub type EnblGpio115inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO115INTToSIO` writer - Enable GPIO115 Interrupt To SIO"]
pub type EnblGpio115inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO115 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio115inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio115inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio115inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO115INTTargetRstTolerance` reader - GPIO115 Interrupt Target Reset Tolerance"]
pub type Gpio115inttargetRstToleranceR = crate::BitReader<Gpio115inttargetRstTolerance>;
impl Gpio115inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio115inttargetRstTolerance {
        match self.bits {
            false => Gpio115inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio115inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio115inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio115inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO115INTTargetRstTolerance` writer - GPIO115 Interrupt Target Reset Tolerance"]
pub type Gpio115inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio115inttargetRstTolerance>;
impl<'a, REG> Gpio115inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio115inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio115inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO115INTTargetWrProt` reader - GPIO115 Interrupt Target Write Protection"]
pub type Gpio115inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO115INTTargetWrProt` writer - GPIO115 Interrupt Target Write Protection"]
pub type Gpio115inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO112 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio112intto_int13018(&self) -> EnblGpio112inttoInt13018R {
        EnblGpio112inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO112 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio112intto_int13019(&self) -> EnblGpio112inttoInt13019R {
        EnblGpio112inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO112 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio112intto_int13020(&self) -> EnblGpio112inttoInt13020R {
        EnblGpio112inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO112 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio112intto_sio(&self) -> EnblGpio112inttoSioR {
        EnblGpio112inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO112 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio112inttarget_rst_tolerance(&self) -> Gpio112inttargetRstToleranceR {
        Gpio112inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO112 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio112inttarget_wr_prot(&self) -> Gpio112inttargetWrProtR {
        Gpio112inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO113 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio113intto_int13018(&self) -> EnblGpio113inttoInt13018R {
        EnblGpio113inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO113 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio113intto_int13019(&self) -> EnblGpio113inttoInt13019R {
        EnblGpio113inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO113 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio113intto_int13020(&self) -> EnblGpio113inttoInt13020R {
        EnblGpio113inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO113 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio113intto_sio(&self) -> EnblGpio113inttoSioR {
        EnblGpio113inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO113 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio113inttarget_rst_tolerance(&self) -> Gpio113inttargetRstToleranceR {
        Gpio113inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO113 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio113inttarget_wr_prot(&self) -> Gpio113inttargetWrProtR {
        Gpio113inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO114 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio114intto_int13018(&self) -> EnblGpio114inttoInt13018R {
        EnblGpio114inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO114 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio114intto_int13019(&self) -> EnblGpio114inttoInt13019R {
        EnblGpio114inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO114 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio114intto_int13020(&self) -> EnblGpio114inttoInt13020R {
        EnblGpio114inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO114 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio114intto_sio(&self) -> EnblGpio114inttoSioR {
        EnblGpio114inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO114 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio114inttarget_rst_tolerance(&self) -> Gpio114inttargetRstToleranceR {
        Gpio114inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO114 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio114inttarget_wr_prot(&self) -> Gpio114inttargetWrProtR {
        Gpio114inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO115 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio115intto_int13018(&self) -> EnblGpio115inttoInt13018R {
        EnblGpio115inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO115 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio115intto_int13019(&self) -> EnblGpio115inttoInt13019R {
        EnblGpio115inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO115 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio115intto_int13020(&self) -> EnblGpio115inttoInt13020R {
        EnblGpio115inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO115 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio115intto_sio(&self) -> EnblGpio115inttoSioR {
        EnblGpio115inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO115 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio115inttarget_rst_tolerance(&self) -> Gpio115inttargetRstToleranceR {
        Gpio115inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO115 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio115inttarget_wr_prot(&self) -> Gpio115inttargetWrProtR {
        Gpio115inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO112 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio112intto_int13018(&mut self) -> EnblGpio112inttoInt13018W<Gpioa80Spec> {
        EnblGpio112inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO112 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio112intto_int13019(&mut self) -> EnblGpio112inttoInt13019W<Gpioa80Spec> {
        EnblGpio112inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO112 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio112intto_int13020(&mut self) -> EnblGpio112inttoInt13020W<Gpioa80Spec> {
        EnblGpio112inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO112 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio112intto_sio(&mut self) -> EnblGpio112inttoSioW<Gpioa80Spec> {
        EnblGpio112inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa80Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa80Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO112 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio112inttarget_rst_tolerance(&mut self) -> Gpio112inttargetRstToleranceW<Gpioa80Spec> {
        Gpio112inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO112 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio112inttarget_wr_prot(&mut self) -> Gpio112inttargetWrProtW<Gpioa80Spec> {
        Gpio112inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO113 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio113intto_int13018(&mut self) -> EnblGpio113inttoInt13018W<Gpioa80Spec> {
        EnblGpio113inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO113 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio113intto_int13019(&mut self) -> EnblGpio113inttoInt13019W<Gpioa80Spec> {
        EnblGpio113inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO113 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio113intto_int13020(&mut self) -> EnblGpio113inttoInt13020W<Gpioa80Spec> {
        EnblGpio113inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO113 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio113intto_sio(&mut self) -> EnblGpio113inttoSioW<Gpioa80Spec> {
        EnblGpio113inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa80Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa80Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO113 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio113inttarget_rst_tolerance(&mut self) -> Gpio113inttargetRstToleranceW<Gpioa80Spec> {
        Gpio113inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO113 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio113inttarget_wr_prot(&mut self) -> Gpio113inttargetWrProtW<Gpioa80Spec> {
        Gpio113inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO114 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio114intto_int13018(&mut self) -> EnblGpio114inttoInt13018W<Gpioa80Spec> {
        EnblGpio114inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO114 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio114intto_int13019(&mut self) -> EnblGpio114inttoInt13019W<Gpioa80Spec> {
        EnblGpio114inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO114 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio114intto_int13020(&mut self) -> EnblGpio114inttoInt13020W<Gpioa80Spec> {
        EnblGpio114inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO114 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio114intto_sio(&mut self) -> EnblGpio114inttoSioW<Gpioa80Spec> {
        EnblGpio114inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa80Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa80Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO114 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio114inttarget_rst_tolerance(&mut self) -> Gpio114inttargetRstToleranceW<Gpioa80Spec> {
        Gpio114inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO114 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio114inttarget_wr_prot(&mut self) -> Gpio114inttargetWrProtW<Gpioa80Spec> {
        Gpio114inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO115 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio115intto_int13018(&mut self) -> EnblGpio115inttoInt13018W<Gpioa80Spec> {
        EnblGpio115inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO115 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio115intto_int13019(&mut self) -> EnblGpio115inttoInt13019W<Gpioa80Spec> {
        EnblGpio115inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO115 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio115intto_int13020(&mut self) -> EnblGpio115inttoInt13020W<Gpioa80Spec> {
        EnblGpio115inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO115 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio115intto_sio(&mut self) -> EnblGpio115inttoSioW<Gpioa80Spec> {
        EnblGpio115inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa80Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO115 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio115inttarget_rst_tolerance(&mut self) -> Gpio115inttargetRstToleranceW<Gpioa80Spec> {
        Gpio115inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO115 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio115inttarget_wr_prot(&mut self) -> Gpio115inttargetWrProtW<Gpioa80Spec> {
        Gpio115inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa80::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa80::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa80Spec;
impl crate::RegisterSpec for Gpioa80Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa80::R`](R) reader structure"]
impl crate::Readable for Gpioa80Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa80::W`](W) writer structure"]
impl crate::Writable for Gpioa80Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA80 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa80Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
