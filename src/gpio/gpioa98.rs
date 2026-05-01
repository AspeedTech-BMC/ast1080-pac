#[doc = "Register `GPIOA98` reader"]
pub type R = crate::R<Gpioa98Spec>;
#[doc = "Register `GPIOA98` writer"]
pub type W = crate::W<Gpioa98Spec>;
#[doc = "Field `EnblGPIO136INTToINT13018` reader - Enable GPIO136 Interrupt To INT#130_18"]
pub type EnblGpio136inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO136INTToINT13018` writer - Enable GPIO136 Interrupt To INT#130_18"]
pub type EnblGpio136inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO136INTToINT13019` reader - Enable GPIO136 Interrupt To INT#130_19"]
pub type EnblGpio136inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO136INTToINT13019` writer - Enable GPIO136 Interrupt To INT#130_19"]
pub type EnblGpio136inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO136INTToINT13020` reader - Enable GPIO136 Interrupt To INT#130_20"]
pub type EnblGpio136inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO136INTToINT13020` writer - Enable GPIO136 Interrupt To INT#130_20"]
pub type EnblGpio136inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO136INTToSIO` reader - Enable GPIO136 Interrupt To SIO"]
pub type EnblGpio136inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO136INTToSIO` writer - Enable GPIO136 Interrupt To SIO"]
pub type EnblGpio136inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO136 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio136inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio136inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio136inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO136INTTargetRstTolerance` reader - GPIO136 Interrupt Target Reset Tolerance"]
pub type Gpio136inttargetRstToleranceR = crate::BitReader<Gpio136inttargetRstTolerance>;
impl Gpio136inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio136inttargetRstTolerance {
        match self.bits {
            false => Gpio136inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio136inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio136inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio136inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO136INTTargetRstTolerance` writer - GPIO136 Interrupt Target Reset Tolerance"]
pub type Gpio136inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio136inttargetRstTolerance>;
impl<'a, REG> Gpio136inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio136inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio136inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO136INTTargetWrProt` reader - GPIO136 Interrupt Target Write Protection"]
pub type Gpio136inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO136INTTargetWrProt` writer - GPIO136 Interrupt Target Write Protection"]
pub type Gpio136inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO137INTToINT13018` reader - Enable GPIO137 Interrupt To INT#130_18"]
pub type EnblGpio137inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO137INTToINT13018` writer - Enable GPIO137 Interrupt To INT#130_18"]
pub type EnblGpio137inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO137INTToINT13019` reader - Enable GPIO137 Interrupt To INT#130_19"]
pub type EnblGpio137inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO137INTToINT13019` writer - Enable GPIO137 Interrupt To INT#130_19"]
pub type EnblGpio137inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO137INTToINT13020` reader - Enable GPIO137 Interrupt To INT#130_20"]
pub type EnblGpio137inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO137INTToINT13020` writer - Enable GPIO137 Interrupt To INT#130_20"]
pub type EnblGpio137inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO137INTToSIO` reader - Enable GPIO137 Interrupt To SIO"]
pub type EnblGpio137inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO137INTToSIO` writer - Enable GPIO137 Interrupt To SIO"]
pub type EnblGpio137inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO137 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio137inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio137inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio137inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO137INTTargetRstTolerance` reader - GPIO137 Interrupt Target Reset Tolerance"]
pub type Gpio137inttargetRstToleranceR = crate::BitReader<Gpio137inttargetRstTolerance>;
impl Gpio137inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio137inttargetRstTolerance {
        match self.bits {
            false => Gpio137inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio137inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio137inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio137inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO137INTTargetRstTolerance` writer - GPIO137 Interrupt Target Reset Tolerance"]
pub type Gpio137inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio137inttargetRstTolerance>;
impl<'a, REG> Gpio137inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio137inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio137inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO137INTTargetWrProt` reader - GPIO137 Interrupt Target Write Protection"]
pub type Gpio137inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO137INTTargetWrProt` writer - GPIO137 Interrupt Target Write Protection"]
pub type Gpio137inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO138INTToINT13018` reader - Enable GPIO138 Interrupt To INT#130_18"]
pub type EnblGpio138inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO138INTToINT13018` writer - Enable GPIO138 Interrupt To INT#130_18"]
pub type EnblGpio138inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO138INTToINT13019` reader - Enable GPIO138 Interrupt To INT#130_19"]
pub type EnblGpio138inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO138INTToINT13019` writer - Enable GPIO138 Interrupt To INT#130_19"]
pub type EnblGpio138inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO138INTToINT13020` reader - Enable GPIO138 Interrupt To INT#130_20"]
pub type EnblGpio138inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO138INTToINT13020` writer - Enable GPIO138 Interrupt To INT#130_20"]
pub type EnblGpio138inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO138INTToSIO` reader - Enable GPIO138 Interrupt To SIO"]
pub type EnblGpio138inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO138INTToSIO` writer - Enable GPIO138 Interrupt To SIO"]
pub type EnblGpio138inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO138 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio138inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio138inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio138inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO138INTTargetRstTolerance` reader - GPIO138 Interrupt Target Reset Tolerance"]
pub type Gpio138inttargetRstToleranceR = crate::BitReader<Gpio138inttargetRstTolerance>;
impl Gpio138inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio138inttargetRstTolerance {
        match self.bits {
            false => Gpio138inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio138inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio138inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio138inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO138INTTargetRstTolerance` writer - GPIO138 Interrupt Target Reset Tolerance"]
pub type Gpio138inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio138inttargetRstTolerance>;
impl<'a, REG> Gpio138inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio138inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio138inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO138INTTargetWrProt` reader - GPIO138 Interrupt Target Write Protection"]
pub type Gpio138inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO138INTTargetWrProt` writer - GPIO138 Interrupt Target Write Protection"]
pub type Gpio138inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO139INTToINT13018` reader - Enable GPIO139 Interrupt To INT#130_18"]
pub type EnblGpio139inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO139INTToINT13018` writer - Enable GPIO139 Interrupt To INT#130_18"]
pub type EnblGpio139inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO139INTToINT13019` reader - Enable GPIO139 Interrupt To INT#130_19"]
pub type EnblGpio139inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO139INTToINT13019` writer - Enable GPIO139 Interrupt To INT#130_19"]
pub type EnblGpio139inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO139INTToINT13020` reader - Enable GPIO139 Interrupt To INT#130_20"]
pub type EnblGpio139inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO139INTToINT13020` writer - Enable GPIO139 Interrupt To INT#130_20"]
pub type EnblGpio139inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO139INTToSIO` reader - Enable GPIO139 Interrupt To SIO"]
pub type EnblGpio139inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO139INTToSIO` writer - Enable GPIO139 Interrupt To SIO"]
pub type EnblGpio139inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO139 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio139inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio139inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio139inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO139INTTargetRstTolerance` reader - GPIO139 Interrupt Target Reset Tolerance"]
pub type Gpio139inttargetRstToleranceR = crate::BitReader<Gpio139inttargetRstTolerance>;
impl Gpio139inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio139inttargetRstTolerance {
        match self.bits {
            false => Gpio139inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio139inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio139inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio139inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO139INTTargetRstTolerance` writer - GPIO139 Interrupt Target Reset Tolerance"]
pub type Gpio139inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio139inttargetRstTolerance>;
impl<'a, REG> Gpio139inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio139inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio139inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO139INTTargetWrProt` reader - GPIO139 Interrupt Target Write Protection"]
pub type Gpio139inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO139INTTargetWrProt` writer - GPIO139 Interrupt Target Write Protection"]
pub type Gpio139inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO136 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio136intto_int13018(&self) -> EnblGpio136inttoInt13018R {
        EnblGpio136inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO136 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio136intto_int13019(&self) -> EnblGpio136inttoInt13019R {
        EnblGpio136inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO136 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio136intto_int13020(&self) -> EnblGpio136inttoInt13020R {
        EnblGpio136inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO136 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio136intto_sio(&self) -> EnblGpio136inttoSioR {
        EnblGpio136inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO136 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio136inttarget_rst_tolerance(&self) -> Gpio136inttargetRstToleranceR {
        Gpio136inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO136 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio136inttarget_wr_prot(&self) -> Gpio136inttargetWrProtR {
        Gpio136inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO137 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio137intto_int13018(&self) -> EnblGpio137inttoInt13018R {
        EnblGpio137inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO137 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio137intto_int13019(&self) -> EnblGpio137inttoInt13019R {
        EnblGpio137inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO137 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio137intto_int13020(&self) -> EnblGpio137inttoInt13020R {
        EnblGpio137inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO137 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio137intto_sio(&self) -> EnblGpio137inttoSioR {
        EnblGpio137inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO137 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio137inttarget_rst_tolerance(&self) -> Gpio137inttargetRstToleranceR {
        Gpio137inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO137 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio137inttarget_wr_prot(&self) -> Gpio137inttargetWrProtR {
        Gpio137inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO138 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio138intto_int13018(&self) -> EnblGpio138inttoInt13018R {
        EnblGpio138inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO138 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio138intto_int13019(&self) -> EnblGpio138inttoInt13019R {
        EnblGpio138inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO138 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio138intto_int13020(&self) -> EnblGpio138inttoInt13020R {
        EnblGpio138inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO138 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio138intto_sio(&self) -> EnblGpio138inttoSioR {
        EnblGpio138inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO138 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio138inttarget_rst_tolerance(&self) -> Gpio138inttargetRstToleranceR {
        Gpio138inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO138 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio138inttarget_wr_prot(&self) -> Gpio138inttargetWrProtR {
        Gpio138inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO139 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio139intto_int13018(&self) -> EnblGpio139inttoInt13018R {
        EnblGpio139inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO139 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio139intto_int13019(&self) -> EnblGpio139inttoInt13019R {
        EnblGpio139inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO139 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio139intto_int13020(&self) -> EnblGpio139inttoInt13020R {
        EnblGpio139inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO139 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio139intto_sio(&self) -> EnblGpio139inttoSioR {
        EnblGpio139inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO139 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio139inttarget_rst_tolerance(&self) -> Gpio139inttargetRstToleranceR {
        Gpio139inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO139 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio139inttarget_wr_prot(&self) -> Gpio139inttargetWrProtR {
        Gpio139inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO136 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio136intto_int13018(&mut self) -> EnblGpio136inttoInt13018W<Gpioa98Spec> {
        EnblGpio136inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO136 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio136intto_int13019(&mut self) -> EnblGpio136inttoInt13019W<Gpioa98Spec> {
        EnblGpio136inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO136 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio136intto_int13020(&mut self) -> EnblGpio136inttoInt13020W<Gpioa98Spec> {
        EnblGpio136inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO136 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio136intto_sio(&mut self) -> EnblGpio136inttoSioW<Gpioa98Spec> {
        EnblGpio136inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa98Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa98Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO136 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio136inttarget_rst_tolerance(&mut self) -> Gpio136inttargetRstToleranceW<Gpioa98Spec> {
        Gpio136inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO136 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio136inttarget_wr_prot(&mut self) -> Gpio136inttargetWrProtW<Gpioa98Spec> {
        Gpio136inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO137 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio137intto_int13018(&mut self) -> EnblGpio137inttoInt13018W<Gpioa98Spec> {
        EnblGpio137inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO137 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio137intto_int13019(&mut self) -> EnblGpio137inttoInt13019W<Gpioa98Spec> {
        EnblGpio137inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO137 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio137intto_int13020(&mut self) -> EnblGpio137inttoInt13020W<Gpioa98Spec> {
        EnblGpio137inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO137 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio137intto_sio(&mut self) -> EnblGpio137inttoSioW<Gpioa98Spec> {
        EnblGpio137inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa98Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa98Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO137 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio137inttarget_rst_tolerance(&mut self) -> Gpio137inttargetRstToleranceW<Gpioa98Spec> {
        Gpio137inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO137 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio137inttarget_wr_prot(&mut self) -> Gpio137inttargetWrProtW<Gpioa98Spec> {
        Gpio137inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO138 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio138intto_int13018(&mut self) -> EnblGpio138inttoInt13018W<Gpioa98Spec> {
        EnblGpio138inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO138 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio138intto_int13019(&mut self) -> EnblGpio138inttoInt13019W<Gpioa98Spec> {
        EnblGpio138inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO138 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio138intto_int13020(&mut self) -> EnblGpio138inttoInt13020W<Gpioa98Spec> {
        EnblGpio138inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO138 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio138intto_sio(&mut self) -> EnblGpio138inttoSioW<Gpioa98Spec> {
        EnblGpio138inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa98Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa98Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO138 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio138inttarget_rst_tolerance(&mut self) -> Gpio138inttargetRstToleranceW<Gpioa98Spec> {
        Gpio138inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO138 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio138inttarget_wr_prot(&mut self) -> Gpio138inttargetWrProtW<Gpioa98Spec> {
        Gpio138inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO139 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio139intto_int13018(&mut self) -> EnblGpio139inttoInt13018W<Gpioa98Spec> {
        EnblGpio139inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO139 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio139intto_int13019(&mut self) -> EnblGpio139inttoInt13019W<Gpioa98Spec> {
        EnblGpio139inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO139 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio139intto_int13020(&mut self) -> EnblGpio139inttoInt13020W<Gpioa98Spec> {
        EnblGpio139inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO139 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio139intto_sio(&mut self) -> EnblGpio139inttoSioW<Gpioa98Spec> {
        EnblGpio139inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa98Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO139 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio139inttarget_rst_tolerance(&mut self) -> Gpio139inttargetRstToleranceW<Gpioa98Spec> {
        Gpio139inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO139 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio139inttarget_wr_prot(&mut self) -> Gpio139inttargetWrProtW<Gpioa98Spec> {
        Gpio139inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa98::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa98::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa98Spec;
impl crate::RegisterSpec for Gpioa98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa98::R`](R) reader structure"]
impl crate::Readable for Gpioa98Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa98::W`](W) writer structure"]
impl crate::Writable for Gpioa98Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA98 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa98Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
