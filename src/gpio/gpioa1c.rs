#[doc = "Register `GPIOA1C` reader"]
pub type R = crate::R<Gpioa1cSpec>;
#[doc = "Register `GPIOA1C` writer"]
pub type W = crate::W<Gpioa1cSpec>;
#[doc = "Field `EnblGPIO012INTToINT13018` reader - Enable GPIO012 Interrupt To INT#130_18"]
pub type EnblGpio012inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO012INTToINT13018` writer - Enable GPIO012 Interrupt To INT#130_18"]
pub type EnblGpio012inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO012INTToINT13019` reader - Enable GPIO012 Interrupt To INT#130_19"]
pub type EnblGpio012inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO012INTToINT13019` writer - Enable GPIO012 Interrupt To INT#130_19"]
pub type EnblGpio012inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO012INTToINT13020` reader - Enable GPIO012 Interrupt To INT#130_20"]
pub type EnblGpio012inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO012INTToINT13020` writer - Enable GPIO012 Interrupt To INT#130_20"]
pub type EnblGpio012inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO012INTToSIO` reader - Enable GPIO012 Interrupt To SIO"]
pub type EnblGpio012inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO012INTToSIO` writer - Enable GPIO012 Interrupt To SIO"]
pub type EnblGpio012inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO012 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio012inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio012inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio012inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO012INTTargetRstTolerance` reader - GPIO012 Interrupt Target Reset Tolerance"]
pub type Gpio012inttargetRstToleranceR = crate::BitReader<Gpio012inttargetRstTolerance>;
impl Gpio012inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio012inttargetRstTolerance {
        match self.bits {
            false => Gpio012inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio012inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio012inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio012inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO012INTTargetRstTolerance` writer - GPIO012 Interrupt Target Reset Tolerance"]
pub type Gpio012inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio012inttargetRstTolerance>;
impl<'a, REG> Gpio012inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio012inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio012inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO012INTTargetWrProt` reader - GPIO012 Interrupt Target Write Protection"]
pub type Gpio012inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO012INTTargetWrProt` writer - GPIO012 Interrupt Target Write Protection"]
pub type Gpio012inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO013INTToINT13018` reader - Enable GPIO013 Interrupt To INT#130_18"]
pub type EnblGpio013inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO013INTToINT13018` writer - Enable GPIO013 Interrupt To INT#130_18"]
pub type EnblGpio013inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO013INTToINT13019` reader - Enable GPIO013 Interrupt To INT#130_19"]
pub type EnblGpio013inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO013INTToINT13019` writer - Enable GPIO013 Interrupt To INT#130_19"]
pub type EnblGpio013inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO013INTToINT13020` reader - Enable GPIO013 Interrupt To INT#130_20"]
pub type EnblGpio013inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO013INTToINT13020` writer - Enable GPIO013 Interrupt To INT#130_20"]
pub type EnblGpio013inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO013INTToSIO` reader - Enable GPIO013 Interrupt To SIO"]
pub type EnblGpio013inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO013INTToSIO` writer - Enable GPIO013 Interrupt To SIO"]
pub type EnblGpio013inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO013 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio013inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio013inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio013inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO013INTTargetRstTolerance` reader - GPIO013 Interrupt Target Reset Tolerance"]
pub type Gpio013inttargetRstToleranceR = crate::BitReader<Gpio013inttargetRstTolerance>;
impl Gpio013inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio013inttargetRstTolerance {
        match self.bits {
            false => Gpio013inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio013inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio013inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio013inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO013INTTargetRstTolerance` writer - GPIO013 Interrupt Target Reset Tolerance"]
pub type Gpio013inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio013inttargetRstTolerance>;
impl<'a, REG> Gpio013inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio013inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio013inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO013INTTargetWrProt` reader - GPIO013 Interrupt Target Write Protection"]
pub type Gpio013inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO013INTTargetWrProt` writer - GPIO013 Interrupt Target Write Protection"]
pub type Gpio013inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO014INTToINT13018` reader - Enable GPIO014 Interrupt To INT#130_18"]
pub type EnblGpio014inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO014INTToINT13018` writer - Enable GPIO014 Interrupt To INT#130_18"]
pub type EnblGpio014inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO014INTToINT13019` reader - Enable GPIO014 Interrupt To INT#130_19"]
pub type EnblGpio014inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO014INTToINT13019` writer - Enable GPIO014 Interrupt To INT#130_19"]
pub type EnblGpio014inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO014INTToINT13020` reader - Enable GPIO014 Interrupt To INT#130_20"]
pub type EnblGpio014inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO014INTToINT13020` writer - Enable GPIO014 Interrupt To INT#130_20"]
pub type EnblGpio014inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO014INTToSIO` reader - Enable GPIO014 Interrupt To SIO"]
pub type EnblGpio014inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO014INTToSIO` writer - Enable GPIO014 Interrupt To SIO"]
pub type EnblGpio014inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO014 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio014inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio014inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio014inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO014INTTargetRstTolerance` reader - GPIO014 Interrupt Target Reset Tolerance"]
pub type Gpio014inttargetRstToleranceR = crate::BitReader<Gpio014inttargetRstTolerance>;
impl Gpio014inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio014inttargetRstTolerance {
        match self.bits {
            false => Gpio014inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio014inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio014inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio014inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO014INTTargetRstTolerance` writer - GPIO014 Interrupt Target Reset Tolerance"]
pub type Gpio014inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio014inttargetRstTolerance>;
impl<'a, REG> Gpio014inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio014inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio014inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO014INTTargetWrProt` reader - GPIO014 Interrupt Target Write Protection"]
pub type Gpio014inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO014INTTargetWrProt` writer - GPIO014 Interrupt Target Write Protection"]
pub type Gpio014inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO015INTToINT13018` reader - Enable GPIO015 Interrupt To INT#130_18"]
pub type EnblGpio015inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO015INTToINT13018` writer - Enable GPIO015 Interrupt To INT#130_18"]
pub type EnblGpio015inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO015INTToINT13019` reader - Enable GPIO015 Interrupt To INT#130_19"]
pub type EnblGpio015inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO015INTToINT13019` writer - Enable GPIO015 Interrupt To INT#130_19"]
pub type EnblGpio015inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO015INTToINT13020` reader - Enable GPIO015 Interrupt To INT#130_20"]
pub type EnblGpio015inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO015INTToINT13020` writer - Enable GPIO015 Interrupt To INT#130_20"]
pub type EnblGpio015inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO015INTToSIO` reader - Enable GPIO015 Interrupt To SIO"]
pub type EnblGpio015inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO015INTToSIO` writer - Enable GPIO015 Interrupt To SIO"]
pub type EnblGpio015inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO015 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio015inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio015inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio015inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO015INTTargetRstTolerance` reader - GPIO015 Interrupt Target Reset Tolerance"]
pub type Gpio015inttargetRstToleranceR = crate::BitReader<Gpio015inttargetRstTolerance>;
impl Gpio015inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio015inttargetRstTolerance {
        match self.bits {
            false => Gpio015inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio015inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio015inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio015inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO015INTTargetRstTolerance` writer - GPIO015 Interrupt Target Reset Tolerance"]
pub type Gpio015inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio015inttargetRstTolerance>;
impl<'a, REG> Gpio015inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio015inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio015inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO015INTTargetWrProt` reader - GPIO015 Interrupt Target Write Protection"]
pub type Gpio015inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO015INTTargetWrProt` writer - GPIO015 Interrupt Target Write Protection"]
pub type Gpio015inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO012 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio012intto_int13018(&self) -> EnblGpio012inttoInt13018R {
        EnblGpio012inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO012 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio012intto_int13019(&self) -> EnblGpio012inttoInt13019R {
        EnblGpio012inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO012 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio012intto_int13020(&self) -> EnblGpio012inttoInt13020R {
        EnblGpio012inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO012 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio012intto_sio(&self) -> EnblGpio012inttoSioR {
        EnblGpio012inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO012 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio012inttarget_rst_tolerance(&self) -> Gpio012inttargetRstToleranceR {
        Gpio012inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO012 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio012inttarget_wr_prot(&self) -> Gpio012inttargetWrProtR {
        Gpio012inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO013 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio013intto_int13018(&self) -> EnblGpio013inttoInt13018R {
        EnblGpio013inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO013 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio013intto_int13019(&self) -> EnblGpio013inttoInt13019R {
        EnblGpio013inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO013 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio013intto_int13020(&self) -> EnblGpio013inttoInt13020R {
        EnblGpio013inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO013 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio013intto_sio(&self) -> EnblGpio013inttoSioR {
        EnblGpio013inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO013 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio013inttarget_rst_tolerance(&self) -> Gpio013inttargetRstToleranceR {
        Gpio013inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO013 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio013inttarget_wr_prot(&self) -> Gpio013inttargetWrProtR {
        Gpio013inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO014 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio014intto_int13018(&self) -> EnblGpio014inttoInt13018R {
        EnblGpio014inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO014 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio014intto_int13019(&self) -> EnblGpio014inttoInt13019R {
        EnblGpio014inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO014 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio014intto_int13020(&self) -> EnblGpio014inttoInt13020R {
        EnblGpio014inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO014 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio014intto_sio(&self) -> EnblGpio014inttoSioR {
        EnblGpio014inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO014 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio014inttarget_rst_tolerance(&self) -> Gpio014inttargetRstToleranceR {
        Gpio014inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO014 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio014inttarget_wr_prot(&self) -> Gpio014inttargetWrProtR {
        Gpio014inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO015 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio015intto_int13018(&self) -> EnblGpio015inttoInt13018R {
        EnblGpio015inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO015 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio015intto_int13019(&self) -> EnblGpio015inttoInt13019R {
        EnblGpio015inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO015 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio015intto_int13020(&self) -> EnblGpio015inttoInt13020R {
        EnblGpio015inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO015 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio015intto_sio(&self) -> EnblGpio015inttoSioR {
        EnblGpio015inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO015 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio015inttarget_rst_tolerance(&self) -> Gpio015inttargetRstToleranceR {
        Gpio015inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO015 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio015inttarget_wr_prot(&self) -> Gpio015inttargetWrProtR {
        Gpio015inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO012 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio012intto_int13018(&mut self) -> EnblGpio012inttoInt13018W<Gpioa1cSpec> {
        EnblGpio012inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO012 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio012intto_int13019(&mut self) -> EnblGpio012inttoInt13019W<Gpioa1cSpec> {
        EnblGpio012inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO012 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio012intto_int13020(&mut self) -> EnblGpio012inttoInt13020W<Gpioa1cSpec> {
        EnblGpio012inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO012 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio012intto_sio(&mut self) -> EnblGpio012inttoSioW<Gpioa1cSpec> {
        EnblGpio012inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa1cSpec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa1cSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO012 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio012inttarget_rst_tolerance(&mut self) -> Gpio012inttargetRstToleranceW<Gpioa1cSpec> {
        Gpio012inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO012 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio012inttarget_wr_prot(&mut self) -> Gpio012inttargetWrProtW<Gpioa1cSpec> {
        Gpio012inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO013 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio013intto_int13018(&mut self) -> EnblGpio013inttoInt13018W<Gpioa1cSpec> {
        EnblGpio013inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO013 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio013intto_int13019(&mut self) -> EnblGpio013inttoInt13019W<Gpioa1cSpec> {
        EnblGpio013inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO013 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio013intto_int13020(&mut self) -> EnblGpio013inttoInt13020W<Gpioa1cSpec> {
        EnblGpio013inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO013 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio013intto_sio(&mut self) -> EnblGpio013inttoSioW<Gpioa1cSpec> {
        EnblGpio013inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa1cSpec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa1cSpec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO013 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio013inttarget_rst_tolerance(&mut self) -> Gpio013inttargetRstToleranceW<Gpioa1cSpec> {
        Gpio013inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO013 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio013inttarget_wr_prot(&mut self) -> Gpio013inttargetWrProtW<Gpioa1cSpec> {
        Gpio013inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO014 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio014intto_int13018(&mut self) -> EnblGpio014inttoInt13018W<Gpioa1cSpec> {
        EnblGpio014inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO014 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio014intto_int13019(&mut self) -> EnblGpio014inttoInt13019W<Gpioa1cSpec> {
        EnblGpio014inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO014 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio014intto_int13020(&mut self) -> EnblGpio014inttoInt13020W<Gpioa1cSpec> {
        EnblGpio014inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO014 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio014intto_sio(&mut self) -> EnblGpio014inttoSioW<Gpioa1cSpec> {
        EnblGpio014inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa1cSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa1cSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO014 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio014inttarget_rst_tolerance(&mut self) -> Gpio014inttargetRstToleranceW<Gpioa1cSpec> {
        Gpio014inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO014 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio014inttarget_wr_prot(&mut self) -> Gpio014inttargetWrProtW<Gpioa1cSpec> {
        Gpio014inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO015 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio015intto_int13018(&mut self) -> EnblGpio015inttoInt13018W<Gpioa1cSpec> {
        EnblGpio015inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO015 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio015intto_int13019(&mut self) -> EnblGpio015inttoInt13019W<Gpioa1cSpec> {
        EnblGpio015inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO015 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio015intto_int13020(&mut self) -> EnblGpio015inttoInt13020W<Gpioa1cSpec> {
        EnblGpio015inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO015 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio015intto_sio(&mut self) -> EnblGpio015inttoSioW<Gpioa1cSpec> {
        EnblGpio015inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa1cSpec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO015 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio015inttarget_rst_tolerance(&mut self) -> Gpio015inttargetRstToleranceW<Gpioa1cSpec> {
        Gpio015inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO015 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio015inttarget_wr_prot(&mut self) -> Gpio015inttargetWrProtW<Gpioa1cSpec> {
        Gpio015inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa1cSpec;
impl crate::RegisterSpec for Gpioa1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa1c::R`](R) reader structure"]
impl crate::Readable for Gpioa1cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioa1c::W`](W) writer structure"]
impl crate::Writable for Gpioa1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA1C to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa1cSpec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
