#[doc = "Register `GPIOAC4` reader"]
pub type R = crate::R<Gpioac4Spec>;
#[doc = "Register `GPIOAC4` writer"]
pub type W = crate::W<Gpioac4Spec>;
#[doc = "Field `EnblGPIO180INTToINT13018` reader - Enable GPIO180 Interrupt To INT#130_18"]
pub type EnblGpio180inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO180INTToINT13018` writer - Enable GPIO180 Interrupt To INT#130_18"]
pub type EnblGpio180inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO180INTToINT13019` reader - Enable GPIO180 Interrupt To INT#130_19"]
pub type EnblGpio180inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO180INTToINT13019` writer - Enable GPIO180 Interrupt To INT#130_19"]
pub type EnblGpio180inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO180INTToINT13020` reader - Enable GPIO180 Interrupt To INT#130_20"]
pub type EnblGpio180inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO180INTToINT13020` writer - Enable GPIO180 Interrupt To INT#130_20"]
pub type EnblGpio180inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO180INTToSIO` reader - Enable GPIO180 Interrupt To SIO"]
pub type EnblGpio180inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO180INTToSIO` writer - Enable GPIO180 Interrupt To SIO"]
pub type EnblGpio180inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO180 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio180inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio180inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio180inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO180INTTargetRstTolerance` reader - GPIO180 Interrupt Target Reset Tolerance"]
pub type Gpio180inttargetRstToleranceR = crate::BitReader<Gpio180inttargetRstTolerance>;
impl Gpio180inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio180inttargetRstTolerance {
        match self.bits {
            false => Gpio180inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio180inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio180inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio180inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO180INTTargetRstTolerance` writer - GPIO180 Interrupt Target Reset Tolerance"]
pub type Gpio180inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio180inttargetRstTolerance>;
impl<'a, REG> Gpio180inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio180inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio180inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO180INTTargetWrProt` reader - GPIO180 Interrupt Target Write Protection"]
pub type Gpio180inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO180INTTargetWrProt` writer - GPIO180 Interrupt Target Write Protection"]
pub type Gpio180inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO181INTToINT13018` reader - Enable GPIO181 Interrupt To INT#130_18"]
pub type EnblGpio181inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO181INTToINT13018` writer - Enable GPIO181 Interrupt To INT#130_18"]
pub type EnblGpio181inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO181INTToINT13019` reader - Enable GPIO181 Interrupt To INT#130_19"]
pub type EnblGpio181inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO181INTToINT13019` writer - Enable GPIO181 Interrupt To INT#130_19"]
pub type EnblGpio181inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO181INTToINT13020` reader - Enable GPIO181 Interrupt To INT#130_20"]
pub type EnblGpio181inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO181INTToINT13020` writer - Enable GPIO181 Interrupt To INT#130_20"]
pub type EnblGpio181inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO181INTToSIO` reader - Enable GPIO181 Interrupt To SIO"]
pub type EnblGpio181inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO181INTToSIO` writer - Enable GPIO181 Interrupt To SIO"]
pub type EnblGpio181inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO181 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio181inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio181inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio181inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO181INTTargetRstTolerance` reader - GPIO181 Interrupt Target Reset Tolerance"]
pub type Gpio181inttargetRstToleranceR = crate::BitReader<Gpio181inttargetRstTolerance>;
impl Gpio181inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio181inttargetRstTolerance {
        match self.bits {
            false => Gpio181inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio181inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio181inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio181inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO181INTTargetRstTolerance` writer - GPIO181 Interrupt Target Reset Tolerance"]
pub type Gpio181inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio181inttargetRstTolerance>;
impl<'a, REG> Gpio181inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio181inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio181inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO181INTTargetWrProt` reader - GPIO181 Interrupt Target Write Protection"]
pub type Gpio181inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO181INTTargetWrProt` writer - GPIO181 Interrupt Target Write Protection"]
pub type Gpio181inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO182INTToINT13018` reader - Enable GPIO182 Interrupt To INT#130_18"]
pub type EnblGpio182inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO182INTToINT13018` writer - Enable GPIO182 Interrupt To INT#130_18"]
pub type EnblGpio182inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO182INTToINT13019` reader - Enable GPIO182 Interrupt To INT#130_19"]
pub type EnblGpio182inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO182INTToINT13019` writer - Enable GPIO182 Interrupt To INT#130_19"]
pub type EnblGpio182inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO182INTToINT13020` reader - Enable GPIO182 Interrupt To INT#130_20"]
pub type EnblGpio182inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO182INTToINT13020` writer - Enable GPIO182 Interrupt To INT#130_20"]
pub type EnblGpio182inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO182INTToSIO` reader - Enable GPIO182 Interrupt To SIO"]
pub type EnblGpio182inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO182INTToSIO` writer - Enable GPIO182 Interrupt To SIO"]
pub type EnblGpio182inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO182 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio182inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio182inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio182inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO182INTTargetRstTolerance` reader - GPIO182 Interrupt Target Reset Tolerance"]
pub type Gpio182inttargetRstToleranceR = crate::BitReader<Gpio182inttargetRstTolerance>;
impl Gpio182inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio182inttargetRstTolerance {
        match self.bits {
            false => Gpio182inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio182inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio182inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio182inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO182INTTargetRstTolerance` writer - GPIO182 Interrupt Target Reset Tolerance"]
pub type Gpio182inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio182inttargetRstTolerance>;
impl<'a, REG> Gpio182inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio182inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio182inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO182INTTargetWrProt` reader - GPIO182 Interrupt Target Write Protection"]
pub type Gpio182inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO182INTTargetWrProt` writer - GPIO182 Interrupt Target Write Protection"]
pub type Gpio182inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO183INTToINT13018` reader - Enable GPIO183 Interrupt To INT#130_18"]
pub type EnblGpio183inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO183INTToINT13018` writer - Enable GPIO183 Interrupt To INT#130_18"]
pub type EnblGpio183inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO183INTToINT13019` reader - Enable GPIO183 Interrupt To INT#130_19"]
pub type EnblGpio183inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO183INTToINT13019` writer - Enable GPIO183 Interrupt To INT#130_19"]
pub type EnblGpio183inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO183INTToINT13020` reader - Enable GPIO183 Interrupt To INT#130_20"]
pub type EnblGpio183inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO183INTToINT13020` writer - Enable GPIO183 Interrupt To INT#130_20"]
pub type EnblGpio183inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO183INTToSIO` reader - Enable GPIO183 Interrupt To SIO"]
pub type EnblGpio183inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO183INTToSIO` writer - Enable GPIO183 Interrupt To SIO"]
pub type EnblGpio183inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO183 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio183inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio183inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio183inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO183INTTargetRstTolerance` reader - GPIO183 Interrupt Target Reset Tolerance"]
pub type Gpio183inttargetRstToleranceR = crate::BitReader<Gpio183inttargetRstTolerance>;
impl Gpio183inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio183inttargetRstTolerance {
        match self.bits {
            false => Gpio183inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio183inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio183inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio183inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO183INTTargetRstTolerance` writer - GPIO183 Interrupt Target Reset Tolerance"]
pub type Gpio183inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio183inttargetRstTolerance>;
impl<'a, REG> Gpio183inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio183inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio183inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO183INTTargetWrProt` reader - GPIO183 Interrupt Target Write Protection"]
pub type Gpio183inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO183INTTargetWrProt` writer - GPIO183 Interrupt Target Write Protection"]
pub type Gpio183inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO180 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio180intto_int13018(&self) -> EnblGpio180inttoInt13018R {
        EnblGpio180inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO180 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio180intto_int13019(&self) -> EnblGpio180inttoInt13019R {
        EnblGpio180inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO180 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio180intto_int13020(&self) -> EnblGpio180inttoInt13020R {
        EnblGpio180inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO180 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio180intto_sio(&self) -> EnblGpio180inttoSioR {
        EnblGpio180inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO180 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio180inttarget_rst_tolerance(&self) -> Gpio180inttargetRstToleranceR {
        Gpio180inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO180 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio180inttarget_wr_prot(&self) -> Gpio180inttargetWrProtR {
        Gpio180inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO181 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio181intto_int13018(&self) -> EnblGpio181inttoInt13018R {
        EnblGpio181inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO181 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio181intto_int13019(&self) -> EnblGpio181inttoInt13019R {
        EnblGpio181inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO181 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio181intto_int13020(&self) -> EnblGpio181inttoInt13020R {
        EnblGpio181inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO181 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio181intto_sio(&self) -> EnblGpio181inttoSioR {
        EnblGpio181inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO181 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio181inttarget_rst_tolerance(&self) -> Gpio181inttargetRstToleranceR {
        Gpio181inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO181 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio181inttarget_wr_prot(&self) -> Gpio181inttargetWrProtR {
        Gpio181inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO182 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio182intto_int13018(&self) -> EnblGpio182inttoInt13018R {
        EnblGpio182inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO182 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio182intto_int13019(&self) -> EnblGpio182inttoInt13019R {
        EnblGpio182inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO182 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio182intto_int13020(&self) -> EnblGpio182inttoInt13020R {
        EnblGpio182inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO182 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio182intto_sio(&self) -> EnblGpio182inttoSioR {
        EnblGpio182inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO182 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio182inttarget_rst_tolerance(&self) -> Gpio182inttargetRstToleranceR {
        Gpio182inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO182 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio182inttarget_wr_prot(&self) -> Gpio182inttargetWrProtR {
        Gpio182inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO183 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio183intto_int13018(&self) -> EnblGpio183inttoInt13018R {
        EnblGpio183inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO183 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio183intto_int13019(&self) -> EnblGpio183inttoInt13019R {
        EnblGpio183inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO183 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio183intto_int13020(&self) -> EnblGpio183inttoInt13020R {
        EnblGpio183inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO183 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio183intto_sio(&self) -> EnblGpio183inttoSioR {
        EnblGpio183inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO183 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio183inttarget_rst_tolerance(&self) -> Gpio183inttargetRstToleranceR {
        Gpio183inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO183 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio183inttarget_wr_prot(&self) -> Gpio183inttargetWrProtR {
        Gpio183inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO180 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio180intto_int13018(&mut self) -> EnblGpio180inttoInt13018W<Gpioac4Spec> {
        EnblGpio180inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO180 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio180intto_int13019(&mut self) -> EnblGpio180inttoInt13019W<Gpioac4Spec> {
        EnblGpio180inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO180 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio180intto_int13020(&mut self) -> EnblGpio180inttoInt13020W<Gpioac4Spec> {
        EnblGpio180inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO180 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio180intto_sio(&mut self) -> EnblGpio180inttoSioW<Gpioac4Spec> {
        EnblGpio180inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioac4Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioac4Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO180 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio180inttarget_rst_tolerance(&mut self) -> Gpio180inttargetRstToleranceW<Gpioac4Spec> {
        Gpio180inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO180 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio180inttarget_wr_prot(&mut self) -> Gpio180inttargetWrProtW<Gpioac4Spec> {
        Gpio180inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO181 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio181intto_int13018(&mut self) -> EnblGpio181inttoInt13018W<Gpioac4Spec> {
        EnblGpio181inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO181 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio181intto_int13019(&mut self) -> EnblGpio181inttoInt13019W<Gpioac4Spec> {
        EnblGpio181inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO181 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio181intto_int13020(&mut self) -> EnblGpio181inttoInt13020W<Gpioac4Spec> {
        EnblGpio181inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO181 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio181intto_sio(&mut self) -> EnblGpio181inttoSioW<Gpioac4Spec> {
        EnblGpio181inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioac4Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioac4Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO181 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio181inttarget_rst_tolerance(&mut self) -> Gpio181inttargetRstToleranceW<Gpioac4Spec> {
        Gpio181inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO181 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio181inttarget_wr_prot(&mut self) -> Gpio181inttargetWrProtW<Gpioac4Spec> {
        Gpio181inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO182 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio182intto_int13018(&mut self) -> EnblGpio182inttoInt13018W<Gpioac4Spec> {
        EnblGpio182inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO182 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio182intto_int13019(&mut self) -> EnblGpio182inttoInt13019W<Gpioac4Spec> {
        EnblGpio182inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO182 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio182intto_int13020(&mut self) -> EnblGpio182inttoInt13020W<Gpioac4Spec> {
        EnblGpio182inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO182 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio182intto_sio(&mut self) -> EnblGpio182inttoSioW<Gpioac4Spec> {
        EnblGpio182inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioac4Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioac4Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO182 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio182inttarget_rst_tolerance(&mut self) -> Gpio182inttargetRstToleranceW<Gpioac4Spec> {
        Gpio182inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO182 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio182inttarget_wr_prot(&mut self) -> Gpio182inttargetWrProtW<Gpioac4Spec> {
        Gpio182inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO183 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio183intto_int13018(&mut self) -> EnblGpio183inttoInt13018W<Gpioac4Spec> {
        EnblGpio183inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO183 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio183intto_int13019(&mut self) -> EnblGpio183inttoInt13019W<Gpioac4Spec> {
        EnblGpio183inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO183 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio183intto_int13020(&mut self) -> EnblGpio183inttoInt13020W<Gpioac4Spec> {
        EnblGpio183inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO183 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio183intto_sio(&mut self) -> EnblGpio183inttoSioW<Gpioac4Spec> {
        EnblGpio183inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioac4Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO183 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio183inttarget_rst_tolerance(&mut self) -> Gpio183inttargetRstToleranceW<Gpioac4Spec> {
        Gpio183inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO183 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio183inttarget_wr_prot(&mut self) -> Gpio183inttargetWrProtW<Gpioac4Spec> {
        Gpio183inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#45\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioac4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioac4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioac4Spec;
impl crate::RegisterSpec for Gpioac4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioac4::R`](R) reader structure"]
impl crate::Readable for Gpioac4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioac4::W`](W) writer structure"]
impl crate::Writable for Gpioac4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOAC4 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioac4Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
