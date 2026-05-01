#[doc = "Register `GPIOA90` reader"]
pub type R = crate::R<Gpioa90Spec>;
#[doc = "Register `GPIOA90` writer"]
pub type W = crate::W<Gpioa90Spec>;
#[doc = "Field `EnblGPIO128INTToINT13018` reader - Enable GPIO128 Interrupt To INT#130_18"]
pub type EnblGpio128inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO128INTToINT13018` writer - Enable GPIO128 Interrupt To INT#130_18"]
pub type EnblGpio128inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO128INTToINT13019` reader - Enable GPIO128 Interrupt To INT#130_19"]
pub type EnblGpio128inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO128INTToINT13019` writer - Enable GPIO128 Interrupt To INT#130_19"]
pub type EnblGpio128inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO128INTToINT13020` reader - Enable GPIO128 Interrupt To INT#130_20"]
pub type EnblGpio128inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO128INTToINT13020` writer - Enable GPIO128 Interrupt To INT#130_20"]
pub type EnblGpio128inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO128INTToSIO` reader - Enable GPIO128 Interrupt To SIO"]
pub type EnblGpio128inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO128INTToSIO` writer - Enable GPIO128 Interrupt To SIO"]
pub type EnblGpio128inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO128 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio128inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio128inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio128inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO128INTTargetRstTolerance` reader - GPIO128 Interrupt Target Reset Tolerance"]
pub type Gpio128inttargetRstToleranceR = crate::BitReader<Gpio128inttargetRstTolerance>;
impl Gpio128inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio128inttargetRstTolerance {
        match self.bits {
            false => Gpio128inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio128inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio128inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio128inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO128INTTargetRstTolerance` writer - GPIO128 Interrupt Target Reset Tolerance"]
pub type Gpio128inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio128inttargetRstTolerance>;
impl<'a, REG> Gpio128inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio128inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio128inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO128INTTargetWrProt` reader - GPIO128 Interrupt Target Write Protection"]
pub type Gpio128inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO128INTTargetWrProt` writer - GPIO128 Interrupt Target Write Protection"]
pub type Gpio128inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO129INTToINT13018` reader - Enable GPIO129 Interrupt To INT#130_18"]
pub type EnblGpio129inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO129INTToINT13018` writer - Enable GPIO129 Interrupt To INT#130_18"]
pub type EnblGpio129inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO129INTToINT13019` reader - Enable GPIO129 Interrupt To INT#130_19"]
pub type EnblGpio129inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO129INTToINT13019` writer - Enable GPIO129 Interrupt To INT#130_19"]
pub type EnblGpio129inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO129INTToINT13020` reader - Enable GPIO129 Interrupt To INT#130_20"]
pub type EnblGpio129inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO129INTToINT13020` writer - Enable GPIO129 Interrupt To INT#130_20"]
pub type EnblGpio129inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO129INTToSIO` reader - Enable GPIO129 Interrupt To SIO"]
pub type EnblGpio129inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO129INTToSIO` writer - Enable GPIO129 Interrupt To SIO"]
pub type EnblGpio129inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO129 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio129inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio129inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio129inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO129INTTargetRstTolerance` reader - GPIO129 Interrupt Target Reset Tolerance"]
pub type Gpio129inttargetRstToleranceR = crate::BitReader<Gpio129inttargetRstTolerance>;
impl Gpio129inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio129inttargetRstTolerance {
        match self.bits {
            false => Gpio129inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio129inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio129inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio129inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO129INTTargetRstTolerance` writer - GPIO129 Interrupt Target Reset Tolerance"]
pub type Gpio129inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio129inttargetRstTolerance>;
impl<'a, REG> Gpio129inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio129inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio129inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO129INTTargetWrProt` reader - GPIO129 Interrupt Target Write Protection"]
pub type Gpio129inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO129INTTargetWrProt` writer - GPIO129 Interrupt Target Write Protection"]
pub type Gpio129inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO130INTToINT13018` reader - Enable GPIO130 Interrupt To INT#130_18"]
pub type EnblGpio130inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO130INTToINT13018` writer - Enable GPIO130 Interrupt To INT#130_18"]
pub type EnblGpio130inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO130INTToINT13019` reader - Enable GPIO130 Interrupt To INT#130_19"]
pub type EnblGpio130inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO130INTToINT13019` writer - Enable GPIO130 Interrupt To INT#130_19"]
pub type EnblGpio130inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO130INTToINT13020` reader - Enable GPIO130 Interrupt To INT#130_20"]
pub type EnblGpio130inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO130INTToINT13020` writer - Enable GPIO130 Interrupt To INT#130_20"]
pub type EnblGpio130inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO130INTToSIO` reader - Enable GPIO130 Interrupt To SIO"]
pub type EnblGpio130inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO130INTToSIO` writer - Enable GPIO130 Interrupt To SIO"]
pub type EnblGpio130inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO130 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio130inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio130inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio130inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO130INTTargetRstTolerance` reader - GPIO130 Interrupt Target Reset Tolerance"]
pub type Gpio130inttargetRstToleranceR = crate::BitReader<Gpio130inttargetRstTolerance>;
impl Gpio130inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio130inttargetRstTolerance {
        match self.bits {
            false => Gpio130inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio130inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio130inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio130inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO130INTTargetRstTolerance` writer - GPIO130 Interrupt Target Reset Tolerance"]
pub type Gpio130inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio130inttargetRstTolerance>;
impl<'a, REG> Gpio130inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio130inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio130inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO130INTTargetWrProt` reader - GPIO130 Interrupt Target Write Protection"]
pub type Gpio130inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO130INTTargetWrProt` writer - GPIO130 Interrupt Target Write Protection"]
pub type Gpio130inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO131INTToINT13018` reader - Enable GPIO131 Interrupt To INT#130_18"]
pub type EnblGpio131inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO131INTToINT13018` writer - Enable GPIO131 Interrupt To INT#130_18"]
pub type EnblGpio131inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO131INTToINT13019` reader - Enable GPIO131 Interrupt To INT#130_19"]
pub type EnblGpio131inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO131INTToINT13019` writer - Enable GPIO131 Interrupt To INT#130_19"]
pub type EnblGpio131inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO131INTToINT13020` reader - Enable GPIO131 Interrupt To INT#130_20"]
pub type EnblGpio131inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO131INTToINT13020` writer - Enable GPIO131 Interrupt To INT#130_20"]
pub type EnblGpio131inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO131INTToSIO` reader - Enable GPIO131 Interrupt To SIO"]
pub type EnblGpio131inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO131INTToSIO` writer - Enable GPIO131 Interrupt To SIO"]
pub type EnblGpio131inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO131 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio131inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio131inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio131inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO131INTTargetRstTolerance` reader - GPIO131 Interrupt Target Reset Tolerance"]
pub type Gpio131inttargetRstToleranceR = crate::BitReader<Gpio131inttargetRstTolerance>;
impl Gpio131inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio131inttargetRstTolerance {
        match self.bits {
            false => Gpio131inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio131inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio131inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio131inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO131INTTargetRstTolerance` writer - GPIO131 Interrupt Target Reset Tolerance"]
pub type Gpio131inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio131inttargetRstTolerance>;
impl<'a, REG> Gpio131inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio131inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio131inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO131INTTargetWrProt` reader - GPIO131 Interrupt Target Write Protection"]
pub type Gpio131inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO131INTTargetWrProt` writer - GPIO131 Interrupt Target Write Protection"]
pub type Gpio131inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO128 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio128intto_int13018(&self) -> EnblGpio128inttoInt13018R {
        EnblGpio128inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO128 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio128intto_int13019(&self) -> EnblGpio128inttoInt13019R {
        EnblGpio128inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO128 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio128intto_int13020(&self) -> EnblGpio128inttoInt13020R {
        EnblGpio128inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO128 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio128intto_sio(&self) -> EnblGpio128inttoSioR {
        EnblGpio128inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO128 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio128inttarget_rst_tolerance(&self) -> Gpio128inttargetRstToleranceR {
        Gpio128inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO128 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio128inttarget_wr_prot(&self) -> Gpio128inttargetWrProtR {
        Gpio128inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO129 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio129intto_int13018(&self) -> EnblGpio129inttoInt13018R {
        EnblGpio129inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO129 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio129intto_int13019(&self) -> EnblGpio129inttoInt13019R {
        EnblGpio129inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO129 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio129intto_int13020(&self) -> EnblGpio129inttoInt13020R {
        EnblGpio129inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO129 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio129intto_sio(&self) -> EnblGpio129inttoSioR {
        EnblGpio129inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO129 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio129inttarget_rst_tolerance(&self) -> Gpio129inttargetRstToleranceR {
        Gpio129inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO129 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio129inttarget_wr_prot(&self) -> Gpio129inttargetWrProtR {
        Gpio129inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO130 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio130intto_int13018(&self) -> EnblGpio130inttoInt13018R {
        EnblGpio130inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO130 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio130intto_int13019(&self) -> EnblGpio130inttoInt13019R {
        EnblGpio130inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO130 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio130intto_int13020(&self) -> EnblGpio130inttoInt13020R {
        EnblGpio130inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO130 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio130intto_sio(&self) -> EnblGpio130inttoSioR {
        EnblGpio130inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO130 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio130inttarget_rst_tolerance(&self) -> Gpio130inttargetRstToleranceR {
        Gpio130inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO130 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio130inttarget_wr_prot(&self) -> Gpio130inttargetWrProtR {
        Gpio130inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO131 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio131intto_int13018(&self) -> EnblGpio131inttoInt13018R {
        EnblGpio131inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO131 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio131intto_int13019(&self) -> EnblGpio131inttoInt13019R {
        EnblGpio131inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO131 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio131intto_int13020(&self) -> EnblGpio131inttoInt13020R {
        EnblGpio131inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO131 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio131intto_sio(&self) -> EnblGpio131inttoSioR {
        EnblGpio131inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO131 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio131inttarget_rst_tolerance(&self) -> Gpio131inttargetRstToleranceR {
        Gpio131inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO131 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio131inttarget_wr_prot(&self) -> Gpio131inttargetWrProtR {
        Gpio131inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO128 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio128intto_int13018(&mut self) -> EnblGpio128inttoInt13018W<Gpioa90Spec> {
        EnblGpio128inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO128 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio128intto_int13019(&mut self) -> EnblGpio128inttoInt13019W<Gpioa90Spec> {
        EnblGpio128inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO128 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio128intto_int13020(&mut self) -> EnblGpio128inttoInt13020W<Gpioa90Spec> {
        EnblGpio128inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO128 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio128intto_sio(&mut self) -> EnblGpio128inttoSioW<Gpioa90Spec> {
        EnblGpio128inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa90Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa90Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO128 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio128inttarget_rst_tolerance(&mut self) -> Gpio128inttargetRstToleranceW<Gpioa90Spec> {
        Gpio128inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO128 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio128inttarget_wr_prot(&mut self) -> Gpio128inttargetWrProtW<Gpioa90Spec> {
        Gpio128inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO129 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio129intto_int13018(&mut self) -> EnblGpio129inttoInt13018W<Gpioa90Spec> {
        EnblGpio129inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO129 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio129intto_int13019(&mut self) -> EnblGpio129inttoInt13019W<Gpioa90Spec> {
        EnblGpio129inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO129 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio129intto_int13020(&mut self) -> EnblGpio129inttoInt13020W<Gpioa90Spec> {
        EnblGpio129inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO129 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio129intto_sio(&mut self) -> EnblGpio129inttoSioW<Gpioa90Spec> {
        EnblGpio129inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa90Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa90Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO129 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio129inttarget_rst_tolerance(&mut self) -> Gpio129inttargetRstToleranceW<Gpioa90Spec> {
        Gpio129inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO129 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio129inttarget_wr_prot(&mut self) -> Gpio129inttargetWrProtW<Gpioa90Spec> {
        Gpio129inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO130 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio130intto_int13018(&mut self) -> EnblGpio130inttoInt13018W<Gpioa90Spec> {
        EnblGpio130inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO130 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio130intto_int13019(&mut self) -> EnblGpio130inttoInt13019W<Gpioa90Spec> {
        EnblGpio130inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO130 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio130intto_int13020(&mut self) -> EnblGpio130inttoInt13020W<Gpioa90Spec> {
        EnblGpio130inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO130 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio130intto_sio(&mut self) -> EnblGpio130inttoSioW<Gpioa90Spec> {
        EnblGpio130inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa90Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa90Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO130 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio130inttarget_rst_tolerance(&mut self) -> Gpio130inttargetRstToleranceW<Gpioa90Spec> {
        Gpio130inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO130 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio130inttarget_wr_prot(&mut self) -> Gpio130inttargetWrProtW<Gpioa90Spec> {
        Gpio130inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO131 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio131intto_int13018(&mut self) -> EnblGpio131inttoInt13018W<Gpioa90Spec> {
        EnblGpio131inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO131 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio131intto_int13019(&mut self) -> EnblGpio131inttoInt13019W<Gpioa90Spec> {
        EnblGpio131inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO131 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio131intto_int13020(&mut self) -> EnblGpio131inttoInt13020W<Gpioa90Spec> {
        EnblGpio131inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO131 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio131intto_sio(&mut self) -> EnblGpio131inttoSioW<Gpioa90Spec> {
        EnblGpio131inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa90Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO131 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio131inttarget_rst_tolerance(&mut self) -> Gpio131inttargetRstToleranceW<Gpioa90Spec> {
        Gpio131inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO131 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio131inttarget_wr_prot(&mut self) -> Gpio131inttargetWrProtW<Gpioa90Spec> {
        Gpio131inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa90::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa90::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa90Spec;
impl crate::RegisterSpec for Gpioa90Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa90::R`](R) reader structure"]
impl crate::Readable for Gpioa90Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa90::W`](W) writer structure"]
impl crate::Writable for Gpioa90Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA90 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa90Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
