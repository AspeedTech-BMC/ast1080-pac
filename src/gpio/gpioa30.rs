#[doc = "Register `GPIOA30` reader"]
pub type R = crate::R<Gpioa30Spec>;
#[doc = "Register `GPIOA30` writer"]
pub type W = crate::W<Gpioa30Spec>;
#[doc = "Field `EnblGPIO032INTToINT13018` reader - Enable GPIO032 Interrupt To INT#130_18"]
pub type EnblGpio032inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO032INTToINT13018` writer - Enable GPIO032 Interrupt To INT#130_18"]
pub type EnblGpio032inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO032INTToINT13019` reader - Enable GPIO032 Interrupt To INT#130_19"]
pub type EnblGpio032inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO032INTToINT13019` writer - Enable GPIO032 Interrupt To INT#130_19"]
pub type EnblGpio032inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO032INTToINT13020` reader - Enable GPIO032 Interrupt To INT#130_20"]
pub type EnblGpio032inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO032INTToINT13020` writer - Enable GPIO032 Interrupt To INT#130_20"]
pub type EnblGpio032inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO032INTToSIO` reader - Enable GPIO032 Interrupt To SIO"]
pub type EnblGpio032inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO032INTToSIO` writer - Enable GPIO032 Interrupt To SIO"]
pub type EnblGpio032inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO032 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio032inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio032inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio032inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO032INTTargetRstTolerance` reader - GPIO032 Interrupt Target Reset Tolerance"]
pub type Gpio032inttargetRstToleranceR = crate::BitReader<Gpio032inttargetRstTolerance>;
impl Gpio032inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio032inttargetRstTolerance {
        match self.bits {
            false => Gpio032inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio032inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio032inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio032inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO032INTTargetRstTolerance` writer - GPIO032 Interrupt Target Reset Tolerance"]
pub type Gpio032inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio032inttargetRstTolerance>;
impl<'a, REG> Gpio032inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio032inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio032inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO032INTTargetWrProt` reader - GPIO032 Interrupt Target Write Protection"]
pub type Gpio032inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO032INTTargetWrProt` writer - GPIO032 Interrupt Target Write Protection"]
pub type Gpio032inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO033INTToINT13018` reader - Enable GPIO033 Interrupt To INT#130_18"]
pub type EnblGpio033inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO033INTToINT13018` writer - Enable GPIO033 Interrupt To INT#130_18"]
pub type EnblGpio033inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO033INTToINT13019` reader - Enable GPIO033 Interrupt To INT#130_19"]
pub type EnblGpio033inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO033INTToINT13019` writer - Enable GPIO033 Interrupt To INT#130_19"]
pub type EnblGpio033inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO033INTToINT13020` reader - Enable GPIO033 Interrupt To INT#130_20"]
pub type EnblGpio033inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO033INTToINT13020` writer - Enable GPIO033 Interrupt To INT#130_20"]
pub type EnblGpio033inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO033INTToSIO` reader - Enable GPIO033 Interrupt To SIO"]
pub type EnblGpio033inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO033INTToSIO` writer - Enable GPIO033 Interrupt To SIO"]
pub type EnblGpio033inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO033 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio033inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio033inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio033inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO033INTTargetRstTolerance` reader - GPIO033 Interrupt Target Reset Tolerance"]
pub type Gpio033inttargetRstToleranceR = crate::BitReader<Gpio033inttargetRstTolerance>;
impl Gpio033inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio033inttargetRstTolerance {
        match self.bits {
            false => Gpio033inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio033inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio033inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio033inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO033INTTargetRstTolerance` writer - GPIO033 Interrupt Target Reset Tolerance"]
pub type Gpio033inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio033inttargetRstTolerance>;
impl<'a, REG> Gpio033inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio033inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio033inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO033INTTargetWrProt` reader - GPIO033 Interrupt Target Write Protection"]
pub type Gpio033inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO033INTTargetWrProt` writer - GPIO033 Interrupt Target Write Protection"]
pub type Gpio033inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO034INTToINT13018` reader - Enable GPIO034 Interrupt To INT#130_18"]
pub type EnblGpio034inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO034INTToINT13018` writer - Enable GPIO034 Interrupt To INT#130_18"]
pub type EnblGpio034inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO034INTToINT13019` reader - Enable GPIO034 Interrupt To INT#130_19"]
pub type EnblGpio034inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO034INTToINT13019` writer - Enable GPIO034 Interrupt To INT#130_19"]
pub type EnblGpio034inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO034INTToINT13020` reader - Enable GPIO034 Interrupt To INT#130_20"]
pub type EnblGpio034inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO034INTToINT13020` writer - Enable GPIO034 Interrupt To INT#130_20"]
pub type EnblGpio034inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO034INTToSIO` reader - Enable GPIO034 Interrupt To SIO"]
pub type EnblGpio034inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO034INTToSIO` writer - Enable GPIO034 Interrupt To SIO"]
pub type EnblGpio034inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO034 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio034inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio034inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio034inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO034INTTargetRstTolerance` reader - GPIO034 Interrupt Target Reset Tolerance"]
pub type Gpio034inttargetRstToleranceR = crate::BitReader<Gpio034inttargetRstTolerance>;
impl Gpio034inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio034inttargetRstTolerance {
        match self.bits {
            false => Gpio034inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio034inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio034inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio034inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO034INTTargetRstTolerance` writer - GPIO034 Interrupt Target Reset Tolerance"]
pub type Gpio034inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio034inttargetRstTolerance>;
impl<'a, REG> Gpio034inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio034inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio034inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO034INTTargetWrProt` reader - GPIO034 Interrupt Target Write Protection"]
pub type Gpio034inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO034INTTargetWrProt` writer - GPIO034 Interrupt Target Write Protection"]
pub type Gpio034inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO035INTToINT13018` reader - Enable GPIO035 Interrupt To INT#130_18"]
pub type EnblGpio035inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO035INTToINT13018` writer - Enable GPIO035 Interrupt To INT#130_18"]
pub type EnblGpio035inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO035INTToINT13019` reader - Enable GPIO035 Interrupt To INT#130_19"]
pub type EnblGpio035inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO035INTToINT13019` writer - Enable GPIO035 Interrupt To INT#130_19"]
pub type EnblGpio035inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO035INTToINT13020` reader - Enable GPIO035 Interrupt To INT#130_20"]
pub type EnblGpio035inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO035INTToINT13020` writer - Enable GPIO035 Interrupt To INT#130_20"]
pub type EnblGpio035inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO035INTToSIO` reader - Enable GPIO035 Interrupt To SIO"]
pub type EnblGpio035inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO035INTToSIO` writer - Enable GPIO035 Interrupt To SIO"]
pub type EnblGpio035inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO035 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio035inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio035inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio035inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO035INTTargetRstTolerance` reader - GPIO035 Interrupt Target Reset Tolerance"]
pub type Gpio035inttargetRstToleranceR = crate::BitReader<Gpio035inttargetRstTolerance>;
impl Gpio035inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio035inttargetRstTolerance {
        match self.bits {
            false => Gpio035inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio035inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio035inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio035inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO035INTTargetRstTolerance` writer - GPIO035 Interrupt Target Reset Tolerance"]
pub type Gpio035inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio035inttargetRstTolerance>;
impl<'a, REG> Gpio035inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio035inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio035inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO035INTTargetWrProt` reader - GPIO035 Interrupt Target Write Protection"]
pub type Gpio035inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO035INTTargetWrProt` writer - GPIO035 Interrupt Target Write Protection"]
pub type Gpio035inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO032 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio032intto_int13018(&self) -> EnblGpio032inttoInt13018R {
        EnblGpio032inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO032 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio032intto_int13019(&self) -> EnblGpio032inttoInt13019R {
        EnblGpio032inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO032 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio032intto_int13020(&self) -> EnblGpio032inttoInt13020R {
        EnblGpio032inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO032 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio032intto_sio(&self) -> EnblGpio032inttoSioR {
        EnblGpio032inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO032 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio032inttarget_rst_tolerance(&self) -> Gpio032inttargetRstToleranceR {
        Gpio032inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO032 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio032inttarget_wr_prot(&self) -> Gpio032inttargetWrProtR {
        Gpio032inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO033 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio033intto_int13018(&self) -> EnblGpio033inttoInt13018R {
        EnblGpio033inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO033 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio033intto_int13019(&self) -> EnblGpio033inttoInt13019R {
        EnblGpio033inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO033 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio033intto_int13020(&self) -> EnblGpio033inttoInt13020R {
        EnblGpio033inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO033 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio033intto_sio(&self) -> EnblGpio033inttoSioR {
        EnblGpio033inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO033 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio033inttarget_rst_tolerance(&self) -> Gpio033inttargetRstToleranceR {
        Gpio033inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO033 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio033inttarget_wr_prot(&self) -> Gpio033inttargetWrProtR {
        Gpio033inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO034 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio034intto_int13018(&self) -> EnblGpio034inttoInt13018R {
        EnblGpio034inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO034 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio034intto_int13019(&self) -> EnblGpio034inttoInt13019R {
        EnblGpio034inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO034 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio034intto_int13020(&self) -> EnblGpio034inttoInt13020R {
        EnblGpio034inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO034 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio034intto_sio(&self) -> EnblGpio034inttoSioR {
        EnblGpio034inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO034 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio034inttarget_rst_tolerance(&self) -> Gpio034inttargetRstToleranceR {
        Gpio034inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO034 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio034inttarget_wr_prot(&self) -> Gpio034inttargetWrProtR {
        Gpio034inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO035 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio035intto_int13018(&self) -> EnblGpio035inttoInt13018R {
        EnblGpio035inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO035 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio035intto_int13019(&self) -> EnblGpio035inttoInt13019R {
        EnblGpio035inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO035 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio035intto_int13020(&self) -> EnblGpio035inttoInt13020R {
        EnblGpio035inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO035 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio035intto_sio(&self) -> EnblGpio035inttoSioR {
        EnblGpio035inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO035 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio035inttarget_rst_tolerance(&self) -> Gpio035inttargetRstToleranceR {
        Gpio035inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO035 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio035inttarget_wr_prot(&self) -> Gpio035inttargetWrProtR {
        Gpio035inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO032 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio032intto_int13018(&mut self) -> EnblGpio032inttoInt13018W<Gpioa30Spec> {
        EnblGpio032inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO032 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio032intto_int13019(&mut self) -> EnblGpio032inttoInt13019W<Gpioa30Spec> {
        EnblGpio032inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO032 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio032intto_int13020(&mut self) -> EnblGpio032inttoInt13020W<Gpioa30Spec> {
        EnblGpio032inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO032 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio032intto_sio(&mut self) -> EnblGpio032inttoSioW<Gpioa30Spec> {
        EnblGpio032inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa30Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa30Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO032 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio032inttarget_rst_tolerance(&mut self) -> Gpio032inttargetRstToleranceW<Gpioa30Spec> {
        Gpio032inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO032 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio032inttarget_wr_prot(&mut self) -> Gpio032inttargetWrProtW<Gpioa30Spec> {
        Gpio032inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO033 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio033intto_int13018(&mut self) -> EnblGpio033inttoInt13018W<Gpioa30Spec> {
        EnblGpio033inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO033 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio033intto_int13019(&mut self) -> EnblGpio033inttoInt13019W<Gpioa30Spec> {
        EnblGpio033inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO033 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio033intto_int13020(&mut self) -> EnblGpio033inttoInt13020W<Gpioa30Spec> {
        EnblGpio033inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO033 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio033intto_sio(&mut self) -> EnblGpio033inttoSioW<Gpioa30Spec> {
        EnblGpio033inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa30Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa30Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO033 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio033inttarget_rst_tolerance(&mut self) -> Gpio033inttargetRstToleranceW<Gpioa30Spec> {
        Gpio033inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO033 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio033inttarget_wr_prot(&mut self) -> Gpio033inttargetWrProtW<Gpioa30Spec> {
        Gpio033inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO034 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio034intto_int13018(&mut self) -> EnblGpio034inttoInt13018W<Gpioa30Spec> {
        EnblGpio034inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO034 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio034intto_int13019(&mut self) -> EnblGpio034inttoInt13019W<Gpioa30Spec> {
        EnblGpio034inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO034 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio034intto_int13020(&mut self) -> EnblGpio034inttoInt13020W<Gpioa30Spec> {
        EnblGpio034inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO034 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio034intto_sio(&mut self) -> EnblGpio034inttoSioW<Gpioa30Spec> {
        EnblGpio034inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa30Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa30Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO034 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio034inttarget_rst_tolerance(&mut self) -> Gpio034inttargetRstToleranceW<Gpioa30Spec> {
        Gpio034inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO034 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio034inttarget_wr_prot(&mut self) -> Gpio034inttargetWrProtW<Gpioa30Spec> {
        Gpio034inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO035 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio035intto_int13018(&mut self) -> EnblGpio035inttoInt13018W<Gpioa30Spec> {
        EnblGpio035inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO035 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio035intto_int13019(&mut self) -> EnblGpio035inttoInt13019W<Gpioa30Spec> {
        EnblGpio035inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO035 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio035intto_int13020(&mut self) -> EnblGpio035inttoInt13020W<Gpioa30Spec> {
        EnblGpio035inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO035 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio035intto_sio(&mut self) -> EnblGpio035inttoSioW<Gpioa30Spec> {
        EnblGpio035inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa30Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO035 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio035inttarget_rst_tolerance(&mut self) -> Gpio035inttargetRstToleranceW<Gpioa30Spec> {
        Gpio035inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO035 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio035inttarget_wr_prot(&mut self) -> Gpio035inttargetWrProtW<Gpioa30Spec> {
        Gpio035inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa30Spec;
impl crate::RegisterSpec for Gpioa30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa30::R`](R) reader structure"]
impl crate::Readable for Gpioa30Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa30::W`](W) writer structure"]
impl crate::Writable for Gpioa30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA30 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa30Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
