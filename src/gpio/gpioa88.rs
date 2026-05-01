#[doc = "Register `GPIOA88` reader"]
pub type R = crate::R<Gpioa88Spec>;
#[doc = "Register `GPIOA88` writer"]
pub type W = crate::W<Gpioa88Spec>;
#[doc = "Field `EnblGPIO120INTToINT13018` reader - Enable GPIO120 Interrupt To INT#130_18"]
pub type EnblGpio120inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO120INTToINT13018` writer - Enable GPIO120 Interrupt To INT#130_18"]
pub type EnblGpio120inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO120INTToINT13019` reader - Enable GPIO120 Interrupt To INT#130_19"]
pub type EnblGpio120inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO120INTToINT13019` writer - Enable GPIO120 Interrupt To INT#130_19"]
pub type EnblGpio120inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO120INTToINT13020` reader - Enable GPIO120 Interrupt To INT#130_20"]
pub type EnblGpio120inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO120INTToINT13020` writer - Enable GPIO120 Interrupt To INT#130_20"]
pub type EnblGpio120inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO120INTToSIO` reader - Enable GPIO120 Interrupt To SIO"]
pub type EnblGpio120inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO120INTToSIO` writer - Enable GPIO120 Interrupt To SIO"]
pub type EnblGpio120inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO120 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio120inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio120inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio120inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO120INTTargetRstTolerance` reader - GPIO120 Interrupt Target Reset Tolerance"]
pub type Gpio120inttargetRstToleranceR = crate::BitReader<Gpio120inttargetRstTolerance>;
impl Gpio120inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio120inttargetRstTolerance {
        match self.bits {
            false => Gpio120inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio120inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio120inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio120inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO120INTTargetRstTolerance` writer - GPIO120 Interrupt Target Reset Tolerance"]
pub type Gpio120inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio120inttargetRstTolerance>;
impl<'a, REG> Gpio120inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio120inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio120inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO120INTTargetWrProt` reader - GPIO120 Interrupt Target Write Protection"]
pub type Gpio120inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO120INTTargetWrProt` writer - GPIO120 Interrupt Target Write Protection"]
pub type Gpio120inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO121INTToINT13018` reader - Enable GPIO121 Interrupt To INT#130_18"]
pub type EnblGpio121inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO121INTToINT13018` writer - Enable GPIO121 Interrupt To INT#130_18"]
pub type EnblGpio121inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO121INTToINT13019` reader - Enable GPIO121 Interrupt To INT#130_19"]
pub type EnblGpio121inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO121INTToINT13019` writer - Enable GPIO121 Interrupt To INT#130_19"]
pub type EnblGpio121inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO121INTToINT13020` reader - Enable GPIO121 Interrupt To INT#130_20"]
pub type EnblGpio121inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO121INTToINT13020` writer - Enable GPIO121 Interrupt To INT#130_20"]
pub type EnblGpio121inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO121INTToSIO` reader - Enable GPIO121 Interrupt To SIO"]
pub type EnblGpio121inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO121INTToSIO` writer - Enable GPIO121 Interrupt To SIO"]
pub type EnblGpio121inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO121 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio121inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio121inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio121inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO121INTTargetRstTolerance` reader - GPIO121 Interrupt Target Reset Tolerance"]
pub type Gpio121inttargetRstToleranceR = crate::BitReader<Gpio121inttargetRstTolerance>;
impl Gpio121inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio121inttargetRstTolerance {
        match self.bits {
            false => Gpio121inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio121inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio121inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio121inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO121INTTargetRstTolerance` writer - GPIO121 Interrupt Target Reset Tolerance"]
pub type Gpio121inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio121inttargetRstTolerance>;
impl<'a, REG> Gpio121inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio121inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio121inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO121INTTargetWrProt` reader - GPIO121 Interrupt Target Write Protection"]
pub type Gpio121inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO121INTTargetWrProt` writer - GPIO121 Interrupt Target Write Protection"]
pub type Gpio121inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO122INTToINT13018` reader - Enable GPIO122 Interrupt To INT#130_18"]
pub type EnblGpio122inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO122INTToINT13018` writer - Enable GPIO122 Interrupt To INT#130_18"]
pub type EnblGpio122inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO122INTToINT13019` reader - Enable GPIO122 Interrupt To INT#130_19"]
pub type EnblGpio122inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO122INTToINT13019` writer - Enable GPIO122 Interrupt To INT#130_19"]
pub type EnblGpio122inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO122INTToINT13020` reader - Enable GPIO122 Interrupt To INT#130_20"]
pub type EnblGpio122inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO122INTToINT13020` writer - Enable GPIO122 Interrupt To INT#130_20"]
pub type EnblGpio122inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO122INTToSIO` reader - Enable GPIO122 Interrupt To SIO"]
pub type EnblGpio122inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO122INTToSIO` writer - Enable GPIO122 Interrupt To SIO"]
pub type EnblGpio122inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO122 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio122inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio122inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio122inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO122INTTargetRstTolerance` reader - GPIO122 Interrupt Target Reset Tolerance"]
pub type Gpio122inttargetRstToleranceR = crate::BitReader<Gpio122inttargetRstTolerance>;
impl Gpio122inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio122inttargetRstTolerance {
        match self.bits {
            false => Gpio122inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio122inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio122inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio122inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO122INTTargetRstTolerance` writer - GPIO122 Interrupt Target Reset Tolerance"]
pub type Gpio122inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio122inttargetRstTolerance>;
impl<'a, REG> Gpio122inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio122inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio122inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO122INTTargetWrProt` reader - GPIO122 Interrupt Target Write Protection"]
pub type Gpio122inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO122INTTargetWrProt` writer - GPIO122 Interrupt Target Write Protection"]
pub type Gpio122inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO123INTToINT13018` reader - Enable GPIO123 Interrupt To INT#130_18"]
pub type EnblGpio123inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO123INTToINT13018` writer - Enable GPIO123 Interrupt To INT#130_18"]
pub type EnblGpio123inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO123INTToINT13019` reader - Enable GPIO123 Interrupt To INT#130_19"]
pub type EnblGpio123inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO123INTToINT13019` writer - Enable GPIO123 Interrupt To INT#130_19"]
pub type EnblGpio123inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO123INTToINT13020` reader - Enable GPIO123 Interrupt To INT#130_20"]
pub type EnblGpio123inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO123INTToINT13020` writer - Enable GPIO123 Interrupt To INT#130_20"]
pub type EnblGpio123inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO123INTToSIO` reader - Enable GPIO123 Interrupt To SIO"]
pub type EnblGpio123inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO123INTToSIO` writer - Enable GPIO123 Interrupt To SIO"]
pub type EnblGpio123inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO123 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio123inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio123inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio123inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO123INTTargetRstTolerance` reader - GPIO123 Interrupt Target Reset Tolerance"]
pub type Gpio123inttargetRstToleranceR = crate::BitReader<Gpio123inttargetRstTolerance>;
impl Gpio123inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio123inttargetRstTolerance {
        match self.bits {
            false => Gpio123inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio123inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio123inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio123inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO123INTTargetRstTolerance` writer - GPIO123 Interrupt Target Reset Tolerance"]
pub type Gpio123inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio123inttargetRstTolerance>;
impl<'a, REG> Gpio123inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio123inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio123inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO123INTTargetWrProt` reader - GPIO123 Interrupt Target Write Protection"]
pub type Gpio123inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO123INTTargetWrProt` writer - GPIO123 Interrupt Target Write Protection"]
pub type Gpio123inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO120 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio120intto_int13018(&self) -> EnblGpio120inttoInt13018R {
        EnblGpio120inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO120 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio120intto_int13019(&self) -> EnblGpio120inttoInt13019R {
        EnblGpio120inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO120 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio120intto_int13020(&self) -> EnblGpio120inttoInt13020R {
        EnblGpio120inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO120 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio120intto_sio(&self) -> EnblGpio120inttoSioR {
        EnblGpio120inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO120 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio120inttarget_rst_tolerance(&self) -> Gpio120inttargetRstToleranceR {
        Gpio120inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO120 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio120inttarget_wr_prot(&self) -> Gpio120inttargetWrProtR {
        Gpio120inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO121 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio121intto_int13018(&self) -> EnblGpio121inttoInt13018R {
        EnblGpio121inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO121 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio121intto_int13019(&self) -> EnblGpio121inttoInt13019R {
        EnblGpio121inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO121 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio121intto_int13020(&self) -> EnblGpio121inttoInt13020R {
        EnblGpio121inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO121 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio121intto_sio(&self) -> EnblGpio121inttoSioR {
        EnblGpio121inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO121 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio121inttarget_rst_tolerance(&self) -> Gpio121inttargetRstToleranceR {
        Gpio121inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO121 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio121inttarget_wr_prot(&self) -> Gpio121inttargetWrProtR {
        Gpio121inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO122 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio122intto_int13018(&self) -> EnblGpio122inttoInt13018R {
        EnblGpio122inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO122 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio122intto_int13019(&self) -> EnblGpio122inttoInt13019R {
        EnblGpio122inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO122 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio122intto_int13020(&self) -> EnblGpio122inttoInt13020R {
        EnblGpio122inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO122 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio122intto_sio(&self) -> EnblGpio122inttoSioR {
        EnblGpio122inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO122 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio122inttarget_rst_tolerance(&self) -> Gpio122inttargetRstToleranceR {
        Gpio122inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO122 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio122inttarget_wr_prot(&self) -> Gpio122inttargetWrProtR {
        Gpio122inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO123 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio123intto_int13018(&self) -> EnblGpio123inttoInt13018R {
        EnblGpio123inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO123 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio123intto_int13019(&self) -> EnblGpio123inttoInt13019R {
        EnblGpio123inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO123 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio123intto_int13020(&self) -> EnblGpio123inttoInt13020R {
        EnblGpio123inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO123 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio123intto_sio(&self) -> EnblGpio123inttoSioR {
        EnblGpio123inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO123 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio123inttarget_rst_tolerance(&self) -> Gpio123inttargetRstToleranceR {
        Gpio123inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO123 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio123inttarget_wr_prot(&self) -> Gpio123inttargetWrProtR {
        Gpio123inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO120 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio120intto_int13018(&mut self) -> EnblGpio120inttoInt13018W<Gpioa88Spec> {
        EnblGpio120inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO120 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio120intto_int13019(&mut self) -> EnblGpio120inttoInt13019W<Gpioa88Spec> {
        EnblGpio120inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO120 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio120intto_int13020(&mut self) -> EnblGpio120inttoInt13020W<Gpioa88Spec> {
        EnblGpio120inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO120 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio120intto_sio(&mut self) -> EnblGpio120inttoSioW<Gpioa88Spec> {
        EnblGpio120inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa88Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa88Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO120 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio120inttarget_rst_tolerance(&mut self) -> Gpio120inttargetRstToleranceW<Gpioa88Spec> {
        Gpio120inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO120 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio120inttarget_wr_prot(&mut self) -> Gpio120inttargetWrProtW<Gpioa88Spec> {
        Gpio120inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO121 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio121intto_int13018(&mut self) -> EnblGpio121inttoInt13018W<Gpioa88Spec> {
        EnblGpio121inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO121 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio121intto_int13019(&mut self) -> EnblGpio121inttoInt13019W<Gpioa88Spec> {
        EnblGpio121inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO121 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio121intto_int13020(&mut self) -> EnblGpio121inttoInt13020W<Gpioa88Spec> {
        EnblGpio121inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO121 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio121intto_sio(&mut self) -> EnblGpio121inttoSioW<Gpioa88Spec> {
        EnblGpio121inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa88Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa88Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO121 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio121inttarget_rst_tolerance(&mut self) -> Gpio121inttargetRstToleranceW<Gpioa88Spec> {
        Gpio121inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO121 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio121inttarget_wr_prot(&mut self) -> Gpio121inttargetWrProtW<Gpioa88Spec> {
        Gpio121inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO122 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio122intto_int13018(&mut self) -> EnblGpio122inttoInt13018W<Gpioa88Spec> {
        EnblGpio122inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO122 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio122intto_int13019(&mut self) -> EnblGpio122inttoInt13019W<Gpioa88Spec> {
        EnblGpio122inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO122 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio122intto_int13020(&mut self) -> EnblGpio122inttoInt13020W<Gpioa88Spec> {
        EnblGpio122inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO122 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio122intto_sio(&mut self) -> EnblGpio122inttoSioW<Gpioa88Spec> {
        EnblGpio122inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa88Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa88Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO122 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio122inttarget_rst_tolerance(&mut self) -> Gpio122inttargetRstToleranceW<Gpioa88Spec> {
        Gpio122inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO122 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio122inttarget_wr_prot(&mut self) -> Gpio122inttargetWrProtW<Gpioa88Spec> {
        Gpio122inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO123 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio123intto_int13018(&mut self) -> EnblGpio123inttoInt13018W<Gpioa88Spec> {
        EnblGpio123inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO123 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio123intto_int13019(&mut self) -> EnblGpio123inttoInt13019W<Gpioa88Spec> {
        EnblGpio123inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO123 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio123intto_int13020(&mut self) -> EnblGpio123inttoInt13020W<Gpioa88Spec> {
        EnblGpio123inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO123 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio123intto_sio(&mut self) -> EnblGpio123inttoSioW<Gpioa88Spec> {
        EnblGpio123inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa88Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO123 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio123inttarget_rst_tolerance(&mut self) -> Gpio123inttargetRstToleranceW<Gpioa88Spec> {
        Gpio123inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO123 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio123inttarget_wr_prot(&mut self) -> Gpio123inttargetWrProtW<Gpioa88Spec> {
        Gpio123inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa88::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa88::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa88Spec;
impl crate::RegisterSpec for Gpioa88Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa88::R`](R) reader structure"]
impl crate::Readable for Gpioa88Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa88::W`](W) writer structure"]
impl crate::Writable for Gpioa88Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA88 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa88Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
