#[doc = "Register `GPIOAA4` reader"]
pub type R = crate::R<Gpioaa4Spec>;
#[doc = "Register `GPIOAA4` writer"]
pub type W = crate::W<Gpioaa4Spec>;
#[doc = "Field `EnblGPIO148INTToINT13018` reader - Enable GPIO148 Interrupt To INT#130_18"]
pub type EnblGpio148inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO148INTToINT13018` writer - Enable GPIO148 Interrupt To INT#130_18"]
pub type EnblGpio148inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO148INTToINT13019` reader - Enable GPIO148 Interrupt To INT#130_19"]
pub type EnblGpio148inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO148INTToINT13019` writer - Enable GPIO148 Interrupt To INT#130_19"]
pub type EnblGpio148inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO148INTToINT13020` reader - Enable GPIO148 Interrupt To INT#130_20"]
pub type EnblGpio148inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO148INTToINT13020` writer - Enable GPIO148 Interrupt To INT#130_20"]
pub type EnblGpio148inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO148INTToSIO` reader - Enable GPIO148 Interrupt To SIO"]
pub type EnblGpio148inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO148INTToSIO` writer - Enable GPIO148 Interrupt To SIO"]
pub type EnblGpio148inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO148 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio148inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio148inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio148inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO148INTTargetRstTolerance` reader - GPIO148 Interrupt Target Reset Tolerance"]
pub type Gpio148inttargetRstToleranceR = crate::BitReader<Gpio148inttargetRstTolerance>;
impl Gpio148inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio148inttargetRstTolerance {
        match self.bits {
            false => Gpio148inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio148inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio148inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio148inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO148INTTargetRstTolerance` writer - GPIO148 Interrupt Target Reset Tolerance"]
pub type Gpio148inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio148inttargetRstTolerance>;
impl<'a, REG> Gpio148inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio148inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio148inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO148INTTargetWrProt` reader - GPIO148 Interrupt Target Write Protection"]
pub type Gpio148inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO148INTTargetWrProt` writer - GPIO148 Interrupt Target Write Protection"]
pub type Gpio148inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO149INTToINT13018` reader - Enable GPIO149 Interrupt To INT#130_18"]
pub type EnblGpio149inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO149INTToINT13018` writer - Enable GPIO149 Interrupt To INT#130_18"]
pub type EnblGpio149inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO149INTToINT13019` reader - Enable GPIO149 Interrupt To INT#130_19"]
pub type EnblGpio149inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO149INTToINT13019` writer - Enable GPIO149 Interrupt To INT#130_19"]
pub type EnblGpio149inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO149INTToINT13020` reader - Enable GPIO149 Interrupt To INT#130_20"]
pub type EnblGpio149inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO149INTToINT13020` writer - Enable GPIO149 Interrupt To INT#130_20"]
pub type EnblGpio149inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO149INTToSIO` reader - Enable GPIO149 Interrupt To SIO"]
pub type EnblGpio149inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO149INTToSIO` writer - Enable GPIO149 Interrupt To SIO"]
pub type EnblGpio149inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO149 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio149inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio149inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio149inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO149INTTargetRstTolerance` reader - GPIO149 Interrupt Target Reset Tolerance"]
pub type Gpio149inttargetRstToleranceR = crate::BitReader<Gpio149inttargetRstTolerance>;
impl Gpio149inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio149inttargetRstTolerance {
        match self.bits {
            false => Gpio149inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio149inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio149inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio149inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO149INTTargetRstTolerance` writer - GPIO149 Interrupt Target Reset Tolerance"]
pub type Gpio149inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio149inttargetRstTolerance>;
impl<'a, REG> Gpio149inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio149inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio149inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO149INTTargetWrProt` reader - GPIO149 Interrupt Target Write Protection"]
pub type Gpio149inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO149INTTargetWrProt` writer - GPIO149 Interrupt Target Write Protection"]
pub type Gpio149inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO150INTToINT13018` reader - Enable GPIO150 Interrupt To INT#130_18"]
pub type EnblGpio150inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO150INTToINT13018` writer - Enable GPIO150 Interrupt To INT#130_18"]
pub type EnblGpio150inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO150INTToINT13019` reader - Enable GPIO150 Interrupt To INT#130_19"]
pub type EnblGpio150inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO150INTToINT13019` writer - Enable GPIO150 Interrupt To INT#130_19"]
pub type EnblGpio150inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO150INTToINT13020` reader - Enable GPIO150 Interrupt To INT#130_20"]
pub type EnblGpio150inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO150INTToINT13020` writer - Enable GPIO150 Interrupt To INT#130_20"]
pub type EnblGpio150inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO150INTToSIO` reader - Enable GPIO150 Interrupt To SIO"]
pub type EnblGpio150inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO150INTToSIO` writer - Enable GPIO150 Interrupt To SIO"]
pub type EnblGpio150inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO150 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio150inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio150inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio150inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO150INTTargetRstTolerance` reader - GPIO150 Interrupt Target Reset Tolerance"]
pub type Gpio150inttargetRstToleranceR = crate::BitReader<Gpio150inttargetRstTolerance>;
impl Gpio150inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio150inttargetRstTolerance {
        match self.bits {
            false => Gpio150inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio150inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio150inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio150inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO150INTTargetRstTolerance` writer - GPIO150 Interrupt Target Reset Tolerance"]
pub type Gpio150inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio150inttargetRstTolerance>;
impl<'a, REG> Gpio150inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio150inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio150inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO150INTTargetWrProt` reader - GPIO150 Interrupt Target Write Protection"]
pub type Gpio150inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO150INTTargetWrProt` writer - GPIO150 Interrupt Target Write Protection"]
pub type Gpio150inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO151INTToINT13018` reader - Enable GPIO151 Interrupt To INT#130_18"]
pub type EnblGpio151inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO151INTToINT13018` writer - Enable GPIO151 Interrupt To INT#130_18"]
pub type EnblGpio151inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO151INTToINT13019` reader - Enable GPIO151 Interrupt To INT#130_19"]
pub type EnblGpio151inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO151INTToINT13019` writer - Enable GPIO151 Interrupt To INT#130_19"]
pub type EnblGpio151inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO151INTToINT13020` reader - Enable GPIO151 Interrupt To INT#130_20"]
pub type EnblGpio151inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO151INTToINT13020` writer - Enable GPIO151 Interrupt To INT#130_20"]
pub type EnblGpio151inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO151INTToSIO` reader - Enable GPIO151 Interrupt To SIO"]
pub type EnblGpio151inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO151INTToSIO` writer - Enable GPIO151 Interrupt To SIO"]
pub type EnblGpio151inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO151 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio151inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio151inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio151inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO151INTTargetRstTolerance` reader - GPIO151 Interrupt Target Reset Tolerance"]
pub type Gpio151inttargetRstToleranceR = crate::BitReader<Gpio151inttargetRstTolerance>;
impl Gpio151inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio151inttargetRstTolerance {
        match self.bits {
            false => Gpio151inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio151inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio151inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio151inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO151INTTargetRstTolerance` writer - GPIO151 Interrupt Target Reset Tolerance"]
pub type Gpio151inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio151inttargetRstTolerance>;
impl<'a, REG> Gpio151inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio151inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio151inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO151INTTargetWrProt` reader - GPIO151 Interrupt Target Write Protection"]
pub type Gpio151inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO151INTTargetWrProt` writer - GPIO151 Interrupt Target Write Protection"]
pub type Gpio151inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO148 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio148intto_int13018(&self) -> EnblGpio148inttoInt13018R {
        EnblGpio148inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO148 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio148intto_int13019(&self) -> EnblGpio148inttoInt13019R {
        EnblGpio148inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO148 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio148intto_int13020(&self) -> EnblGpio148inttoInt13020R {
        EnblGpio148inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO148 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio148intto_sio(&self) -> EnblGpio148inttoSioR {
        EnblGpio148inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO148 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio148inttarget_rst_tolerance(&self) -> Gpio148inttargetRstToleranceR {
        Gpio148inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO148 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio148inttarget_wr_prot(&self) -> Gpio148inttargetWrProtR {
        Gpio148inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO149 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio149intto_int13018(&self) -> EnblGpio149inttoInt13018R {
        EnblGpio149inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO149 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio149intto_int13019(&self) -> EnblGpio149inttoInt13019R {
        EnblGpio149inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO149 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio149intto_int13020(&self) -> EnblGpio149inttoInt13020R {
        EnblGpio149inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO149 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio149intto_sio(&self) -> EnblGpio149inttoSioR {
        EnblGpio149inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO149 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio149inttarget_rst_tolerance(&self) -> Gpio149inttargetRstToleranceR {
        Gpio149inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO149 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio149inttarget_wr_prot(&self) -> Gpio149inttargetWrProtR {
        Gpio149inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO150 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio150intto_int13018(&self) -> EnblGpio150inttoInt13018R {
        EnblGpio150inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO150 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio150intto_int13019(&self) -> EnblGpio150inttoInt13019R {
        EnblGpio150inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO150 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio150intto_int13020(&self) -> EnblGpio150inttoInt13020R {
        EnblGpio150inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO150 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio150intto_sio(&self) -> EnblGpio150inttoSioR {
        EnblGpio150inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO150 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio150inttarget_rst_tolerance(&self) -> Gpio150inttargetRstToleranceR {
        Gpio150inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO150 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio150inttarget_wr_prot(&self) -> Gpio150inttargetWrProtR {
        Gpio150inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO151 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio151intto_int13018(&self) -> EnblGpio151inttoInt13018R {
        EnblGpio151inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO151 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio151intto_int13019(&self) -> EnblGpio151inttoInt13019R {
        EnblGpio151inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO151 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio151intto_int13020(&self) -> EnblGpio151inttoInt13020R {
        EnblGpio151inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO151 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio151intto_sio(&self) -> EnblGpio151inttoSioR {
        EnblGpio151inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO151 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio151inttarget_rst_tolerance(&self) -> Gpio151inttargetRstToleranceR {
        Gpio151inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO151 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio151inttarget_wr_prot(&self) -> Gpio151inttargetWrProtR {
        Gpio151inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO148 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio148intto_int13018(&mut self) -> EnblGpio148inttoInt13018W<Gpioaa4Spec> {
        EnblGpio148inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO148 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio148intto_int13019(&mut self) -> EnblGpio148inttoInt13019W<Gpioaa4Spec> {
        EnblGpio148inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO148 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio148intto_int13020(&mut self) -> EnblGpio148inttoInt13020W<Gpioaa4Spec> {
        EnblGpio148inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO148 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio148intto_sio(&mut self) -> EnblGpio148inttoSioW<Gpioaa4Spec> {
        EnblGpio148inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioaa4Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioaa4Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO148 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio148inttarget_rst_tolerance(&mut self) -> Gpio148inttargetRstToleranceW<Gpioaa4Spec> {
        Gpio148inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO148 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio148inttarget_wr_prot(&mut self) -> Gpio148inttargetWrProtW<Gpioaa4Spec> {
        Gpio148inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO149 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio149intto_int13018(&mut self) -> EnblGpio149inttoInt13018W<Gpioaa4Spec> {
        EnblGpio149inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO149 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio149intto_int13019(&mut self) -> EnblGpio149inttoInt13019W<Gpioaa4Spec> {
        EnblGpio149inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO149 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio149intto_int13020(&mut self) -> EnblGpio149inttoInt13020W<Gpioaa4Spec> {
        EnblGpio149inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO149 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio149intto_sio(&mut self) -> EnblGpio149inttoSioW<Gpioaa4Spec> {
        EnblGpio149inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioaa4Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioaa4Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO149 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio149inttarget_rst_tolerance(&mut self) -> Gpio149inttargetRstToleranceW<Gpioaa4Spec> {
        Gpio149inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO149 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio149inttarget_wr_prot(&mut self) -> Gpio149inttargetWrProtW<Gpioaa4Spec> {
        Gpio149inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO150 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio150intto_int13018(&mut self) -> EnblGpio150inttoInt13018W<Gpioaa4Spec> {
        EnblGpio150inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO150 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio150intto_int13019(&mut self) -> EnblGpio150inttoInt13019W<Gpioaa4Spec> {
        EnblGpio150inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO150 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio150intto_int13020(&mut self) -> EnblGpio150inttoInt13020W<Gpioaa4Spec> {
        EnblGpio150inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO150 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio150intto_sio(&mut self) -> EnblGpio150inttoSioW<Gpioaa4Spec> {
        EnblGpio150inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioaa4Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioaa4Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO150 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio150inttarget_rst_tolerance(&mut self) -> Gpio150inttargetRstToleranceW<Gpioaa4Spec> {
        Gpio150inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO150 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio150inttarget_wr_prot(&mut self) -> Gpio150inttargetWrProtW<Gpioaa4Spec> {
        Gpio150inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO151 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio151intto_int13018(&mut self) -> EnblGpio151inttoInt13018W<Gpioaa4Spec> {
        EnblGpio151inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO151 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio151intto_int13019(&mut self) -> EnblGpio151inttoInt13019W<Gpioaa4Spec> {
        EnblGpio151inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO151 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio151intto_int13020(&mut self) -> EnblGpio151inttoInt13020W<Gpioaa4Spec> {
        EnblGpio151inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO151 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio151intto_sio(&mut self) -> EnblGpio151inttoSioW<Gpioaa4Spec> {
        EnblGpio151inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioaa4Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO151 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio151inttarget_rst_tolerance(&mut self) -> Gpio151inttargetRstToleranceW<Gpioaa4Spec> {
        Gpio151inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO151 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio151inttarget_wr_prot(&mut self) -> Gpio151inttargetWrProtW<Gpioaa4Spec> {
        Gpio151inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#37\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioaa4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioaa4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioaa4Spec;
impl crate::RegisterSpec for Gpioaa4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioaa4::R`](R) reader structure"]
impl crate::Readable for Gpioaa4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioaa4::W`](W) writer structure"]
impl crate::Writable for Gpioaa4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOAA4 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioaa4Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
