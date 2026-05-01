#[doc = "Register `GPIOA3C` reader"]
pub type R = crate::R<Gpioa3cSpec>;
#[doc = "Register `GPIOA3C` writer"]
pub type W = crate::W<Gpioa3cSpec>;
#[doc = "Field `EnblGPIO044INTToINT13018` reader - Enable GPIO044 Interrupt To INT#130_18"]
pub type EnblGpio044inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO044INTToINT13018` writer - Enable GPIO044 Interrupt To INT#130_18"]
pub type EnblGpio044inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO044INTToINT13019` reader - Enable GPIO044 Interrupt To INT#130_19"]
pub type EnblGpio044inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO044INTToINT13019` writer - Enable GPIO044 Interrupt To INT#130_19"]
pub type EnblGpio044inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO044INTToINT13020` reader - Enable GPIO044 Interrupt To INT#130_20"]
pub type EnblGpio044inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO044INTToINT13020` writer - Enable GPIO044 Interrupt To INT#130_20"]
pub type EnblGpio044inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO044INTToSIO` reader - Enable GPIO044 Interrupt To SIO"]
pub type EnblGpio044inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO044INTToSIO` writer - Enable GPIO044 Interrupt To SIO"]
pub type EnblGpio044inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO044 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio044inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio044inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio044inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO044INTTargetRstTolerance` reader - GPIO044 Interrupt Target Reset Tolerance"]
pub type Gpio044inttargetRstToleranceR = crate::BitReader<Gpio044inttargetRstTolerance>;
impl Gpio044inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio044inttargetRstTolerance {
        match self.bits {
            false => Gpio044inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio044inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio044inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio044inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO044INTTargetRstTolerance` writer - GPIO044 Interrupt Target Reset Tolerance"]
pub type Gpio044inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio044inttargetRstTolerance>;
impl<'a, REG> Gpio044inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio044inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio044inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO044INTTargetWrProt` reader - GPIO044 Interrupt Target Write Protection"]
pub type Gpio044inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO044INTTargetWrProt` writer - GPIO044 Interrupt Target Write Protection"]
pub type Gpio044inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO045INTToINT13018` reader - Enable GPIO045 Interrupt To INT#130_18"]
pub type EnblGpio045inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO045INTToINT13018` writer - Enable GPIO045 Interrupt To INT#130_18"]
pub type EnblGpio045inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO045INTToINT13019` reader - Enable GPIO045 Interrupt To INT#130_19"]
pub type EnblGpio045inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO045INTToINT13019` writer - Enable GPIO045 Interrupt To INT#130_19"]
pub type EnblGpio045inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO045INTToINT13020` reader - Enable GPIO045 Interrupt To INT#130_20"]
pub type EnblGpio045inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO045INTToINT13020` writer - Enable GPIO045 Interrupt To INT#130_20"]
pub type EnblGpio045inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO045INTToSIO` reader - Enable GPIO045 Interrupt To SIO"]
pub type EnblGpio045inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO045INTToSIO` writer - Enable GPIO045 Interrupt To SIO"]
pub type EnblGpio045inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO045 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio045inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio045inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio045inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO045INTTargetRstTolerance` reader - GPIO045 Interrupt Target Reset Tolerance"]
pub type Gpio045inttargetRstToleranceR = crate::BitReader<Gpio045inttargetRstTolerance>;
impl Gpio045inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio045inttargetRstTolerance {
        match self.bits {
            false => Gpio045inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio045inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio045inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio045inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO045INTTargetRstTolerance` writer - GPIO045 Interrupt Target Reset Tolerance"]
pub type Gpio045inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio045inttargetRstTolerance>;
impl<'a, REG> Gpio045inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio045inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio045inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO045INTTargetWrProt` reader - GPIO045 Interrupt Target Write Protection"]
pub type Gpio045inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO045INTTargetWrProt` writer - GPIO045 Interrupt Target Write Protection"]
pub type Gpio045inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO046INTToINT13018` reader - Enable GPIO046 Interrupt To INT#130_18"]
pub type EnblGpio046inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO046INTToINT13018` writer - Enable GPIO046 Interrupt To INT#130_18"]
pub type EnblGpio046inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO046INTToINT13019` reader - Enable GPIO046 Interrupt To INT#130_19"]
pub type EnblGpio046inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO046INTToINT13019` writer - Enable GPIO046 Interrupt To INT#130_19"]
pub type EnblGpio046inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO046INTToINT13020` reader - Enable GPIO046 Interrupt To INT#130_20"]
pub type EnblGpio046inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO046INTToINT13020` writer - Enable GPIO046 Interrupt To INT#130_20"]
pub type EnblGpio046inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO046INTToSIO` reader - Enable GPIO046 Interrupt To SIO"]
pub type EnblGpio046inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO046INTToSIO` writer - Enable GPIO046 Interrupt To SIO"]
pub type EnblGpio046inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO046 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio046inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio046inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio046inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO046INTTargetRstTolerance` reader - GPIO046 Interrupt Target Reset Tolerance"]
pub type Gpio046inttargetRstToleranceR = crate::BitReader<Gpio046inttargetRstTolerance>;
impl Gpio046inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio046inttargetRstTolerance {
        match self.bits {
            false => Gpio046inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio046inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio046inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio046inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO046INTTargetRstTolerance` writer - GPIO046 Interrupt Target Reset Tolerance"]
pub type Gpio046inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio046inttargetRstTolerance>;
impl<'a, REG> Gpio046inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio046inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio046inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO046INTTargetWrProt` reader - GPIO046 Interrupt Target Write Protection"]
pub type Gpio046inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO046INTTargetWrProt` writer - GPIO046 Interrupt Target Write Protection"]
pub type Gpio046inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO047INTToINT13018` reader - Enable GPIO047 Interrupt To INT#130_18"]
pub type EnblGpio047inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO047INTToINT13018` writer - Enable GPIO047 Interrupt To INT#130_18"]
pub type EnblGpio047inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO047INTToINT13019` reader - Enable GPIO047 Interrupt To INT#130_19"]
pub type EnblGpio047inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO047INTToINT13019` writer - Enable GPIO047 Interrupt To INT#130_19"]
pub type EnblGpio047inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO047INTToINT13020` reader - Enable GPIO047 Interrupt To INT#130_20"]
pub type EnblGpio047inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO047INTToINT13020` writer - Enable GPIO047 Interrupt To INT#130_20"]
pub type EnblGpio047inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO047INTToSIO` reader - Enable GPIO047 Interrupt To SIO"]
pub type EnblGpio047inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO047INTToSIO` writer - Enable GPIO047 Interrupt To SIO"]
pub type EnblGpio047inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO047 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio047inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio047inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio047inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO047INTTargetRstTolerance` reader - GPIO047 Interrupt Target Reset Tolerance"]
pub type Gpio047inttargetRstToleranceR = crate::BitReader<Gpio047inttargetRstTolerance>;
impl Gpio047inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio047inttargetRstTolerance {
        match self.bits {
            false => Gpio047inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio047inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio047inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio047inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO047INTTargetRstTolerance` writer - GPIO047 Interrupt Target Reset Tolerance"]
pub type Gpio047inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio047inttargetRstTolerance>;
impl<'a, REG> Gpio047inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio047inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio047inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO047INTTargetWrProt` reader - GPIO047 Interrupt Target Write Protection"]
pub type Gpio047inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO047INTTargetWrProt` writer - GPIO047 Interrupt Target Write Protection"]
pub type Gpio047inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO044 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio044intto_int13018(&self) -> EnblGpio044inttoInt13018R {
        EnblGpio044inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO044 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio044intto_int13019(&self) -> EnblGpio044inttoInt13019R {
        EnblGpio044inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO044 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio044intto_int13020(&self) -> EnblGpio044inttoInt13020R {
        EnblGpio044inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO044 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio044intto_sio(&self) -> EnblGpio044inttoSioR {
        EnblGpio044inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO044 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio044inttarget_rst_tolerance(&self) -> Gpio044inttargetRstToleranceR {
        Gpio044inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO044 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio044inttarget_wr_prot(&self) -> Gpio044inttargetWrProtR {
        Gpio044inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO045 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio045intto_int13018(&self) -> EnblGpio045inttoInt13018R {
        EnblGpio045inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO045 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio045intto_int13019(&self) -> EnblGpio045inttoInt13019R {
        EnblGpio045inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO045 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio045intto_int13020(&self) -> EnblGpio045inttoInt13020R {
        EnblGpio045inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO045 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio045intto_sio(&self) -> EnblGpio045inttoSioR {
        EnblGpio045inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO045 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio045inttarget_rst_tolerance(&self) -> Gpio045inttargetRstToleranceR {
        Gpio045inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO045 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio045inttarget_wr_prot(&self) -> Gpio045inttargetWrProtR {
        Gpio045inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO046 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio046intto_int13018(&self) -> EnblGpio046inttoInt13018R {
        EnblGpio046inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO046 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio046intto_int13019(&self) -> EnblGpio046inttoInt13019R {
        EnblGpio046inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO046 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio046intto_int13020(&self) -> EnblGpio046inttoInt13020R {
        EnblGpio046inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO046 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio046intto_sio(&self) -> EnblGpio046inttoSioR {
        EnblGpio046inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO046 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio046inttarget_rst_tolerance(&self) -> Gpio046inttargetRstToleranceR {
        Gpio046inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO046 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio046inttarget_wr_prot(&self) -> Gpio046inttargetWrProtR {
        Gpio046inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO047 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio047intto_int13018(&self) -> EnblGpio047inttoInt13018R {
        EnblGpio047inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO047 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio047intto_int13019(&self) -> EnblGpio047inttoInt13019R {
        EnblGpio047inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO047 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio047intto_int13020(&self) -> EnblGpio047inttoInt13020R {
        EnblGpio047inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO047 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio047intto_sio(&self) -> EnblGpio047inttoSioR {
        EnblGpio047inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO047 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio047inttarget_rst_tolerance(&self) -> Gpio047inttargetRstToleranceR {
        Gpio047inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO047 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio047inttarget_wr_prot(&self) -> Gpio047inttargetWrProtR {
        Gpio047inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO044 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio044intto_int13018(&mut self) -> EnblGpio044inttoInt13018W<Gpioa3cSpec> {
        EnblGpio044inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO044 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio044intto_int13019(&mut self) -> EnblGpio044inttoInt13019W<Gpioa3cSpec> {
        EnblGpio044inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO044 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio044intto_int13020(&mut self) -> EnblGpio044inttoInt13020W<Gpioa3cSpec> {
        EnblGpio044inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO044 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio044intto_sio(&mut self) -> EnblGpio044inttoSioW<Gpioa3cSpec> {
        EnblGpio044inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa3cSpec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa3cSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO044 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio044inttarget_rst_tolerance(&mut self) -> Gpio044inttargetRstToleranceW<Gpioa3cSpec> {
        Gpio044inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO044 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio044inttarget_wr_prot(&mut self) -> Gpio044inttargetWrProtW<Gpioa3cSpec> {
        Gpio044inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO045 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio045intto_int13018(&mut self) -> EnblGpio045inttoInt13018W<Gpioa3cSpec> {
        EnblGpio045inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO045 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio045intto_int13019(&mut self) -> EnblGpio045inttoInt13019W<Gpioa3cSpec> {
        EnblGpio045inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO045 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio045intto_int13020(&mut self) -> EnblGpio045inttoInt13020W<Gpioa3cSpec> {
        EnblGpio045inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO045 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio045intto_sio(&mut self) -> EnblGpio045inttoSioW<Gpioa3cSpec> {
        EnblGpio045inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa3cSpec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa3cSpec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO045 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio045inttarget_rst_tolerance(&mut self) -> Gpio045inttargetRstToleranceW<Gpioa3cSpec> {
        Gpio045inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO045 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio045inttarget_wr_prot(&mut self) -> Gpio045inttargetWrProtW<Gpioa3cSpec> {
        Gpio045inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO046 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio046intto_int13018(&mut self) -> EnblGpio046inttoInt13018W<Gpioa3cSpec> {
        EnblGpio046inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO046 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio046intto_int13019(&mut self) -> EnblGpio046inttoInt13019W<Gpioa3cSpec> {
        EnblGpio046inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO046 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio046intto_int13020(&mut self) -> EnblGpio046inttoInt13020W<Gpioa3cSpec> {
        EnblGpio046inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO046 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio046intto_sio(&mut self) -> EnblGpio046inttoSioW<Gpioa3cSpec> {
        EnblGpio046inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa3cSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa3cSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO046 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio046inttarget_rst_tolerance(&mut self) -> Gpio046inttargetRstToleranceW<Gpioa3cSpec> {
        Gpio046inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO046 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio046inttarget_wr_prot(&mut self) -> Gpio046inttargetWrProtW<Gpioa3cSpec> {
        Gpio046inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO047 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio047intto_int13018(&mut self) -> EnblGpio047inttoInt13018W<Gpioa3cSpec> {
        EnblGpio047inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO047 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio047intto_int13019(&mut self) -> EnblGpio047inttoInt13019W<Gpioa3cSpec> {
        EnblGpio047inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO047 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio047intto_int13020(&mut self) -> EnblGpio047inttoInt13020W<Gpioa3cSpec> {
        EnblGpio047inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO047 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio047intto_sio(&mut self) -> EnblGpio047inttoSioW<Gpioa3cSpec> {
        EnblGpio047inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa3cSpec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO047 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio047inttarget_rst_tolerance(&mut self) -> Gpio047inttargetRstToleranceW<Gpioa3cSpec> {
        Gpio047inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO047 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio047inttarget_wr_prot(&mut self) -> Gpio047inttargetWrProtW<Gpioa3cSpec> {
        Gpio047inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa3cSpec;
impl crate::RegisterSpec for Gpioa3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa3c::R`](R) reader structure"]
impl crate::Readable for Gpioa3cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioa3c::W`](W) writer structure"]
impl crate::Writable for Gpioa3cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA3C to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa3cSpec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
