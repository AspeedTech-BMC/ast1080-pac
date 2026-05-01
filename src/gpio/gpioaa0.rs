#[doc = "Register `GPIOAA0` reader"]
pub type R = crate::R<Gpioaa0Spec>;
#[doc = "Register `GPIOAA0` writer"]
pub type W = crate::W<Gpioaa0Spec>;
#[doc = "Field `EnblGPIO144INTToINT13018` reader - Enable GPIO144 Interrupt To INT#130_18"]
pub type EnblGpio144inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO144INTToINT13018` writer - Enable GPIO144 Interrupt To INT#130_18"]
pub type EnblGpio144inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO144INTToINT13019` reader - Enable GPIO144 Interrupt To INT#130_19"]
pub type EnblGpio144inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO144INTToINT13019` writer - Enable GPIO144 Interrupt To INT#130_19"]
pub type EnblGpio144inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO144INTToINT13020` reader - Enable GPIO144 Interrupt To INT#130_20"]
pub type EnblGpio144inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO144INTToINT13020` writer - Enable GPIO144 Interrupt To INT#130_20"]
pub type EnblGpio144inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO144INTToSIO` reader - Enable GPIO144 Interrupt To SIO"]
pub type EnblGpio144inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO144INTToSIO` writer - Enable GPIO144 Interrupt To SIO"]
pub type EnblGpio144inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO144 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio144inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio144inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio144inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO144INTTargetRstTolerance` reader - GPIO144 Interrupt Target Reset Tolerance"]
pub type Gpio144inttargetRstToleranceR = crate::BitReader<Gpio144inttargetRstTolerance>;
impl Gpio144inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio144inttargetRstTolerance {
        match self.bits {
            false => Gpio144inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio144inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio144inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio144inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO144INTTargetRstTolerance` writer - GPIO144 Interrupt Target Reset Tolerance"]
pub type Gpio144inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio144inttargetRstTolerance>;
impl<'a, REG> Gpio144inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio144inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio144inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO144INTTargetWrProt` reader - GPIO144 Interrupt Target Write Protection"]
pub type Gpio144inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO144INTTargetWrProt` writer - GPIO144 Interrupt Target Write Protection"]
pub type Gpio144inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO145INTToINT13018` reader - Enable GPIO145 Interrupt To INT#130_18"]
pub type EnblGpio145inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO145INTToINT13018` writer - Enable GPIO145 Interrupt To INT#130_18"]
pub type EnblGpio145inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO145INTToINT13019` reader - Enable GPIO145 Interrupt To INT#130_19"]
pub type EnblGpio145inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO145INTToINT13019` writer - Enable GPIO145 Interrupt To INT#130_19"]
pub type EnblGpio145inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO145INTToINT13020` reader - Enable GPIO145 Interrupt To INT#130_20"]
pub type EnblGpio145inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO145INTToINT13020` writer - Enable GPIO145 Interrupt To INT#130_20"]
pub type EnblGpio145inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO145INTToSIO` reader - Enable GPIO145 Interrupt To SIO"]
pub type EnblGpio145inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO145INTToSIO` writer - Enable GPIO145 Interrupt To SIO"]
pub type EnblGpio145inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO145 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio145inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio145inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio145inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO145INTTargetRstTolerance` reader - GPIO145 Interrupt Target Reset Tolerance"]
pub type Gpio145inttargetRstToleranceR = crate::BitReader<Gpio145inttargetRstTolerance>;
impl Gpio145inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio145inttargetRstTolerance {
        match self.bits {
            false => Gpio145inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio145inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio145inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio145inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO145INTTargetRstTolerance` writer - GPIO145 Interrupt Target Reset Tolerance"]
pub type Gpio145inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio145inttargetRstTolerance>;
impl<'a, REG> Gpio145inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio145inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio145inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO145INTTargetWrProt` reader - GPIO145 Interrupt Target Write Protection"]
pub type Gpio145inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO145INTTargetWrProt` writer - GPIO145 Interrupt Target Write Protection"]
pub type Gpio145inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO146INTToINT13018` reader - Enable GPIO146 Interrupt To INT#130_18"]
pub type EnblGpio146inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO146INTToINT13018` writer - Enable GPIO146 Interrupt To INT#130_18"]
pub type EnblGpio146inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO146INTToINT13019` reader - Enable GPIO146 Interrupt To INT#130_19"]
pub type EnblGpio146inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO146INTToINT13019` writer - Enable GPIO146 Interrupt To INT#130_19"]
pub type EnblGpio146inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO146INTToINT13020` reader - Enable GPIO146 Interrupt To INT#130_20"]
pub type EnblGpio146inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO146INTToINT13020` writer - Enable GPIO146 Interrupt To INT#130_20"]
pub type EnblGpio146inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO146INTToSIO` reader - Enable GPIO146 Interrupt To SIO"]
pub type EnblGpio146inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO146INTToSIO` writer - Enable GPIO146 Interrupt To SIO"]
pub type EnblGpio146inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO146 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio146inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio146inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio146inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO146INTTargetRstTolerance` reader - GPIO146 Interrupt Target Reset Tolerance"]
pub type Gpio146inttargetRstToleranceR = crate::BitReader<Gpio146inttargetRstTolerance>;
impl Gpio146inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio146inttargetRstTolerance {
        match self.bits {
            false => Gpio146inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio146inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio146inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio146inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO146INTTargetRstTolerance` writer - GPIO146 Interrupt Target Reset Tolerance"]
pub type Gpio146inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio146inttargetRstTolerance>;
impl<'a, REG> Gpio146inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio146inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio146inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO146INTTargetWrProt` reader - GPIO146 Interrupt Target Write Protection"]
pub type Gpio146inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO146INTTargetWrProt` writer - GPIO146 Interrupt Target Write Protection"]
pub type Gpio146inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO147INTToINT13018` reader - Enable GPIO147 Interrupt To INT#130_18"]
pub type EnblGpio147inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO147INTToINT13018` writer - Enable GPIO147 Interrupt To INT#130_18"]
pub type EnblGpio147inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO147INTToINT13019` reader - Enable GPIO147 Interrupt To INT#130_19"]
pub type EnblGpio147inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO147INTToINT13019` writer - Enable GPIO147 Interrupt To INT#130_19"]
pub type EnblGpio147inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO147INTToINT13020` reader - Enable GPIO147 Interrupt To INT#130_20"]
pub type EnblGpio147inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO147INTToINT13020` writer - Enable GPIO147 Interrupt To INT#130_20"]
pub type EnblGpio147inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO147INTToSIO` reader - Enable GPIO147 Interrupt To SIO"]
pub type EnblGpio147inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO147INTToSIO` writer - Enable GPIO147 Interrupt To SIO"]
pub type EnblGpio147inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO147 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio147inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio147inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio147inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO147INTTargetRstTolerance` reader - GPIO147 Interrupt Target Reset Tolerance"]
pub type Gpio147inttargetRstToleranceR = crate::BitReader<Gpio147inttargetRstTolerance>;
impl Gpio147inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio147inttargetRstTolerance {
        match self.bits {
            false => Gpio147inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio147inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio147inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio147inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO147INTTargetRstTolerance` writer - GPIO147 Interrupt Target Reset Tolerance"]
pub type Gpio147inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio147inttargetRstTolerance>;
impl<'a, REG> Gpio147inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio147inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio147inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO147INTTargetWrProt` reader - GPIO147 Interrupt Target Write Protection"]
pub type Gpio147inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO147INTTargetWrProt` writer - GPIO147 Interrupt Target Write Protection"]
pub type Gpio147inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO144 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio144intto_int13018(&self) -> EnblGpio144inttoInt13018R {
        EnblGpio144inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO144 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio144intto_int13019(&self) -> EnblGpio144inttoInt13019R {
        EnblGpio144inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO144 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio144intto_int13020(&self) -> EnblGpio144inttoInt13020R {
        EnblGpio144inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO144 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio144intto_sio(&self) -> EnblGpio144inttoSioR {
        EnblGpio144inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO144 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio144inttarget_rst_tolerance(&self) -> Gpio144inttargetRstToleranceR {
        Gpio144inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO144 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio144inttarget_wr_prot(&self) -> Gpio144inttargetWrProtR {
        Gpio144inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO145 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio145intto_int13018(&self) -> EnblGpio145inttoInt13018R {
        EnblGpio145inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO145 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio145intto_int13019(&self) -> EnblGpio145inttoInt13019R {
        EnblGpio145inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO145 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio145intto_int13020(&self) -> EnblGpio145inttoInt13020R {
        EnblGpio145inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO145 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio145intto_sio(&self) -> EnblGpio145inttoSioR {
        EnblGpio145inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO145 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio145inttarget_rst_tolerance(&self) -> Gpio145inttargetRstToleranceR {
        Gpio145inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO145 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio145inttarget_wr_prot(&self) -> Gpio145inttargetWrProtR {
        Gpio145inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO146 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio146intto_int13018(&self) -> EnblGpio146inttoInt13018R {
        EnblGpio146inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO146 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio146intto_int13019(&self) -> EnblGpio146inttoInt13019R {
        EnblGpio146inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO146 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio146intto_int13020(&self) -> EnblGpio146inttoInt13020R {
        EnblGpio146inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO146 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio146intto_sio(&self) -> EnblGpio146inttoSioR {
        EnblGpio146inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO146 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio146inttarget_rst_tolerance(&self) -> Gpio146inttargetRstToleranceR {
        Gpio146inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO146 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio146inttarget_wr_prot(&self) -> Gpio146inttargetWrProtR {
        Gpio146inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO147 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio147intto_int13018(&self) -> EnblGpio147inttoInt13018R {
        EnblGpio147inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO147 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio147intto_int13019(&self) -> EnblGpio147inttoInt13019R {
        EnblGpio147inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO147 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio147intto_int13020(&self) -> EnblGpio147inttoInt13020R {
        EnblGpio147inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO147 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio147intto_sio(&self) -> EnblGpio147inttoSioR {
        EnblGpio147inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO147 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio147inttarget_rst_tolerance(&self) -> Gpio147inttargetRstToleranceR {
        Gpio147inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO147 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio147inttarget_wr_prot(&self) -> Gpio147inttargetWrProtR {
        Gpio147inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO144 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio144intto_int13018(&mut self) -> EnblGpio144inttoInt13018W<Gpioaa0Spec> {
        EnblGpio144inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO144 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio144intto_int13019(&mut self) -> EnblGpio144inttoInt13019W<Gpioaa0Spec> {
        EnblGpio144inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO144 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio144intto_int13020(&mut self) -> EnblGpio144inttoInt13020W<Gpioaa0Spec> {
        EnblGpio144inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO144 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio144intto_sio(&mut self) -> EnblGpio144inttoSioW<Gpioaa0Spec> {
        EnblGpio144inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioaa0Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioaa0Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO144 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio144inttarget_rst_tolerance(&mut self) -> Gpio144inttargetRstToleranceW<Gpioaa0Spec> {
        Gpio144inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO144 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio144inttarget_wr_prot(&mut self) -> Gpio144inttargetWrProtW<Gpioaa0Spec> {
        Gpio144inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO145 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio145intto_int13018(&mut self) -> EnblGpio145inttoInt13018W<Gpioaa0Spec> {
        EnblGpio145inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO145 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio145intto_int13019(&mut self) -> EnblGpio145inttoInt13019W<Gpioaa0Spec> {
        EnblGpio145inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO145 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio145intto_int13020(&mut self) -> EnblGpio145inttoInt13020W<Gpioaa0Spec> {
        EnblGpio145inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO145 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio145intto_sio(&mut self) -> EnblGpio145inttoSioW<Gpioaa0Spec> {
        EnblGpio145inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioaa0Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioaa0Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO145 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio145inttarget_rst_tolerance(&mut self) -> Gpio145inttargetRstToleranceW<Gpioaa0Spec> {
        Gpio145inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO145 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio145inttarget_wr_prot(&mut self) -> Gpio145inttargetWrProtW<Gpioaa0Spec> {
        Gpio145inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO146 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio146intto_int13018(&mut self) -> EnblGpio146inttoInt13018W<Gpioaa0Spec> {
        EnblGpio146inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO146 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio146intto_int13019(&mut self) -> EnblGpio146inttoInt13019W<Gpioaa0Spec> {
        EnblGpio146inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO146 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio146intto_int13020(&mut self) -> EnblGpio146inttoInt13020W<Gpioaa0Spec> {
        EnblGpio146inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO146 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio146intto_sio(&mut self) -> EnblGpio146inttoSioW<Gpioaa0Spec> {
        EnblGpio146inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioaa0Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioaa0Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO146 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio146inttarget_rst_tolerance(&mut self) -> Gpio146inttargetRstToleranceW<Gpioaa0Spec> {
        Gpio146inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO146 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio146inttarget_wr_prot(&mut self) -> Gpio146inttargetWrProtW<Gpioaa0Spec> {
        Gpio146inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO147 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio147intto_int13018(&mut self) -> EnblGpio147inttoInt13018W<Gpioaa0Spec> {
        EnblGpio147inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO147 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio147intto_int13019(&mut self) -> EnblGpio147inttoInt13019W<Gpioaa0Spec> {
        EnblGpio147inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO147 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio147intto_int13020(&mut self) -> EnblGpio147inttoInt13020W<Gpioaa0Spec> {
        EnblGpio147inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO147 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio147intto_sio(&mut self) -> EnblGpio147inttoSioW<Gpioaa0Spec> {
        EnblGpio147inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioaa0Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO147 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio147inttarget_rst_tolerance(&mut self) -> Gpio147inttargetRstToleranceW<Gpioaa0Spec> {
        Gpio147inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO147 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio147inttarget_wr_prot(&mut self) -> Gpio147inttargetWrProtW<Gpioaa0Spec> {
        Gpio147inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioaa0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioaa0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioaa0Spec;
impl crate::RegisterSpec for Gpioaa0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioaa0::R`](R) reader structure"]
impl crate::Readable for Gpioaa0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioaa0::W`](W) writer structure"]
impl crate::Writable for Gpioaa0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOAA0 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioaa0Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
