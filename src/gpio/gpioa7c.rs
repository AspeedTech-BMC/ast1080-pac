#[doc = "Register `GPIOA7C` reader"]
pub type R = crate::R<Gpioa7cSpec>;
#[doc = "Register `GPIOA7C` writer"]
pub type W = crate::W<Gpioa7cSpec>;
#[doc = "Field `EnblGPIO108INTToINT13018` reader - Enable GPIO108 Interrupt To INT#130_18"]
pub type EnblGpio108inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO108INTToINT13018` writer - Enable GPIO108 Interrupt To INT#130_18"]
pub type EnblGpio108inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO108INTToINT13019` reader - Enable GPIO108 Interrupt To INT#130_19"]
pub type EnblGpio108inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO108INTToINT13019` writer - Enable GPIO108 Interrupt To INT#130_19"]
pub type EnblGpio108inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO108INTToINT13020` reader - Enable GPIO108 Interrupt To INT#130_20"]
pub type EnblGpio108inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO108INTToINT13020` writer - Enable GPIO108 Interrupt To INT#130_20"]
pub type EnblGpio108inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO108INTToSIO` reader - Enable GPIO108 Interrupt To SIO"]
pub type EnblGpio108inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO108INTToSIO` writer - Enable GPIO108 Interrupt To SIO"]
pub type EnblGpio108inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO108 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio108inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio108inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio108inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO108INTTargetRstTolerance` reader - GPIO108 Interrupt Target Reset Tolerance"]
pub type Gpio108inttargetRstToleranceR = crate::BitReader<Gpio108inttargetRstTolerance>;
impl Gpio108inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio108inttargetRstTolerance {
        match self.bits {
            false => Gpio108inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio108inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio108inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio108inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO108INTTargetRstTolerance` writer - GPIO108 Interrupt Target Reset Tolerance"]
pub type Gpio108inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio108inttargetRstTolerance>;
impl<'a, REG> Gpio108inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio108inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio108inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO108INTTargetWrProt` reader - GPIO108 Interrupt Target Write Protection"]
pub type Gpio108inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO108INTTargetWrProt` writer - GPIO108 Interrupt Target Write Protection"]
pub type Gpio108inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO109INTToINT13018` reader - Enable GPIO109 Interrupt To INT#130_18"]
pub type EnblGpio109inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO109INTToINT13018` writer - Enable GPIO109 Interrupt To INT#130_18"]
pub type EnblGpio109inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO109INTToINT13019` reader - Enable GPIO109 Interrupt To INT#130_19"]
pub type EnblGpio109inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO109INTToINT13019` writer - Enable GPIO109 Interrupt To INT#130_19"]
pub type EnblGpio109inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO109INTToINT13020` reader - Enable GPIO109 Interrupt To INT#130_20"]
pub type EnblGpio109inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO109INTToINT13020` writer - Enable GPIO109 Interrupt To INT#130_20"]
pub type EnblGpio109inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO109INTToSIO` reader - Enable GPIO109 Interrupt To SIO"]
pub type EnblGpio109inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO109INTToSIO` writer - Enable GPIO109 Interrupt To SIO"]
pub type EnblGpio109inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO109 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio109inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio109inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio109inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO109INTTargetRstTolerance` reader - GPIO109 Interrupt Target Reset Tolerance"]
pub type Gpio109inttargetRstToleranceR = crate::BitReader<Gpio109inttargetRstTolerance>;
impl Gpio109inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio109inttargetRstTolerance {
        match self.bits {
            false => Gpio109inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio109inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio109inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio109inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO109INTTargetRstTolerance` writer - GPIO109 Interrupt Target Reset Tolerance"]
pub type Gpio109inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio109inttargetRstTolerance>;
impl<'a, REG> Gpio109inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio109inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio109inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO109INTTargetWrProt` reader - GPIO109 Interrupt Target Write Protection"]
pub type Gpio109inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO109INTTargetWrProt` writer - GPIO109 Interrupt Target Write Protection"]
pub type Gpio109inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO110INTToINT13018` reader - Enable GPIO110 Interrupt To INT#130_18"]
pub type EnblGpio110inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO110INTToINT13018` writer - Enable GPIO110 Interrupt To INT#130_18"]
pub type EnblGpio110inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO110INTToINT13019` reader - Enable GPIO110 Interrupt To INT#130_19"]
pub type EnblGpio110inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO110INTToINT13019` writer - Enable GPIO110 Interrupt To INT#130_19"]
pub type EnblGpio110inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO110INTToINT13020` reader - Enable GPIO110 Interrupt To INT#130_20"]
pub type EnblGpio110inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO110INTToINT13020` writer - Enable GPIO110 Interrupt To INT#130_20"]
pub type EnblGpio110inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO110INTToSIO` reader - Enable GPIO110 Interrupt To SIO"]
pub type EnblGpio110inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO110INTToSIO` writer - Enable GPIO110 Interrupt To SIO"]
pub type EnblGpio110inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO110 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio110inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio110inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio110inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO110INTTargetRstTolerance` reader - GPIO110 Interrupt Target Reset Tolerance"]
pub type Gpio110inttargetRstToleranceR = crate::BitReader<Gpio110inttargetRstTolerance>;
impl Gpio110inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio110inttargetRstTolerance {
        match self.bits {
            false => Gpio110inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio110inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio110inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio110inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO110INTTargetRstTolerance` writer - GPIO110 Interrupt Target Reset Tolerance"]
pub type Gpio110inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio110inttargetRstTolerance>;
impl<'a, REG> Gpio110inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio110inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio110inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO110INTTargetWrProt` reader - GPIO110 Interrupt Target Write Protection"]
pub type Gpio110inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO110INTTargetWrProt` writer - GPIO110 Interrupt Target Write Protection"]
pub type Gpio110inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO111INTToINT13018` reader - Enable GPIO111 Interrupt To INT#130_18"]
pub type EnblGpio111inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO111INTToINT13018` writer - Enable GPIO111 Interrupt To INT#130_18"]
pub type EnblGpio111inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO111INTToINT13019` reader - Enable GPIO111 Interrupt To INT#130_19"]
pub type EnblGpio111inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO111INTToINT13019` writer - Enable GPIO111 Interrupt To INT#130_19"]
pub type EnblGpio111inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO111INTToINT13020` reader - Enable GPIO111 Interrupt To INT#130_20"]
pub type EnblGpio111inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO111INTToINT13020` writer - Enable GPIO111 Interrupt To INT#130_20"]
pub type EnblGpio111inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO111INTToSIO` reader - Enable GPIO111 Interrupt To SIO"]
pub type EnblGpio111inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO111INTToSIO` writer - Enable GPIO111 Interrupt To SIO"]
pub type EnblGpio111inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO111 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio111inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio111inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio111inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO111INTTargetRstTolerance` reader - GPIO111 Interrupt Target Reset Tolerance"]
pub type Gpio111inttargetRstToleranceR = crate::BitReader<Gpio111inttargetRstTolerance>;
impl Gpio111inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio111inttargetRstTolerance {
        match self.bits {
            false => Gpio111inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio111inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio111inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio111inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO111INTTargetRstTolerance` writer - GPIO111 Interrupt Target Reset Tolerance"]
pub type Gpio111inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio111inttargetRstTolerance>;
impl<'a, REG> Gpio111inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio111inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio111inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO111INTTargetWrProt` reader - GPIO111 Interrupt Target Write Protection"]
pub type Gpio111inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO111INTTargetWrProt` writer - GPIO111 Interrupt Target Write Protection"]
pub type Gpio111inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO108 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio108intto_int13018(&self) -> EnblGpio108inttoInt13018R {
        EnblGpio108inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO108 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio108intto_int13019(&self) -> EnblGpio108inttoInt13019R {
        EnblGpio108inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO108 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio108intto_int13020(&self) -> EnblGpio108inttoInt13020R {
        EnblGpio108inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO108 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio108intto_sio(&self) -> EnblGpio108inttoSioR {
        EnblGpio108inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO108 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio108inttarget_rst_tolerance(&self) -> Gpio108inttargetRstToleranceR {
        Gpio108inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO108 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio108inttarget_wr_prot(&self) -> Gpio108inttargetWrProtR {
        Gpio108inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO109 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio109intto_int13018(&self) -> EnblGpio109inttoInt13018R {
        EnblGpio109inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO109 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio109intto_int13019(&self) -> EnblGpio109inttoInt13019R {
        EnblGpio109inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO109 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio109intto_int13020(&self) -> EnblGpio109inttoInt13020R {
        EnblGpio109inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO109 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio109intto_sio(&self) -> EnblGpio109inttoSioR {
        EnblGpio109inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO109 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio109inttarget_rst_tolerance(&self) -> Gpio109inttargetRstToleranceR {
        Gpio109inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO109 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio109inttarget_wr_prot(&self) -> Gpio109inttargetWrProtR {
        Gpio109inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO110 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio110intto_int13018(&self) -> EnblGpio110inttoInt13018R {
        EnblGpio110inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO110 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio110intto_int13019(&self) -> EnblGpio110inttoInt13019R {
        EnblGpio110inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO110 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio110intto_int13020(&self) -> EnblGpio110inttoInt13020R {
        EnblGpio110inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO110 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio110intto_sio(&self) -> EnblGpio110inttoSioR {
        EnblGpio110inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO110 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio110inttarget_rst_tolerance(&self) -> Gpio110inttargetRstToleranceR {
        Gpio110inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO110 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio110inttarget_wr_prot(&self) -> Gpio110inttargetWrProtR {
        Gpio110inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO111 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio111intto_int13018(&self) -> EnblGpio111inttoInt13018R {
        EnblGpio111inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO111 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio111intto_int13019(&self) -> EnblGpio111inttoInt13019R {
        EnblGpio111inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO111 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio111intto_int13020(&self) -> EnblGpio111inttoInt13020R {
        EnblGpio111inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO111 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio111intto_sio(&self) -> EnblGpio111inttoSioR {
        EnblGpio111inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO111 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio111inttarget_rst_tolerance(&self) -> Gpio111inttargetRstToleranceR {
        Gpio111inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO111 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio111inttarget_wr_prot(&self) -> Gpio111inttargetWrProtR {
        Gpio111inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO108 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio108intto_int13018(&mut self) -> EnblGpio108inttoInt13018W<Gpioa7cSpec> {
        EnblGpio108inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO108 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio108intto_int13019(&mut self) -> EnblGpio108inttoInt13019W<Gpioa7cSpec> {
        EnblGpio108inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO108 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio108intto_int13020(&mut self) -> EnblGpio108inttoInt13020W<Gpioa7cSpec> {
        EnblGpio108inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO108 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio108intto_sio(&mut self) -> EnblGpio108inttoSioW<Gpioa7cSpec> {
        EnblGpio108inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa7cSpec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa7cSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO108 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio108inttarget_rst_tolerance(&mut self) -> Gpio108inttargetRstToleranceW<Gpioa7cSpec> {
        Gpio108inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO108 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio108inttarget_wr_prot(&mut self) -> Gpio108inttargetWrProtW<Gpioa7cSpec> {
        Gpio108inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO109 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio109intto_int13018(&mut self) -> EnblGpio109inttoInt13018W<Gpioa7cSpec> {
        EnblGpio109inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO109 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio109intto_int13019(&mut self) -> EnblGpio109inttoInt13019W<Gpioa7cSpec> {
        EnblGpio109inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO109 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio109intto_int13020(&mut self) -> EnblGpio109inttoInt13020W<Gpioa7cSpec> {
        EnblGpio109inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO109 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio109intto_sio(&mut self) -> EnblGpio109inttoSioW<Gpioa7cSpec> {
        EnblGpio109inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa7cSpec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa7cSpec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO109 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio109inttarget_rst_tolerance(&mut self) -> Gpio109inttargetRstToleranceW<Gpioa7cSpec> {
        Gpio109inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO109 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio109inttarget_wr_prot(&mut self) -> Gpio109inttargetWrProtW<Gpioa7cSpec> {
        Gpio109inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO110 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio110intto_int13018(&mut self) -> EnblGpio110inttoInt13018W<Gpioa7cSpec> {
        EnblGpio110inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO110 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio110intto_int13019(&mut self) -> EnblGpio110inttoInt13019W<Gpioa7cSpec> {
        EnblGpio110inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO110 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio110intto_int13020(&mut self) -> EnblGpio110inttoInt13020W<Gpioa7cSpec> {
        EnblGpio110inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO110 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio110intto_sio(&mut self) -> EnblGpio110inttoSioW<Gpioa7cSpec> {
        EnblGpio110inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa7cSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa7cSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO110 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio110inttarget_rst_tolerance(&mut self) -> Gpio110inttargetRstToleranceW<Gpioa7cSpec> {
        Gpio110inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO110 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio110inttarget_wr_prot(&mut self) -> Gpio110inttargetWrProtW<Gpioa7cSpec> {
        Gpio110inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO111 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio111intto_int13018(&mut self) -> EnblGpio111inttoInt13018W<Gpioa7cSpec> {
        EnblGpio111inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO111 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio111intto_int13019(&mut self) -> EnblGpio111inttoInt13019W<Gpioa7cSpec> {
        EnblGpio111inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO111 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio111intto_int13020(&mut self) -> EnblGpio111inttoInt13020W<Gpioa7cSpec> {
        EnblGpio111inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO111 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio111intto_sio(&mut self) -> EnblGpio111inttoSioW<Gpioa7cSpec> {
        EnblGpio111inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa7cSpec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO111 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio111inttarget_rst_tolerance(&mut self) -> Gpio111inttargetRstToleranceW<Gpioa7cSpec> {
        Gpio111inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO111 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio111inttarget_wr_prot(&mut self) -> Gpio111inttargetWrProtW<Gpioa7cSpec> {
        Gpio111inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa7c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa7c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa7cSpec;
impl crate::RegisterSpec for Gpioa7cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa7c::R`](R) reader structure"]
impl crate::Readable for Gpioa7cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioa7c::W`](W) writer structure"]
impl crate::Writable for Gpioa7cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA7C to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa7cSpec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
