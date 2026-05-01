#[doc = "Register `GPIOA68` reader"]
pub type R = crate::R<Gpioa68Spec>;
#[doc = "Register `GPIOA68` writer"]
pub type W = crate::W<Gpioa68Spec>;
#[doc = "Field `EnblGPIO088INTToINT13018` reader - Enable GPIO088 Interrupt To INT#130_18"]
pub type EnblGpio088inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO088INTToINT13018` writer - Enable GPIO088 Interrupt To INT#130_18"]
pub type EnblGpio088inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO088INTToINT13019` reader - Enable GPIO088 Interrupt To INT#130_19"]
pub type EnblGpio088inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO088INTToINT13019` writer - Enable GPIO088 Interrupt To INT#130_19"]
pub type EnblGpio088inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO088INTToINT13020` reader - Enable GPIO088 Interrupt To INT#130_20"]
pub type EnblGpio088inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO088INTToINT13020` writer - Enable GPIO088 Interrupt To INT#130_20"]
pub type EnblGpio088inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO088INTToSIO` reader - Enable GPIO088 Interrupt To SIO"]
pub type EnblGpio088inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO088INTToSIO` writer - Enable GPIO088 Interrupt To SIO"]
pub type EnblGpio088inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO088 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio088inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio088inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio088inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO088INTTargetRstTolerance` reader - GPIO088 Interrupt Target Reset Tolerance"]
pub type Gpio088inttargetRstToleranceR = crate::BitReader<Gpio088inttargetRstTolerance>;
impl Gpio088inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio088inttargetRstTolerance {
        match self.bits {
            false => Gpio088inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio088inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio088inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio088inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO088INTTargetRstTolerance` writer - GPIO088 Interrupt Target Reset Tolerance"]
pub type Gpio088inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio088inttargetRstTolerance>;
impl<'a, REG> Gpio088inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio088inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio088inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO088INTTargetWrProt` reader - GPIO088 Interrupt Target Write Protection"]
pub type Gpio088inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO088INTTargetWrProt` writer - GPIO088 Interrupt Target Write Protection"]
pub type Gpio088inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO089INTToINT13018` reader - Enable GPIO089 Interrupt To INT#130_18"]
pub type EnblGpio089inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO089INTToINT13018` writer - Enable GPIO089 Interrupt To INT#130_18"]
pub type EnblGpio089inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO089INTToINT13019` reader - Enable GPIO089 Interrupt To INT#130_19"]
pub type EnblGpio089inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO089INTToINT13019` writer - Enable GPIO089 Interrupt To INT#130_19"]
pub type EnblGpio089inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO089INTToINT13020` reader - Enable GPIO089 Interrupt To INT#130_20"]
pub type EnblGpio089inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO089INTToINT13020` writer - Enable GPIO089 Interrupt To INT#130_20"]
pub type EnblGpio089inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO089INTToSIO` reader - Enable GPIO089 Interrupt To SIO"]
pub type EnblGpio089inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO089INTToSIO` writer - Enable GPIO089 Interrupt To SIO"]
pub type EnblGpio089inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO089 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio089inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio089inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio089inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO089INTTargetRstTolerance` reader - GPIO089 Interrupt Target Reset Tolerance"]
pub type Gpio089inttargetRstToleranceR = crate::BitReader<Gpio089inttargetRstTolerance>;
impl Gpio089inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio089inttargetRstTolerance {
        match self.bits {
            false => Gpio089inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio089inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio089inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio089inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO089INTTargetRstTolerance` writer - GPIO089 Interrupt Target Reset Tolerance"]
pub type Gpio089inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio089inttargetRstTolerance>;
impl<'a, REG> Gpio089inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio089inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio089inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO089INTTargetWrProt` reader - GPIO089 Interrupt Target Write Protection"]
pub type Gpio089inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO089INTTargetWrProt` writer - GPIO089 Interrupt Target Write Protection"]
pub type Gpio089inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO090INTToINT13018` reader - Enable GPIO090 Interrupt To INT#130_18"]
pub type EnblGpio090inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO090INTToINT13018` writer - Enable GPIO090 Interrupt To INT#130_18"]
pub type EnblGpio090inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO090INTToINT13019` reader - Enable GPIO090 Interrupt To INT#130_19"]
pub type EnblGpio090inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO090INTToINT13019` writer - Enable GPIO090 Interrupt To INT#130_19"]
pub type EnblGpio090inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO090INTToINT13020` reader - Enable GPIO090 Interrupt To INT#130_20"]
pub type EnblGpio090inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO090INTToINT13020` writer - Enable GPIO090 Interrupt To INT#130_20"]
pub type EnblGpio090inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO090INTToSIO` reader - Enable GPIO090 Interrupt To SIO"]
pub type EnblGpio090inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO090INTToSIO` writer - Enable GPIO090 Interrupt To SIO"]
pub type EnblGpio090inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO090 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio090inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio090inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio090inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO090INTTargetRstTolerance` reader - GPIO090 Interrupt Target Reset Tolerance"]
pub type Gpio090inttargetRstToleranceR = crate::BitReader<Gpio090inttargetRstTolerance>;
impl Gpio090inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio090inttargetRstTolerance {
        match self.bits {
            false => Gpio090inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio090inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio090inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio090inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO090INTTargetRstTolerance` writer - GPIO090 Interrupt Target Reset Tolerance"]
pub type Gpio090inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio090inttargetRstTolerance>;
impl<'a, REG> Gpio090inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio090inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio090inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO090INTTargetWrProt` reader - GPIO090 Interrupt Target Write Protection"]
pub type Gpio090inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO090INTTargetWrProt` writer - GPIO090 Interrupt Target Write Protection"]
pub type Gpio090inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO091INTToINT13018` reader - Enable GPIO091 Interrupt To INT#130_18"]
pub type EnblGpio091inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO091INTToINT13018` writer - Enable GPIO091 Interrupt To INT#130_18"]
pub type EnblGpio091inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO091INTToINT13019` reader - Enable GPIO091 Interrupt To INT#130_19"]
pub type EnblGpio091inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO091INTToINT13019` writer - Enable GPIO091 Interrupt To INT#130_19"]
pub type EnblGpio091inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO091INTToINT13020` reader - Enable GPIO091 Interrupt To INT#130_20"]
pub type EnblGpio091inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO091INTToINT13020` writer - Enable GPIO091 Interrupt To INT#130_20"]
pub type EnblGpio091inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO091INTToSIO` reader - Enable GPIO091 Interrupt To SIO"]
pub type EnblGpio091inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO091INTToSIO` writer - Enable GPIO091 Interrupt To SIO"]
pub type EnblGpio091inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO091 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio091inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio091inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio091inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO091INTTargetRstTolerance` reader - GPIO091 Interrupt Target Reset Tolerance"]
pub type Gpio091inttargetRstToleranceR = crate::BitReader<Gpio091inttargetRstTolerance>;
impl Gpio091inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio091inttargetRstTolerance {
        match self.bits {
            false => Gpio091inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio091inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio091inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio091inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO091INTTargetRstTolerance` writer - GPIO091 Interrupt Target Reset Tolerance"]
pub type Gpio091inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio091inttargetRstTolerance>;
impl<'a, REG> Gpio091inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio091inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio091inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO091INTTargetWrProt` reader - GPIO091 Interrupt Target Write Protection"]
pub type Gpio091inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO091INTTargetWrProt` writer - GPIO091 Interrupt Target Write Protection"]
pub type Gpio091inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO088 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio088intto_int13018(&self) -> EnblGpio088inttoInt13018R {
        EnblGpio088inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO088 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio088intto_int13019(&self) -> EnblGpio088inttoInt13019R {
        EnblGpio088inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO088 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio088intto_int13020(&self) -> EnblGpio088inttoInt13020R {
        EnblGpio088inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO088 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio088intto_sio(&self) -> EnblGpio088inttoSioR {
        EnblGpio088inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO088 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio088inttarget_rst_tolerance(&self) -> Gpio088inttargetRstToleranceR {
        Gpio088inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO088 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio088inttarget_wr_prot(&self) -> Gpio088inttargetWrProtR {
        Gpio088inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO089 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio089intto_int13018(&self) -> EnblGpio089inttoInt13018R {
        EnblGpio089inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO089 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio089intto_int13019(&self) -> EnblGpio089inttoInt13019R {
        EnblGpio089inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO089 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio089intto_int13020(&self) -> EnblGpio089inttoInt13020R {
        EnblGpio089inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO089 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio089intto_sio(&self) -> EnblGpio089inttoSioR {
        EnblGpio089inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO089 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio089inttarget_rst_tolerance(&self) -> Gpio089inttargetRstToleranceR {
        Gpio089inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO089 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio089inttarget_wr_prot(&self) -> Gpio089inttargetWrProtR {
        Gpio089inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO090 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio090intto_int13018(&self) -> EnblGpio090inttoInt13018R {
        EnblGpio090inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO090 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio090intto_int13019(&self) -> EnblGpio090inttoInt13019R {
        EnblGpio090inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO090 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio090intto_int13020(&self) -> EnblGpio090inttoInt13020R {
        EnblGpio090inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO090 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio090intto_sio(&self) -> EnblGpio090inttoSioR {
        EnblGpio090inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO090 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio090inttarget_rst_tolerance(&self) -> Gpio090inttargetRstToleranceR {
        Gpio090inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO090 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio090inttarget_wr_prot(&self) -> Gpio090inttargetWrProtR {
        Gpio090inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO091 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio091intto_int13018(&self) -> EnblGpio091inttoInt13018R {
        EnblGpio091inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO091 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio091intto_int13019(&self) -> EnblGpio091inttoInt13019R {
        EnblGpio091inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO091 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio091intto_int13020(&self) -> EnblGpio091inttoInt13020R {
        EnblGpio091inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO091 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio091intto_sio(&self) -> EnblGpio091inttoSioR {
        EnblGpio091inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO091 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio091inttarget_rst_tolerance(&self) -> Gpio091inttargetRstToleranceR {
        Gpio091inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO091 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio091inttarget_wr_prot(&self) -> Gpio091inttargetWrProtR {
        Gpio091inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO088 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio088intto_int13018(&mut self) -> EnblGpio088inttoInt13018W<Gpioa68Spec> {
        EnblGpio088inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO088 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio088intto_int13019(&mut self) -> EnblGpio088inttoInt13019W<Gpioa68Spec> {
        EnblGpio088inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO088 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio088intto_int13020(&mut self) -> EnblGpio088inttoInt13020W<Gpioa68Spec> {
        EnblGpio088inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO088 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio088intto_sio(&mut self) -> EnblGpio088inttoSioW<Gpioa68Spec> {
        EnblGpio088inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa68Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa68Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO088 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio088inttarget_rst_tolerance(&mut self) -> Gpio088inttargetRstToleranceW<Gpioa68Spec> {
        Gpio088inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO088 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio088inttarget_wr_prot(&mut self) -> Gpio088inttargetWrProtW<Gpioa68Spec> {
        Gpio088inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO089 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio089intto_int13018(&mut self) -> EnblGpio089inttoInt13018W<Gpioa68Spec> {
        EnblGpio089inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO089 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio089intto_int13019(&mut self) -> EnblGpio089inttoInt13019W<Gpioa68Spec> {
        EnblGpio089inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO089 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio089intto_int13020(&mut self) -> EnblGpio089inttoInt13020W<Gpioa68Spec> {
        EnblGpio089inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO089 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio089intto_sio(&mut self) -> EnblGpio089inttoSioW<Gpioa68Spec> {
        EnblGpio089inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa68Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa68Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO089 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio089inttarget_rst_tolerance(&mut self) -> Gpio089inttargetRstToleranceW<Gpioa68Spec> {
        Gpio089inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO089 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio089inttarget_wr_prot(&mut self) -> Gpio089inttargetWrProtW<Gpioa68Spec> {
        Gpio089inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO090 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio090intto_int13018(&mut self) -> EnblGpio090inttoInt13018W<Gpioa68Spec> {
        EnblGpio090inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO090 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio090intto_int13019(&mut self) -> EnblGpio090inttoInt13019W<Gpioa68Spec> {
        EnblGpio090inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO090 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio090intto_int13020(&mut self) -> EnblGpio090inttoInt13020W<Gpioa68Spec> {
        EnblGpio090inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO090 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio090intto_sio(&mut self) -> EnblGpio090inttoSioW<Gpioa68Spec> {
        EnblGpio090inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa68Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa68Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO090 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio090inttarget_rst_tolerance(&mut self) -> Gpio090inttargetRstToleranceW<Gpioa68Spec> {
        Gpio090inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO090 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio090inttarget_wr_prot(&mut self) -> Gpio090inttargetWrProtW<Gpioa68Spec> {
        Gpio090inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO091 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio091intto_int13018(&mut self) -> EnblGpio091inttoInt13018W<Gpioa68Spec> {
        EnblGpio091inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO091 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio091intto_int13019(&mut self) -> EnblGpio091inttoInt13019W<Gpioa68Spec> {
        EnblGpio091inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO091 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio091intto_int13020(&mut self) -> EnblGpio091inttoInt13020W<Gpioa68Spec> {
        EnblGpio091inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO091 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio091intto_sio(&mut self) -> EnblGpio091inttoSioW<Gpioa68Spec> {
        EnblGpio091inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa68Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO091 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio091inttarget_rst_tolerance(&mut self) -> Gpio091inttargetRstToleranceW<Gpioa68Spec> {
        Gpio091inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO091 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio091inttarget_wr_prot(&mut self) -> Gpio091inttargetWrProtW<Gpioa68Spec> {
        Gpio091inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa68::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa68::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa68Spec;
impl crate::RegisterSpec for Gpioa68Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa68::R`](R) reader structure"]
impl crate::Readable for Gpioa68Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa68::W`](W) writer structure"]
impl crate::Writable for Gpioa68Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA68 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa68Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
