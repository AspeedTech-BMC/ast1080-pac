#[doc = "Register `GPIOAB0` reader"]
pub type R = crate::R<Gpioab0Spec>;
#[doc = "Register `GPIOAB0` writer"]
pub type W = crate::W<Gpioab0Spec>;
#[doc = "Field `EnblGPIO160INTToINT13018` reader - Enable GPIO160 Interrupt To INT#130_18"]
pub type EnblGpio160inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO160INTToINT13018` writer - Enable GPIO160 Interrupt To INT#130_18"]
pub type EnblGpio160inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO160INTToINT13019` reader - Enable GPIO160 Interrupt To INT#130_19"]
pub type EnblGpio160inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO160INTToINT13019` writer - Enable GPIO160 Interrupt To INT#130_19"]
pub type EnblGpio160inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO160INTToINT13020` reader - Enable GPIO160 Interrupt To INT#130_20"]
pub type EnblGpio160inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO160INTToINT13020` writer - Enable GPIO160 Interrupt To INT#130_20"]
pub type EnblGpio160inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO160INTToSIO` reader - Enable GPIO160 Interrupt To SIO"]
pub type EnblGpio160inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO160INTToSIO` writer - Enable GPIO160 Interrupt To SIO"]
pub type EnblGpio160inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO160 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio160inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio160inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio160inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO160INTTargetRstTolerance` reader - GPIO160 Interrupt Target Reset Tolerance"]
pub type Gpio160inttargetRstToleranceR = crate::BitReader<Gpio160inttargetRstTolerance>;
impl Gpio160inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio160inttargetRstTolerance {
        match self.bits {
            false => Gpio160inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio160inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio160inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio160inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO160INTTargetRstTolerance` writer - GPIO160 Interrupt Target Reset Tolerance"]
pub type Gpio160inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio160inttargetRstTolerance>;
impl<'a, REG> Gpio160inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio160inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio160inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO160INTTargetWrProt` reader - GPIO160 Interrupt Target Write Protection"]
pub type Gpio160inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO160INTTargetWrProt` writer - GPIO160 Interrupt Target Write Protection"]
pub type Gpio160inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO161INTToINT13018` reader - Enable GPIO161 Interrupt To INT#130_18"]
pub type EnblGpio161inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO161INTToINT13018` writer - Enable GPIO161 Interrupt To INT#130_18"]
pub type EnblGpio161inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO161INTToINT13019` reader - Enable GPIO161 Interrupt To INT#130_19"]
pub type EnblGpio161inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO161INTToINT13019` writer - Enable GPIO161 Interrupt To INT#130_19"]
pub type EnblGpio161inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO161INTToINT13020` reader - Enable GPIO161 Interrupt To INT#130_20"]
pub type EnblGpio161inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO161INTToINT13020` writer - Enable GPIO161 Interrupt To INT#130_20"]
pub type EnblGpio161inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO161INTToSIO` reader - Enable GPIO161 Interrupt To SIO"]
pub type EnblGpio161inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO161INTToSIO` writer - Enable GPIO161 Interrupt To SIO"]
pub type EnblGpio161inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO161 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio161inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio161inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio161inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO161INTTargetRstTolerance` reader - GPIO161 Interrupt Target Reset Tolerance"]
pub type Gpio161inttargetRstToleranceR = crate::BitReader<Gpio161inttargetRstTolerance>;
impl Gpio161inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio161inttargetRstTolerance {
        match self.bits {
            false => Gpio161inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio161inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio161inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio161inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO161INTTargetRstTolerance` writer - GPIO161 Interrupt Target Reset Tolerance"]
pub type Gpio161inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio161inttargetRstTolerance>;
impl<'a, REG> Gpio161inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio161inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio161inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO161INTTargetWrProt` reader - GPIO161 Interrupt Target Write Protection"]
pub type Gpio161inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO161INTTargetWrProt` writer - GPIO161 Interrupt Target Write Protection"]
pub type Gpio161inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO162INTToINT13018` reader - Enable GPIO162 Interrupt To INT#130_18"]
pub type EnblGpio162inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO162INTToINT13018` writer - Enable GPIO162 Interrupt To INT#130_18"]
pub type EnblGpio162inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO162INTToINT13019` reader - Enable GPIO162 Interrupt To INT#130_19"]
pub type EnblGpio162inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO162INTToINT13019` writer - Enable GPIO162 Interrupt To INT#130_19"]
pub type EnblGpio162inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO162INTToINT13020` reader - Enable GPIO162 Interrupt To INT#130_20"]
pub type EnblGpio162inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO162INTToINT13020` writer - Enable GPIO162 Interrupt To INT#130_20"]
pub type EnblGpio162inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO162INTToSIO` reader - Enable GPIO162 Interrupt To SIO"]
pub type EnblGpio162inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO162INTToSIO` writer - Enable GPIO162 Interrupt To SIO"]
pub type EnblGpio162inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO162 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio162inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio162inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio162inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO162INTTargetRstTolerance` reader - GPIO162 Interrupt Target Reset Tolerance"]
pub type Gpio162inttargetRstToleranceR = crate::BitReader<Gpio162inttargetRstTolerance>;
impl Gpio162inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio162inttargetRstTolerance {
        match self.bits {
            false => Gpio162inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio162inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio162inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio162inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO162INTTargetRstTolerance` writer - GPIO162 Interrupt Target Reset Tolerance"]
pub type Gpio162inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio162inttargetRstTolerance>;
impl<'a, REG> Gpio162inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio162inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio162inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO162INTTargetWrProt` reader - GPIO162 Interrupt Target Write Protection"]
pub type Gpio162inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO162INTTargetWrProt` writer - GPIO162 Interrupt Target Write Protection"]
pub type Gpio162inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO163INTToINT13018` reader - Enable GPIO163 Interrupt To INT#130_18"]
pub type EnblGpio163inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO163INTToINT13018` writer - Enable GPIO163 Interrupt To INT#130_18"]
pub type EnblGpio163inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO163INTToINT13019` reader - Enable GPIO163 Interrupt To INT#130_19"]
pub type EnblGpio163inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO163INTToINT13019` writer - Enable GPIO163 Interrupt To INT#130_19"]
pub type EnblGpio163inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO163INTToINT13020` reader - Enable GPIO163 Interrupt To INT#130_20"]
pub type EnblGpio163inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO163INTToINT13020` writer - Enable GPIO163 Interrupt To INT#130_20"]
pub type EnblGpio163inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO163INTToSIO` reader - Enable GPIO163 Interrupt To SIO"]
pub type EnblGpio163inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO163INTToSIO` writer - Enable GPIO163 Interrupt To SIO"]
pub type EnblGpio163inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO163 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio163inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio163inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio163inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO163INTTargetRstTolerance` reader - GPIO163 Interrupt Target Reset Tolerance"]
pub type Gpio163inttargetRstToleranceR = crate::BitReader<Gpio163inttargetRstTolerance>;
impl Gpio163inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio163inttargetRstTolerance {
        match self.bits {
            false => Gpio163inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio163inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio163inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio163inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO163INTTargetRstTolerance` writer - GPIO163 Interrupt Target Reset Tolerance"]
pub type Gpio163inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio163inttargetRstTolerance>;
impl<'a, REG> Gpio163inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio163inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio163inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO163INTTargetWrProt` reader - GPIO163 Interrupt Target Write Protection"]
pub type Gpio163inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO163INTTargetWrProt` writer - GPIO163 Interrupt Target Write Protection"]
pub type Gpio163inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO160 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio160intto_int13018(&self) -> EnblGpio160inttoInt13018R {
        EnblGpio160inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO160 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio160intto_int13019(&self) -> EnblGpio160inttoInt13019R {
        EnblGpio160inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO160 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio160intto_int13020(&self) -> EnblGpio160inttoInt13020R {
        EnblGpio160inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO160 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio160intto_sio(&self) -> EnblGpio160inttoSioR {
        EnblGpio160inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO160 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio160inttarget_rst_tolerance(&self) -> Gpio160inttargetRstToleranceR {
        Gpio160inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO160 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio160inttarget_wr_prot(&self) -> Gpio160inttargetWrProtR {
        Gpio160inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO161 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio161intto_int13018(&self) -> EnblGpio161inttoInt13018R {
        EnblGpio161inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO161 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio161intto_int13019(&self) -> EnblGpio161inttoInt13019R {
        EnblGpio161inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO161 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio161intto_int13020(&self) -> EnblGpio161inttoInt13020R {
        EnblGpio161inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO161 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio161intto_sio(&self) -> EnblGpio161inttoSioR {
        EnblGpio161inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO161 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio161inttarget_rst_tolerance(&self) -> Gpio161inttargetRstToleranceR {
        Gpio161inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO161 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio161inttarget_wr_prot(&self) -> Gpio161inttargetWrProtR {
        Gpio161inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO162 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio162intto_int13018(&self) -> EnblGpio162inttoInt13018R {
        EnblGpio162inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO162 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio162intto_int13019(&self) -> EnblGpio162inttoInt13019R {
        EnblGpio162inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO162 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio162intto_int13020(&self) -> EnblGpio162inttoInt13020R {
        EnblGpio162inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO162 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio162intto_sio(&self) -> EnblGpio162inttoSioR {
        EnblGpio162inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO162 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio162inttarget_rst_tolerance(&self) -> Gpio162inttargetRstToleranceR {
        Gpio162inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO162 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio162inttarget_wr_prot(&self) -> Gpio162inttargetWrProtR {
        Gpio162inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO163 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio163intto_int13018(&self) -> EnblGpio163inttoInt13018R {
        EnblGpio163inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO163 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio163intto_int13019(&self) -> EnblGpio163inttoInt13019R {
        EnblGpio163inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO163 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio163intto_int13020(&self) -> EnblGpio163inttoInt13020R {
        EnblGpio163inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO163 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio163intto_sio(&self) -> EnblGpio163inttoSioR {
        EnblGpio163inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO163 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio163inttarget_rst_tolerance(&self) -> Gpio163inttargetRstToleranceR {
        Gpio163inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO163 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio163inttarget_wr_prot(&self) -> Gpio163inttargetWrProtR {
        Gpio163inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO160 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio160intto_int13018(&mut self) -> EnblGpio160inttoInt13018W<Gpioab0Spec> {
        EnblGpio160inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO160 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio160intto_int13019(&mut self) -> EnblGpio160inttoInt13019W<Gpioab0Spec> {
        EnblGpio160inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO160 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio160intto_int13020(&mut self) -> EnblGpio160inttoInt13020W<Gpioab0Spec> {
        EnblGpio160inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO160 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio160intto_sio(&mut self) -> EnblGpio160inttoSioW<Gpioab0Spec> {
        EnblGpio160inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioab0Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioab0Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO160 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio160inttarget_rst_tolerance(&mut self) -> Gpio160inttargetRstToleranceW<Gpioab0Spec> {
        Gpio160inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO160 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio160inttarget_wr_prot(&mut self) -> Gpio160inttargetWrProtW<Gpioab0Spec> {
        Gpio160inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO161 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio161intto_int13018(&mut self) -> EnblGpio161inttoInt13018W<Gpioab0Spec> {
        EnblGpio161inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO161 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio161intto_int13019(&mut self) -> EnblGpio161inttoInt13019W<Gpioab0Spec> {
        EnblGpio161inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO161 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio161intto_int13020(&mut self) -> EnblGpio161inttoInt13020W<Gpioab0Spec> {
        EnblGpio161inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO161 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio161intto_sio(&mut self) -> EnblGpio161inttoSioW<Gpioab0Spec> {
        EnblGpio161inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioab0Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioab0Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO161 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio161inttarget_rst_tolerance(&mut self) -> Gpio161inttargetRstToleranceW<Gpioab0Spec> {
        Gpio161inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO161 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio161inttarget_wr_prot(&mut self) -> Gpio161inttargetWrProtW<Gpioab0Spec> {
        Gpio161inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO162 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio162intto_int13018(&mut self) -> EnblGpio162inttoInt13018W<Gpioab0Spec> {
        EnblGpio162inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO162 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio162intto_int13019(&mut self) -> EnblGpio162inttoInt13019W<Gpioab0Spec> {
        EnblGpio162inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO162 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio162intto_int13020(&mut self) -> EnblGpio162inttoInt13020W<Gpioab0Spec> {
        EnblGpio162inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO162 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio162intto_sio(&mut self) -> EnblGpio162inttoSioW<Gpioab0Spec> {
        EnblGpio162inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioab0Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioab0Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO162 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio162inttarget_rst_tolerance(&mut self) -> Gpio162inttargetRstToleranceW<Gpioab0Spec> {
        Gpio162inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO162 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio162inttarget_wr_prot(&mut self) -> Gpio162inttargetWrProtW<Gpioab0Spec> {
        Gpio162inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO163 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio163intto_int13018(&mut self) -> EnblGpio163inttoInt13018W<Gpioab0Spec> {
        EnblGpio163inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO163 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio163intto_int13019(&mut self) -> EnblGpio163inttoInt13019W<Gpioab0Spec> {
        EnblGpio163inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO163 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio163intto_int13020(&mut self) -> EnblGpio163inttoInt13020W<Gpioab0Spec> {
        EnblGpio163inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO163 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio163intto_sio(&mut self) -> EnblGpio163inttoSioW<Gpioab0Spec> {
        EnblGpio163inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioab0Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO163 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio163inttarget_rst_tolerance(&mut self) -> Gpio163inttargetRstToleranceW<Gpioab0Spec> {
        Gpio163inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO163 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio163inttarget_wr_prot(&mut self) -> Gpio163inttargetWrProtW<Gpioab0Spec> {
        Gpio163inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioab0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioab0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioab0Spec;
impl crate::RegisterSpec for Gpioab0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioab0::R`](R) reader structure"]
impl crate::Readable for Gpioab0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioab0::W`](W) writer structure"]
impl crate::Writable for Gpioab0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOAB0 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioab0Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
