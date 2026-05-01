#[doc = "Register `GPIOA18` reader"]
pub type R = crate::R<Gpioa18Spec>;
#[doc = "Register `GPIOA18` writer"]
pub type W = crate::W<Gpioa18Spec>;
#[doc = "Field `EnblGPIO008INTToINT13018` reader - Enable GPIO008 Interrupt To INT#130_18"]
pub type EnblGpio008inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO008INTToINT13018` writer - Enable GPIO008 Interrupt To INT#130_18"]
pub type EnblGpio008inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO008INTToINT13019` reader - Enable GPIO008 Interrupt To INT#130_19"]
pub type EnblGpio008inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO008INTToINT13019` writer - Enable GPIO008 Interrupt To INT#130_19"]
pub type EnblGpio008inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO008INTToINT13020` reader - Enable GPIO008 Interrupt To INT#130_20"]
pub type EnblGpio008inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO008INTToINT13020` writer - Enable GPIO008 Interrupt To INT#130_20"]
pub type EnblGpio008inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO008INTToSIO` reader - Enable GPIO008 Interrupt To SIO"]
pub type EnblGpio008inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO008INTToSIO` writer - Enable GPIO008 Interrupt To SIO"]
pub type EnblGpio008inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO008 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio008inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio008inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio008inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO008INTTargetRstTolerance` reader - GPIO008 Interrupt Target Reset Tolerance"]
pub type Gpio008inttargetRstToleranceR = crate::BitReader<Gpio008inttargetRstTolerance>;
impl Gpio008inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio008inttargetRstTolerance {
        match self.bits {
            false => Gpio008inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio008inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio008inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio008inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO008INTTargetRstTolerance` writer - GPIO008 Interrupt Target Reset Tolerance"]
pub type Gpio008inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio008inttargetRstTolerance>;
impl<'a, REG> Gpio008inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio008inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio008inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO008INTTargetWrProt` reader - GPIO008 Interrupt Target Write Protection"]
pub type Gpio008inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO008INTTargetWrProt` writer - GPIO008 Interrupt Target Write Protection"]
pub type Gpio008inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO009INTToINT13018` reader - Enable GPIO009 Interrupt To INT#130_18"]
pub type EnblGpio009inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO009INTToINT13018` writer - Enable GPIO009 Interrupt To INT#130_18"]
pub type EnblGpio009inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO009INTToINT13019` reader - Enable GPIO009 Interrupt To INT#130_19"]
pub type EnblGpio009inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO009INTToINT13019` writer - Enable GPIO009 Interrupt To INT#130_19"]
pub type EnblGpio009inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO009INTToINT13020` reader - Enable GPIO009 Interrupt To INT#130_20"]
pub type EnblGpio009inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO009INTToINT13020` writer - Enable GPIO009 Interrupt To INT#130_20"]
pub type EnblGpio009inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO009INTToSIO` reader - Enable GPIO009 Interrupt To SIO"]
pub type EnblGpio009inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO009INTToSIO` writer - Enable GPIO009 Interrupt To SIO"]
pub type EnblGpio009inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO009 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio009inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio009inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio009inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO009INTTargetRstTolerance` reader - GPIO009 Interrupt Target Reset Tolerance"]
pub type Gpio009inttargetRstToleranceR = crate::BitReader<Gpio009inttargetRstTolerance>;
impl Gpio009inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio009inttargetRstTolerance {
        match self.bits {
            false => Gpio009inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio009inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio009inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio009inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO009INTTargetRstTolerance` writer - GPIO009 Interrupt Target Reset Tolerance"]
pub type Gpio009inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio009inttargetRstTolerance>;
impl<'a, REG> Gpio009inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio009inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio009inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO009INTTargetWrProt` reader - GPIO009 Interrupt Target Write Protection"]
pub type Gpio009inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO009INTTargetWrProt` writer - GPIO009 Interrupt Target Write Protection"]
pub type Gpio009inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO010INTToINT13018` reader - Enable GPIO010 Interrupt To INT#130_18"]
pub type EnblGpio010inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO010INTToINT13018` writer - Enable GPIO010 Interrupt To INT#130_18"]
pub type EnblGpio010inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO010INTToINT13019` reader - Enable GPIO010 Interrupt To INT#130_19"]
pub type EnblGpio010inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO010INTToINT13019` writer - Enable GPIO010 Interrupt To INT#130_19"]
pub type EnblGpio010inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO010INTToINT13020` reader - Enable GPIO010 Interrupt To INT#130_20"]
pub type EnblGpio010inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO010INTToINT13020` writer - Enable GPIO010 Interrupt To INT#130_20"]
pub type EnblGpio010inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO010INTToSIO` reader - Enable GPIO010 Interrupt To SIO"]
pub type EnblGpio010inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO010INTToSIO` writer - Enable GPIO010 Interrupt To SIO"]
pub type EnblGpio010inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO010 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio010inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio010inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio010inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO010INTTargetRstTolerance` reader - GPIO010 Interrupt Target Reset Tolerance"]
pub type Gpio010inttargetRstToleranceR = crate::BitReader<Gpio010inttargetRstTolerance>;
impl Gpio010inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio010inttargetRstTolerance {
        match self.bits {
            false => Gpio010inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio010inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio010inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio010inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO010INTTargetRstTolerance` writer - GPIO010 Interrupt Target Reset Tolerance"]
pub type Gpio010inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio010inttargetRstTolerance>;
impl<'a, REG> Gpio010inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio010inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio010inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO010INTTargetWrProt` reader - GPIO010 Interrupt Target Write Protection"]
pub type Gpio010inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO010INTTargetWrProt` writer - GPIO010 Interrupt Target Write Protection"]
pub type Gpio010inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO011INTToINT13018` reader - Enable GPIO011 Interrupt To INT#130_18"]
pub type EnblGpio011inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO011INTToINT13018` writer - Enable GPIO011 Interrupt To INT#130_18"]
pub type EnblGpio011inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO011INTToINT13019` reader - Enable GPIO011 Interrupt To INT#130_19"]
pub type EnblGpio011inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO011INTToINT13019` writer - Enable GPIO011 Interrupt To INT#130_19"]
pub type EnblGpio011inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO011INTToINT13020` reader - Enable GPIO011 Interrupt To INT#130_20"]
pub type EnblGpio011inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO011INTToINT13020` writer - Enable GPIO011 Interrupt To INT#130_20"]
pub type EnblGpio011inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO011INTToSIO` reader - Enable GPIO011 Interrupt To SIO"]
pub type EnblGpio011inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO011INTToSIO` writer - Enable GPIO011 Interrupt To SIO"]
pub type EnblGpio011inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO011 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio011inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio011inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio011inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO011INTTargetRstTolerance` reader - GPIO011 Interrupt Target Reset Tolerance"]
pub type Gpio011inttargetRstToleranceR = crate::BitReader<Gpio011inttargetRstTolerance>;
impl Gpio011inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio011inttargetRstTolerance {
        match self.bits {
            false => Gpio011inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio011inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio011inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio011inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO011INTTargetRstTolerance` writer - GPIO011 Interrupt Target Reset Tolerance"]
pub type Gpio011inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio011inttargetRstTolerance>;
impl<'a, REG> Gpio011inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio011inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio011inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO011INTTargetWrProt` reader - GPIO011 Interrupt Target Write Protection"]
pub type Gpio011inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO011INTTargetWrProt` writer - GPIO011 Interrupt Target Write Protection"]
pub type Gpio011inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO008 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio008intto_int13018(&self) -> EnblGpio008inttoInt13018R {
        EnblGpio008inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO008 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio008intto_int13019(&self) -> EnblGpio008inttoInt13019R {
        EnblGpio008inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO008 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio008intto_int13020(&self) -> EnblGpio008inttoInt13020R {
        EnblGpio008inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO008 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio008intto_sio(&self) -> EnblGpio008inttoSioR {
        EnblGpio008inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO008 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio008inttarget_rst_tolerance(&self) -> Gpio008inttargetRstToleranceR {
        Gpio008inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO008 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio008inttarget_wr_prot(&self) -> Gpio008inttargetWrProtR {
        Gpio008inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO009 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio009intto_int13018(&self) -> EnblGpio009inttoInt13018R {
        EnblGpio009inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO009 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio009intto_int13019(&self) -> EnblGpio009inttoInt13019R {
        EnblGpio009inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO009 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio009intto_int13020(&self) -> EnblGpio009inttoInt13020R {
        EnblGpio009inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO009 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio009intto_sio(&self) -> EnblGpio009inttoSioR {
        EnblGpio009inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO009 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio009inttarget_rst_tolerance(&self) -> Gpio009inttargetRstToleranceR {
        Gpio009inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO009 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio009inttarget_wr_prot(&self) -> Gpio009inttargetWrProtR {
        Gpio009inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO010 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio010intto_int13018(&self) -> EnblGpio010inttoInt13018R {
        EnblGpio010inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO010 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio010intto_int13019(&self) -> EnblGpio010inttoInt13019R {
        EnblGpio010inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO010 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio010intto_int13020(&self) -> EnblGpio010inttoInt13020R {
        EnblGpio010inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO010 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio010intto_sio(&self) -> EnblGpio010inttoSioR {
        EnblGpio010inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO010 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio010inttarget_rst_tolerance(&self) -> Gpio010inttargetRstToleranceR {
        Gpio010inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO010 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio010inttarget_wr_prot(&self) -> Gpio010inttargetWrProtR {
        Gpio010inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO011 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio011intto_int13018(&self) -> EnblGpio011inttoInt13018R {
        EnblGpio011inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO011 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio011intto_int13019(&self) -> EnblGpio011inttoInt13019R {
        EnblGpio011inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO011 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio011intto_int13020(&self) -> EnblGpio011inttoInt13020R {
        EnblGpio011inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO011 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio011intto_sio(&self) -> EnblGpio011inttoSioR {
        EnblGpio011inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO011 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio011inttarget_rst_tolerance(&self) -> Gpio011inttargetRstToleranceR {
        Gpio011inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO011 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio011inttarget_wr_prot(&self) -> Gpio011inttargetWrProtR {
        Gpio011inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO008 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio008intto_int13018(&mut self) -> EnblGpio008inttoInt13018W<Gpioa18Spec> {
        EnblGpio008inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO008 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio008intto_int13019(&mut self) -> EnblGpio008inttoInt13019W<Gpioa18Spec> {
        EnblGpio008inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO008 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio008intto_int13020(&mut self) -> EnblGpio008inttoInt13020W<Gpioa18Spec> {
        EnblGpio008inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO008 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio008intto_sio(&mut self) -> EnblGpio008inttoSioW<Gpioa18Spec> {
        EnblGpio008inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa18Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa18Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO008 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio008inttarget_rst_tolerance(&mut self) -> Gpio008inttargetRstToleranceW<Gpioa18Spec> {
        Gpio008inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO008 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio008inttarget_wr_prot(&mut self) -> Gpio008inttargetWrProtW<Gpioa18Spec> {
        Gpio008inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO009 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio009intto_int13018(&mut self) -> EnblGpio009inttoInt13018W<Gpioa18Spec> {
        EnblGpio009inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO009 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio009intto_int13019(&mut self) -> EnblGpio009inttoInt13019W<Gpioa18Spec> {
        EnblGpio009inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO009 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio009intto_int13020(&mut self) -> EnblGpio009inttoInt13020W<Gpioa18Spec> {
        EnblGpio009inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO009 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio009intto_sio(&mut self) -> EnblGpio009inttoSioW<Gpioa18Spec> {
        EnblGpio009inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa18Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa18Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO009 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio009inttarget_rst_tolerance(&mut self) -> Gpio009inttargetRstToleranceW<Gpioa18Spec> {
        Gpio009inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO009 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio009inttarget_wr_prot(&mut self) -> Gpio009inttargetWrProtW<Gpioa18Spec> {
        Gpio009inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO010 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio010intto_int13018(&mut self) -> EnblGpio010inttoInt13018W<Gpioa18Spec> {
        EnblGpio010inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO010 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio010intto_int13019(&mut self) -> EnblGpio010inttoInt13019W<Gpioa18Spec> {
        EnblGpio010inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO010 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio010intto_int13020(&mut self) -> EnblGpio010inttoInt13020W<Gpioa18Spec> {
        EnblGpio010inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO010 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio010intto_sio(&mut self) -> EnblGpio010inttoSioW<Gpioa18Spec> {
        EnblGpio010inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa18Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa18Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO010 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio010inttarget_rst_tolerance(&mut self) -> Gpio010inttargetRstToleranceW<Gpioa18Spec> {
        Gpio010inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO010 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio010inttarget_wr_prot(&mut self) -> Gpio010inttargetWrProtW<Gpioa18Spec> {
        Gpio010inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO011 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio011intto_int13018(&mut self) -> EnblGpio011inttoInt13018W<Gpioa18Spec> {
        EnblGpio011inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO011 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio011intto_int13019(&mut self) -> EnblGpio011inttoInt13019W<Gpioa18Spec> {
        EnblGpio011inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO011 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio011intto_int13020(&mut self) -> EnblGpio011inttoInt13020W<Gpioa18Spec> {
        EnblGpio011inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO011 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio011intto_sio(&mut self) -> EnblGpio011inttoSioW<Gpioa18Spec> {
        EnblGpio011inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa18Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO011 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio011inttarget_rst_tolerance(&mut self) -> Gpio011inttargetRstToleranceW<Gpioa18Spec> {
        Gpio011inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO011 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio011inttarget_wr_prot(&mut self) -> Gpio011inttargetWrProtW<Gpioa18Spec> {
        Gpio011inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa18Spec;
impl crate::RegisterSpec for Gpioa18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa18::R`](R) reader structure"]
impl crate::Readable for Gpioa18Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa18::W`](W) writer structure"]
impl crate::Writable for Gpioa18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA18 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa18Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
