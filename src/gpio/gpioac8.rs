#[doc = "Register `GPIOAC8` reader"]
pub type R = crate::R<Gpioac8Spec>;
#[doc = "Register `GPIOAC8` writer"]
pub type W = crate::W<Gpioac8Spec>;
#[doc = "Field `EnblGPIO184INTToINT13018` reader - Enable GPIO184 Interrupt To INT#130_18"]
pub type EnblGpio184inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO184INTToINT13018` writer - Enable GPIO184 Interrupt To INT#130_18"]
pub type EnblGpio184inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO184INTToINT13019` reader - Enable GPIO184 Interrupt To INT#130_19"]
pub type EnblGpio184inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO184INTToINT13019` writer - Enable GPIO184 Interrupt To INT#130_19"]
pub type EnblGpio184inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO184INTToINT13020` reader - Enable GPIO184 Interrupt To INT#130_20"]
pub type EnblGpio184inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO184INTToINT13020` writer - Enable GPIO184 Interrupt To INT#130_20"]
pub type EnblGpio184inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO184INTToSIO` reader - Enable GPIO184 Interrupt To SIO"]
pub type EnblGpio184inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO184INTToSIO` writer - Enable GPIO184 Interrupt To SIO"]
pub type EnblGpio184inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO184 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio184inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio184inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio184inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO184INTTargetRstTolerance` reader - GPIO184 Interrupt Target Reset Tolerance"]
pub type Gpio184inttargetRstToleranceR = crate::BitReader<Gpio184inttargetRstTolerance>;
impl Gpio184inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio184inttargetRstTolerance {
        match self.bits {
            false => Gpio184inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio184inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio184inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio184inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO184INTTargetRstTolerance` writer - GPIO184 Interrupt Target Reset Tolerance"]
pub type Gpio184inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio184inttargetRstTolerance>;
impl<'a, REG> Gpio184inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio184inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio184inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO184INTTargetWrProt` reader - GPIO184 Interrupt Target Write Protection"]
pub type Gpio184inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO184INTTargetWrProt` writer - GPIO184 Interrupt Target Write Protection"]
pub type Gpio184inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO185INTToINT13018` reader - Enable GPIO185 Interrupt To INT#130_18"]
pub type EnblGpio185inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO185INTToINT13018` writer - Enable GPIO185 Interrupt To INT#130_18"]
pub type EnblGpio185inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO185INTToINT13019` reader - Enable GPIO185 Interrupt To INT#130_19"]
pub type EnblGpio185inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO185INTToINT13019` writer - Enable GPIO185 Interrupt To INT#130_19"]
pub type EnblGpio185inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO185INTToINT13020` reader - Enable GPIO185 Interrupt To INT#130_20"]
pub type EnblGpio185inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO185INTToINT13020` writer - Enable GPIO185 Interrupt To INT#130_20"]
pub type EnblGpio185inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO185INTToSIO` reader - Enable GPIO185 Interrupt To SIO"]
pub type EnblGpio185inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO185INTToSIO` writer - Enable GPIO185 Interrupt To SIO"]
pub type EnblGpio185inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO185 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio185inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio185inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio185inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO185INTTargetRstTolerance` reader - GPIO185 Interrupt Target Reset Tolerance"]
pub type Gpio185inttargetRstToleranceR = crate::BitReader<Gpio185inttargetRstTolerance>;
impl Gpio185inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio185inttargetRstTolerance {
        match self.bits {
            false => Gpio185inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio185inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio185inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio185inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO185INTTargetRstTolerance` writer - GPIO185 Interrupt Target Reset Tolerance"]
pub type Gpio185inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio185inttargetRstTolerance>;
impl<'a, REG> Gpio185inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio185inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio185inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO185INTTargetWrProt` reader - GPIO185 Interrupt Target Write Protection"]
pub type Gpio185inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO185INTTargetWrProt` writer - GPIO185 Interrupt Target Write Protection"]
pub type Gpio185inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO186INTToINT13018` reader - Enable GPIO186 Interrupt To INT#130_18"]
pub type EnblGpio186inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO186INTToINT13018` writer - Enable GPIO186 Interrupt To INT#130_18"]
pub type EnblGpio186inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO186INTToINT13019` reader - Enable GPIO186 Interrupt To INT#130_19"]
pub type EnblGpio186inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO186INTToINT13019` writer - Enable GPIO186 Interrupt To INT#130_19"]
pub type EnblGpio186inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO186INTToINT13020` reader - Enable GPIO186 Interrupt To INT#130_20"]
pub type EnblGpio186inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO186INTToINT13020` writer - Enable GPIO186 Interrupt To INT#130_20"]
pub type EnblGpio186inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO186INTToSIO` reader - Enable GPIO186 Interrupt To SIO"]
pub type EnblGpio186inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO186INTToSIO` writer - Enable GPIO186 Interrupt To SIO"]
pub type EnblGpio186inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO186 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio186inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio186inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio186inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO186INTTargetRstTolerance` reader - GPIO186 Interrupt Target Reset Tolerance"]
pub type Gpio186inttargetRstToleranceR = crate::BitReader<Gpio186inttargetRstTolerance>;
impl Gpio186inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio186inttargetRstTolerance {
        match self.bits {
            false => Gpio186inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio186inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio186inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio186inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO186INTTargetRstTolerance` writer - GPIO186 Interrupt Target Reset Tolerance"]
pub type Gpio186inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio186inttargetRstTolerance>;
impl<'a, REG> Gpio186inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio186inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio186inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO186INTTargetWrProt` reader - GPIO186 Interrupt Target Write Protection"]
pub type Gpio186inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO186INTTargetWrProt` writer - GPIO186 Interrupt Target Write Protection"]
pub type Gpio186inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO187INTToINT13018` reader - Enable GPIO187 Interrupt To INT#130_18"]
pub type EnblGpio187inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO187INTToINT13018` writer - Enable GPIO187 Interrupt To INT#130_18"]
pub type EnblGpio187inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO187INTToINT13019` reader - Enable GPIO187 Interrupt To INT#130_19"]
pub type EnblGpio187inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO187INTToINT13019` writer - Enable GPIO187 Interrupt To INT#130_19"]
pub type EnblGpio187inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO187INTToINT13020` reader - Enable GPIO187 Interrupt To INT#130_20"]
pub type EnblGpio187inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO187INTToINT13020` writer - Enable GPIO187 Interrupt To INT#130_20"]
pub type EnblGpio187inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO187INTToSIO` reader - Enable GPIO187 Interrupt To SIO"]
pub type EnblGpio187inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO187INTToSIO` writer - Enable GPIO187 Interrupt To SIO"]
pub type EnblGpio187inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO187 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio187inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio187inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio187inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO187INTTargetRstTolerance` reader - GPIO187 Interrupt Target Reset Tolerance"]
pub type Gpio187inttargetRstToleranceR = crate::BitReader<Gpio187inttargetRstTolerance>;
impl Gpio187inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio187inttargetRstTolerance {
        match self.bits {
            false => Gpio187inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio187inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio187inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio187inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO187INTTargetRstTolerance` writer - GPIO187 Interrupt Target Reset Tolerance"]
pub type Gpio187inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio187inttargetRstTolerance>;
impl<'a, REG> Gpio187inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio187inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio187inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO187INTTargetWrProt` reader - GPIO187 Interrupt Target Write Protection"]
pub type Gpio187inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO187INTTargetWrProt` writer - GPIO187 Interrupt Target Write Protection"]
pub type Gpio187inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO184 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio184intto_int13018(&self) -> EnblGpio184inttoInt13018R {
        EnblGpio184inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO184 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio184intto_int13019(&self) -> EnblGpio184inttoInt13019R {
        EnblGpio184inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO184 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio184intto_int13020(&self) -> EnblGpio184inttoInt13020R {
        EnblGpio184inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO184 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio184intto_sio(&self) -> EnblGpio184inttoSioR {
        EnblGpio184inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO184 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio184inttarget_rst_tolerance(&self) -> Gpio184inttargetRstToleranceR {
        Gpio184inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO184 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio184inttarget_wr_prot(&self) -> Gpio184inttargetWrProtR {
        Gpio184inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO185 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio185intto_int13018(&self) -> EnblGpio185inttoInt13018R {
        EnblGpio185inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO185 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio185intto_int13019(&self) -> EnblGpio185inttoInt13019R {
        EnblGpio185inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO185 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio185intto_int13020(&self) -> EnblGpio185inttoInt13020R {
        EnblGpio185inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO185 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio185intto_sio(&self) -> EnblGpio185inttoSioR {
        EnblGpio185inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO185 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio185inttarget_rst_tolerance(&self) -> Gpio185inttargetRstToleranceR {
        Gpio185inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO185 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio185inttarget_wr_prot(&self) -> Gpio185inttargetWrProtR {
        Gpio185inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO186 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio186intto_int13018(&self) -> EnblGpio186inttoInt13018R {
        EnblGpio186inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO186 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio186intto_int13019(&self) -> EnblGpio186inttoInt13019R {
        EnblGpio186inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO186 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio186intto_int13020(&self) -> EnblGpio186inttoInt13020R {
        EnblGpio186inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO186 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio186intto_sio(&self) -> EnblGpio186inttoSioR {
        EnblGpio186inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO186 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio186inttarget_rst_tolerance(&self) -> Gpio186inttargetRstToleranceR {
        Gpio186inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO186 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio186inttarget_wr_prot(&self) -> Gpio186inttargetWrProtR {
        Gpio186inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO187 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio187intto_int13018(&self) -> EnblGpio187inttoInt13018R {
        EnblGpio187inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO187 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio187intto_int13019(&self) -> EnblGpio187inttoInt13019R {
        EnblGpio187inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO187 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio187intto_int13020(&self) -> EnblGpio187inttoInt13020R {
        EnblGpio187inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO187 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio187intto_sio(&self) -> EnblGpio187inttoSioR {
        EnblGpio187inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO187 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio187inttarget_rst_tolerance(&self) -> Gpio187inttargetRstToleranceR {
        Gpio187inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO187 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio187inttarget_wr_prot(&self) -> Gpio187inttargetWrProtR {
        Gpio187inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO184 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio184intto_int13018(&mut self) -> EnblGpio184inttoInt13018W<Gpioac8Spec> {
        EnblGpio184inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO184 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio184intto_int13019(&mut self) -> EnblGpio184inttoInt13019W<Gpioac8Spec> {
        EnblGpio184inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO184 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio184intto_int13020(&mut self) -> EnblGpio184inttoInt13020W<Gpioac8Spec> {
        EnblGpio184inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO184 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio184intto_sio(&mut self) -> EnblGpio184inttoSioW<Gpioac8Spec> {
        EnblGpio184inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioac8Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioac8Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO184 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio184inttarget_rst_tolerance(&mut self) -> Gpio184inttargetRstToleranceW<Gpioac8Spec> {
        Gpio184inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO184 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio184inttarget_wr_prot(&mut self) -> Gpio184inttargetWrProtW<Gpioac8Spec> {
        Gpio184inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO185 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio185intto_int13018(&mut self) -> EnblGpio185inttoInt13018W<Gpioac8Spec> {
        EnblGpio185inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO185 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio185intto_int13019(&mut self) -> EnblGpio185inttoInt13019W<Gpioac8Spec> {
        EnblGpio185inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO185 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio185intto_int13020(&mut self) -> EnblGpio185inttoInt13020W<Gpioac8Spec> {
        EnblGpio185inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO185 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio185intto_sio(&mut self) -> EnblGpio185inttoSioW<Gpioac8Spec> {
        EnblGpio185inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioac8Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioac8Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO185 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio185inttarget_rst_tolerance(&mut self) -> Gpio185inttargetRstToleranceW<Gpioac8Spec> {
        Gpio185inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO185 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio185inttarget_wr_prot(&mut self) -> Gpio185inttargetWrProtW<Gpioac8Spec> {
        Gpio185inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO186 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio186intto_int13018(&mut self) -> EnblGpio186inttoInt13018W<Gpioac8Spec> {
        EnblGpio186inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO186 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio186intto_int13019(&mut self) -> EnblGpio186inttoInt13019W<Gpioac8Spec> {
        EnblGpio186inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO186 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio186intto_int13020(&mut self) -> EnblGpio186inttoInt13020W<Gpioac8Spec> {
        EnblGpio186inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO186 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio186intto_sio(&mut self) -> EnblGpio186inttoSioW<Gpioac8Spec> {
        EnblGpio186inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioac8Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioac8Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO186 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio186inttarget_rst_tolerance(&mut self) -> Gpio186inttargetRstToleranceW<Gpioac8Spec> {
        Gpio186inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO186 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio186inttarget_wr_prot(&mut self) -> Gpio186inttargetWrProtW<Gpioac8Spec> {
        Gpio186inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO187 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio187intto_int13018(&mut self) -> EnblGpio187inttoInt13018W<Gpioac8Spec> {
        EnblGpio187inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO187 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio187intto_int13019(&mut self) -> EnblGpio187inttoInt13019W<Gpioac8Spec> {
        EnblGpio187inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO187 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio187intto_int13020(&mut self) -> EnblGpio187inttoInt13020W<Gpioac8Spec> {
        EnblGpio187inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO187 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio187intto_sio(&mut self) -> EnblGpio187inttoSioW<Gpioac8Spec> {
        EnblGpio187inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioac8Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO187 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio187inttarget_rst_tolerance(&mut self) -> Gpio187inttargetRstToleranceW<Gpioac8Spec> {
        Gpio187inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO187 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio187inttarget_wr_prot(&mut self) -> Gpio187inttargetWrProtW<Gpioac8Spec> {
        Gpio187inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#46\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioac8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioac8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioac8Spec;
impl crate::RegisterSpec for Gpioac8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioac8::R`](R) reader structure"]
impl crate::Readable for Gpioac8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioac8::W`](W) writer structure"]
impl crate::Writable for Gpioac8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOAC8 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioac8Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
