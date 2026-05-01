#[doc = "Register `GPIOA6C` reader"]
pub type R = crate::R<Gpioa6cSpec>;
#[doc = "Register `GPIOA6C` writer"]
pub type W = crate::W<Gpioa6cSpec>;
#[doc = "Field `EnblGPIO092INTToINT13018` reader - Enable GPIO092 Interrupt To INT#130_18"]
pub type EnblGpio092inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO092INTToINT13018` writer - Enable GPIO092 Interrupt To INT#130_18"]
pub type EnblGpio092inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO092INTToINT13019` reader - Enable GPIO092 Interrupt To INT#130_19"]
pub type EnblGpio092inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO092INTToINT13019` writer - Enable GPIO092 Interrupt To INT#130_19"]
pub type EnblGpio092inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO092INTToINT13020` reader - Enable GPIO092 Interrupt To INT#130_20"]
pub type EnblGpio092inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO092INTToINT13020` writer - Enable GPIO092 Interrupt To INT#130_20"]
pub type EnblGpio092inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO092INTToSIO` reader - Enable GPIO092 Interrupt To SIO"]
pub type EnblGpio092inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO092INTToSIO` writer - Enable GPIO092 Interrupt To SIO"]
pub type EnblGpio092inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO092 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio092inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio092inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio092inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO092INTTargetRstTolerance` reader - GPIO092 Interrupt Target Reset Tolerance"]
pub type Gpio092inttargetRstToleranceR = crate::BitReader<Gpio092inttargetRstTolerance>;
impl Gpio092inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio092inttargetRstTolerance {
        match self.bits {
            false => Gpio092inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio092inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio092inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio092inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO092INTTargetRstTolerance` writer - GPIO092 Interrupt Target Reset Tolerance"]
pub type Gpio092inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio092inttargetRstTolerance>;
impl<'a, REG> Gpio092inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio092inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio092inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO092INTTargetWrProt` reader - GPIO092 Interrupt Target Write Protection"]
pub type Gpio092inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO092INTTargetWrProt` writer - GPIO092 Interrupt Target Write Protection"]
pub type Gpio092inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO093INTToINT13018` reader - Enable GPIO093 Interrupt To INT#130_18"]
pub type EnblGpio093inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO093INTToINT13018` writer - Enable GPIO093 Interrupt To INT#130_18"]
pub type EnblGpio093inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO093INTToINT13019` reader - Enable GPIO093 Interrupt To INT#130_19"]
pub type EnblGpio093inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO093INTToINT13019` writer - Enable GPIO093 Interrupt To INT#130_19"]
pub type EnblGpio093inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO093INTToINT13020` reader - Enable GPIO093 Interrupt To INT#130_20"]
pub type EnblGpio093inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO093INTToINT13020` writer - Enable GPIO093 Interrupt To INT#130_20"]
pub type EnblGpio093inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO093INTToSIO` reader - Enable GPIO093 Interrupt To SIO"]
pub type EnblGpio093inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO093INTToSIO` writer - Enable GPIO093 Interrupt To SIO"]
pub type EnblGpio093inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO093 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio093inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio093inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio093inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO093INTTargetRstTolerance` reader - GPIO093 Interrupt Target Reset Tolerance"]
pub type Gpio093inttargetRstToleranceR = crate::BitReader<Gpio093inttargetRstTolerance>;
impl Gpio093inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio093inttargetRstTolerance {
        match self.bits {
            false => Gpio093inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio093inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio093inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio093inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO093INTTargetRstTolerance` writer - GPIO093 Interrupt Target Reset Tolerance"]
pub type Gpio093inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio093inttargetRstTolerance>;
impl<'a, REG> Gpio093inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio093inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio093inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO093INTTargetWrProt` reader - GPIO093 Interrupt Target Write Protection"]
pub type Gpio093inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO093INTTargetWrProt` writer - GPIO093 Interrupt Target Write Protection"]
pub type Gpio093inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO094INTToINT13018` reader - Enable GPIO094 Interrupt To INT#130_18"]
pub type EnblGpio094inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO094INTToINT13018` writer - Enable GPIO094 Interrupt To INT#130_18"]
pub type EnblGpio094inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO094INTToINT13019` reader - Enable GPIO094 Interrupt To INT#130_19"]
pub type EnblGpio094inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO094INTToINT13019` writer - Enable GPIO094 Interrupt To INT#130_19"]
pub type EnblGpio094inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO094INTToINT13020` reader - Enable GPIO094 Interrupt To INT#130_20"]
pub type EnblGpio094inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO094INTToINT13020` writer - Enable GPIO094 Interrupt To INT#130_20"]
pub type EnblGpio094inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO094INTToSIO` reader - Enable GPIO094 Interrupt To SIO"]
pub type EnblGpio094inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO094INTToSIO` writer - Enable GPIO094 Interrupt To SIO"]
pub type EnblGpio094inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO094 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio094inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio094inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio094inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO094INTTargetRstTolerance` reader - GPIO094 Interrupt Target Reset Tolerance"]
pub type Gpio094inttargetRstToleranceR = crate::BitReader<Gpio094inttargetRstTolerance>;
impl Gpio094inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio094inttargetRstTolerance {
        match self.bits {
            false => Gpio094inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio094inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio094inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio094inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO094INTTargetRstTolerance` writer - GPIO094 Interrupt Target Reset Tolerance"]
pub type Gpio094inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio094inttargetRstTolerance>;
impl<'a, REG> Gpio094inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio094inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio094inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO094INTTargetWrProt` reader - GPIO094 Interrupt Target Write Protection"]
pub type Gpio094inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO094INTTargetWrProt` writer - GPIO094 Interrupt Target Write Protection"]
pub type Gpio094inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO095INTToINT13018` reader - Enable GPIO095 Interrupt To INT#130_18"]
pub type EnblGpio095inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO095INTToINT13018` writer - Enable GPIO095 Interrupt To INT#130_18"]
pub type EnblGpio095inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO095INTToINT13019` reader - Enable GPIO095 Interrupt To INT#130_19"]
pub type EnblGpio095inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO095INTToINT13019` writer - Enable GPIO095 Interrupt To INT#130_19"]
pub type EnblGpio095inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO095INTToINT13020` reader - Enable GPIO095 Interrupt To INT#130_20"]
pub type EnblGpio095inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO095INTToINT13020` writer - Enable GPIO095 Interrupt To INT#130_20"]
pub type EnblGpio095inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO095INTToSIO` reader - Enable GPIO095 Interrupt To SIO"]
pub type EnblGpio095inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO095INTToSIO` writer - Enable GPIO095 Interrupt To SIO"]
pub type EnblGpio095inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO095 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio095inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio095inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio095inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO095INTTargetRstTolerance` reader - GPIO095 Interrupt Target Reset Tolerance"]
pub type Gpio095inttargetRstToleranceR = crate::BitReader<Gpio095inttargetRstTolerance>;
impl Gpio095inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio095inttargetRstTolerance {
        match self.bits {
            false => Gpio095inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio095inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio095inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio095inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO095INTTargetRstTolerance` writer - GPIO095 Interrupt Target Reset Tolerance"]
pub type Gpio095inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio095inttargetRstTolerance>;
impl<'a, REG> Gpio095inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio095inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio095inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO095INTTargetWrProt` reader - GPIO095 Interrupt Target Write Protection"]
pub type Gpio095inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO095INTTargetWrProt` writer - GPIO095 Interrupt Target Write Protection"]
pub type Gpio095inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO092 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio092intto_int13018(&self) -> EnblGpio092inttoInt13018R {
        EnblGpio092inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO092 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio092intto_int13019(&self) -> EnblGpio092inttoInt13019R {
        EnblGpio092inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO092 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio092intto_int13020(&self) -> EnblGpio092inttoInt13020R {
        EnblGpio092inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO092 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio092intto_sio(&self) -> EnblGpio092inttoSioR {
        EnblGpio092inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO092 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio092inttarget_rst_tolerance(&self) -> Gpio092inttargetRstToleranceR {
        Gpio092inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO092 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio092inttarget_wr_prot(&self) -> Gpio092inttargetWrProtR {
        Gpio092inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO093 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio093intto_int13018(&self) -> EnblGpio093inttoInt13018R {
        EnblGpio093inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO093 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio093intto_int13019(&self) -> EnblGpio093inttoInt13019R {
        EnblGpio093inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO093 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio093intto_int13020(&self) -> EnblGpio093inttoInt13020R {
        EnblGpio093inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO093 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio093intto_sio(&self) -> EnblGpio093inttoSioR {
        EnblGpio093inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO093 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio093inttarget_rst_tolerance(&self) -> Gpio093inttargetRstToleranceR {
        Gpio093inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO093 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio093inttarget_wr_prot(&self) -> Gpio093inttargetWrProtR {
        Gpio093inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO094 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio094intto_int13018(&self) -> EnblGpio094inttoInt13018R {
        EnblGpio094inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO094 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio094intto_int13019(&self) -> EnblGpio094inttoInt13019R {
        EnblGpio094inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO094 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio094intto_int13020(&self) -> EnblGpio094inttoInt13020R {
        EnblGpio094inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO094 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio094intto_sio(&self) -> EnblGpio094inttoSioR {
        EnblGpio094inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO094 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio094inttarget_rst_tolerance(&self) -> Gpio094inttargetRstToleranceR {
        Gpio094inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO094 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio094inttarget_wr_prot(&self) -> Gpio094inttargetWrProtR {
        Gpio094inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO095 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio095intto_int13018(&self) -> EnblGpio095inttoInt13018R {
        EnblGpio095inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO095 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio095intto_int13019(&self) -> EnblGpio095inttoInt13019R {
        EnblGpio095inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO095 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio095intto_int13020(&self) -> EnblGpio095inttoInt13020R {
        EnblGpio095inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO095 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio095intto_sio(&self) -> EnblGpio095inttoSioR {
        EnblGpio095inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO095 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio095inttarget_rst_tolerance(&self) -> Gpio095inttargetRstToleranceR {
        Gpio095inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO095 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio095inttarget_wr_prot(&self) -> Gpio095inttargetWrProtR {
        Gpio095inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO092 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio092intto_int13018(&mut self) -> EnblGpio092inttoInt13018W<Gpioa6cSpec> {
        EnblGpio092inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO092 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio092intto_int13019(&mut self) -> EnblGpio092inttoInt13019W<Gpioa6cSpec> {
        EnblGpio092inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO092 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio092intto_int13020(&mut self) -> EnblGpio092inttoInt13020W<Gpioa6cSpec> {
        EnblGpio092inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO092 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio092intto_sio(&mut self) -> EnblGpio092inttoSioW<Gpioa6cSpec> {
        EnblGpio092inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa6cSpec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa6cSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO092 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio092inttarget_rst_tolerance(&mut self) -> Gpio092inttargetRstToleranceW<Gpioa6cSpec> {
        Gpio092inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO092 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio092inttarget_wr_prot(&mut self) -> Gpio092inttargetWrProtW<Gpioa6cSpec> {
        Gpio092inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO093 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio093intto_int13018(&mut self) -> EnblGpio093inttoInt13018W<Gpioa6cSpec> {
        EnblGpio093inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO093 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio093intto_int13019(&mut self) -> EnblGpio093inttoInt13019W<Gpioa6cSpec> {
        EnblGpio093inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO093 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio093intto_int13020(&mut self) -> EnblGpio093inttoInt13020W<Gpioa6cSpec> {
        EnblGpio093inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO093 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio093intto_sio(&mut self) -> EnblGpio093inttoSioW<Gpioa6cSpec> {
        EnblGpio093inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa6cSpec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa6cSpec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO093 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio093inttarget_rst_tolerance(&mut self) -> Gpio093inttargetRstToleranceW<Gpioa6cSpec> {
        Gpio093inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO093 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio093inttarget_wr_prot(&mut self) -> Gpio093inttargetWrProtW<Gpioa6cSpec> {
        Gpio093inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO094 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio094intto_int13018(&mut self) -> EnblGpio094inttoInt13018W<Gpioa6cSpec> {
        EnblGpio094inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO094 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio094intto_int13019(&mut self) -> EnblGpio094inttoInt13019W<Gpioa6cSpec> {
        EnblGpio094inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO094 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio094intto_int13020(&mut self) -> EnblGpio094inttoInt13020W<Gpioa6cSpec> {
        EnblGpio094inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO094 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio094intto_sio(&mut self) -> EnblGpio094inttoSioW<Gpioa6cSpec> {
        EnblGpio094inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa6cSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa6cSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO094 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio094inttarget_rst_tolerance(&mut self) -> Gpio094inttargetRstToleranceW<Gpioa6cSpec> {
        Gpio094inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO094 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio094inttarget_wr_prot(&mut self) -> Gpio094inttargetWrProtW<Gpioa6cSpec> {
        Gpio094inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO095 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio095intto_int13018(&mut self) -> EnblGpio095inttoInt13018W<Gpioa6cSpec> {
        EnblGpio095inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO095 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio095intto_int13019(&mut self) -> EnblGpio095inttoInt13019W<Gpioa6cSpec> {
        EnblGpio095inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO095 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio095intto_int13020(&mut self) -> EnblGpio095inttoInt13020W<Gpioa6cSpec> {
        EnblGpio095inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO095 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio095intto_sio(&mut self) -> EnblGpio095inttoSioW<Gpioa6cSpec> {
        EnblGpio095inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa6cSpec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO095 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio095inttarget_rst_tolerance(&mut self) -> Gpio095inttargetRstToleranceW<Gpioa6cSpec> {
        Gpio095inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO095 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio095inttarget_wr_prot(&mut self) -> Gpio095inttargetWrProtW<Gpioa6cSpec> {
        Gpio095inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa6c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa6c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa6cSpec;
impl crate::RegisterSpec for Gpioa6cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa6c::R`](R) reader structure"]
impl crate::Readable for Gpioa6cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioa6c::W`](W) writer structure"]
impl crate::Writable for Gpioa6cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA6C to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa6cSpec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
