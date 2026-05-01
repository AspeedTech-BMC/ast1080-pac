#[doc = "Register `GPIOA5C` reader"]
pub type R = crate::R<Gpioa5cSpec>;
#[doc = "Register `GPIOA5C` writer"]
pub type W = crate::W<Gpioa5cSpec>;
#[doc = "Field `EnblGPIO076INTToINT13018` reader - Enable GPIO076 Interrupt To INT#130_18"]
pub type EnblGpio076inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO076INTToINT13018` writer - Enable GPIO076 Interrupt To INT#130_18"]
pub type EnblGpio076inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO076INTToINT13019` reader - Enable GPIO076 Interrupt To INT#130_19"]
pub type EnblGpio076inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO076INTToINT13019` writer - Enable GPIO076 Interrupt To INT#130_19"]
pub type EnblGpio076inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO076INTToINT13020` reader - Enable GPIO076 Interrupt To INT#130_20"]
pub type EnblGpio076inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO076INTToINT13020` writer - Enable GPIO076 Interrupt To INT#130_20"]
pub type EnblGpio076inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO076INTToSIO` reader - Enable GPIO076 Interrupt To SIO"]
pub type EnblGpio076inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO076INTToSIO` writer - Enable GPIO076 Interrupt To SIO"]
pub type EnblGpio076inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO076 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio076inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio076inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio076inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO076INTTargetRstTolerance` reader - GPIO076 Interrupt Target Reset Tolerance"]
pub type Gpio076inttargetRstToleranceR = crate::BitReader<Gpio076inttargetRstTolerance>;
impl Gpio076inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio076inttargetRstTolerance {
        match self.bits {
            false => Gpio076inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio076inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio076inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio076inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO076INTTargetRstTolerance` writer - GPIO076 Interrupt Target Reset Tolerance"]
pub type Gpio076inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio076inttargetRstTolerance>;
impl<'a, REG> Gpio076inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio076inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio076inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO076INTTargetWrProt` reader - GPIO076 Interrupt Target Write Protection"]
pub type Gpio076inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO076INTTargetWrProt` writer - GPIO076 Interrupt Target Write Protection"]
pub type Gpio076inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO077INTToINT13018` reader - Enable GPIO077 Interrupt To INT#130_18"]
pub type EnblGpio077inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO077INTToINT13018` writer - Enable GPIO077 Interrupt To INT#130_18"]
pub type EnblGpio077inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO077INTToINT13019` reader - Enable GPIO077 Interrupt To INT#130_19"]
pub type EnblGpio077inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO077INTToINT13019` writer - Enable GPIO077 Interrupt To INT#130_19"]
pub type EnblGpio077inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO077INTToINT13020` reader - Enable GPIO077 Interrupt To INT#130_20"]
pub type EnblGpio077inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO077INTToINT13020` writer - Enable GPIO077 Interrupt To INT#130_20"]
pub type EnblGpio077inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO077INTToSIO` reader - Enable GPIO077 Interrupt To SIO"]
pub type EnblGpio077inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO077INTToSIO` writer - Enable GPIO077 Interrupt To SIO"]
pub type EnblGpio077inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO077 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio077inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio077inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio077inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO077INTTargetRstTolerance` reader - GPIO077 Interrupt Target Reset Tolerance"]
pub type Gpio077inttargetRstToleranceR = crate::BitReader<Gpio077inttargetRstTolerance>;
impl Gpio077inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio077inttargetRstTolerance {
        match self.bits {
            false => Gpio077inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio077inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio077inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio077inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO077INTTargetRstTolerance` writer - GPIO077 Interrupt Target Reset Tolerance"]
pub type Gpio077inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio077inttargetRstTolerance>;
impl<'a, REG> Gpio077inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio077inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio077inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO077INTTargetWrProt` reader - GPIO077 Interrupt Target Write Protection"]
pub type Gpio077inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO077INTTargetWrProt` writer - GPIO077 Interrupt Target Write Protection"]
pub type Gpio077inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO078INTToINT13018` reader - Enable GPIO078 Interrupt To INT#130_18"]
pub type EnblGpio078inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO078INTToINT13018` writer - Enable GPIO078 Interrupt To INT#130_18"]
pub type EnblGpio078inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO078INTToINT13019` reader - Enable GPIO078 Interrupt To INT#130_19"]
pub type EnblGpio078inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO078INTToINT13019` writer - Enable GPIO078 Interrupt To INT#130_19"]
pub type EnblGpio078inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO078INTToINT13020` reader - Enable GPIO078 Interrupt To INT#130_20"]
pub type EnblGpio078inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO078INTToINT13020` writer - Enable GPIO078 Interrupt To INT#130_20"]
pub type EnblGpio078inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO078INTToSIO` reader - Enable GPIO078 Interrupt To SIO"]
pub type EnblGpio078inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO078INTToSIO` writer - Enable GPIO078 Interrupt To SIO"]
pub type EnblGpio078inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO078 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio078inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio078inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio078inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO078INTTargetRstTolerance` reader - GPIO078 Interrupt Target Reset Tolerance"]
pub type Gpio078inttargetRstToleranceR = crate::BitReader<Gpio078inttargetRstTolerance>;
impl Gpio078inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio078inttargetRstTolerance {
        match self.bits {
            false => Gpio078inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio078inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio078inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio078inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO078INTTargetRstTolerance` writer - GPIO078 Interrupt Target Reset Tolerance"]
pub type Gpio078inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio078inttargetRstTolerance>;
impl<'a, REG> Gpio078inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio078inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio078inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO078INTTargetWrProt` reader - GPIO078 Interrupt Target Write Protection"]
pub type Gpio078inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO078INTTargetWrProt` writer - GPIO078 Interrupt Target Write Protection"]
pub type Gpio078inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO079INTToINT13018` reader - Enable GPIO079 Interrupt To INT#130_18"]
pub type EnblGpio079inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO079INTToINT13018` writer - Enable GPIO079 Interrupt To INT#130_18"]
pub type EnblGpio079inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO079INTToINT13019` reader - Enable GPIO079 Interrupt To INT#130_19"]
pub type EnblGpio079inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO079INTToINT13019` writer - Enable GPIO079 Interrupt To INT#130_19"]
pub type EnblGpio079inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO079INTToINT13020` reader - Enable GPIO079 Interrupt To INT#130_20"]
pub type EnblGpio079inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO079INTToINT13020` writer - Enable GPIO079 Interrupt To INT#130_20"]
pub type EnblGpio079inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO079INTToSIO` reader - Enable GPIO079 Interrupt To SIO"]
pub type EnblGpio079inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO079INTToSIO` writer - Enable GPIO079 Interrupt To SIO"]
pub type EnblGpio079inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO079 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio079inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio079inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio079inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO079INTTargetRstTolerance` reader - GPIO079 Interrupt Target Reset Tolerance"]
pub type Gpio079inttargetRstToleranceR = crate::BitReader<Gpio079inttargetRstTolerance>;
impl Gpio079inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio079inttargetRstTolerance {
        match self.bits {
            false => Gpio079inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio079inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio079inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio079inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO079INTTargetRstTolerance` writer - GPIO079 Interrupt Target Reset Tolerance"]
pub type Gpio079inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio079inttargetRstTolerance>;
impl<'a, REG> Gpio079inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio079inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio079inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO079INTTargetWrProt` reader - GPIO079 Interrupt Target Write Protection"]
pub type Gpio079inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO079INTTargetWrProt` writer - GPIO079 Interrupt Target Write Protection"]
pub type Gpio079inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO076 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio076intto_int13018(&self) -> EnblGpio076inttoInt13018R {
        EnblGpio076inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO076 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio076intto_int13019(&self) -> EnblGpio076inttoInt13019R {
        EnblGpio076inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO076 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio076intto_int13020(&self) -> EnblGpio076inttoInt13020R {
        EnblGpio076inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO076 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio076intto_sio(&self) -> EnblGpio076inttoSioR {
        EnblGpio076inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO076 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio076inttarget_rst_tolerance(&self) -> Gpio076inttargetRstToleranceR {
        Gpio076inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO076 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio076inttarget_wr_prot(&self) -> Gpio076inttargetWrProtR {
        Gpio076inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO077 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio077intto_int13018(&self) -> EnblGpio077inttoInt13018R {
        EnblGpio077inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO077 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio077intto_int13019(&self) -> EnblGpio077inttoInt13019R {
        EnblGpio077inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO077 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio077intto_int13020(&self) -> EnblGpio077inttoInt13020R {
        EnblGpio077inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO077 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio077intto_sio(&self) -> EnblGpio077inttoSioR {
        EnblGpio077inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO077 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio077inttarget_rst_tolerance(&self) -> Gpio077inttargetRstToleranceR {
        Gpio077inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO077 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio077inttarget_wr_prot(&self) -> Gpio077inttargetWrProtR {
        Gpio077inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO078 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio078intto_int13018(&self) -> EnblGpio078inttoInt13018R {
        EnblGpio078inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO078 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio078intto_int13019(&self) -> EnblGpio078inttoInt13019R {
        EnblGpio078inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO078 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio078intto_int13020(&self) -> EnblGpio078inttoInt13020R {
        EnblGpio078inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO078 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio078intto_sio(&self) -> EnblGpio078inttoSioR {
        EnblGpio078inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO078 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio078inttarget_rst_tolerance(&self) -> Gpio078inttargetRstToleranceR {
        Gpio078inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO078 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio078inttarget_wr_prot(&self) -> Gpio078inttargetWrProtR {
        Gpio078inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO079 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio079intto_int13018(&self) -> EnblGpio079inttoInt13018R {
        EnblGpio079inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO079 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio079intto_int13019(&self) -> EnblGpio079inttoInt13019R {
        EnblGpio079inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO079 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio079intto_int13020(&self) -> EnblGpio079inttoInt13020R {
        EnblGpio079inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO079 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio079intto_sio(&self) -> EnblGpio079inttoSioR {
        EnblGpio079inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO079 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio079inttarget_rst_tolerance(&self) -> Gpio079inttargetRstToleranceR {
        Gpio079inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO079 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio079inttarget_wr_prot(&self) -> Gpio079inttargetWrProtR {
        Gpio079inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO076 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio076intto_int13018(&mut self) -> EnblGpio076inttoInt13018W<Gpioa5cSpec> {
        EnblGpio076inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO076 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio076intto_int13019(&mut self) -> EnblGpio076inttoInt13019W<Gpioa5cSpec> {
        EnblGpio076inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO076 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio076intto_int13020(&mut self) -> EnblGpio076inttoInt13020W<Gpioa5cSpec> {
        EnblGpio076inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO076 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio076intto_sio(&mut self) -> EnblGpio076inttoSioW<Gpioa5cSpec> {
        EnblGpio076inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa5cSpec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa5cSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO076 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio076inttarget_rst_tolerance(&mut self) -> Gpio076inttargetRstToleranceW<Gpioa5cSpec> {
        Gpio076inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO076 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio076inttarget_wr_prot(&mut self) -> Gpio076inttargetWrProtW<Gpioa5cSpec> {
        Gpio076inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO077 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio077intto_int13018(&mut self) -> EnblGpio077inttoInt13018W<Gpioa5cSpec> {
        EnblGpio077inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO077 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio077intto_int13019(&mut self) -> EnblGpio077inttoInt13019W<Gpioa5cSpec> {
        EnblGpio077inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO077 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio077intto_int13020(&mut self) -> EnblGpio077inttoInt13020W<Gpioa5cSpec> {
        EnblGpio077inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO077 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio077intto_sio(&mut self) -> EnblGpio077inttoSioW<Gpioa5cSpec> {
        EnblGpio077inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa5cSpec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa5cSpec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO077 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio077inttarget_rst_tolerance(&mut self) -> Gpio077inttargetRstToleranceW<Gpioa5cSpec> {
        Gpio077inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO077 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio077inttarget_wr_prot(&mut self) -> Gpio077inttargetWrProtW<Gpioa5cSpec> {
        Gpio077inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO078 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio078intto_int13018(&mut self) -> EnblGpio078inttoInt13018W<Gpioa5cSpec> {
        EnblGpio078inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO078 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio078intto_int13019(&mut self) -> EnblGpio078inttoInt13019W<Gpioa5cSpec> {
        EnblGpio078inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO078 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio078intto_int13020(&mut self) -> EnblGpio078inttoInt13020W<Gpioa5cSpec> {
        EnblGpio078inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO078 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio078intto_sio(&mut self) -> EnblGpio078inttoSioW<Gpioa5cSpec> {
        EnblGpio078inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa5cSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa5cSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO078 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio078inttarget_rst_tolerance(&mut self) -> Gpio078inttargetRstToleranceW<Gpioa5cSpec> {
        Gpio078inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO078 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio078inttarget_wr_prot(&mut self) -> Gpio078inttargetWrProtW<Gpioa5cSpec> {
        Gpio078inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO079 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio079intto_int13018(&mut self) -> EnblGpio079inttoInt13018W<Gpioa5cSpec> {
        EnblGpio079inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO079 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio079intto_int13019(&mut self) -> EnblGpio079inttoInt13019W<Gpioa5cSpec> {
        EnblGpio079inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO079 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio079intto_int13020(&mut self) -> EnblGpio079inttoInt13020W<Gpioa5cSpec> {
        EnblGpio079inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO079 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio079intto_sio(&mut self) -> EnblGpio079inttoSioW<Gpioa5cSpec> {
        EnblGpio079inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa5cSpec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO079 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio079inttarget_rst_tolerance(&mut self) -> Gpio079inttargetRstToleranceW<Gpioa5cSpec> {
        Gpio079inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO079 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio079inttarget_wr_prot(&mut self) -> Gpio079inttargetWrProtW<Gpioa5cSpec> {
        Gpio079inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa5c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa5c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa5cSpec;
impl crate::RegisterSpec for Gpioa5cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa5c::R`](R) reader structure"]
impl crate::Readable for Gpioa5cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioa5c::W`](W) writer structure"]
impl crate::Writable for Gpioa5cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA5C to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa5cSpec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
