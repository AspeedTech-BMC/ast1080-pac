#[doc = "Register `GPIOABC` reader"]
pub type R = crate::R<GpioabcSpec>;
#[doc = "Register `GPIOABC` writer"]
pub type W = crate::W<GpioabcSpec>;
#[doc = "Field `EnblGPIO172INTToINT13018` reader - Enable GPIO172 Interrupt To INT#130_18"]
pub type EnblGpio172inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO172INTToINT13018` writer - Enable GPIO172 Interrupt To INT#130_18"]
pub type EnblGpio172inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO172INTToINT13019` reader - Enable GPIO172 Interrupt To INT#130_19"]
pub type EnblGpio172inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO172INTToINT13019` writer - Enable GPIO172 Interrupt To INT#130_19"]
pub type EnblGpio172inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO172INTToINT13020` reader - Enable GPIO172 Interrupt To INT#130_20"]
pub type EnblGpio172inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO172INTToINT13020` writer - Enable GPIO172 Interrupt To INT#130_20"]
pub type EnblGpio172inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO172INTToSIO` reader - Enable GPIO172 Interrupt To SIO"]
pub type EnblGpio172inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO172INTToSIO` writer - Enable GPIO172 Interrupt To SIO"]
pub type EnblGpio172inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO172 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio172inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio172inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio172inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO172INTTargetRstTolerance` reader - GPIO172 Interrupt Target Reset Tolerance"]
pub type Gpio172inttargetRstToleranceR = crate::BitReader<Gpio172inttargetRstTolerance>;
impl Gpio172inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio172inttargetRstTolerance {
        match self.bits {
            false => Gpio172inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio172inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio172inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio172inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO172INTTargetRstTolerance` writer - GPIO172 Interrupt Target Reset Tolerance"]
pub type Gpio172inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio172inttargetRstTolerance>;
impl<'a, REG> Gpio172inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio172inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio172inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO172INTTargetWrProt` reader - GPIO172 Interrupt Target Write Protection"]
pub type Gpio172inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO172INTTargetWrProt` writer - GPIO172 Interrupt Target Write Protection"]
pub type Gpio172inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO173INTToINT13018` reader - Enable GPIO173 Interrupt To INT#130_18"]
pub type EnblGpio173inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO173INTToINT13018` writer - Enable GPIO173 Interrupt To INT#130_18"]
pub type EnblGpio173inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO173INTToINT13019` reader - Enable GPIO173 Interrupt To INT#130_19"]
pub type EnblGpio173inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO173INTToINT13019` writer - Enable GPIO173 Interrupt To INT#130_19"]
pub type EnblGpio173inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO173INTToINT13020` reader - Enable GPIO173 Interrupt To INT#130_20"]
pub type EnblGpio173inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO173INTToINT13020` writer - Enable GPIO173 Interrupt To INT#130_20"]
pub type EnblGpio173inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO173INTToSIO` reader - Enable GPIO173 Interrupt To SIO"]
pub type EnblGpio173inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO173INTToSIO` writer - Enable GPIO173 Interrupt To SIO"]
pub type EnblGpio173inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO173 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio173inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio173inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio173inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO173INTTargetRstTolerance` reader - GPIO173 Interrupt Target Reset Tolerance"]
pub type Gpio173inttargetRstToleranceR = crate::BitReader<Gpio173inttargetRstTolerance>;
impl Gpio173inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio173inttargetRstTolerance {
        match self.bits {
            false => Gpio173inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio173inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio173inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio173inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO173INTTargetRstTolerance` writer - GPIO173 Interrupt Target Reset Tolerance"]
pub type Gpio173inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio173inttargetRstTolerance>;
impl<'a, REG> Gpio173inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio173inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio173inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO173INTTargetWrProt` reader - GPIO173 Interrupt Target Write Protection"]
pub type Gpio173inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO173INTTargetWrProt` writer - GPIO173 Interrupt Target Write Protection"]
pub type Gpio173inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO174INTToINT13018` reader - Enable GPIO174 Interrupt To INT#130_18"]
pub type EnblGpio174inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO174INTToINT13018` writer - Enable GPIO174 Interrupt To INT#130_18"]
pub type EnblGpio174inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO174INTToINT13019` reader - Enable GPIO174 Interrupt To INT#130_19"]
pub type EnblGpio174inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO174INTToINT13019` writer - Enable GPIO174 Interrupt To INT#130_19"]
pub type EnblGpio174inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO174INTToINT13020` reader - Enable GPIO174 Interrupt To INT#130_20"]
pub type EnblGpio174inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO174INTToINT13020` writer - Enable GPIO174 Interrupt To INT#130_20"]
pub type EnblGpio174inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO174INTToSIO` reader - Enable GPIO174 Interrupt To SIO"]
pub type EnblGpio174inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO174INTToSIO` writer - Enable GPIO174 Interrupt To SIO"]
pub type EnblGpio174inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO174 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio174inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio174inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio174inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO174INTTargetRstTolerance` reader - GPIO174 Interrupt Target Reset Tolerance"]
pub type Gpio174inttargetRstToleranceR = crate::BitReader<Gpio174inttargetRstTolerance>;
impl Gpio174inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio174inttargetRstTolerance {
        match self.bits {
            false => Gpio174inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio174inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio174inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio174inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO174INTTargetRstTolerance` writer - GPIO174 Interrupt Target Reset Tolerance"]
pub type Gpio174inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio174inttargetRstTolerance>;
impl<'a, REG> Gpio174inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio174inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio174inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO174INTTargetWrProt` reader - GPIO174 Interrupt Target Write Protection"]
pub type Gpio174inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO174INTTargetWrProt` writer - GPIO174 Interrupt Target Write Protection"]
pub type Gpio174inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO175INTToINT13018` reader - Enable GPIO175 Interrupt To INT#130_18"]
pub type EnblGpio175inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO175INTToINT13018` writer - Enable GPIO175 Interrupt To INT#130_18"]
pub type EnblGpio175inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO175INTToINT13019` reader - Enable GPIO175 Interrupt To INT#130_19"]
pub type EnblGpio175inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO175INTToINT13019` writer - Enable GPIO175 Interrupt To INT#130_19"]
pub type EnblGpio175inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO175INTToINT13020` reader - Enable GPIO175 Interrupt To INT#130_20"]
pub type EnblGpio175inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO175INTToINT13020` writer - Enable GPIO175 Interrupt To INT#130_20"]
pub type EnblGpio175inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO175INTToSIO` reader - Enable GPIO175 Interrupt To SIO"]
pub type EnblGpio175inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO175INTToSIO` writer - Enable GPIO175 Interrupt To SIO"]
pub type EnblGpio175inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO175 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio175inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio175inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio175inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO175INTTargetRstTolerance` reader - GPIO175 Interrupt Target Reset Tolerance"]
pub type Gpio175inttargetRstToleranceR = crate::BitReader<Gpio175inttargetRstTolerance>;
impl Gpio175inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio175inttargetRstTolerance {
        match self.bits {
            false => Gpio175inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio175inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio175inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio175inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO175INTTargetRstTolerance` writer - GPIO175 Interrupt Target Reset Tolerance"]
pub type Gpio175inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio175inttargetRstTolerance>;
impl<'a, REG> Gpio175inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio175inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio175inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO175INTTargetWrProt` reader - GPIO175 Interrupt Target Write Protection"]
pub type Gpio175inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO175INTTargetWrProt` writer - GPIO175 Interrupt Target Write Protection"]
pub type Gpio175inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO172 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio172intto_int13018(&self) -> EnblGpio172inttoInt13018R {
        EnblGpio172inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO172 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio172intto_int13019(&self) -> EnblGpio172inttoInt13019R {
        EnblGpio172inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO172 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio172intto_int13020(&self) -> EnblGpio172inttoInt13020R {
        EnblGpio172inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO172 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio172intto_sio(&self) -> EnblGpio172inttoSioR {
        EnblGpio172inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO172 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio172inttarget_rst_tolerance(&self) -> Gpio172inttargetRstToleranceR {
        Gpio172inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO172 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio172inttarget_wr_prot(&self) -> Gpio172inttargetWrProtR {
        Gpio172inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO173 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio173intto_int13018(&self) -> EnblGpio173inttoInt13018R {
        EnblGpio173inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO173 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio173intto_int13019(&self) -> EnblGpio173inttoInt13019R {
        EnblGpio173inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO173 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio173intto_int13020(&self) -> EnblGpio173inttoInt13020R {
        EnblGpio173inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO173 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio173intto_sio(&self) -> EnblGpio173inttoSioR {
        EnblGpio173inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO173 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio173inttarget_rst_tolerance(&self) -> Gpio173inttargetRstToleranceR {
        Gpio173inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO173 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio173inttarget_wr_prot(&self) -> Gpio173inttargetWrProtR {
        Gpio173inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO174 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio174intto_int13018(&self) -> EnblGpio174inttoInt13018R {
        EnblGpio174inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO174 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio174intto_int13019(&self) -> EnblGpio174inttoInt13019R {
        EnblGpio174inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO174 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio174intto_int13020(&self) -> EnblGpio174inttoInt13020R {
        EnblGpio174inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO174 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio174intto_sio(&self) -> EnblGpio174inttoSioR {
        EnblGpio174inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO174 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio174inttarget_rst_tolerance(&self) -> Gpio174inttargetRstToleranceR {
        Gpio174inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO174 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio174inttarget_wr_prot(&self) -> Gpio174inttargetWrProtR {
        Gpio174inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO175 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio175intto_int13018(&self) -> EnblGpio175inttoInt13018R {
        EnblGpio175inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO175 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio175intto_int13019(&self) -> EnblGpio175inttoInt13019R {
        EnblGpio175inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO175 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio175intto_int13020(&self) -> EnblGpio175inttoInt13020R {
        EnblGpio175inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO175 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio175intto_sio(&self) -> EnblGpio175inttoSioR {
        EnblGpio175inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO175 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio175inttarget_rst_tolerance(&self) -> Gpio175inttargetRstToleranceR {
        Gpio175inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO175 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio175inttarget_wr_prot(&self) -> Gpio175inttargetWrProtR {
        Gpio175inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO172 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio172intto_int13018(&mut self) -> EnblGpio172inttoInt13018W<GpioabcSpec> {
        EnblGpio172inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO172 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio172intto_int13019(&mut self) -> EnblGpio172inttoInt13019W<GpioabcSpec> {
        EnblGpio172inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO172 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio172intto_int13020(&mut self) -> EnblGpio172inttoInt13020W<GpioabcSpec> {
        EnblGpio172inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO172 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio172intto_sio(&mut self) -> EnblGpio172inttoSioW<GpioabcSpec> {
        EnblGpio172inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<GpioabcSpec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<GpioabcSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO172 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio172inttarget_rst_tolerance(&mut self) -> Gpio172inttargetRstToleranceW<GpioabcSpec> {
        Gpio172inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO172 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio172inttarget_wr_prot(&mut self) -> Gpio172inttargetWrProtW<GpioabcSpec> {
        Gpio172inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO173 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio173intto_int13018(&mut self) -> EnblGpio173inttoInt13018W<GpioabcSpec> {
        EnblGpio173inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO173 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio173intto_int13019(&mut self) -> EnblGpio173inttoInt13019W<GpioabcSpec> {
        EnblGpio173inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO173 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio173intto_int13020(&mut self) -> EnblGpio173inttoInt13020W<GpioabcSpec> {
        EnblGpio173inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO173 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio173intto_sio(&mut self) -> EnblGpio173inttoSioW<GpioabcSpec> {
        EnblGpio173inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<GpioabcSpec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<GpioabcSpec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO173 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio173inttarget_rst_tolerance(&mut self) -> Gpio173inttargetRstToleranceW<GpioabcSpec> {
        Gpio173inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO173 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio173inttarget_wr_prot(&mut self) -> Gpio173inttargetWrProtW<GpioabcSpec> {
        Gpio173inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO174 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio174intto_int13018(&mut self) -> EnblGpio174inttoInt13018W<GpioabcSpec> {
        EnblGpio174inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO174 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio174intto_int13019(&mut self) -> EnblGpio174inttoInt13019W<GpioabcSpec> {
        EnblGpio174inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO174 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio174intto_int13020(&mut self) -> EnblGpio174inttoInt13020W<GpioabcSpec> {
        EnblGpio174inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO174 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio174intto_sio(&mut self) -> EnblGpio174inttoSioW<GpioabcSpec> {
        EnblGpio174inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<GpioabcSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<GpioabcSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO174 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio174inttarget_rst_tolerance(&mut self) -> Gpio174inttargetRstToleranceW<GpioabcSpec> {
        Gpio174inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO174 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio174inttarget_wr_prot(&mut self) -> Gpio174inttargetWrProtW<GpioabcSpec> {
        Gpio174inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO175 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio175intto_int13018(&mut self) -> EnblGpio175inttoInt13018W<GpioabcSpec> {
        EnblGpio175inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO175 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio175intto_int13019(&mut self) -> EnblGpio175inttoInt13019W<GpioabcSpec> {
        EnblGpio175inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO175 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio175intto_int13020(&mut self) -> EnblGpio175inttoInt13020W<GpioabcSpec> {
        EnblGpio175inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO175 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio175intto_sio(&mut self) -> EnblGpio175inttoSioW<GpioabcSpec> {
        EnblGpio175inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<GpioabcSpec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO175 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio175inttarget_rst_tolerance(&mut self) -> Gpio175inttargetRstToleranceW<GpioabcSpec> {
        Gpio175inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO175 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio175inttarget_wr_prot(&mut self) -> Gpio175inttargetWrProtW<GpioabcSpec> {
        Gpio175inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#43\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioabc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioabc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioabcSpec;
impl crate::RegisterSpec for GpioabcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioabc::R`](R) reader structure"]
impl crate::Readable for GpioabcSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioabc::W`](W) writer structure"]
impl crate::Writable for GpioabcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOABC to value 0x1f1f_1f1f"]
impl crate::Resettable for GpioabcSpec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
