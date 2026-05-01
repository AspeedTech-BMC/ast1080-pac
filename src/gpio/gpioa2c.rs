#[doc = "Register `GPIOA2C` reader"]
pub type R = crate::R<Gpioa2cSpec>;
#[doc = "Register `GPIOA2C` writer"]
pub type W = crate::W<Gpioa2cSpec>;
#[doc = "Field `EnblGPIO028INTToINT13018` reader - Enable GPIO028 Interrupt To INT#130_18"]
pub type EnblGpio028inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO028INTToINT13018` writer - Enable GPIO028 Interrupt To INT#130_18"]
pub type EnblGpio028inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO028INTToINT13019` reader - Enable GPIO028 Interrupt To INT#130_19"]
pub type EnblGpio028inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO028INTToINT13019` writer - Enable GPIO028 Interrupt To INT#130_19"]
pub type EnblGpio028inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO028INTToINT13020` reader - Enable GPIO028 Interrupt To INT#130_20"]
pub type EnblGpio028inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO028INTToINT13020` writer - Enable GPIO028 Interrupt To INT#130_20"]
pub type EnblGpio028inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO028INTToSIO` reader - Enable GPIO028 Interrupt To SIO"]
pub type EnblGpio028inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO028INTToSIO` writer - Enable GPIO028 Interrupt To SIO"]
pub type EnblGpio028inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO028 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio028inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio028inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio028inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO028INTTargetRstTolerance` reader - GPIO028 Interrupt Target Reset Tolerance"]
pub type Gpio028inttargetRstToleranceR = crate::BitReader<Gpio028inttargetRstTolerance>;
impl Gpio028inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio028inttargetRstTolerance {
        match self.bits {
            false => Gpio028inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio028inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio028inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio028inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO028INTTargetRstTolerance` writer - GPIO028 Interrupt Target Reset Tolerance"]
pub type Gpio028inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio028inttargetRstTolerance>;
impl<'a, REG> Gpio028inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio028inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio028inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO028INTTargetWrProt` reader - GPIO028 Interrupt Target Write Protection"]
pub type Gpio028inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO028INTTargetWrProt` writer - GPIO028 Interrupt Target Write Protection"]
pub type Gpio028inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO029INTToINT13018` reader - Enable GPIO029 Interrupt To INT#130_18"]
pub type EnblGpio029inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO029INTToINT13018` writer - Enable GPIO029 Interrupt To INT#130_18"]
pub type EnblGpio029inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO029INTToINT13019` reader - Enable GPIO029 Interrupt To INT#130_19"]
pub type EnblGpio029inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO029INTToINT13019` writer - Enable GPIO029 Interrupt To INT#130_19"]
pub type EnblGpio029inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO029INTToINT13020` reader - Enable GPIO029 Interrupt To INT#130_20"]
pub type EnblGpio029inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO029INTToINT13020` writer - Enable GPIO029 Interrupt To INT#130_20"]
pub type EnblGpio029inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO029INTToSIO` reader - Enable GPIO029 Interrupt To SIO"]
pub type EnblGpio029inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO029INTToSIO` writer - Enable GPIO029 Interrupt To SIO"]
pub type EnblGpio029inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO029 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio029inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio029inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio029inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO029INTTargetRstTolerance` reader - GPIO029 Interrupt Target Reset Tolerance"]
pub type Gpio029inttargetRstToleranceR = crate::BitReader<Gpio029inttargetRstTolerance>;
impl Gpio029inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio029inttargetRstTolerance {
        match self.bits {
            false => Gpio029inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio029inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio029inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio029inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO029INTTargetRstTolerance` writer - GPIO029 Interrupt Target Reset Tolerance"]
pub type Gpio029inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio029inttargetRstTolerance>;
impl<'a, REG> Gpio029inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio029inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio029inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO029INTTargetWrProt` reader - GPIO029 Interrupt Target Write Protection"]
pub type Gpio029inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO029INTTargetWrProt` writer - GPIO029 Interrupt Target Write Protection"]
pub type Gpio029inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO030INTToINT13018` reader - Enable GPIO030 Interrupt To INT#130_18"]
pub type EnblGpio030inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO030INTToINT13018` writer - Enable GPIO030 Interrupt To INT#130_18"]
pub type EnblGpio030inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO030INTToINT13019` reader - Enable GPIO030 Interrupt To INT#130_19"]
pub type EnblGpio030inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO030INTToINT13019` writer - Enable GPIO030 Interrupt To INT#130_19"]
pub type EnblGpio030inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO030INTToINT13020` reader - Enable GPIO030 Interrupt To INT#130_20"]
pub type EnblGpio030inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO030INTToINT13020` writer - Enable GPIO030 Interrupt To INT#130_20"]
pub type EnblGpio030inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO030INTToSIO` reader - Enable GPIO030 Interrupt To SIO"]
pub type EnblGpio030inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO030INTToSIO` writer - Enable GPIO030 Interrupt To SIO"]
pub type EnblGpio030inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO030 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio030inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio030inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio030inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO030INTTargetRstTolerance` reader - GPIO030 Interrupt Target Reset Tolerance"]
pub type Gpio030inttargetRstToleranceR = crate::BitReader<Gpio030inttargetRstTolerance>;
impl Gpio030inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio030inttargetRstTolerance {
        match self.bits {
            false => Gpio030inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio030inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio030inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio030inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO030INTTargetRstTolerance` writer - GPIO030 Interrupt Target Reset Tolerance"]
pub type Gpio030inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio030inttargetRstTolerance>;
impl<'a, REG> Gpio030inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio030inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio030inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO030INTTargetWrProt` reader - GPIO030 Interrupt Target Write Protection"]
pub type Gpio030inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO030INTTargetWrProt` writer - GPIO030 Interrupt Target Write Protection"]
pub type Gpio030inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO031INTToINT13018` reader - Enable GPIO031 Interrupt To INT#130_18"]
pub type EnblGpio031inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO031INTToINT13018` writer - Enable GPIO031 Interrupt To INT#130_18"]
pub type EnblGpio031inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO031INTToINT13019` reader - Enable GPIO031 Interrupt To INT#130_19"]
pub type EnblGpio031inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO031INTToINT13019` writer - Enable GPIO031 Interrupt To INT#130_19"]
pub type EnblGpio031inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO031INTToINT13020` reader - Enable GPIO031 Interrupt To INT#130_20"]
pub type EnblGpio031inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO031INTToINT13020` writer - Enable GPIO031 Interrupt To INT#130_20"]
pub type EnblGpio031inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO031INTToSIO` reader - Enable GPIO031 Interrupt To SIO"]
pub type EnblGpio031inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO031INTToSIO` writer - Enable GPIO031 Interrupt To SIO"]
pub type EnblGpio031inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO031 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio031inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio031inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio031inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO031INTTargetRstTolerance` reader - GPIO031 Interrupt Target Reset Tolerance"]
pub type Gpio031inttargetRstToleranceR = crate::BitReader<Gpio031inttargetRstTolerance>;
impl Gpio031inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio031inttargetRstTolerance {
        match self.bits {
            false => Gpio031inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio031inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio031inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio031inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO031INTTargetRstTolerance` writer - GPIO031 Interrupt Target Reset Tolerance"]
pub type Gpio031inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio031inttargetRstTolerance>;
impl<'a, REG> Gpio031inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio031inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio031inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO031INTTargetWrProt` reader - GPIO031 Interrupt Target Write Protection"]
pub type Gpio031inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO031INTTargetWrProt` writer - GPIO031 Interrupt Target Write Protection"]
pub type Gpio031inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO028 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio028intto_int13018(&self) -> EnblGpio028inttoInt13018R {
        EnblGpio028inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO028 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio028intto_int13019(&self) -> EnblGpio028inttoInt13019R {
        EnblGpio028inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO028 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio028intto_int13020(&self) -> EnblGpio028inttoInt13020R {
        EnblGpio028inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO028 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio028intto_sio(&self) -> EnblGpio028inttoSioR {
        EnblGpio028inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO028 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio028inttarget_rst_tolerance(&self) -> Gpio028inttargetRstToleranceR {
        Gpio028inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO028 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio028inttarget_wr_prot(&self) -> Gpio028inttargetWrProtR {
        Gpio028inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO029 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio029intto_int13018(&self) -> EnblGpio029inttoInt13018R {
        EnblGpio029inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO029 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio029intto_int13019(&self) -> EnblGpio029inttoInt13019R {
        EnblGpio029inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO029 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio029intto_int13020(&self) -> EnblGpio029inttoInt13020R {
        EnblGpio029inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO029 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio029intto_sio(&self) -> EnblGpio029inttoSioR {
        EnblGpio029inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO029 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio029inttarget_rst_tolerance(&self) -> Gpio029inttargetRstToleranceR {
        Gpio029inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO029 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio029inttarget_wr_prot(&self) -> Gpio029inttargetWrProtR {
        Gpio029inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO030 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio030intto_int13018(&self) -> EnblGpio030inttoInt13018R {
        EnblGpio030inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO030 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio030intto_int13019(&self) -> EnblGpio030inttoInt13019R {
        EnblGpio030inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO030 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio030intto_int13020(&self) -> EnblGpio030inttoInt13020R {
        EnblGpio030inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO030 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio030intto_sio(&self) -> EnblGpio030inttoSioR {
        EnblGpio030inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO030 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio030inttarget_rst_tolerance(&self) -> Gpio030inttargetRstToleranceR {
        Gpio030inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO030 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio030inttarget_wr_prot(&self) -> Gpio030inttargetWrProtR {
        Gpio030inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO031 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio031intto_int13018(&self) -> EnblGpio031inttoInt13018R {
        EnblGpio031inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO031 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio031intto_int13019(&self) -> EnblGpio031inttoInt13019R {
        EnblGpio031inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO031 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio031intto_int13020(&self) -> EnblGpio031inttoInt13020R {
        EnblGpio031inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO031 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio031intto_sio(&self) -> EnblGpio031inttoSioR {
        EnblGpio031inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO031 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio031inttarget_rst_tolerance(&self) -> Gpio031inttargetRstToleranceR {
        Gpio031inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO031 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio031inttarget_wr_prot(&self) -> Gpio031inttargetWrProtR {
        Gpio031inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO028 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio028intto_int13018(&mut self) -> EnblGpio028inttoInt13018W<Gpioa2cSpec> {
        EnblGpio028inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO028 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio028intto_int13019(&mut self) -> EnblGpio028inttoInt13019W<Gpioa2cSpec> {
        EnblGpio028inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO028 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio028intto_int13020(&mut self) -> EnblGpio028inttoInt13020W<Gpioa2cSpec> {
        EnblGpio028inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO028 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio028intto_sio(&mut self) -> EnblGpio028inttoSioW<Gpioa2cSpec> {
        EnblGpio028inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa2cSpec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa2cSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO028 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio028inttarget_rst_tolerance(&mut self) -> Gpio028inttargetRstToleranceW<Gpioa2cSpec> {
        Gpio028inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO028 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio028inttarget_wr_prot(&mut self) -> Gpio028inttargetWrProtW<Gpioa2cSpec> {
        Gpio028inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO029 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio029intto_int13018(&mut self) -> EnblGpio029inttoInt13018W<Gpioa2cSpec> {
        EnblGpio029inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO029 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio029intto_int13019(&mut self) -> EnblGpio029inttoInt13019W<Gpioa2cSpec> {
        EnblGpio029inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO029 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio029intto_int13020(&mut self) -> EnblGpio029inttoInt13020W<Gpioa2cSpec> {
        EnblGpio029inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO029 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio029intto_sio(&mut self) -> EnblGpio029inttoSioW<Gpioa2cSpec> {
        EnblGpio029inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa2cSpec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa2cSpec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO029 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio029inttarget_rst_tolerance(&mut self) -> Gpio029inttargetRstToleranceW<Gpioa2cSpec> {
        Gpio029inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO029 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio029inttarget_wr_prot(&mut self) -> Gpio029inttargetWrProtW<Gpioa2cSpec> {
        Gpio029inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO030 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio030intto_int13018(&mut self) -> EnblGpio030inttoInt13018W<Gpioa2cSpec> {
        EnblGpio030inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO030 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio030intto_int13019(&mut self) -> EnblGpio030inttoInt13019W<Gpioa2cSpec> {
        EnblGpio030inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO030 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio030intto_int13020(&mut self) -> EnblGpio030inttoInt13020W<Gpioa2cSpec> {
        EnblGpio030inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO030 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio030intto_sio(&mut self) -> EnblGpio030inttoSioW<Gpioa2cSpec> {
        EnblGpio030inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa2cSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa2cSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO030 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio030inttarget_rst_tolerance(&mut self) -> Gpio030inttargetRstToleranceW<Gpioa2cSpec> {
        Gpio030inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO030 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio030inttarget_wr_prot(&mut self) -> Gpio030inttargetWrProtW<Gpioa2cSpec> {
        Gpio030inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO031 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio031intto_int13018(&mut self) -> EnblGpio031inttoInt13018W<Gpioa2cSpec> {
        EnblGpio031inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO031 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio031intto_int13019(&mut self) -> EnblGpio031inttoInt13019W<Gpioa2cSpec> {
        EnblGpio031inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO031 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio031intto_int13020(&mut self) -> EnblGpio031inttoInt13020W<Gpioa2cSpec> {
        EnblGpio031inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO031 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio031intto_sio(&mut self) -> EnblGpio031inttoSioW<Gpioa2cSpec> {
        EnblGpio031inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa2cSpec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO031 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio031inttarget_rst_tolerance(&mut self) -> Gpio031inttargetRstToleranceW<Gpioa2cSpec> {
        Gpio031inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO031 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio031inttarget_wr_prot(&mut self) -> Gpio031inttargetWrProtW<Gpioa2cSpec> {
        Gpio031inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa2cSpec;
impl crate::RegisterSpec for Gpioa2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa2c::R`](R) reader structure"]
impl crate::Readable for Gpioa2cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioa2c::W`](W) writer structure"]
impl crate::Writable for Gpioa2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA2C to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa2cSpec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
