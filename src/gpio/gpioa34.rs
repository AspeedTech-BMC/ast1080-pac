#[doc = "Register `GPIOA34` reader"]
pub type R = crate::R<Gpioa34Spec>;
#[doc = "Register `GPIOA34` writer"]
pub type W = crate::W<Gpioa34Spec>;
#[doc = "Field `EnblGPIO036INTToINT13018` reader - Enable GPIO036 Interrupt To INT#130_18"]
pub type EnblGpio036inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO036INTToINT13018` writer - Enable GPIO036 Interrupt To INT#130_18"]
pub type EnblGpio036inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO036INTToINT13019` reader - Enable GPIO036 Interrupt To INT#130_19"]
pub type EnblGpio036inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO036INTToINT13019` writer - Enable GPIO036 Interrupt To INT#130_19"]
pub type EnblGpio036inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO036INTToINT13020` reader - Enable GPIO036 Interrupt To INT#130_20"]
pub type EnblGpio036inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO036INTToINT13020` writer - Enable GPIO036 Interrupt To INT#130_20"]
pub type EnblGpio036inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO036INTToSIO` reader - Enable GPIO036 Interrupt To SIO"]
pub type EnblGpio036inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO036INTToSIO` writer - Enable GPIO036 Interrupt To SIO"]
pub type EnblGpio036inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO036 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio036inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio036inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio036inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO036INTTargetRstTolerance` reader - GPIO036 Interrupt Target Reset Tolerance"]
pub type Gpio036inttargetRstToleranceR = crate::BitReader<Gpio036inttargetRstTolerance>;
impl Gpio036inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio036inttargetRstTolerance {
        match self.bits {
            false => Gpio036inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio036inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio036inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio036inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO036INTTargetRstTolerance` writer - GPIO036 Interrupt Target Reset Tolerance"]
pub type Gpio036inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio036inttargetRstTolerance>;
impl<'a, REG> Gpio036inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio036inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio036inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO036INTTargetWrProt` reader - GPIO036 Interrupt Target Write Protection"]
pub type Gpio036inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO036INTTargetWrProt` writer - GPIO036 Interrupt Target Write Protection"]
pub type Gpio036inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO037INTToINT13018` reader - Enable GPIO037 Interrupt To INT#130_18"]
pub type EnblGpio037inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO037INTToINT13018` writer - Enable GPIO037 Interrupt To INT#130_18"]
pub type EnblGpio037inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO037INTToINT13019` reader - Enable GPIO037 Interrupt To INT#130_19"]
pub type EnblGpio037inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO037INTToINT13019` writer - Enable GPIO037 Interrupt To INT#130_19"]
pub type EnblGpio037inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO037INTToINT13020` reader - Enable GPIO037 Interrupt To INT#130_20"]
pub type EnblGpio037inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO037INTToINT13020` writer - Enable GPIO037 Interrupt To INT#130_20"]
pub type EnblGpio037inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO037INTToSIO` reader - Enable GPIO037 Interrupt To SIO"]
pub type EnblGpio037inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO037INTToSIO` writer - Enable GPIO037 Interrupt To SIO"]
pub type EnblGpio037inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO037 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio037inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio037inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio037inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO037INTTargetRstTolerance` reader - GPIO037 Interrupt Target Reset Tolerance"]
pub type Gpio037inttargetRstToleranceR = crate::BitReader<Gpio037inttargetRstTolerance>;
impl Gpio037inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio037inttargetRstTolerance {
        match self.bits {
            false => Gpio037inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio037inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio037inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio037inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO037INTTargetRstTolerance` writer - GPIO037 Interrupt Target Reset Tolerance"]
pub type Gpio037inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio037inttargetRstTolerance>;
impl<'a, REG> Gpio037inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio037inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio037inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO037INTTargetWrProt` reader - GPIO037 Interrupt Target Write Protection"]
pub type Gpio037inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO037INTTargetWrProt` writer - GPIO037 Interrupt Target Write Protection"]
pub type Gpio037inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO038INTToINT13018` reader - Enable GPIO038 Interrupt To INT#130_18"]
pub type EnblGpio038inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO038INTToINT13018` writer - Enable GPIO038 Interrupt To INT#130_18"]
pub type EnblGpio038inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO038INTToINT13019` reader - Enable GPIO038 Interrupt To INT#130_19"]
pub type EnblGpio038inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO038INTToINT13019` writer - Enable GPIO038 Interrupt To INT#130_19"]
pub type EnblGpio038inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO038INTToINT13020` reader - Enable GPIO038 Interrupt To INT#130_20"]
pub type EnblGpio038inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO038INTToINT13020` writer - Enable GPIO038 Interrupt To INT#130_20"]
pub type EnblGpio038inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO038INTToSIO` reader - Enable GPIO038 Interrupt To SIO"]
pub type EnblGpio038inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO038INTToSIO` writer - Enable GPIO038 Interrupt To SIO"]
pub type EnblGpio038inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO038 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio038inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio038inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio038inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO038INTTargetRstTolerance` reader - GPIO038 Interrupt Target Reset Tolerance"]
pub type Gpio038inttargetRstToleranceR = crate::BitReader<Gpio038inttargetRstTolerance>;
impl Gpio038inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio038inttargetRstTolerance {
        match self.bits {
            false => Gpio038inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio038inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio038inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio038inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO038INTTargetRstTolerance` writer - GPIO038 Interrupt Target Reset Tolerance"]
pub type Gpio038inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio038inttargetRstTolerance>;
impl<'a, REG> Gpio038inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio038inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio038inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO038INTTargetWrProt` reader - GPIO038 Interrupt Target Write Protection"]
pub type Gpio038inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO038INTTargetWrProt` writer - GPIO038 Interrupt Target Write Protection"]
pub type Gpio038inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO039INTToINT13018` reader - Enable GPIO039 Interrupt To INT#130_18"]
pub type EnblGpio039inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO039INTToINT13018` writer - Enable GPIO039 Interrupt To INT#130_18"]
pub type EnblGpio039inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO039INTToINT13019` reader - Enable GPIO039 Interrupt To INT#130_19"]
pub type EnblGpio039inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO039INTToINT13019` writer - Enable GPIO039 Interrupt To INT#130_19"]
pub type EnblGpio039inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO039INTToINT13020` reader - Enable GPIO039 Interrupt To INT#130_20"]
pub type EnblGpio039inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO039INTToINT13020` writer - Enable GPIO039 Interrupt To INT#130_20"]
pub type EnblGpio039inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO039INTToSIO` reader - Enable GPIO039 Interrupt To SIO"]
pub type EnblGpio039inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO039INTToSIO` writer - Enable GPIO039 Interrupt To SIO"]
pub type EnblGpio039inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO039 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio039inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio039inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio039inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO039INTTargetRstTolerance` reader - GPIO039 Interrupt Target Reset Tolerance"]
pub type Gpio039inttargetRstToleranceR = crate::BitReader<Gpio039inttargetRstTolerance>;
impl Gpio039inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio039inttargetRstTolerance {
        match self.bits {
            false => Gpio039inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio039inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio039inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio039inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO039INTTargetRstTolerance` writer - GPIO039 Interrupt Target Reset Tolerance"]
pub type Gpio039inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio039inttargetRstTolerance>;
impl<'a, REG> Gpio039inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio039inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio039inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO039INTTargetWrProt` reader - GPIO039 Interrupt Target Write Protection"]
pub type Gpio039inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO039INTTargetWrProt` writer - GPIO039 Interrupt Target Write Protection"]
pub type Gpio039inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO036 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio036intto_int13018(&self) -> EnblGpio036inttoInt13018R {
        EnblGpio036inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO036 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio036intto_int13019(&self) -> EnblGpio036inttoInt13019R {
        EnblGpio036inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO036 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio036intto_int13020(&self) -> EnblGpio036inttoInt13020R {
        EnblGpio036inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO036 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio036intto_sio(&self) -> EnblGpio036inttoSioR {
        EnblGpio036inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO036 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio036inttarget_rst_tolerance(&self) -> Gpio036inttargetRstToleranceR {
        Gpio036inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO036 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio036inttarget_wr_prot(&self) -> Gpio036inttargetWrProtR {
        Gpio036inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO037 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio037intto_int13018(&self) -> EnblGpio037inttoInt13018R {
        EnblGpio037inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO037 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio037intto_int13019(&self) -> EnblGpio037inttoInt13019R {
        EnblGpio037inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO037 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio037intto_int13020(&self) -> EnblGpio037inttoInt13020R {
        EnblGpio037inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO037 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio037intto_sio(&self) -> EnblGpio037inttoSioR {
        EnblGpio037inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO037 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio037inttarget_rst_tolerance(&self) -> Gpio037inttargetRstToleranceR {
        Gpio037inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO037 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio037inttarget_wr_prot(&self) -> Gpio037inttargetWrProtR {
        Gpio037inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO038 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio038intto_int13018(&self) -> EnblGpio038inttoInt13018R {
        EnblGpio038inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO038 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio038intto_int13019(&self) -> EnblGpio038inttoInt13019R {
        EnblGpio038inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO038 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio038intto_int13020(&self) -> EnblGpio038inttoInt13020R {
        EnblGpio038inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO038 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio038intto_sio(&self) -> EnblGpio038inttoSioR {
        EnblGpio038inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO038 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio038inttarget_rst_tolerance(&self) -> Gpio038inttargetRstToleranceR {
        Gpio038inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO038 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio038inttarget_wr_prot(&self) -> Gpio038inttargetWrProtR {
        Gpio038inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO039 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio039intto_int13018(&self) -> EnblGpio039inttoInt13018R {
        EnblGpio039inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO039 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio039intto_int13019(&self) -> EnblGpio039inttoInt13019R {
        EnblGpio039inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO039 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio039intto_int13020(&self) -> EnblGpio039inttoInt13020R {
        EnblGpio039inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO039 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio039intto_sio(&self) -> EnblGpio039inttoSioR {
        EnblGpio039inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO039 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio039inttarget_rst_tolerance(&self) -> Gpio039inttargetRstToleranceR {
        Gpio039inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO039 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio039inttarget_wr_prot(&self) -> Gpio039inttargetWrProtR {
        Gpio039inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO036 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio036intto_int13018(&mut self) -> EnblGpio036inttoInt13018W<Gpioa34Spec> {
        EnblGpio036inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO036 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio036intto_int13019(&mut self) -> EnblGpio036inttoInt13019W<Gpioa34Spec> {
        EnblGpio036inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO036 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio036intto_int13020(&mut self) -> EnblGpio036inttoInt13020W<Gpioa34Spec> {
        EnblGpio036inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO036 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio036intto_sio(&mut self) -> EnblGpio036inttoSioW<Gpioa34Spec> {
        EnblGpio036inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa34Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa34Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO036 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio036inttarget_rst_tolerance(&mut self) -> Gpio036inttargetRstToleranceW<Gpioa34Spec> {
        Gpio036inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO036 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio036inttarget_wr_prot(&mut self) -> Gpio036inttargetWrProtW<Gpioa34Spec> {
        Gpio036inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO037 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio037intto_int13018(&mut self) -> EnblGpio037inttoInt13018W<Gpioa34Spec> {
        EnblGpio037inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO037 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio037intto_int13019(&mut self) -> EnblGpio037inttoInt13019W<Gpioa34Spec> {
        EnblGpio037inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO037 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio037intto_int13020(&mut self) -> EnblGpio037inttoInt13020W<Gpioa34Spec> {
        EnblGpio037inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO037 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio037intto_sio(&mut self) -> EnblGpio037inttoSioW<Gpioa34Spec> {
        EnblGpio037inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa34Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa34Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO037 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio037inttarget_rst_tolerance(&mut self) -> Gpio037inttargetRstToleranceW<Gpioa34Spec> {
        Gpio037inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO037 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio037inttarget_wr_prot(&mut self) -> Gpio037inttargetWrProtW<Gpioa34Spec> {
        Gpio037inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO038 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio038intto_int13018(&mut self) -> EnblGpio038inttoInt13018W<Gpioa34Spec> {
        EnblGpio038inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO038 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio038intto_int13019(&mut self) -> EnblGpio038inttoInt13019W<Gpioa34Spec> {
        EnblGpio038inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO038 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio038intto_int13020(&mut self) -> EnblGpio038inttoInt13020W<Gpioa34Spec> {
        EnblGpio038inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO038 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio038intto_sio(&mut self) -> EnblGpio038inttoSioW<Gpioa34Spec> {
        EnblGpio038inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa34Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa34Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO038 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio038inttarget_rst_tolerance(&mut self) -> Gpio038inttargetRstToleranceW<Gpioa34Spec> {
        Gpio038inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO038 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio038inttarget_wr_prot(&mut self) -> Gpio038inttargetWrProtW<Gpioa34Spec> {
        Gpio038inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO039 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio039intto_int13018(&mut self) -> EnblGpio039inttoInt13018W<Gpioa34Spec> {
        EnblGpio039inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO039 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio039intto_int13019(&mut self) -> EnblGpio039inttoInt13019W<Gpioa34Spec> {
        EnblGpio039inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO039 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio039intto_int13020(&mut self) -> EnblGpio039inttoInt13020W<Gpioa34Spec> {
        EnblGpio039inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO039 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio039intto_sio(&mut self) -> EnblGpio039inttoSioW<Gpioa34Spec> {
        EnblGpio039inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa34Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO039 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio039inttarget_rst_tolerance(&mut self) -> Gpio039inttargetRstToleranceW<Gpioa34Spec> {
        Gpio039inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO039 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio039inttarget_wr_prot(&mut self) -> Gpio039inttargetWrProtW<Gpioa34Spec> {
        Gpio039inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa34Spec;
impl crate::RegisterSpec for Gpioa34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa34::R`](R) reader structure"]
impl crate::Readable for Gpioa34Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa34::W`](W) writer structure"]
impl crate::Writable for Gpioa34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA34 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa34Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
