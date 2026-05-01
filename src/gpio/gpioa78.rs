#[doc = "Register `GPIOA78` reader"]
pub type R = crate::R<Gpioa78Spec>;
#[doc = "Register `GPIOA78` writer"]
pub type W = crate::W<Gpioa78Spec>;
#[doc = "Field `EnblGPIO104INTToINT13018` reader - Enable GPIO104 Interrupt To INT#130_18"]
pub type EnblGpio104inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO104INTToINT13018` writer - Enable GPIO104 Interrupt To INT#130_18"]
pub type EnblGpio104inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO104INTToINT13019` reader - Enable GPIO104 Interrupt To INT#130_19"]
pub type EnblGpio104inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO104INTToINT13019` writer - Enable GPIO104 Interrupt To INT#130_19"]
pub type EnblGpio104inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO104INTToINT13020` reader - Enable GPIO104 Interrupt To INT#130_20"]
pub type EnblGpio104inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO104INTToINT13020` writer - Enable GPIO104 Interrupt To INT#130_20"]
pub type EnblGpio104inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO104INTToSIO` reader - Enable GPIO104 Interrupt To SIO"]
pub type EnblGpio104inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO104INTToSIO` writer - Enable GPIO104 Interrupt To SIO"]
pub type EnblGpio104inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO104 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio104inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio104inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio104inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO104INTTargetRstTolerance` reader - GPIO104 Interrupt Target Reset Tolerance"]
pub type Gpio104inttargetRstToleranceR = crate::BitReader<Gpio104inttargetRstTolerance>;
impl Gpio104inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio104inttargetRstTolerance {
        match self.bits {
            false => Gpio104inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio104inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio104inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio104inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO104INTTargetRstTolerance` writer - GPIO104 Interrupt Target Reset Tolerance"]
pub type Gpio104inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio104inttargetRstTolerance>;
impl<'a, REG> Gpio104inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio104inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio104inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO104INTTargetWrProt` reader - GPIO104 Interrupt Target Write Protection"]
pub type Gpio104inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO104INTTargetWrProt` writer - GPIO104 Interrupt Target Write Protection"]
pub type Gpio104inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO105INTToINT13018` reader - Enable GPIO105 Interrupt To INT#130_18"]
pub type EnblGpio105inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO105INTToINT13018` writer - Enable GPIO105 Interrupt To INT#130_18"]
pub type EnblGpio105inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO105INTToINT13019` reader - Enable GPIO105 Interrupt To INT#130_19"]
pub type EnblGpio105inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO105INTToINT13019` writer - Enable GPIO105 Interrupt To INT#130_19"]
pub type EnblGpio105inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO105INTToINT13020` reader - Enable GPIO105 Interrupt To INT#130_20"]
pub type EnblGpio105inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO105INTToINT13020` writer - Enable GPIO105 Interrupt To INT#130_20"]
pub type EnblGpio105inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO105INTToSIO` reader - Enable GPIO105 Interrupt To SIO"]
pub type EnblGpio105inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO105INTToSIO` writer - Enable GPIO105 Interrupt To SIO"]
pub type EnblGpio105inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO105 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio105inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio105inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio105inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO105INTTargetRstTolerance` reader - GPIO105 Interrupt Target Reset Tolerance"]
pub type Gpio105inttargetRstToleranceR = crate::BitReader<Gpio105inttargetRstTolerance>;
impl Gpio105inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio105inttargetRstTolerance {
        match self.bits {
            false => Gpio105inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio105inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio105inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio105inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO105INTTargetRstTolerance` writer - GPIO105 Interrupt Target Reset Tolerance"]
pub type Gpio105inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio105inttargetRstTolerance>;
impl<'a, REG> Gpio105inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio105inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio105inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO105INTTargetWrProt` reader - GPIO105 Interrupt Target Write Protection"]
pub type Gpio105inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO105INTTargetWrProt` writer - GPIO105 Interrupt Target Write Protection"]
pub type Gpio105inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO106INTToINT13018` reader - Enable GPIO106 Interrupt To INT#130_18"]
pub type EnblGpio106inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO106INTToINT13018` writer - Enable GPIO106 Interrupt To INT#130_18"]
pub type EnblGpio106inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO106INTToINT13019` reader - Enable GPIO106 Interrupt To INT#130_19"]
pub type EnblGpio106inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO106INTToINT13019` writer - Enable GPIO106 Interrupt To INT#130_19"]
pub type EnblGpio106inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO106INTToINT13020` reader - Enable GPIO106 Interrupt To INT#130_20"]
pub type EnblGpio106inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO106INTToINT13020` writer - Enable GPIO106 Interrupt To INT#130_20"]
pub type EnblGpio106inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO106INTToSIO` reader - Enable GPIO106 Interrupt To SIO"]
pub type EnblGpio106inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO106INTToSIO` writer - Enable GPIO106 Interrupt To SIO"]
pub type EnblGpio106inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO106 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio106inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio106inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio106inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO106INTTargetRstTolerance` reader - GPIO106 Interrupt Target Reset Tolerance"]
pub type Gpio106inttargetRstToleranceR = crate::BitReader<Gpio106inttargetRstTolerance>;
impl Gpio106inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio106inttargetRstTolerance {
        match self.bits {
            false => Gpio106inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio106inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio106inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio106inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO106INTTargetRstTolerance` writer - GPIO106 Interrupt Target Reset Tolerance"]
pub type Gpio106inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio106inttargetRstTolerance>;
impl<'a, REG> Gpio106inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio106inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio106inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO106INTTargetWrProt` reader - GPIO106 Interrupt Target Write Protection"]
pub type Gpio106inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO106INTTargetWrProt` writer - GPIO106 Interrupt Target Write Protection"]
pub type Gpio106inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO107INTToINT13018` reader - Enable GPIO107 Interrupt To INT#130_18"]
pub type EnblGpio107inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO107INTToINT13018` writer - Enable GPIO107 Interrupt To INT#130_18"]
pub type EnblGpio107inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO107INTToINT13019` reader - Enable GPIO107 Interrupt To INT#130_19"]
pub type EnblGpio107inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO107INTToINT13019` writer - Enable GPIO107 Interrupt To INT#130_19"]
pub type EnblGpio107inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO107INTToINT13020` reader - Enable GPIO107 Interrupt To INT#130_20"]
pub type EnblGpio107inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO107INTToINT13020` writer - Enable GPIO107 Interrupt To INT#130_20"]
pub type EnblGpio107inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO107INTToSIO` reader - Enable GPIO107 Interrupt To SIO"]
pub type EnblGpio107inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO107INTToSIO` writer - Enable GPIO107 Interrupt To SIO"]
pub type EnblGpio107inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO107 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio107inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio107inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio107inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO107INTTargetRstTolerance` reader - GPIO107 Interrupt Target Reset Tolerance"]
pub type Gpio107inttargetRstToleranceR = crate::BitReader<Gpio107inttargetRstTolerance>;
impl Gpio107inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio107inttargetRstTolerance {
        match self.bits {
            false => Gpio107inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio107inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio107inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio107inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO107INTTargetRstTolerance` writer - GPIO107 Interrupt Target Reset Tolerance"]
pub type Gpio107inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio107inttargetRstTolerance>;
impl<'a, REG> Gpio107inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio107inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio107inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO107INTTargetWrProt` reader - GPIO107 Interrupt Target Write Protection"]
pub type Gpio107inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO107INTTargetWrProt` writer - GPIO107 Interrupt Target Write Protection"]
pub type Gpio107inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO104 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio104intto_int13018(&self) -> EnblGpio104inttoInt13018R {
        EnblGpio104inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO104 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio104intto_int13019(&self) -> EnblGpio104inttoInt13019R {
        EnblGpio104inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO104 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio104intto_int13020(&self) -> EnblGpio104inttoInt13020R {
        EnblGpio104inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO104 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio104intto_sio(&self) -> EnblGpio104inttoSioR {
        EnblGpio104inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO104 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio104inttarget_rst_tolerance(&self) -> Gpio104inttargetRstToleranceR {
        Gpio104inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO104 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio104inttarget_wr_prot(&self) -> Gpio104inttargetWrProtR {
        Gpio104inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO105 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio105intto_int13018(&self) -> EnblGpio105inttoInt13018R {
        EnblGpio105inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO105 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio105intto_int13019(&self) -> EnblGpio105inttoInt13019R {
        EnblGpio105inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO105 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio105intto_int13020(&self) -> EnblGpio105inttoInt13020R {
        EnblGpio105inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO105 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio105intto_sio(&self) -> EnblGpio105inttoSioR {
        EnblGpio105inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO105 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio105inttarget_rst_tolerance(&self) -> Gpio105inttargetRstToleranceR {
        Gpio105inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO105 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio105inttarget_wr_prot(&self) -> Gpio105inttargetWrProtR {
        Gpio105inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO106 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio106intto_int13018(&self) -> EnblGpio106inttoInt13018R {
        EnblGpio106inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO106 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio106intto_int13019(&self) -> EnblGpio106inttoInt13019R {
        EnblGpio106inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO106 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio106intto_int13020(&self) -> EnblGpio106inttoInt13020R {
        EnblGpio106inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO106 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio106intto_sio(&self) -> EnblGpio106inttoSioR {
        EnblGpio106inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO106 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio106inttarget_rst_tolerance(&self) -> Gpio106inttargetRstToleranceR {
        Gpio106inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO106 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio106inttarget_wr_prot(&self) -> Gpio106inttargetWrProtR {
        Gpio106inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO107 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio107intto_int13018(&self) -> EnblGpio107inttoInt13018R {
        EnblGpio107inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO107 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio107intto_int13019(&self) -> EnblGpio107inttoInt13019R {
        EnblGpio107inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO107 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio107intto_int13020(&self) -> EnblGpio107inttoInt13020R {
        EnblGpio107inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO107 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio107intto_sio(&self) -> EnblGpio107inttoSioR {
        EnblGpio107inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO107 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio107inttarget_rst_tolerance(&self) -> Gpio107inttargetRstToleranceR {
        Gpio107inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO107 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio107inttarget_wr_prot(&self) -> Gpio107inttargetWrProtR {
        Gpio107inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO104 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio104intto_int13018(&mut self) -> EnblGpio104inttoInt13018W<Gpioa78Spec> {
        EnblGpio104inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO104 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio104intto_int13019(&mut self) -> EnblGpio104inttoInt13019W<Gpioa78Spec> {
        EnblGpio104inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO104 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio104intto_int13020(&mut self) -> EnblGpio104inttoInt13020W<Gpioa78Spec> {
        EnblGpio104inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO104 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio104intto_sio(&mut self) -> EnblGpio104inttoSioW<Gpioa78Spec> {
        EnblGpio104inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa78Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa78Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO104 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio104inttarget_rst_tolerance(&mut self) -> Gpio104inttargetRstToleranceW<Gpioa78Spec> {
        Gpio104inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO104 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio104inttarget_wr_prot(&mut self) -> Gpio104inttargetWrProtW<Gpioa78Spec> {
        Gpio104inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO105 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio105intto_int13018(&mut self) -> EnblGpio105inttoInt13018W<Gpioa78Spec> {
        EnblGpio105inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO105 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio105intto_int13019(&mut self) -> EnblGpio105inttoInt13019W<Gpioa78Spec> {
        EnblGpio105inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO105 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio105intto_int13020(&mut self) -> EnblGpio105inttoInt13020W<Gpioa78Spec> {
        EnblGpio105inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO105 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio105intto_sio(&mut self) -> EnblGpio105inttoSioW<Gpioa78Spec> {
        EnblGpio105inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa78Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa78Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO105 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio105inttarget_rst_tolerance(&mut self) -> Gpio105inttargetRstToleranceW<Gpioa78Spec> {
        Gpio105inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO105 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio105inttarget_wr_prot(&mut self) -> Gpio105inttargetWrProtW<Gpioa78Spec> {
        Gpio105inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO106 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio106intto_int13018(&mut self) -> EnblGpio106inttoInt13018W<Gpioa78Spec> {
        EnblGpio106inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO106 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio106intto_int13019(&mut self) -> EnblGpio106inttoInt13019W<Gpioa78Spec> {
        EnblGpio106inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO106 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio106intto_int13020(&mut self) -> EnblGpio106inttoInt13020W<Gpioa78Spec> {
        EnblGpio106inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO106 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio106intto_sio(&mut self) -> EnblGpio106inttoSioW<Gpioa78Spec> {
        EnblGpio106inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa78Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa78Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO106 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio106inttarget_rst_tolerance(&mut self) -> Gpio106inttargetRstToleranceW<Gpioa78Spec> {
        Gpio106inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO106 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio106inttarget_wr_prot(&mut self) -> Gpio106inttargetWrProtW<Gpioa78Spec> {
        Gpio106inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO107 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio107intto_int13018(&mut self) -> EnblGpio107inttoInt13018W<Gpioa78Spec> {
        EnblGpio107inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO107 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio107intto_int13019(&mut self) -> EnblGpio107inttoInt13019W<Gpioa78Spec> {
        EnblGpio107inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO107 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio107intto_int13020(&mut self) -> EnblGpio107inttoInt13020W<Gpioa78Spec> {
        EnblGpio107inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO107 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio107intto_sio(&mut self) -> EnblGpio107inttoSioW<Gpioa78Spec> {
        EnblGpio107inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa78Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO107 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio107inttarget_rst_tolerance(&mut self) -> Gpio107inttargetRstToleranceW<Gpioa78Spec> {
        Gpio107inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO107 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio107inttarget_wr_prot(&mut self) -> Gpio107inttargetWrProtW<Gpioa78Spec> {
        Gpio107inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa78::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa78::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa78Spec;
impl crate::RegisterSpec for Gpioa78Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa78::R`](R) reader structure"]
impl crate::Readable for Gpioa78Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa78::W`](W) writer structure"]
impl crate::Writable for Gpioa78Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA78 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa78Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
