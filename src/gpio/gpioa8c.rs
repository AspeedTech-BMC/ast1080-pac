#[doc = "Register `GPIOA8C` reader"]
pub type R = crate::R<Gpioa8cSpec>;
#[doc = "Register `GPIOA8C` writer"]
pub type W = crate::W<Gpioa8cSpec>;
#[doc = "Field `EnblGPIO124INTToINT13018` reader - Enable GPIO124 Interrupt To INT#130_18"]
pub type EnblGpio124inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO124INTToINT13018` writer - Enable GPIO124 Interrupt To INT#130_18"]
pub type EnblGpio124inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO124INTToINT13019` reader - Enable GPIO124 Interrupt To INT#130_19"]
pub type EnblGpio124inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO124INTToINT13019` writer - Enable GPIO124 Interrupt To INT#130_19"]
pub type EnblGpio124inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO124INTToINT13020` reader - Enable GPIO124 Interrupt To INT#130_20"]
pub type EnblGpio124inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO124INTToINT13020` writer - Enable GPIO124 Interrupt To INT#130_20"]
pub type EnblGpio124inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO124INTToSIO` reader - Enable GPIO124 Interrupt To SIO"]
pub type EnblGpio124inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO124INTToSIO` writer - Enable GPIO124 Interrupt To SIO"]
pub type EnblGpio124inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO124 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio124inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio124inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio124inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO124INTTargetRstTolerance` reader - GPIO124 Interrupt Target Reset Tolerance"]
pub type Gpio124inttargetRstToleranceR = crate::BitReader<Gpio124inttargetRstTolerance>;
impl Gpio124inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio124inttargetRstTolerance {
        match self.bits {
            false => Gpio124inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio124inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio124inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio124inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO124INTTargetRstTolerance` writer - GPIO124 Interrupt Target Reset Tolerance"]
pub type Gpio124inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio124inttargetRstTolerance>;
impl<'a, REG> Gpio124inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio124inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio124inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO124INTTargetWrProt` reader - GPIO124 Interrupt Target Write Protection"]
pub type Gpio124inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO124INTTargetWrProt` writer - GPIO124 Interrupt Target Write Protection"]
pub type Gpio124inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO125INTToINT13018` reader - Enable GPIO125 Interrupt To INT#130_18"]
pub type EnblGpio125inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO125INTToINT13018` writer - Enable GPIO125 Interrupt To INT#130_18"]
pub type EnblGpio125inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO125INTToINT13019` reader - Enable GPIO125 Interrupt To INT#130_19"]
pub type EnblGpio125inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO125INTToINT13019` writer - Enable GPIO125 Interrupt To INT#130_19"]
pub type EnblGpio125inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO125INTToINT13020` reader - Enable GPIO125 Interrupt To INT#130_20"]
pub type EnblGpio125inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO125INTToINT13020` writer - Enable GPIO125 Interrupt To INT#130_20"]
pub type EnblGpio125inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO125INTToSIO` reader - Enable GPIO125 Interrupt To SIO"]
pub type EnblGpio125inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO125INTToSIO` writer - Enable GPIO125 Interrupt To SIO"]
pub type EnblGpio125inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO125 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio125inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio125inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio125inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO125INTTargetRstTolerance` reader - GPIO125 Interrupt Target Reset Tolerance"]
pub type Gpio125inttargetRstToleranceR = crate::BitReader<Gpio125inttargetRstTolerance>;
impl Gpio125inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio125inttargetRstTolerance {
        match self.bits {
            false => Gpio125inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio125inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio125inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio125inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO125INTTargetRstTolerance` writer - GPIO125 Interrupt Target Reset Tolerance"]
pub type Gpio125inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio125inttargetRstTolerance>;
impl<'a, REG> Gpio125inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio125inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio125inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO125INTTargetWrProt` reader - GPIO125 Interrupt Target Write Protection"]
pub type Gpio125inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO125INTTargetWrProt` writer - GPIO125 Interrupt Target Write Protection"]
pub type Gpio125inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO126INTToINT13018` reader - Enable GPIO126 Interrupt To INT#130_18"]
pub type EnblGpio126inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO126INTToINT13018` writer - Enable GPIO126 Interrupt To INT#130_18"]
pub type EnblGpio126inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO126INTToINT13019` reader - Enable GPIO126 Interrupt To INT#130_19"]
pub type EnblGpio126inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO126INTToINT13019` writer - Enable GPIO126 Interrupt To INT#130_19"]
pub type EnblGpio126inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO126INTToINT13020` reader - Enable GPIO126 Interrupt To INT#130_20"]
pub type EnblGpio126inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO126INTToINT13020` writer - Enable GPIO126 Interrupt To INT#130_20"]
pub type EnblGpio126inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO126INTToSIO` reader - Enable GPIO126 Interrupt To SIO"]
pub type EnblGpio126inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO126INTToSIO` writer - Enable GPIO126 Interrupt To SIO"]
pub type EnblGpio126inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO126 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio126inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio126inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio126inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO126INTTargetRstTolerance` reader - GPIO126 Interrupt Target Reset Tolerance"]
pub type Gpio126inttargetRstToleranceR = crate::BitReader<Gpio126inttargetRstTolerance>;
impl Gpio126inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio126inttargetRstTolerance {
        match self.bits {
            false => Gpio126inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio126inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio126inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio126inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO126INTTargetRstTolerance` writer - GPIO126 Interrupt Target Reset Tolerance"]
pub type Gpio126inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio126inttargetRstTolerance>;
impl<'a, REG> Gpio126inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio126inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio126inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO126INTTargetWrProt` reader - GPIO126 Interrupt Target Write Protection"]
pub type Gpio126inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO126INTTargetWrProt` writer - GPIO126 Interrupt Target Write Protection"]
pub type Gpio126inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO127INTToINT13018` reader - Enable GPIO127 Interrupt To INT#130_18"]
pub type EnblGpio127inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO127INTToINT13018` writer - Enable GPIO127 Interrupt To INT#130_18"]
pub type EnblGpio127inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO127INTToINT13019` reader - Enable GPIO127 Interrupt To INT#130_19"]
pub type EnblGpio127inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO127INTToINT13019` writer - Enable GPIO127 Interrupt To INT#130_19"]
pub type EnblGpio127inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO127INTToINT13020` reader - Enable GPIO127 Interrupt To INT#130_20"]
pub type EnblGpio127inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO127INTToINT13020` writer - Enable GPIO127 Interrupt To INT#130_20"]
pub type EnblGpio127inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO127INTToSIO` reader - Enable GPIO127 Interrupt To SIO"]
pub type EnblGpio127inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO127INTToSIO` writer - Enable GPIO127 Interrupt To SIO"]
pub type EnblGpio127inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO127 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio127inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio127inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio127inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO127INTTargetRstTolerance` reader - GPIO127 Interrupt Target Reset Tolerance"]
pub type Gpio127inttargetRstToleranceR = crate::BitReader<Gpio127inttargetRstTolerance>;
impl Gpio127inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio127inttargetRstTolerance {
        match self.bits {
            false => Gpio127inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio127inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio127inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio127inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO127INTTargetRstTolerance` writer - GPIO127 Interrupt Target Reset Tolerance"]
pub type Gpio127inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio127inttargetRstTolerance>;
impl<'a, REG> Gpio127inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio127inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio127inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO127INTTargetWrProt` reader - GPIO127 Interrupt Target Write Protection"]
pub type Gpio127inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO127INTTargetWrProt` writer - GPIO127 Interrupt Target Write Protection"]
pub type Gpio127inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO124 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio124intto_int13018(&self) -> EnblGpio124inttoInt13018R {
        EnblGpio124inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO124 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio124intto_int13019(&self) -> EnblGpio124inttoInt13019R {
        EnblGpio124inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO124 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio124intto_int13020(&self) -> EnblGpio124inttoInt13020R {
        EnblGpio124inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO124 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio124intto_sio(&self) -> EnblGpio124inttoSioR {
        EnblGpio124inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO124 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio124inttarget_rst_tolerance(&self) -> Gpio124inttargetRstToleranceR {
        Gpio124inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO124 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio124inttarget_wr_prot(&self) -> Gpio124inttargetWrProtR {
        Gpio124inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO125 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio125intto_int13018(&self) -> EnblGpio125inttoInt13018R {
        EnblGpio125inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO125 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio125intto_int13019(&self) -> EnblGpio125inttoInt13019R {
        EnblGpio125inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO125 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio125intto_int13020(&self) -> EnblGpio125inttoInt13020R {
        EnblGpio125inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO125 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio125intto_sio(&self) -> EnblGpio125inttoSioR {
        EnblGpio125inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO125 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio125inttarget_rst_tolerance(&self) -> Gpio125inttargetRstToleranceR {
        Gpio125inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO125 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio125inttarget_wr_prot(&self) -> Gpio125inttargetWrProtR {
        Gpio125inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO126 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio126intto_int13018(&self) -> EnblGpio126inttoInt13018R {
        EnblGpio126inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO126 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio126intto_int13019(&self) -> EnblGpio126inttoInt13019R {
        EnblGpio126inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO126 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio126intto_int13020(&self) -> EnblGpio126inttoInt13020R {
        EnblGpio126inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO126 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio126intto_sio(&self) -> EnblGpio126inttoSioR {
        EnblGpio126inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO126 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio126inttarget_rst_tolerance(&self) -> Gpio126inttargetRstToleranceR {
        Gpio126inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO126 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio126inttarget_wr_prot(&self) -> Gpio126inttargetWrProtR {
        Gpio126inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO127 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio127intto_int13018(&self) -> EnblGpio127inttoInt13018R {
        EnblGpio127inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO127 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio127intto_int13019(&self) -> EnblGpio127inttoInt13019R {
        EnblGpio127inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO127 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio127intto_int13020(&self) -> EnblGpio127inttoInt13020R {
        EnblGpio127inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO127 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio127intto_sio(&self) -> EnblGpio127inttoSioR {
        EnblGpio127inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO127 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio127inttarget_rst_tolerance(&self) -> Gpio127inttargetRstToleranceR {
        Gpio127inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO127 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio127inttarget_wr_prot(&self) -> Gpio127inttargetWrProtR {
        Gpio127inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO124 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio124intto_int13018(&mut self) -> EnblGpio124inttoInt13018W<Gpioa8cSpec> {
        EnblGpio124inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO124 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio124intto_int13019(&mut self) -> EnblGpio124inttoInt13019W<Gpioa8cSpec> {
        EnblGpio124inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO124 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio124intto_int13020(&mut self) -> EnblGpio124inttoInt13020W<Gpioa8cSpec> {
        EnblGpio124inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO124 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio124intto_sio(&mut self) -> EnblGpio124inttoSioW<Gpioa8cSpec> {
        EnblGpio124inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa8cSpec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa8cSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO124 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio124inttarget_rst_tolerance(&mut self) -> Gpio124inttargetRstToleranceW<Gpioa8cSpec> {
        Gpio124inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO124 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio124inttarget_wr_prot(&mut self) -> Gpio124inttargetWrProtW<Gpioa8cSpec> {
        Gpio124inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO125 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio125intto_int13018(&mut self) -> EnblGpio125inttoInt13018W<Gpioa8cSpec> {
        EnblGpio125inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO125 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio125intto_int13019(&mut self) -> EnblGpio125inttoInt13019W<Gpioa8cSpec> {
        EnblGpio125inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO125 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio125intto_int13020(&mut self) -> EnblGpio125inttoInt13020W<Gpioa8cSpec> {
        EnblGpio125inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO125 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio125intto_sio(&mut self) -> EnblGpio125inttoSioW<Gpioa8cSpec> {
        EnblGpio125inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa8cSpec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa8cSpec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO125 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio125inttarget_rst_tolerance(&mut self) -> Gpio125inttargetRstToleranceW<Gpioa8cSpec> {
        Gpio125inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO125 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio125inttarget_wr_prot(&mut self) -> Gpio125inttargetWrProtW<Gpioa8cSpec> {
        Gpio125inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO126 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio126intto_int13018(&mut self) -> EnblGpio126inttoInt13018W<Gpioa8cSpec> {
        EnblGpio126inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO126 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio126intto_int13019(&mut self) -> EnblGpio126inttoInt13019W<Gpioa8cSpec> {
        EnblGpio126inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO126 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio126intto_int13020(&mut self) -> EnblGpio126inttoInt13020W<Gpioa8cSpec> {
        EnblGpio126inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO126 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio126intto_sio(&mut self) -> EnblGpio126inttoSioW<Gpioa8cSpec> {
        EnblGpio126inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa8cSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa8cSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO126 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio126inttarget_rst_tolerance(&mut self) -> Gpio126inttargetRstToleranceW<Gpioa8cSpec> {
        Gpio126inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO126 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio126inttarget_wr_prot(&mut self) -> Gpio126inttargetWrProtW<Gpioa8cSpec> {
        Gpio126inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO127 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio127intto_int13018(&mut self) -> EnblGpio127inttoInt13018W<Gpioa8cSpec> {
        EnblGpio127inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO127 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio127intto_int13019(&mut self) -> EnblGpio127inttoInt13019W<Gpioa8cSpec> {
        EnblGpio127inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO127 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio127intto_int13020(&mut self) -> EnblGpio127inttoInt13020W<Gpioa8cSpec> {
        EnblGpio127inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO127 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio127intto_sio(&mut self) -> EnblGpio127inttoSioW<Gpioa8cSpec> {
        EnblGpio127inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa8cSpec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO127 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio127inttarget_rst_tolerance(&mut self) -> Gpio127inttargetRstToleranceW<Gpioa8cSpec> {
        Gpio127inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO127 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio127inttarget_wr_prot(&mut self) -> Gpio127inttargetWrProtW<Gpioa8cSpec> {
        Gpio127inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#31\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa8c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa8c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa8cSpec;
impl crate::RegisterSpec for Gpioa8cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa8c::R`](R) reader structure"]
impl crate::Readable for Gpioa8cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioa8c::W`](W) writer structure"]
impl crate::Writable for Gpioa8cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA8C to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa8cSpec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
