#[doc = "Register `GPIOA84` reader"]
pub type R = crate::R<Gpioa84Spec>;
#[doc = "Register `GPIOA84` writer"]
pub type W = crate::W<Gpioa84Spec>;
#[doc = "Field `EnblGPIO116INTToINT13018` reader - Enable GPIO116 Interrupt To INT#130_18"]
pub type EnblGpio116inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO116INTToINT13018` writer - Enable GPIO116 Interrupt To INT#130_18"]
pub type EnblGpio116inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO116INTToINT13019` reader - Enable GPIO116 Interrupt To INT#130_19"]
pub type EnblGpio116inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO116INTToINT13019` writer - Enable GPIO116 Interrupt To INT#130_19"]
pub type EnblGpio116inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO116INTToINT13020` reader - Enable GPIO116 Interrupt To INT#130_20"]
pub type EnblGpio116inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO116INTToINT13020` writer - Enable GPIO116 Interrupt To INT#130_20"]
pub type EnblGpio116inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO116INTToSIO` reader - Enable GPIO116 Interrupt To SIO"]
pub type EnblGpio116inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO116INTToSIO` writer - Enable GPIO116 Interrupt To SIO"]
pub type EnblGpio116inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO116 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio116inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio116inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio116inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO116INTTargetRstTolerance` reader - GPIO116 Interrupt Target Reset Tolerance"]
pub type Gpio116inttargetRstToleranceR = crate::BitReader<Gpio116inttargetRstTolerance>;
impl Gpio116inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio116inttargetRstTolerance {
        match self.bits {
            false => Gpio116inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio116inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio116inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio116inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO116INTTargetRstTolerance` writer - GPIO116 Interrupt Target Reset Tolerance"]
pub type Gpio116inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio116inttargetRstTolerance>;
impl<'a, REG> Gpio116inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio116inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio116inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO116INTTargetWrProt` reader - GPIO116 Interrupt Target Write Protection"]
pub type Gpio116inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO116INTTargetWrProt` writer - GPIO116 Interrupt Target Write Protection"]
pub type Gpio116inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO117INTToINT13018` reader - Enable GPIO117 Interrupt To INT#130_18"]
pub type EnblGpio117inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO117INTToINT13018` writer - Enable GPIO117 Interrupt To INT#130_18"]
pub type EnblGpio117inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO117INTToINT13019` reader - Enable GPIO117 Interrupt To INT#130_19"]
pub type EnblGpio117inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO117INTToINT13019` writer - Enable GPIO117 Interrupt To INT#130_19"]
pub type EnblGpio117inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO117INTToINT13020` reader - Enable GPIO117 Interrupt To INT#130_20"]
pub type EnblGpio117inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO117INTToINT13020` writer - Enable GPIO117 Interrupt To INT#130_20"]
pub type EnblGpio117inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO117INTToSIO` reader - Enable GPIO117 Interrupt To SIO"]
pub type EnblGpio117inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO117INTToSIO` writer - Enable GPIO117 Interrupt To SIO"]
pub type EnblGpio117inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO117 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio117inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio117inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio117inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO117INTTargetRstTolerance` reader - GPIO117 Interrupt Target Reset Tolerance"]
pub type Gpio117inttargetRstToleranceR = crate::BitReader<Gpio117inttargetRstTolerance>;
impl Gpio117inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio117inttargetRstTolerance {
        match self.bits {
            false => Gpio117inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio117inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio117inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio117inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO117INTTargetRstTolerance` writer - GPIO117 Interrupt Target Reset Tolerance"]
pub type Gpio117inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio117inttargetRstTolerance>;
impl<'a, REG> Gpio117inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio117inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio117inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO117INTTargetWrProt` reader - GPIO117 Interrupt Target Write Protection"]
pub type Gpio117inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO117INTTargetWrProt` writer - GPIO117 Interrupt Target Write Protection"]
pub type Gpio117inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO118INTToINT13018` reader - Enable GPIO118 Interrupt To INT#130_18"]
pub type EnblGpio118inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO118INTToINT13018` writer - Enable GPIO118 Interrupt To INT#130_18"]
pub type EnblGpio118inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO118INTToINT13019` reader - Enable GPIO118 Interrupt To INT#130_19"]
pub type EnblGpio118inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO118INTToINT13019` writer - Enable GPIO118 Interrupt To INT#130_19"]
pub type EnblGpio118inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO118INTToINT13020` reader - Enable GPIO118 Interrupt To INT#130_20"]
pub type EnblGpio118inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO118INTToINT13020` writer - Enable GPIO118 Interrupt To INT#130_20"]
pub type EnblGpio118inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO118INTToSIO` reader - Enable GPIO118 Interrupt To SIO"]
pub type EnblGpio118inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO118INTToSIO` writer - Enable GPIO118 Interrupt To SIO"]
pub type EnblGpio118inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO118 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio118inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio118inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio118inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO118INTTargetRstTolerance` reader - GPIO118 Interrupt Target Reset Tolerance"]
pub type Gpio118inttargetRstToleranceR = crate::BitReader<Gpio118inttargetRstTolerance>;
impl Gpio118inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio118inttargetRstTolerance {
        match self.bits {
            false => Gpio118inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio118inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio118inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio118inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO118INTTargetRstTolerance` writer - GPIO118 Interrupt Target Reset Tolerance"]
pub type Gpio118inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio118inttargetRstTolerance>;
impl<'a, REG> Gpio118inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio118inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio118inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO118INTTargetWrProt` reader - GPIO118 Interrupt Target Write Protection"]
pub type Gpio118inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO118INTTargetWrProt` writer - GPIO118 Interrupt Target Write Protection"]
pub type Gpio118inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO119INTToINT13018` reader - Enable GPIO119 Interrupt To INT#130_18"]
pub type EnblGpio119inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO119INTToINT13018` writer - Enable GPIO119 Interrupt To INT#130_18"]
pub type EnblGpio119inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO119INTToINT13019` reader - Enable GPIO119 Interrupt To INT#130_19"]
pub type EnblGpio119inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO119INTToINT13019` writer - Enable GPIO119 Interrupt To INT#130_19"]
pub type EnblGpio119inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO119INTToINT13020` reader - Enable GPIO119 Interrupt To INT#130_20"]
pub type EnblGpio119inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO119INTToINT13020` writer - Enable GPIO119 Interrupt To INT#130_20"]
pub type EnblGpio119inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO119INTToSIO` reader - Enable GPIO119 Interrupt To SIO"]
pub type EnblGpio119inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO119INTToSIO` writer - Enable GPIO119 Interrupt To SIO"]
pub type EnblGpio119inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO119 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio119inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio119inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio119inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO119INTTargetRstTolerance` reader - GPIO119 Interrupt Target Reset Tolerance"]
pub type Gpio119inttargetRstToleranceR = crate::BitReader<Gpio119inttargetRstTolerance>;
impl Gpio119inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio119inttargetRstTolerance {
        match self.bits {
            false => Gpio119inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio119inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio119inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio119inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO119INTTargetRstTolerance` writer - GPIO119 Interrupt Target Reset Tolerance"]
pub type Gpio119inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio119inttargetRstTolerance>;
impl<'a, REG> Gpio119inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio119inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio119inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO119INTTargetWrProt` reader - GPIO119 Interrupt Target Write Protection"]
pub type Gpio119inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO119INTTargetWrProt` writer - GPIO119 Interrupt Target Write Protection"]
pub type Gpio119inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO116 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio116intto_int13018(&self) -> EnblGpio116inttoInt13018R {
        EnblGpio116inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO116 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio116intto_int13019(&self) -> EnblGpio116inttoInt13019R {
        EnblGpio116inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO116 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio116intto_int13020(&self) -> EnblGpio116inttoInt13020R {
        EnblGpio116inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO116 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio116intto_sio(&self) -> EnblGpio116inttoSioR {
        EnblGpio116inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO116 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio116inttarget_rst_tolerance(&self) -> Gpio116inttargetRstToleranceR {
        Gpio116inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO116 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio116inttarget_wr_prot(&self) -> Gpio116inttargetWrProtR {
        Gpio116inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO117 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio117intto_int13018(&self) -> EnblGpio117inttoInt13018R {
        EnblGpio117inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO117 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio117intto_int13019(&self) -> EnblGpio117inttoInt13019R {
        EnblGpio117inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO117 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio117intto_int13020(&self) -> EnblGpio117inttoInt13020R {
        EnblGpio117inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO117 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio117intto_sio(&self) -> EnblGpio117inttoSioR {
        EnblGpio117inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO117 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio117inttarget_rst_tolerance(&self) -> Gpio117inttargetRstToleranceR {
        Gpio117inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO117 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio117inttarget_wr_prot(&self) -> Gpio117inttargetWrProtR {
        Gpio117inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO118 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio118intto_int13018(&self) -> EnblGpio118inttoInt13018R {
        EnblGpio118inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO118 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio118intto_int13019(&self) -> EnblGpio118inttoInt13019R {
        EnblGpio118inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO118 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio118intto_int13020(&self) -> EnblGpio118inttoInt13020R {
        EnblGpio118inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO118 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio118intto_sio(&self) -> EnblGpio118inttoSioR {
        EnblGpio118inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO118 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio118inttarget_rst_tolerance(&self) -> Gpio118inttargetRstToleranceR {
        Gpio118inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO118 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio118inttarget_wr_prot(&self) -> Gpio118inttargetWrProtR {
        Gpio118inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO119 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio119intto_int13018(&self) -> EnblGpio119inttoInt13018R {
        EnblGpio119inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO119 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio119intto_int13019(&self) -> EnblGpio119inttoInt13019R {
        EnblGpio119inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO119 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio119intto_int13020(&self) -> EnblGpio119inttoInt13020R {
        EnblGpio119inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO119 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio119intto_sio(&self) -> EnblGpio119inttoSioR {
        EnblGpio119inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO119 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio119inttarget_rst_tolerance(&self) -> Gpio119inttargetRstToleranceR {
        Gpio119inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO119 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio119inttarget_wr_prot(&self) -> Gpio119inttargetWrProtR {
        Gpio119inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO116 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio116intto_int13018(&mut self) -> EnblGpio116inttoInt13018W<Gpioa84Spec> {
        EnblGpio116inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO116 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio116intto_int13019(&mut self) -> EnblGpio116inttoInt13019W<Gpioa84Spec> {
        EnblGpio116inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO116 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio116intto_int13020(&mut self) -> EnblGpio116inttoInt13020W<Gpioa84Spec> {
        EnblGpio116inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO116 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio116intto_sio(&mut self) -> EnblGpio116inttoSioW<Gpioa84Spec> {
        EnblGpio116inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa84Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa84Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO116 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio116inttarget_rst_tolerance(&mut self) -> Gpio116inttargetRstToleranceW<Gpioa84Spec> {
        Gpio116inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO116 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio116inttarget_wr_prot(&mut self) -> Gpio116inttargetWrProtW<Gpioa84Spec> {
        Gpio116inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO117 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio117intto_int13018(&mut self) -> EnblGpio117inttoInt13018W<Gpioa84Spec> {
        EnblGpio117inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO117 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio117intto_int13019(&mut self) -> EnblGpio117inttoInt13019W<Gpioa84Spec> {
        EnblGpio117inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO117 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio117intto_int13020(&mut self) -> EnblGpio117inttoInt13020W<Gpioa84Spec> {
        EnblGpio117inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO117 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio117intto_sio(&mut self) -> EnblGpio117inttoSioW<Gpioa84Spec> {
        EnblGpio117inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa84Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa84Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO117 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio117inttarget_rst_tolerance(&mut self) -> Gpio117inttargetRstToleranceW<Gpioa84Spec> {
        Gpio117inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO117 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio117inttarget_wr_prot(&mut self) -> Gpio117inttargetWrProtW<Gpioa84Spec> {
        Gpio117inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO118 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio118intto_int13018(&mut self) -> EnblGpio118inttoInt13018W<Gpioa84Spec> {
        EnblGpio118inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO118 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio118intto_int13019(&mut self) -> EnblGpio118inttoInt13019W<Gpioa84Spec> {
        EnblGpio118inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO118 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio118intto_int13020(&mut self) -> EnblGpio118inttoInt13020W<Gpioa84Spec> {
        EnblGpio118inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO118 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio118intto_sio(&mut self) -> EnblGpio118inttoSioW<Gpioa84Spec> {
        EnblGpio118inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa84Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa84Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO118 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio118inttarget_rst_tolerance(&mut self) -> Gpio118inttargetRstToleranceW<Gpioa84Spec> {
        Gpio118inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO118 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio118inttarget_wr_prot(&mut self) -> Gpio118inttargetWrProtW<Gpioa84Spec> {
        Gpio118inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO119 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio119intto_int13018(&mut self) -> EnblGpio119inttoInt13018W<Gpioa84Spec> {
        EnblGpio119inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO119 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio119intto_int13019(&mut self) -> EnblGpio119inttoInt13019W<Gpioa84Spec> {
        EnblGpio119inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO119 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio119intto_int13020(&mut self) -> EnblGpio119inttoInt13020W<Gpioa84Spec> {
        EnblGpio119inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO119 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio119intto_sio(&mut self) -> EnblGpio119inttoSioW<Gpioa84Spec> {
        EnblGpio119inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa84Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO119 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio119inttarget_rst_tolerance(&mut self) -> Gpio119inttargetRstToleranceW<Gpioa84Spec> {
        Gpio119inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO119 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio119inttarget_wr_prot(&mut self) -> Gpio119inttargetWrProtW<Gpioa84Spec> {
        Gpio119inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa84::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa84::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa84Spec;
impl crate::RegisterSpec for Gpioa84Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa84::R`](R) reader structure"]
impl crate::Readable for Gpioa84Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa84::W`](W) writer structure"]
impl crate::Writable for Gpioa84Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA84 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa84Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
