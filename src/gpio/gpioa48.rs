#[doc = "Register `GPIOA48` reader"]
pub type R = crate::R<Gpioa48Spec>;
#[doc = "Register `GPIOA48` writer"]
pub type W = crate::W<Gpioa48Spec>;
#[doc = "Field `EnblGPIO056INTToINT13018` reader - Enable GPIO056 Interrupt To INT#130_18"]
pub type EnblGpio056inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO056INTToINT13018` writer - Enable GPIO056 Interrupt To INT#130_18"]
pub type EnblGpio056inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO056INTToINT13019` reader - Enable GPIO056 Interrupt To INT#130_19"]
pub type EnblGpio056inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO056INTToINT13019` writer - Enable GPIO056 Interrupt To INT#130_19"]
pub type EnblGpio056inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO056INTToINT13020` reader - Enable GPIO056 Interrupt To INT#130_20"]
pub type EnblGpio056inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO056INTToINT13020` writer - Enable GPIO056 Interrupt To INT#130_20"]
pub type EnblGpio056inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO056INTToSIO` reader - Enable GPIO056 Interrupt To SIO"]
pub type EnblGpio056inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO056INTToSIO` writer - Enable GPIO056 Interrupt To SIO"]
pub type EnblGpio056inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO056 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio056inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio056inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio056inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO056INTTargetRstTolerance` reader - GPIO056 Interrupt Target Reset Tolerance"]
pub type Gpio056inttargetRstToleranceR = crate::BitReader<Gpio056inttargetRstTolerance>;
impl Gpio056inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio056inttargetRstTolerance {
        match self.bits {
            false => Gpio056inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio056inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio056inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio056inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO056INTTargetRstTolerance` writer - GPIO056 Interrupt Target Reset Tolerance"]
pub type Gpio056inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio056inttargetRstTolerance>;
impl<'a, REG> Gpio056inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio056inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio056inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO056INTTargetWrProt` reader - GPIO056 Interrupt Target Write Protection"]
pub type Gpio056inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO056INTTargetWrProt` writer - GPIO056 Interrupt Target Write Protection"]
pub type Gpio056inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO057INTToINT13018` reader - Enable GPIO057 Interrupt To INT#130_18"]
pub type EnblGpio057inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO057INTToINT13018` writer - Enable GPIO057 Interrupt To INT#130_18"]
pub type EnblGpio057inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO057INTToINT13019` reader - Enable GPIO057 Interrupt To INT#130_19"]
pub type EnblGpio057inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO057INTToINT13019` writer - Enable GPIO057 Interrupt To INT#130_19"]
pub type EnblGpio057inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO057INTToINT13020` reader - Enable GPIO057 Interrupt To INT#130_20"]
pub type EnblGpio057inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO057INTToINT13020` writer - Enable GPIO057 Interrupt To INT#130_20"]
pub type EnblGpio057inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO057INTToSIO` reader - Enable GPIO057 Interrupt To SIO"]
pub type EnblGpio057inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO057INTToSIO` writer - Enable GPIO057 Interrupt To SIO"]
pub type EnblGpio057inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO057 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio057inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio057inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio057inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO057INTTargetRstTolerance` reader - GPIO057 Interrupt Target Reset Tolerance"]
pub type Gpio057inttargetRstToleranceR = crate::BitReader<Gpio057inttargetRstTolerance>;
impl Gpio057inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio057inttargetRstTolerance {
        match self.bits {
            false => Gpio057inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio057inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio057inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio057inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO057INTTargetRstTolerance` writer - GPIO057 Interrupt Target Reset Tolerance"]
pub type Gpio057inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio057inttargetRstTolerance>;
impl<'a, REG> Gpio057inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio057inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio057inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO057INTTargetWrProt` reader - GPIO057 Interrupt Target Write Protection"]
pub type Gpio057inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO057INTTargetWrProt` writer - GPIO057 Interrupt Target Write Protection"]
pub type Gpio057inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO058INTToINT13018` reader - Enable GPIO058 Interrupt To INT#130_18"]
pub type EnblGpio058inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO058INTToINT13018` writer - Enable GPIO058 Interrupt To INT#130_18"]
pub type EnblGpio058inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO058INTToINT13019` reader - Enable GPIO058 Interrupt To INT#130_19"]
pub type EnblGpio058inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO058INTToINT13019` writer - Enable GPIO058 Interrupt To INT#130_19"]
pub type EnblGpio058inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO058INTToINT13020` reader - Enable GPIO058 Interrupt To INT#130_20"]
pub type EnblGpio058inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO058INTToINT13020` writer - Enable GPIO058 Interrupt To INT#130_20"]
pub type EnblGpio058inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO058INTToSIO` reader - Enable GPIO058 Interrupt To SIO"]
pub type EnblGpio058inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO058INTToSIO` writer - Enable GPIO058 Interrupt To SIO"]
pub type EnblGpio058inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO058 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio058inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio058inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio058inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO058INTTargetRstTolerance` reader - GPIO058 Interrupt Target Reset Tolerance"]
pub type Gpio058inttargetRstToleranceR = crate::BitReader<Gpio058inttargetRstTolerance>;
impl Gpio058inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio058inttargetRstTolerance {
        match self.bits {
            false => Gpio058inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio058inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio058inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio058inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO058INTTargetRstTolerance` writer - GPIO058 Interrupt Target Reset Tolerance"]
pub type Gpio058inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio058inttargetRstTolerance>;
impl<'a, REG> Gpio058inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio058inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio058inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO058INTTargetWrProt` reader - GPIO058 Interrupt Target Write Protection"]
pub type Gpio058inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO058INTTargetWrProt` writer - GPIO058 Interrupt Target Write Protection"]
pub type Gpio058inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO059INTToINT13018` reader - Enable GPIO059 Interrupt To INT#130_18"]
pub type EnblGpio059inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO059INTToINT13018` writer - Enable GPIO059 Interrupt To INT#130_18"]
pub type EnblGpio059inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO059INTToINT13019` reader - Enable GPIO059 Interrupt To INT#130_19"]
pub type EnblGpio059inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO059INTToINT13019` writer - Enable GPIO059 Interrupt To INT#130_19"]
pub type EnblGpio059inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO059INTToINT13020` reader - Enable GPIO059 Interrupt To INT#130_20"]
pub type EnblGpio059inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO059INTToINT13020` writer - Enable GPIO059 Interrupt To INT#130_20"]
pub type EnblGpio059inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO059INTToSIO` reader - Enable GPIO059 Interrupt To SIO"]
pub type EnblGpio059inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO059INTToSIO` writer - Enable GPIO059 Interrupt To SIO"]
pub type EnblGpio059inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO059 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio059inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio059inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio059inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO059INTTargetRstTolerance` reader - GPIO059 Interrupt Target Reset Tolerance"]
pub type Gpio059inttargetRstToleranceR = crate::BitReader<Gpio059inttargetRstTolerance>;
impl Gpio059inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio059inttargetRstTolerance {
        match self.bits {
            false => Gpio059inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio059inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio059inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio059inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO059INTTargetRstTolerance` writer - GPIO059 Interrupt Target Reset Tolerance"]
pub type Gpio059inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio059inttargetRstTolerance>;
impl<'a, REG> Gpio059inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio059inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio059inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO059INTTargetWrProt` reader - GPIO059 Interrupt Target Write Protection"]
pub type Gpio059inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO059INTTargetWrProt` writer - GPIO059 Interrupt Target Write Protection"]
pub type Gpio059inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO056 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio056intto_int13018(&self) -> EnblGpio056inttoInt13018R {
        EnblGpio056inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO056 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio056intto_int13019(&self) -> EnblGpio056inttoInt13019R {
        EnblGpio056inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO056 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio056intto_int13020(&self) -> EnblGpio056inttoInt13020R {
        EnblGpio056inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO056 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio056intto_sio(&self) -> EnblGpio056inttoSioR {
        EnblGpio056inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO056 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio056inttarget_rst_tolerance(&self) -> Gpio056inttargetRstToleranceR {
        Gpio056inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO056 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio056inttarget_wr_prot(&self) -> Gpio056inttargetWrProtR {
        Gpio056inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO057 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio057intto_int13018(&self) -> EnblGpio057inttoInt13018R {
        EnblGpio057inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO057 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio057intto_int13019(&self) -> EnblGpio057inttoInt13019R {
        EnblGpio057inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO057 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio057intto_int13020(&self) -> EnblGpio057inttoInt13020R {
        EnblGpio057inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO057 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio057intto_sio(&self) -> EnblGpio057inttoSioR {
        EnblGpio057inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO057 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio057inttarget_rst_tolerance(&self) -> Gpio057inttargetRstToleranceR {
        Gpio057inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO057 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio057inttarget_wr_prot(&self) -> Gpio057inttargetWrProtR {
        Gpio057inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO058 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio058intto_int13018(&self) -> EnblGpio058inttoInt13018R {
        EnblGpio058inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO058 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio058intto_int13019(&self) -> EnblGpio058inttoInt13019R {
        EnblGpio058inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO058 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio058intto_int13020(&self) -> EnblGpio058inttoInt13020R {
        EnblGpio058inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO058 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio058intto_sio(&self) -> EnblGpio058inttoSioR {
        EnblGpio058inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO058 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio058inttarget_rst_tolerance(&self) -> Gpio058inttargetRstToleranceR {
        Gpio058inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO058 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio058inttarget_wr_prot(&self) -> Gpio058inttargetWrProtR {
        Gpio058inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO059 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio059intto_int13018(&self) -> EnblGpio059inttoInt13018R {
        EnblGpio059inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO059 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio059intto_int13019(&self) -> EnblGpio059inttoInt13019R {
        EnblGpio059inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO059 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio059intto_int13020(&self) -> EnblGpio059inttoInt13020R {
        EnblGpio059inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO059 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio059intto_sio(&self) -> EnblGpio059inttoSioR {
        EnblGpio059inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO059 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio059inttarget_rst_tolerance(&self) -> Gpio059inttargetRstToleranceR {
        Gpio059inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO059 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio059inttarget_wr_prot(&self) -> Gpio059inttargetWrProtR {
        Gpio059inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO056 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio056intto_int13018(&mut self) -> EnblGpio056inttoInt13018W<Gpioa48Spec> {
        EnblGpio056inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO056 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio056intto_int13019(&mut self) -> EnblGpio056inttoInt13019W<Gpioa48Spec> {
        EnblGpio056inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO056 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio056intto_int13020(&mut self) -> EnblGpio056inttoInt13020W<Gpioa48Spec> {
        EnblGpio056inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO056 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio056intto_sio(&mut self) -> EnblGpio056inttoSioW<Gpioa48Spec> {
        EnblGpio056inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa48Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa48Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO056 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio056inttarget_rst_tolerance(&mut self) -> Gpio056inttargetRstToleranceW<Gpioa48Spec> {
        Gpio056inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO056 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio056inttarget_wr_prot(&mut self) -> Gpio056inttargetWrProtW<Gpioa48Spec> {
        Gpio056inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO057 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio057intto_int13018(&mut self) -> EnblGpio057inttoInt13018W<Gpioa48Spec> {
        EnblGpio057inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO057 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio057intto_int13019(&mut self) -> EnblGpio057inttoInt13019W<Gpioa48Spec> {
        EnblGpio057inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO057 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio057intto_int13020(&mut self) -> EnblGpio057inttoInt13020W<Gpioa48Spec> {
        EnblGpio057inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO057 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio057intto_sio(&mut self) -> EnblGpio057inttoSioW<Gpioa48Spec> {
        EnblGpio057inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa48Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa48Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO057 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio057inttarget_rst_tolerance(&mut self) -> Gpio057inttargetRstToleranceW<Gpioa48Spec> {
        Gpio057inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO057 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio057inttarget_wr_prot(&mut self) -> Gpio057inttargetWrProtW<Gpioa48Spec> {
        Gpio057inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO058 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio058intto_int13018(&mut self) -> EnblGpio058inttoInt13018W<Gpioa48Spec> {
        EnblGpio058inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO058 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio058intto_int13019(&mut self) -> EnblGpio058inttoInt13019W<Gpioa48Spec> {
        EnblGpio058inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO058 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio058intto_int13020(&mut self) -> EnblGpio058inttoInt13020W<Gpioa48Spec> {
        EnblGpio058inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO058 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio058intto_sio(&mut self) -> EnblGpio058inttoSioW<Gpioa48Spec> {
        EnblGpio058inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa48Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa48Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO058 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio058inttarget_rst_tolerance(&mut self) -> Gpio058inttargetRstToleranceW<Gpioa48Spec> {
        Gpio058inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO058 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio058inttarget_wr_prot(&mut self) -> Gpio058inttargetWrProtW<Gpioa48Spec> {
        Gpio058inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO059 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio059intto_int13018(&mut self) -> EnblGpio059inttoInt13018W<Gpioa48Spec> {
        EnblGpio059inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO059 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio059intto_int13019(&mut self) -> EnblGpio059inttoInt13019W<Gpioa48Spec> {
        EnblGpio059inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO059 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio059intto_int13020(&mut self) -> EnblGpio059inttoInt13020W<Gpioa48Spec> {
        EnblGpio059inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO059 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio059intto_sio(&mut self) -> EnblGpio059inttoSioW<Gpioa48Spec> {
        EnblGpio059inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa48Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO059 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio059inttarget_rst_tolerance(&mut self) -> Gpio059inttargetRstToleranceW<Gpioa48Spec> {
        Gpio059inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO059 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio059inttarget_wr_prot(&mut self) -> Gpio059inttargetWrProtW<Gpioa48Spec> {
        Gpio059inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa48Spec;
impl crate::RegisterSpec for Gpioa48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa48::R`](R) reader structure"]
impl crate::Readable for Gpioa48Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa48::W`](W) writer structure"]
impl crate::Writable for Gpioa48Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA48 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa48Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
