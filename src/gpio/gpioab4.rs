#[doc = "Register `GPIOAB4` reader"]
pub type R = crate::R<Gpioab4Spec>;
#[doc = "Register `GPIOAB4` writer"]
pub type W = crate::W<Gpioab4Spec>;
#[doc = "Field `EnblGPIO164INTToINT13018` reader - Enable GPIO164 Interrupt To INT#130_18"]
pub type EnblGpio164inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO164INTToINT13018` writer - Enable GPIO164 Interrupt To INT#130_18"]
pub type EnblGpio164inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO164INTToINT13019` reader - Enable GPIO164 Interrupt To INT#130_19"]
pub type EnblGpio164inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO164INTToINT13019` writer - Enable GPIO164 Interrupt To INT#130_19"]
pub type EnblGpio164inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO164INTToINT13020` reader - Enable GPIO164 Interrupt To INT#130_20"]
pub type EnblGpio164inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO164INTToINT13020` writer - Enable GPIO164 Interrupt To INT#130_20"]
pub type EnblGpio164inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO164INTToSIO` reader - Enable GPIO164 Interrupt To SIO"]
pub type EnblGpio164inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO164INTToSIO` writer - Enable GPIO164 Interrupt To SIO"]
pub type EnblGpio164inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO164 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio164inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio164inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio164inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO164INTTargetRstTolerance` reader - GPIO164 Interrupt Target Reset Tolerance"]
pub type Gpio164inttargetRstToleranceR = crate::BitReader<Gpio164inttargetRstTolerance>;
impl Gpio164inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio164inttargetRstTolerance {
        match self.bits {
            false => Gpio164inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio164inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio164inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio164inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO164INTTargetRstTolerance` writer - GPIO164 Interrupt Target Reset Tolerance"]
pub type Gpio164inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio164inttargetRstTolerance>;
impl<'a, REG> Gpio164inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio164inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio164inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO164INTTargetWrProt` reader - GPIO164 Interrupt Target Write Protection"]
pub type Gpio164inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO164INTTargetWrProt` writer - GPIO164 Interrupt Target Write Protection"]
pub type Gpio164inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO165INTToINT13018` reader - Enable GPIO165 Interrupt To INT#130_18"]
pub type EnblGpio165inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO165INTToINT13018` writer - Enable GPIO165 Interrupt To INT#130_18"]
pub type EnblGpio165inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO165INTToINT13019` reader - Enable GPIO165 Interrupt To INT#130_19"]
pub type EnblGpio165inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO165INTToINT13019` writer - Enable GPIO165 Interrupt To INT#130_19"]
pub type EnblGpio165inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO165INTToINT13020` reader - Enable GPIO165 Interrupt To INT#130_20"]
pub type EnblGpio165inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO165INTToINT13020` writer - Enable GPIO165 Interrupt To INT#130_20"]
pub type EnblGpio165inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO165INTToSIO` reader - Enable GPIO165 Interrupt To SIO"]
pub type EnblGpio165inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO165INTToSIO` writer - Enable GPIO165 Interrupt To SIO"]
pub type EnblGpio165inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO165 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio165inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio165inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio165inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO165INTTargetRstTolerance` reader - GPIO165 Interrupt Target Reset Tolerance"]
pub type Gpio165inttargetRstToleranceR = crate::BitReader<Gpio165inttargetRstTolerance>;
impl Gpio165inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio165inttargetRstTolerance {
        match self.bits {
            false => Gpio165inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio165inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio165inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio165inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO165INTTargetRstTolerance` writer - GPIO165 Interrupt Target Reset Tolerance"]
pub type Gpio165inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio165inttargetRstTolerance>;
impl<'a, REG> Gpio165inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio165inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio165inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO165INTTargetWrProt` reader - GPIO165 Interrupt Target Write Protection"]
pub type Gpio165inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO165INTTargetWrProt` writer - GPIO165 Interrupt Target Write Protection"]
pub type Gpio165inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO166INTToINT13018` reader - Enable GPIO166 Interrupt To INT#130_18"]
pub type EnblGpio166inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO166INTToINT13018` writer - Enable GPIO166 Interrupt To INT#130_18"]
pub type EnblGpio166inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO166INTToINT13019` reader - Enable GPIO166 Interrupt To INT#130_19"]
pub type EnblGpio166inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO166INTToINT13019` writer - Enable GPIO166 Interrupt To INT#130_19"]
pub type EnblGpio166inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO166INTToINT13020` reader - Enable GPIO166 Interrupt To INT#130_20"]
pub type EnblGpio166inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO166INTToINT13020` writer - Enable GPIO166 Interrupt To INT#130_20"]
pub type EnblGpio166inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO166INTToSIO` reader - Enable GPIO166 Interrupt To SIO"]
pub type EnblGpio166inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO166INTToSIO` writer - Enable GPIO166 Interrupt To SIO"]
pub type EnblGpio166inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO166 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio166inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio166inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio166inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO166INTTargetRstTolerance` reader - GPIO166 Interrupt Target Reset Tolerance"]
pub type Gpio166inttargetRstToleranceR = crate::BitReader<Gpio166inttargetRstTolerance>;
impl Gpio166inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio166inttargetRstTolerance {
        match self.bits {
            false => Gpio166inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio166inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio166inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio166inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO166INTTargetRstTolerance` writer - GPIO166 Interrupt Target Reset Tolerance"]
pub type Gpio166inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio166inttargetRstTolerance>;
impl<'a, REG> Gpio166inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio166inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio166inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO166INTTargetWrProt` reader - GPIO166 Interrupt Target Write Protection"]
pub type Gpio166inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO166INTTargetWrProt` writer - GPIO166 Interrupt Target Write Protection"]
pub type Gpio166inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO167INTToINT13018` reader - Enable GPIO167 Interrupt To INT#130_18"]
pub type EnblGpio167inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO167INTToINT13018` writer - Enable GPIO167 Interrupt To INT#130_18"]
pub type EnblGpio167inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO167INTToINT13019` reader - Enable GPIO167 Interrupt To INT#130_19"]
pub type EnblGpio167inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO167INTToINT13019` writer - Enable GPIO167 Interrupt To INT#130_19"]
pub type EnblGpio167inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO167INTToINT13020` reader - Enable GPIO167 Interrupt To INT#130_20"]
pub type EnblGpio167inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO167INTToINT13020` writer - Enable GPIO167 Interrupt To INT#130_20"]
pub type EnblGpio167inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO167INTToSIO` reader - Enable GPIO167 Interrupt To SIO"]
pub type EnblGpio167inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO167INTToSIO` writer - Enable GPIO167 Interrupt To SIO"]
pub type EnblGpio167inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO167 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio167inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio167inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio167inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO167INTTargetRstTolerance` reader - GPIO167 Interrupt Target Reset Tolerance"]
pub type Gpio167inttargetRstToleranceR = crate::BitReader<Gpio167inttargetRstTolerance>;
impl Gpio167inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio167inttargetRstTolerance {
        match self.bits {
            false => Gpio167inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio167inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio167inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio167inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO167INTTargetRstTolerance` writer - GPIO167 Interrupt Target Reset Tolerance"]
pub type Gpio167inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio167inttargetRstTolerance>;
impl<'a, REG> Gpio167inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio167inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio167inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO167INTTargetWrProt` reader - GPIO167 Interrupt Target Write Protection"]
pub type Gpio167inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO167INTTargetWrProt` writer - GPIO167 Interrupt Target Write Protection"]
pub type Gpio167inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO164 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio164intto_int13018(&self) -> EnblGpio164inttoInt13018R {
        EnblGpio164inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO164 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio164intto_int13019(&self) -> EnblGpio164inttoInt13019R {
        EnblGpio164inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO164 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio164intto_int13020(&self) -> EnblGpio164inttoInt13020R {
        EnblGpio164inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO164 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio164intto_sio(&self) -> EnblGpio164inttoSioR {
        EnblGpio164inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO164 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio164inttarget_rst_tolerance(&self) -> Gpio164inttargetRstToleranceR {
        Gpio164inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO164 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio164inttarget_wr_prot(&self) -> Gpio164inttargetWrProtR {
        Gpio164inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO165 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio165intto_int13018(&self) -> EnblGpio165inttoInt13018R {
        EnblGpio165inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO165 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio165intto_int13019(&self) -> EnblGpio165inttoInt13019R {
        EnblGpio165inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO165 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio165intto_int13020(&self) -> EnblGpio165inttoInt13020R {
        EnblGpio165inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO165 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio165intto_sio(&self) -> EnblGpio165inttoSioR {
        EnblGpio165inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO165 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio165inttarget_rst_tolerance(&self) -> Gpio165inttargetRstToleranceR {
        Gpio165inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO165 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio165inttarget_wr_prot(&self) -> Gpio165inttargetWrProtR {
        Gpio165inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO166 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio166intto_int13018(&self) -> EnblGpio166inttoInt13018R {
        EnblGpio166inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO166 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio166intto_int13019(&self) -> EnblGpio166inttoInt13019R {
        EnblGpio166inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO166 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio166intto_int13020(&self) -> EnblGpio166inttoInt13020R {
        EnblGpio166inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO166 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio166intto_sio(&self) -> EnblGpio166inttoSioR {
        EnblGpio166inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO166 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio166inttarget_rst_tolerance(&self) -> Gpio166inttargetRstToleranceR {
        Gpio166inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO166 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio166inttarget_wr_prot(&self) -> Gpio166inttargetWrProtR {
        Gpio166inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO167 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio167intto_int13018(&self) -> EnblGpio167inttoInt13018R {
        EnblGpio167inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO167 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio167intto_int13019(&self) -> EnblGpio167inttoInt13019R {
        EnblGpio167inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO167 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio167intto_int13020(&self) -> EnblGpio167inttoInt13020R {
        EnblGpio167inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO167 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio167intto_sio(&self) -> EnblGpio167inttoSioR {
        EnblGpio167inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO167 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio167inttarget_rst_tolerance(&self) -> Gpio167inttargetRstToleranceR {
        Gpio167inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO167 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio167inttarget_wr_prot(&self) -> Gpio167inttargetWrProtR {
        Gpio167inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO164 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio164intto_int13018(&mut self) -> EnblGpio164inttoInt13018W<Gpioab4Spec> {
        EnblGpio164inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO164 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio164intto_int13019(&mut self) -> EnblGpio164inttoInt13019W<Gpioab4Spec> {
        EnblGpio164inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO164 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio164intto_int13020(&mut self) -> EnblGpio164inttoInt13020W<Gpioab4Spec> {
        EnblGpio164inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO164 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio164intto_sio(&mut self) -> EnblGpio164inttoSioW<Gpioab4Spec> {
        EnblGpio164inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioab4Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioab4Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO164 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio164inttarget_rst_tolerance(&mut self) -> Gpio164inttargetRstToleranceW<Gpioab4Spec> {
        Gpio164inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO164 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio164inttarget_wr_prot(&mut self) -> Gpio164inttargetWrProtW<Gpioab4Spec> {
        Gpio164inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO165 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio165intto_int13018(&mut self) -> EnblGpio165inttoInt13018W<Gpioab4Spec> {
        EnblGpio165inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO165 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio165intto_int13019(&mut self) -> EnblGpio165inttoInt13019W<Gpioab4Spec> {
        EnblGpio165inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO165 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio165intto_int13020(&mut self) -> EnblGpio165inttoInt13020W<Gpioab4Spec> {
        EnblGpio165inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO165 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio165intto_sio(&mut self) -> EnblGpio165inttoSioW<Gpioab4Spec> {
        EnblGpio165inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioab4Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioab4Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO165 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio165inttarget_rst_tolerance(&mut self) -> Gpio165inttargetRstToleranceW<Gpioab4Spec> {
        Gpio165inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO165 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio165inttarget_wr_prot(&mut self) -> Gpio165inttargetWrProtW<Gpioab4Spec> {
        Gpio165inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO166 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio166intto_int13018(&mut self) -> EnblGpio166inttoInt13018W<Gpioab4Spec> {
        EnblGpio166inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO166 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio166intto_int13019(&mut self) -> EnblGpio166inttoInt13019W<Gpioab4Spec> {
        EnblGpio166inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO166 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio166intto_int13020(&mut self) -> EnblGpio166inttoInt13020W<Gpioab4Spec> {
        EnblGpio166inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO166 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio166intto_sio(&mut self) -> EnblGpio166inttoSioW<Gpioab4Spec> {
        EnblGpio166inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioab4Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioab4Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO166 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio166inttarget_rst_tolerance(&mut self) -> Gpio166inttargetRstToleranceW<Gpioab4Spec> {
        Gpio166inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO166 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio166inttarget_wr_prot(&mut self) -> Gpio166inttargetWrProtW<Gpioab4Spec> {
        Gpio166inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO167 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio167intto_int13018(&mut self) -> EnblGpio167inttoInt13018W<Gpioab4Spec> {
        EnblGpio167inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO167 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio167intto_int13019(&mut self) -> EnblGpio167inttoInt13019W<Gpioab4Spec> {
        EnblGpio167inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO167 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio167intto_int13020(&mut self) -> EnblGpio167inttoInt13020W<Gpioab4Spec> {
        EnblGpio167inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO167 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio167intto_sio(&mut self) -> EnblGpio167inttoSioW<Gpioab4Spec> {
        EnblGpio167inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioab4Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO167 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio167inttarget_rst_tolerance(&mut self) -> Gpio167inttargetRstToleranceW<Gpioab4Spec> {
        Gpio167inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO167 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio167inttarget_wr_prot(&mut self) -> Gpio167inttargetWrProtW<Gpioab4Spec> {
        Gpio167inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioab4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioab4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioab4Spec;
impl crate::RegisterSpec for Gpioab4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioab4::R`](R) reader structure"]
impl crate::Readable for Gpioab4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioab4::W`](W) writer structure"]
impl crate::Writable for Gpioab4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOAB4 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioab4Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
