#[doc = "Register `GPIOA24` reader"]
pub type R = crate::R<Gpioa24Spec>;
#[doc = "Register `GPIOA24` writer"]
pub type W = crate::W<Gpioa24Spec>;
#[doc = "Field `EnblGPIO020INTToINT13018` reader - Enable GPIO020 Interrupt To INT#130_18"]
pub type EnblGpio020inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO020INTToINT13018` writer - Enable GPIO020 Interrupt To INT#130_18"]
pub type EnblGpio020inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO020INTToINT13019` reader - Enable GPIO020 Interrupt To INT#130_19"]
pub type EnblGpio020inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO020INTToINT13019` writer - Enable GPIO020 Interrupt To INT#130_19"]
pub type EnblGpio020inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO020INTToINT13020` reader - Enable GPIO020 Interrupt To INT#130_20"]
pub type EnblGpio020inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO020INTToINT13020` writer - Enable GPIO020 Interrupt To INT#130_20"]
pub type EnblGpio020inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO020INTToSIO` reader - Enable GPIO020 Interrupt To SIO"]
pub type EnblGpio020inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO020INTToSIO` writer - Enable GPIO020 Interrupt To SIO"]
pub type EnblGpio020inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO020 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio020inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio020inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio020inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO020INTTargetRstTolerance` reader - GPIO020 Interrupt Target Reset Tolerance"]
pub type Gpio020inttargetRstToleranceR = crate::BitReader<Gpio020inttargetRstTolerance>;
impl Gpio020inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio020inttargetRstTolerance {
        match self.bits {
            false => Gpio020inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio020inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio020inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio020inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO020INTTargetRstTolerance` writer - GPIO020 Interrupt Target Reset Tolerance"]
pub type Gpio020inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio020inttargetRstTolerance>;
impl<'a, REG> Gpio020inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio020inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio020inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO020INTTargetWrProt` reader - GPIO020 Interrupt Target Write Protection"]
pub type Gpio020inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO020INTTargetWrProt` writer - GPIO020 Interrupt Target Write Protection"]
pub type Gpio020inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO021INTToINT13018` reader - Enable GPIO021 Interrupt To INT#130_18"]
pub type EnblGpio021inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO021INTToINT13018` writer - Enable GPIO021 Interrupt To INT#130_18"]
pub type EnblGpio021inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO021INTToINT13019` reader - Enable GPIO021 Interrupt To INT#130_19"]
pub type EnblGpio021inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO021INTToINT13019` writer - Enable GPIO021 Interrupt To INT#130_19"]
pub type EnblGpio021inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO021INTToINT13020` reader - Enable GPIO021 Interrupt To INT#130_20"]
pub type EnblGpio021inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO021INTToINT13020` writer - Enable GPIO021 Interrupt To INT#130_20"]
pub type EnblGpio021inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO021INTToSIO` reader - Enable GPIO021 Interrupt To SIO"]
pub type EnblGpio021inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO021INTToSIO` writer - Enable GPIO021 Interrupt To SIO"]
pub type EnblGpio021inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO021 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio021inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio021inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio021inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO021INTTargetRstTolerance` reader - GPIO021 Interrupt Target Reset Tolerance"]
pub type Gpio021inttargetRstToleranceR = crate::BitReader<Gpio021inttargetRstTolerance>;
impl Gpio021inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio021inttargetRstTolerance {
        match self.bits {
            false => Gpio021inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio021inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio021inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio021inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO021INTTargetRstTolerance` writer - GPIO021 Interrupt Target Reset Tolerance"]
pub type Gpio021inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio021inttargetRstTolerance>;
impl<'a, REG> Gpio021inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio021inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio021inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO021INTTargetWrProt` reader - GPIO021 Interrupt Target Write Protection"]
pub type Gpio021inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO021INTTargetWrProt` writer - GPIO021 Interrupt Target Write Protection"]
pub type Gpio021inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO022INTToINT13018` reader - Enable GPIO022 Interrupt To INT#130_18"]
pub type EnblGpio022inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO022INTToINT13018` writer - Enable GPIO022 Interrupt To INT#130_18"]
pub type EnblGpio022inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO022INTToINT13019` reader - Enable GPIO022 Interrupt To INT#130_19"]
pub type EnblGpio022inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO022INTToINT13019` writer - Enable GPIO022 Interrupt To INT#130_19"]
pub type EnblGpio022inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO022INTToINT13020` reader - Enable GPIO022 Interrupt To INT#130_20"]
pub type EnblGpio022inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO022INTToINT13020` writer - Enable GPIO022 Interrupt To INT#130_20"]
pub type EnblGpio022inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO022INTToSIO` reader - Enable GPIO022 Interrupt To SIO"]
pub type EnblGpio022inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO022INTToSIO` writer - Enable GPIO022 Interrupt To SIO"]
pub type EnblGpio022inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO022 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio022inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio022inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio022inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO022INTTargetRstTolerance` reader - GPIO022 Interrupt Target Reset Tolerance"]
pub type Gpio022inttargetRstToleranceR = crate::BitReader<Gpio022inttargetRstTolerance>;
impl Gpio022inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio022inttargetRstTolerance {
        match self.bits {
            false => Gpio022inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio022inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio022inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio022inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO022INTTargetRstTolerance` writer - GPIO022 Interrupt Target Reset Tolerance"]
pub type Gpio022inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio022inttargetRstTolerance>;
impl<'a, REG> Gpio022inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio022inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio022inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO022INTTargetWrProt` reader - GPIO022 Interrupt Target Write Protection"]
pub type Gpio022inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO022INTTargetWrProt` writer - GPIO022 Interrupt Target Write Protection"]
pub type Gpio022inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO023INTToINT13018` reader - Enable GPIO023 Interrupt To INT#130_18"]
pub type EnblGpio023inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO023INTToINT13018` writer - Enable GPIO023 Interrupt To INT#130_18"]
pub type EnblGpio023inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO023INTToINT13019` reader - Enable GPIO023 Interrupt To INT#130_19"]
pub type EnblGpio023inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO023INTToINT13019` writer - Enable GPIO023 Interrupt To INT#130_19"]
pub type EnblGpio023inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO023INTToINT13020` reader - Enable GPIO023 Interrupt To INT#130_20"]
pub type EnblGpio023inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO023INTToINT13020` writer - Enable GPIO023 Interrupt To INT#130_20"]
pub type EnblGpio023inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO023INTToSIO` reader - Enable GPIO023 Interrupt To SIO"]
pub type EnblGpio023inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO023INTToSIO` writer - Enable GPIO023 Interrupt To SIO"]
pub type EnblGpio023inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO023 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio023inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio023inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio023inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO023INTTargetRstTolerance` reader - GPIO023 Interrupt Target Reset Tolerance"]
pub type Gpio023inttargetRstToleranceR = crate::BitReader<Gpio023inttargetRstTolerance>;
impl Gpio023inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio023inttargetRstTolerance {
        match self.bits {
            false => Gpio023inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio023inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio023inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio023inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO023INTTargetRstTolerance` writer - GPIO023 Interrupt Target Reset Tolerance"]
pub type Gpio023inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio023inttargetRstTolerance>;
impl<'a, REG> Gpio023inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio023inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio023inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO023INTTargetWrProt` reader - GPIO023 Interrupt Target Write Protection"]
pub type Gpio023inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO023INTTargetWrProt` writer - GPIO023 Interrupt Target Write Protection"]
pub type Gpio023inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO020 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio020intto_int13018(&self) -> EnblGpio020inttoInt13018R {
        EnblGpio020inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO020 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio020intto_int13019(&self) -> EnblGpio020inttoInt13019R {
        EnblGpio020inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO020 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio020intto_int13020(&self) -> EnblGpio020inttoInt13020R {
        EnblGpio020inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO020 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio020intto_sio(&self) -> EnblGpio020inttoSioR {
        EnblGpio020inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO020 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio020inttarget_rst_tolerance(&self) -> Gpio020inttargetRstToleranceR {
        Gpio020inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO020 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio020inttarget_wr_prot(&self) -> Gpio020inttargetWrProtR {
        Gpio020inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO021 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio021intto_int13018(&self) -> EnblGpio021inttoInt13018R {
        EnblGpio021inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO021 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio021intto_int13019(&self) -> EnblGpio021inttoInt13019R {
        EnblGpio021inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO021 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio021intto_int13020(&self) -> EnblGpio021inttoInt13020R {
        EnblGpio021inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO021 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio021intto_sio(&self) -> EnblGpio021inttoSioR {
        EnblGpio021inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO021 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio021inttarget_rst_tolerance(&self) -> Gpio021inttargetRstToleranceR {
        Gpio021inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO021 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio021inttarget_wr_prot(&self) -> Gpio021inttargetWrProtR {
        Gpio021inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO022 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio022intto_int13018(&self) -> EnblGpio022inttoInt13018R {
        EnblGpio022inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO022 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio022intto_int13019(&self) -> EnblGpio022inttoInt13019R {
        EnblGpio022inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO022 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio022intto_int13020(&self) -> EnblGpio022inttoInt13020R {
        EnblGpio022inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO022 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio022intto_sio(&self) -> EnblGpio022inttoSioR {
        EnblGpio022inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO022 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio022inttarget_rst_tolerance(&self) -> Gpio022inttargetRstToleranceR {
        Gpio022inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO022 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio022inttarget_wr_prot(&self) -> Gpio022inttargetWrProtR {
        Gpio022inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO023 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio023intto_int13018(&self) -> EnblGpio023inttoInt13018R {
        EnblGpio023inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO023 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio023intto_int13019(&self) -> EnblGpio023inttoInt13019R {
        EnblGpio023inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO023 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio023intto_int13020(&self) -> EnblGpio023inttoInt13020R {
        EnblGpio023inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO023 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio023intto_sio(&self) -> EnblGpio023inttoSioR {
        EnblGpio023inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO023 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio023inttarget_rst_tolerance(&self) -> Gpio023inttargetRstToleranceR {
        Gpio023inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO023 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio023inttarget_wr_prot(&self) -> Gpio023inttargetWrProtR {
        Gpio023inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO020 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio020intto_int13018(&mut self) -> EnblGpio020inttoInt13018W<Gpioa24Spec> {
        EnblGpio020inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO020 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio020intto_int13019(&mut self) -> EnblGpio020inttoInt13019W<Gpioa24Spec> {
        EnblGpio020inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO020 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio020intto_int13020(&mut self) -> EnblGpio020inttoInt13020W<Gpioa24Spec> {
        EnblGpio020inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO020 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio020intto_sio(&mut self) -> EnblGpio020inttoSioW<Gpioa24Spec> {
        EnblGpio020inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa24Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa24Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO020 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio020inttarget_rst_tolerance(&mut self) -> Gpio020inttargetRstToleranceW<Gpioa24Spec> {
        Gpio020inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO020 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio020inttarget_wr_prot(&mut self) -> Gpio020inttargetWrProtW<Gpioa24Spec> {
        Gpio020inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO021 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio021intto_int13018(&mut self) -> EnblGpio021inttoInt13018W<Gpioa24Spec> {
        EnblGpio021inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO021 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio021intto_int13019(&mut self) -> EnblGpio021inttoInt13019W<Gpioa24Spec> {
        EnblGpio021inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO021 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio021intto_int13020(&mut self) -> EnblGpio021inttoInt13020W<Gpioa24Spec> {
        EnblGpio021inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO021 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio021intto_sio(&mut self) -> EnblGpio021inttoSioW<Gpioa24Spec> {
        EnblGpio021inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa24Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa24Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO021 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio021inttarget_rst_tolerance(&mut self) -> Gpio021inttargetRstToleranceW<Gpioa24Spec> {
        Gpio021inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO021 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio021inttarget_wr_prot(&mut self) -> Gpio021inttargetWrProtW<Gpioa24Spec> {
        Gpio021inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO022 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio022intto_int13018(&mut self) -> EnblGpio022inttoInt13018W<Gpioa24Spec> {
        EnblGpio022inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO022 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio022intto_int13019(&mut self) -> EnblGpio022inttoInt13019W<Gpioa24Spec> {
        EnblGpio022inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO022 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio022intto_int13020(&mut self) -> EnblGpio022inttoInt13020W<Gpioa24Spec> {
        EnblGpio022inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO022 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio022intto_sio(&mut self) -> EnblGpio022inttoSioW<Gpioa24Spec> {
        EnblGpio022inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa24Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa24Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO022 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio022inttarget_rst_tolerance(&mut self) -> Gpio022inttargetRstToleranceW<Gpioa24Spec> {
        Gpio022inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO022 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio022inttarget_wr_prot(&mut self) -> Gpio022inttargetWrProtW<Gpioa24Spec> {
        Gpio022inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO023 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio023intto_int13018(&mut self) -> EnblGpio023inttoInt13018W<Gpioa24Spec> {
        EnblGpio023inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO023 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio023intto_int13019(&mut self) -> EnblGpio023inttoInt13019W<Gpioa24Spec> {
        EnblGpio023inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO023 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio023intto_int13020(&mut self) -> EnblGpio023inttoInt13020W<Gpioa24Spec> {
        EnblGpio023inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO023 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio023intto_sio(&mut self) -> EnblGpio023inttoSioW<Gpioa24Spec> {
        EnblGpio023inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa24Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO023 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio023inttarget_rst_tolerance(&mut self) -> Gpio023inttargetRstToleranceW<Gpioa24Spec> {
        Gpio023inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO023 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio023inttarget_wr_prot(&mut self) -> Gpio023inttargetWrProtW<Gpioa24Spec> {
        Gpio023inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa24Spec;
impl crate::RegisterSpec for Gpioa24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa24::R`](R) reader structure"]
impl crate::Readable for Gpioa24Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa24::W`](W) writer structure"]
impl crate::Writable for Gpioa24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA24 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa24Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
