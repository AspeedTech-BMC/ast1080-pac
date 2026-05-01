#[doc = "Register `GPIOA94` reader"]
pub type R = crate::R<Gpioa94Spec>;
#[doc = "Register `GPIOA94` writer"]
pub type W = crate::W<Gpioa94Spec>;
#[doc = "Field `EnblGPIO132INTToINT13018` reader - Enable GPIO132 Interrupt To INT#130_18"]
pub type EnblGpio132inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO132INTToINT13018` writer - Enable GPIO132 Interrupt To INT#130_18"]
pub type EnblGpio132inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO132INTToINT13019` reader - Enable GPIO132 Interrupt To INT#130_19"]
pub type EnblGpio132inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO132INTToINT13019` writer - Enable GPIO132 Interrupt To INT#130_19"]
pub type EnblGpio132inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO132INTToINT13020` reader - Enable GPIO132 Interrupt To INT#130_20"]
pub type EnblGpio132inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO132INTToINT13020` writer - Enable GPIO132 Interrupt To INT#130_20"]
pub type EnblGpio132inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO132INTToSIO` reader - Enable GPIO132 Interrupt To SIO"]
pub type EnblGpio132inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO132INTToSIO` writer - Enable GPIO132 Interrupt To SIO"]
pub type EnblGpio132inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO132 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio132inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio132inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio132inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO132INTTargetRstTolerance` reader - GPIO132 Interrupt Target Reset Tolerance"]
pub type Gpio132inttargetRstToleranceR = crate::BitReader<Gpio132inttargetRstTolerance>;
impl Gpio132inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio132inttargetRstTolerance {
        match self.bits {
            false => Gpio132inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio132inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio132inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio132inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO132INTTargetRstTolerance` writer - GPIO132 Interrupt Target Reset Tolerance"]
pub type Gpio132inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio132inttargetRstTolerance>;
impl<'a, REG> Gpio132inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio132inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio132inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO132INTTargetWrProt` reader - GPIO132 Interrupt Target Write Protection"]
pub type Gpio132inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO132INTTargetWrProt` writer - GPIO132 Interrupt Target Write Protection"]
pub type Gpio132inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO133INTToINT13018` reader - Enable GPIO133 Interrupt To INT#130_18"]
pub type EnblGpio133inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO133INTToINT13018` writer - Enable GPIO133 Interrupt To INT#130_18"]
pub type EnblGpio133inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO133INTToINT13019` reader - Enable GPIO133 Interrupt To INT#130_19"]
pub type EnblGpio133inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO133INTToINT13019` writer - Enable GPIO133 Interrupt To INT#130_19"]
pub type EnblGpio133inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO133INTToINT13020` reader - Enable GPIO133 Interrupt To INT#130_20"]
pub type EnblGpio133inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO133INTToINT13020` writer - Enable GPIO133 Interrupt To INT#130_20"]
pub type EnblGpio133inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO133INTToSIO` reader - Enable GPIO133 Interrupt To SIO"]
pub type EnblGpio133inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO133INTToSIO` writer - Enable GPIO133 Interrupt To SIO"]
pub type EnblGpio133inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO133 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio133inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio133inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio133inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO133INTTargetRstTolerance` reader - GPIO133 Interrupt Target Reset Tolerance"]
pub type Gpio133inttargetRstToleranceR = crate::BitReader<Gpio133inttargetRstTolerance>;
impl Gpio133inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio133inttargetRstTolerance {
        match self.bits {
            false => Gpio133inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio133inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio133inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio133inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO133INTTargetRstTolerance` writer - GPIO133 Interrupt Target Reset Tolerance"]
pub type Gpio133inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio133inttargetRstTolerance>;
impl<'a, REG> Gpio133inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio133inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio133inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO133INTTargetWrProt` reader - GPIO133 Interrupt Target Write Protection"]
pub type Gpio133inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO133INTTargetWrProt` writer - GPIO133 Interrupt Target Write Protection"]
pub type Gpio133inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO134INTToINT13018` reader - Enable GPIO134 Interrupt To INT#130_18"]
pub type EnblGpio134inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO134INTToINT13018` writer - Enable GPIO134 Interrupt To INT#130_18"]
pub type EnblGpio134inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO134INTToINT13019` reader - Enable GPIO134 Interrupt To INT#130_19"]
pub type EnblGpio134inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO134INTToINT13019` writer - Enable GPIO134 Interrupt To INT#130_19"]
pub type EnblGpio134inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO134INTToINT13020` reader - Enable GPIO134 Interrupt To INT#130_20"]
pub type EnblGpio134inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO134INTToINT13020` writer - Enable GPIO134 Interrupt To INT#130_20"]
pub type EnblGpio134inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO134INTToSIO` reader - Enable GPIO134 Interrupt To SIO"]
pub type EnblGpio134inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO134INTToSIO` writer - Enable GPIO134 Interrupt To SIO"]
pub type EnblGpio134inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO134 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio134inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio134inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio134inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO134INTTargetRstTolerance` reader - GPIO134 Interrupt Target Reset Tolerance"]
pub type Gpio134inttargetRstToleranceR = crate::BitReader<Gpio134inttargetRstTolerance>;
impl Gpio134inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio134inttargetRstTolerance {
        match self.bits {
            false => Gpio134inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio134inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio134inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio134inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO134INTTargetRstTolerance` writer - GPIO134 Interrupt Target Reset Tolerance"]
pub type Gpio134inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio134inttargetRstTolerance>;
impl<'a, REG> Gpio134inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio134inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio134inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO134INTTargetWrProt` reader - GPIO134 Interrupt Target Write Protection"]
pub type Gpio134inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO134INTTargetWrProt` writer - GPIO134 Interrupt Target Write Protection"]
pub type Gpio134inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO135INTToINT13018` reader - Enable GPIO135 Interrupt To INT#130_18"]
pub type EnblGpio135inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO135INTToINT13018` writer - Enable GPIO135 Interrupt To INT#130_18"]
pub type EnblGpio135inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO135INTToINT13019` reader - Enable GPIO135 Interrupt To INT#130_19"]
pub type EnblGpio135inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO135INTToINT13019` writer - Enable GPIO135 Interrupt To INT#130_19"]
pub type EnblGpio135inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO135INTToINT13020` reader - Enable GPIO135 Interrupt To INT#130_20"]
pub type EnblGpio135inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO135INTToINT13020` writer - Enable GPIO135 Interrupt To INT#130_20"]
pub type EnblGpio135inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO135INTToSIO` reader - Enable GPIO135 Interrupt To SIO"]
pub type EnblGpio135inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO135INTToSIO` writer - Enable GPIO135 Interrupt To SIO"]
pub type EnblGpio135inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO135 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio135inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio135inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio135inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO135INTTargetRstTolerance` reader - GPIO135 Interrupt Target Reset Tolerance"]
pub type Gpio135inttargetRstToleranceR = crate::BitReader<Gpio135inttargetRstTolerance>;
impl Gpio135inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio135inttargetRstTolerance {
        match self.bits {
            false => Gpio135inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio135inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio135inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio135inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO135INTTargetRstTolerance` writer - GPIO135 Interrupt Target Reset Tolerance"]
pub type Gpio135inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio135inttargetRstTolerance>;
impl<'a, REG> Gpio135inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio135inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio135inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO135INTTargetWrProt` reader - GPIO135 Interrupt Target Write Protection"]
pub type Gpio135inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO135INTTargetWrProt` writer - GPIO135 Interrupt Target Write Protection"]
pub type Gpio135inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO132 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio132intto_int13018(&self) -> EnblGpio132inttoInt13018R {
        EnblGpio132inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO132 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio132intto_int13019(&self) -> EnblGpio132inttoInt13019R {
        EnblGpio132inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO132 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio132intto_int13020(&self) -> EnblGpio132inttoInt13020R {
        EnblGpio132inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO132 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio132intto_sio(&self) -> EnblGpio132inttoSioR {
        EnblGpio132inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO132 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio132inttarget_rst_tolerance(&self) -> Gpio132inttargetRstToleranceR {
        Gpio132inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO132 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio132inttarget_wr_prot(&self) -> Gpio132inttargetWrProtR {
        Gpio132inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO133 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio133intto_int13018(&self) -> EnblGpio133inttoInt13018R {
        EnblGpio133inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO133 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio133intto_int13019(&self) -> EnblGpio133inttoInt13019R {
        EnblGpio133inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO133 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio133intto_int13020(&self) -> EnblGpio133inttoInt13020R {
        EnblGpio133inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO133 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio133intto_sio(&self) -> EnblGpio133inttoSioR {
        EnblGpio133inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO133 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio133inttarget_rst_tolerance(&self) -> Gpio133inttargetRstToleranceR {
        Gpio133inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO133 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio133inttarget_wr_prot(&self) -> Gpio133inttargetWrProtR {
        Gpio133inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO134 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio134intto_int13018(&self) -> EnblGpio134inttoInt13018R {
        EnblGpio134inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO134 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio134intto_int13019(&self) -> EnblGpio134inttoInt13019R {
        EnblGpio134inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO134 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio134intto_int13020(&self) -> EnblGpio134inttoInt13020R {
        EnblGpio134inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO134 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio134intto_sio(&self) -> EnblGpio134inttoSioR {
        EnblGpio134inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO134 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio134inttarget_rst_tolerance(&self) -> Gpio134inttargetRstToleranceR {
        Gpio134inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO134 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio134inttarget_wr_prot(&self) -> Gpio134inttargetWrProtR {
        Gpio134inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO135 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio135intto_int13018(&self) -> EnblGpio135inttoInt13018R {
        EnblGpio135inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO135 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio135intto_int13019(&self) -> EnblGpio135inttoInt13019R {
        EnblGpio135inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO135 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio135intto_int13020(&self) -> EnblGpio135inttoInt13020R {
        EnblGpio135inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO135 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio135intto_sio(&self) -> EnblGpio135inttoSioR {
        EnblGpio135inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO135 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio135inttarget_rst_tolerance(&self) -> Gpio135inttargetRstToleranceR {
        Gpio135inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO135 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio135inttarget_wr_prot(&self) -> Gpio135inttargetWrProtR {
        Gpio135inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO132 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio132intto_int13018(&mut self) -> EnblGpio132inttoInt13018W<Gpioa94Spec> {
        EnblGpio132inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO132 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio132intto_int13019(&mut self) -> EnblGpio132inttoInt13019W<Gpioa94Spec> {
        EnblGpio132inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO132 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio132intto_int13020(&mut self) -> EnblGpio132inttoInt13020W<Gpioa94Spec> {
        EnblGpio132inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO132 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio132intto_sio(&mut self) -> EnblGpio132inttoSioW<Gpioa94Spec> {
        EnblGpio132inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa94Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa94Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO132 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio132inttarget_rst_tolerance(&mut self) -> Gpio132inttargetRstToleranceW<Gpioa94Spec> {
        Gpio132inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO132 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio132inttarget_wr_prot(&mut self) -> Gpio132inttargetWrProtW<Gpioa94Spec> {
        Gpio132inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO133 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio133intto_int13018(&mut self) -> EnblGpio133inttoInt13018W<Gpioa94Spec> {
        EnblGpio133inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO133 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio133intto_int13019(&mut self) -> EnblGpio133inttoInt13019W<Gpioa94Spec> {
        EnblGpio133inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO133 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio133intto_int13020(&mut self) -> EnblGpio133inttoInt13020W<Gpioa94Spec> {
        EnblGpio133inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO133 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio133intto_sio(&mut self) -> EnblGpio133inttoSioW<Gpioa94Spec> {
        EnblGpio133inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa94Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa94Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO133 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio133inttarget_rst_tolerance(&mut self) -> Gpio133inttargetRstToleranceW<Gpioa94Spec> {
        Gpio133inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO133 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio133inttarget_wr_prot(&mut self) -> Gpio133inttargetWrProtW<Gpioa94Spec> {
        Gpio133inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO134 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio134intto_int13018(&mut self) -> EnblGpio134inttoInt13018W<Gpioa94Spec> {
        EnblGpio134inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO134 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio134intto_int13019(&mut self) -> EnblGpio134inttoInt13019W<Gpioa94Spec> {
        EnblGpio134inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO134 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio134intto_int13020(&mut self) -> EnblGpio134inttoInt13020W<Gpioa94Spec> {
        EnblGpio134inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO134 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio134intto_sio(&mut self) -> EnblGpio134inttoSioW<Gpioa94Spec> {
        EnblGpio134inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa94Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa94Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO134 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio134inttarget_rst_tolerance(&mut self) -> Gpio134inttargetRstToleranceW<Gpioa94Spec> {
        Gpio134inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO134 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio134inttarget_wr_prot(&mut self) -> Gpio134inttargetWrProtW<Gpioa94Spec> {
        Gpio134inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO135 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio135intto_int13018(&mut self) -> EnblGpio135inttoInt13018W<Gpioa94Spec> {
        EnblGpio135inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO135 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio135intto_int13019(&mut self) -> EnblGpio135inttoInt13019W<Gpioa94Spec> {
        EnblGpio135inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO135 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio135intto_int13020(&mut self) -> EnblGpio135inttoInt13020W<Gpioa94Spec> {
        EnblGpio135inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO135 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio135intto_sio(&mut self) -> EnblGpio135inttoSioW<Gpioa94Spec> {
        EnblGpio135inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa94Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO135 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio135inttarget_rst_tolerance(&mut self) -> Gpio135inttargetRstToleranceW<Gpioa94Spec> {
        Gpio135inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO135 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio135inttarget_wr_prot(&mut self) -> Gpio135inttargetWrProtW<Gpioa94Spec> {
        Gpio135inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa94::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa94::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa94Spec;
impl crate::RegisterSpec for Gpioa94Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa94::R`](R) reader structure"]
impl crate::Readable for Gpioa94Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa94::W`](W) writer structure"]
impl crate::Writable for Gpioa94Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA94 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa94Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
