#[doc = "Register `GPIOA28` reader"]
pub type R = crate::R<Gpioa28Spec>;
#[doc = "Register `GPIOA28` writer"]
pub type W = crate::W<Gpioa28Spec>;
#[doc = "Field `EnblGPIO024INTToINT13018` reader - Enable GPIO024 Interrupt To INT#130_18"]
pub type EnblGpio024inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO024INTToINT13018` writer - Enable GPIO024 Interrupt To INT#130_18"]
pub type EnblGpio024inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO024INTToINT13019` reader - Enable GPIO024 Interrupt To INT#130_19"]
pub type EnblGpio024inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO024INTToINT13019` writer - Enable GPIO024 Interrupt To INT#130_19"]
pub type EnblGpio024inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO024INTToINT13020` reader - Enable GPIO024 Interrupt To INT#130_20"]
pub type EnblGpio024inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO024INTToINT13020` writer - Enable GPIO024 Interrupt To INT#130_20"]
pub type EnblGpio024inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO024INTToSIO` reader - Enable GPIO024 Interrupt To SIO"]
pub type EnblGpio024inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO024INTToSIO` writer - Enable GPIO024 Interrupt To SIO"]
pub type EnblGpio024inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO024 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio024inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio024inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio024inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO024INTTargetRstTolerance` reader - GPIO024 Interrupt Target Reset Tolerance"]
pub type Gpio024inttargetRstToleranceR = crate::BitReader<Gpio024inttargetRstTolerance>;
impl Gpio024inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio024inttargetRstTolerance {
        match self.bits {
            false => Gpio024inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio024inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio024inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio024inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO024INTTargetRstTolerance` writer - GPIO024 Interrupt Target Reset Tolerance"]
pub type Gpio024inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio024inttargetRstTolerance>;
impl<'a, REG> Gpio024inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio024inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio024inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO024INTTargetWrProt` reader - GPIO024 Interrupt Target Write Protection"]
pub type Gpio024inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO024INTTargetWrProt` writer - GPIO024 Interrupt Target Write Protection"]
pub type Gpio024inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO025INTToINT13018` reader - Enable GPIO025 Interrupt To INT#130_18"]
pub type EnblGpio025inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO025INTToINT13018` writer - Enable GPIO025 Interrupt To INT#130_18"]
pub type EnblGpio025inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO025INTToINT13019` reader - Enable GPIO025 Interrupt To INT#130_19"]
pub type EnblGpio025inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO025INTToINT13019` writer - Enable GPIO025 Interrupt To INT#130_19"]
pub type EnblGpio025inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO025INTToINT13020` reader - Enable GPIO025 Interrupt To INT#130_20"]
pub type EnblGpio025inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO025INTToINT13020` writer - Enable GPIO025 Interrupt To INT#130_20"]
pub type EnblGpio025inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO025INTToSIO` reader - Enable GPIO025 Interrupt To SIO"]
pub type EnblGpio025inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO025INTToSIO` writer - Enable GPIO025 Interrupt To SIO"]
pub type EnblGpio025inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO025 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio025inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio025inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio025inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO025INTTargetRstTolerance` reader - GPIO025 Interrupt Target Reset Tolerance"]
pub type Gpio025inttargetRstToleranceR = crate::BitReader<Gpio025inttargetRstTolerance>;
impl Gpio025inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio025inttargetRstTolerance {
        match self.bits {
            false => Gpio025inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio025inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio025inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio025inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO025INTTargetRstTolerance` writer - GPIO025 Interrupt Target Reset Tolerance"]
pub type Gpio025inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio025inttargetRstTolerance>;
impl<'a, REG> Gpio025inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio025inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio025inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO025INTTargetWrProt` reader - GPIO025 Interrupt Target Write Protection"]
pub type Gpio025inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO025INTTargetWrProt` writer - GPIO025 Interrupt Target Write Protection"]
pub type Gpio025inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO026INTToINT13018` reader - Enable GPIO026 Interrupt To INT#130_18"]
pub type EnblGpio026inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO026INTToINT13018` writer - Enable GPIO026 Interrupt To INT#130_18"]
pub type EnblGpio026inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO026INTToINT13019` reader - Enable GPIO026 Interrupt To INT#130_19"]
pub type EnblGpio026inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO026INTToINT13019` writer - Enable GPIO026 Interrupt To INT#130_19"]
pub type EnblGpio026inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO026INTToINT13020` reader - Enable GPIO026 Interrupt To INT#130_20"]
pub type EnblGpio026inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO026INTToINT13020` writer - Enable GPIO026 Interrupt To INT#130_20"]
pub type EnblGpio026inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO026INTToSIO` reader - Enable GPIO026 Interrupt To SIO"]
pub type EnblGpio026inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO026INTToSIO` writer - Enable GPIO026 Interrupt To SIO"]
pub type EnblGpio026inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO026 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio026inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio026inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio026inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO026INTTargetRstTolerance` reader - GPIO026 Interrupt Target Reset Tolerance"]
pub type Gpio026inttargetRstToleranceR = crate::BitReader<Gpio026inttargetRstTolerance>;
impl Gpio026inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio026inttargetRstTolerance {
        match self.bits {
            false => Gpio026inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio026inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio026inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio026inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO026INTTargetRstTolerance` writer - GPIO026 Interrupt Target Reset Tolerance"]
pub type Gpio026inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio026inttargetRstTolerance>;
impl<'a, REG> Gpio026inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio026inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio026inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO026INTTargetWrProt` reader - GPIO026 Interrupt Target Write Protection"]
pub type Gpio026inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO026INTTargetWrProt` writer - GPIO026 Interrupt Target Write Protection"]
pub type Gpio026inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO027INTToINT13018` reader - Enable GPIO027 Interrupt To INT#130_18"]
pub type EnblGpio027inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO027INTToINT13018` writer - Enable GPIO027 Interrupt To INT#130_18"]
pub type EnblGpio027inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO027INTToINT13019` reader - Enable GPIO027 Interrupt To INT#130_19"]
pub type EnblGpio027inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO027INTToINT13019` writer - Enable GPIO027 Interrupt To INT#130_19"]
pub type EnblGpio027inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO027INTToINT13020` reader - Enable GPIO027 Interrupt To INT#130_20"]
pub type EnblGpio027inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO027INTToINT13020` writer - Enable GPIO027 Interrupt To INT#130_20"]
pub type EnblGpio027inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO027INTToSIO` reader - Enable GPIO027 Interrupt To SIO"]
pub type EnblGpio027inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO027INTToSIO` writer - Enable GPIO027 Interrupt To SIO"]
pub type EnblGpio027inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO027 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio027inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio027inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio027inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO027INTTargetRstTolerance` reader - GPIO027 Interrupt Target Reset Tolerance"]
pub type Gpio027inttargetRstToleranceR = crate::BitReader<Gpio027inttargetRstTolerance>;
impl Gpio027inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio027inttargetRstTolerance {
        match self.bits {
            false => Gpio027inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio027inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio027inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio027inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO027INTTargetRstTolerance` writer - GPIO027 Interrupt Target Reset Tolerance"]
pub type Gpio027inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio027inttargetRstTolerance>;
impl<'a, REG> Gpio027inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio027inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio027inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO027INTTargetWrProt` reader - GPIO027 Interrupt Target Write Protection"]
pub type Gpio027inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO027INTTargetWrProt` writer - GPIO027 Interrupt Target Write Protection"]
pub type Gpio027inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO024 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio024intto_int13018(&self) -> EnblGpio024inttoInt13018R {
        EnblGpio024inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO024 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio024intto_int13019(&self) -> EnblGpio024inttoInt13019R {
        EnblGpio024inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO024 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio024intto_int13020(&self) -> EnblGpio024inttoInt13020R {
        EnblGpio024inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO024 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio024intto_sio(&self) -> EnblGpio024inttoSioR {
        EnblGpio024inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO024 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio024inttarget_rst_tolerance(&self) -> Gpio024inttargetRstToleranceR {
        Gpio024inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO024 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio024inttarget_wr_prot(&self) -> Gpio024inttargetWrProtR {
        Gpio024inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO025 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio025intto_int13018(&self) -> EnblGpio025inttoInt13018R {
        EnblGpio025inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO025 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio025intto_int13019(&self) -> EnblGpio025inttoInt13019R {
        EnblGpio025inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO025 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio025intto_int13020(&self) -> EnblGpio025inttoInt13020R {
        EnblGpio025inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO025 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio025intto_sio(&self) -> EnblGpio025inttoSioR {
        EnblGpio025inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO025 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio025inttarget_rst_tolerance(&self) -> Gpio025inttargetRstToleranceR {
        Gpio025inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO025 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio025inttarget_wr_prot(&self) -> Gpio025inttargetWrProtR {
        Gpio025inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO026 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio026intto_int13018(&self) -> EnblGpio026inttoInt13018R {
        EnblGpio026inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO026 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio026intto_int13019(&self) -> EnblGpio026inttoInt13019R {
        EnblGpio026inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO026 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio026intto_int13020(&self) -> EnblGpio026inttoInt13020R {
        EnblGpio026inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO026 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio026intto_sio(&self) -> EnblGpio026inttoSioR {
        EnblGpio026inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO026 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio026inttarget_rst_tolerance(&self) -> Gpio026inttargetRstToleranceR {
        Gpio026inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO026 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio026inttarget_wr_prot(&self) -> Gpio026inttargetWrProtR {
        Gpio026inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO027 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio027intto_int13018(&self) -> EnblGpio027inttoInt13018R {
        EnblGpio027inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO027 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio027intto_int13019(&self) -> EnblGpio027inttoInt13019R {
        EnblGpio027inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO027 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio027intto_int13020(&self) -> EnblGpio027inttoInt13020R {
        EnblGpio027inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO027 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio027intto_sio(&self) -> EnblGpio027inttoSioR {
        EnblGpio027inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO027 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio027inttarget_rst_tolerance(&self) -> Gpio027inttargetRstToleranceR {
        Gpio027inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO027 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio027inttarget_wr_prot(&self) -> Gpio027inttargetWrProtR {
        Gpio027inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO024 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio024intto_int13018(&mut self) -> EnblGpio024inttoInt13018W<Gpioa28Spec> {
        EnblGpio024inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO024 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio024intto_int13019(&mut self) -> EnblGpio024inttoInt13019W<Gpioa28Spec> {
        EnblGpio024inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO024 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio024intto_int13020(&mut self) -> EnblGpio024inttoInt13020W<Gpioa28Spec> {
        EnblGpio024inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO024 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio024intto_sio(&mut self) -> EnblGpio024inttoSioW<Gpioa28Spec> {
        EnblGpio024inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa28Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa28Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO024 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio024inttarget_rst_tolerance(&mut self) -> Gpio024inttargetRstToleranceW<Gpioa28Spec> {
        Gpio024inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO024 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio024inttarget_wr_prot(&mut self) -> Gpio024inttargetWrProtW<Gpioa28Spec> {
        Gpio024inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO025 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio025intto_int13018(&mut self) -> EnblGpio025inttoInt13018W<Gpioa28Spec> {
        EnblGpio025inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO025 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio025intto_int13019(&mut self) -> EnblGpio025inttoInt13019W<Gpioa28Spec> {
        EnblGpio025inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO025 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio025intto_int13020(&mut self) -> EnblGpio025inttoInt13020W<Gpioa28Spec> {
        EnblGpio025inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO025 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio025intto_sio(&mut self) -> EnblGpio025inttoSioW<Gpioa28Spec> {
        EnblGpio025inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa28Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa28Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO025 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio025inttarget_rst_tolerance(&mut self) -> Gpio025inttargetRstToleranceW<Gpioa28Spec> {
        Gpio025inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO025 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio025inttarget_wr_prot(&mut self) -> Gpio025inttargetWrProtW<Gpioa28Spec> {
        Gpio025inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO026 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio026intto_int13018(&mut self) -> EnblGpio026inttoInt13018W<Gpioa28Spec> {
        EnblGpio026inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO026 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio026intto_int13019(&mut self) -> EnblGpio026inttoInt13019W<Gpioa28Spec> {
        EnblGpio026inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO026 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio026intto_int13020(&mut self) -> EnblGpio026inttoInt13020W<Gpioa28Spec> {
        EnblGpio026inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO026 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio026intto_sio(&mut self) -> EnblGpio026inttoSioW<Gpioa28Spec> {
        EnblGpio026inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa28Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa28Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO026 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio026inttarget_rst_tolerance(&mut self) -> Gpio026inttargetRstToleranceW<Gpioa28Spec> {
        Gpio026inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO026 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio026inttarget_wr_prot(&mut self) -> Gpio026inttargetWrProtW<Gpioa28Spec> {
        Gpio026inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO027 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio027intto_int13018(&mut self) -> EnblGpio027inttoInt13018W<Gpioa28Spec> {
        EnblGpio027inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO027 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio027intto_int13019(&mut self) -> EnblGpio027inttoInt13019W<Gpioa28Spec> {
        EnblGpio027inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO027 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio027intto_int13020(&mut self) -> EnblGpio027inttoInt13020W<Gpioa28Spec> {
        EnblGpio027inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO027 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio027intto_sio(&mut self) -> EnblGpio027inttoSioW<Gpioa28Spec> {
        EnblGpio027inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa28Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO027 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio027inttarget_rst_tolerance(&mut self) -> Gpio027inttargetRstToleranceW<Gpioa28Spec> {
        Gpio027inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO027 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio027inttarget_wr_prot(&mut self) -> Gpio027inttargetWrProtW<Gpioa28Spec> {
        Gpio027inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa28Spec;
impl crate::RegisterSpec for Gpioa28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa28::R`](R) reader structure"]
impl crate::Readable for Gpioa28Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa28::W`](W) writer structure"]
impl crate::Writable for Gpioa28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA28 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa28Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
