#[doc = "Register `GPIOA60` reader"]
pub type R = crate::R<Gpioa60Spec>;
#[doc = "Register `GPIOA60` writer"]
pub type W = crate::W<Gpioa60Spec>;
#[doc = "Field `EnblGPIO080INTToINT13018` reader - Enable GPIO080 Interrupt To INT#130_18"]
pub type EnblGpio080inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO080INTToINT13018` writer - Enable GPIO080 Interrupt To INT#130_18"]
pub type EnblGpio080inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO080INTToINT13019` reader - Enable GPIO080 Interrupt To INT#130_19"]
pub type EnblGpio080inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO080INTToINT13019` writer - Enable GPIO080 Interrupt To INT#130_19"]
pub type EnblGpio080inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO080INTToINT13020` reader - Enable GPIO080 Interrupt To INT#130_20"]
pub type EnblGpio080inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO080INTToINT13020` writer - Enable GPIO080 Interrupt To INT#130_20"]
pub type EnblGpio080inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO080INTToSIO` reader - Enable GPIO080 Interrupt To SIO"]
pub type EnblGpio080inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO080INTToSIO` writer - Enable GPIO080 Interrupt To SIO"]
pub type EnblGpio080inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO080 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio080inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio080inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio080inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO080INTTargetRstTolerance` reader - GPIO080 Interrupt Target Reset Tolerance"]
pub type Gpio080inttargetRstToleranceR = crate::BitReader<Gpio080inttargetRstTolerance>;
impl Gpio080inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio080inttargetRstTolerance {
        match self.bits {
            false => Gpio080inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio080inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio080inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio080inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO080INTTargetRstTolerance` writer - GPIO080 Interrupt Target Reset Tolerance"]
pub type Gpio080inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio080inttargetRstTolerance>;
impl<'a, REG> Gpio080inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio080inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio080inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO080INTTargetWrProt` reader - GPIO080 Interrupt Target Write Protection"]
pub type Gpio080inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO080INTTargetWrProt` writer - GPIO080 Interrupt Target Write Protection"]
pub type Gpio080inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO081INTToINT13018` reader - Enable GPIO081 Interrupt To INT#130_18"]
pub type EnblGpio081inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO081INTToINT13018` writer - Enable GPIO081 Interrupt To INT#130_18"]
pub type EnblGpio081inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO081INTToINT13019` reader - Enable GPIO081 Interrupt To INT#130_19"]
pub type EnblGpio081inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO081INTToINT13019` writer - Enable GPIO081 Interrupt To INT#130_19"]
pub type EnblGpio081inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO081INTToINT13020` reader - Enable GPIO081 Interrupt To INT#130_20"]
pub type EnblGpio081inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO081INTToINT13020` writer - Enable GPIO081 Interrupt To INT#130_20"]
pub type EnblGpio081inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO081INTToSIO` reader - Enable GPIO081 Interrupt To SIO"]
pub type EnblGpio081inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO081INTToSIO` writer - Enable GPIO081 Interrupt To SIO"]
pub type EnblGpio081inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO081 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio081inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio081inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio081inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO081INTTargetRstTolerance` reader - GPIO081 Interrupt Target Reset Tolerance"]
pub type Gpio081inttargetRstToleranceR = crate::BitReader<Gpio081inttargetRstTolerance>;
impl Gpio081inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio081inttargetRstTolerance {
        match self.bits {
            false => Gpio081inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio081inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio081inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio081inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO081INTTargetRstTolerance` writer - GPIO081 Interrupt Target Reset Tolerance"]
pub type Gpio081inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio081inttargetRstTolerance>;
impl<'a, REG> Gpio081inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio081inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio081inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO081INTTargetWrProt` reader - GPIO081 Interrupt Target Write Protection"]
pub type Gpio081inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO081INTTargetWrProt` writer - GPIO081 Interrupt Target Write Protection"]
pub type Gpio081inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO082INTToINT13018` reader - Enable GPIO082 Interrupt To INT#130_18"]
pub type EnblGpio082inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO082INTToINT13018` writer - Enable GPIO082 Interrupt To INT#130_18"]
pub type EnblGpio082inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO082INTToINT13019` reader - Enable GPIO082 Interrupt To INT#130_19"]
pub type EnblGpio082inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO082INTToINT13019` writer - Enable GPIO082 Interrupt To INT#130_19"]
pub type EnblGpio082inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO082INTToINT13020` reader - Enable GPIO082 Interrupt To INT#130_20"]
pub type EnblGpio082inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO082INTToINT13020` writer - Enable GPIO082 Interrupt To INT#130_20"]
pub type EnblGpio082inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO082INTToSIO` reader - Enable GPIO082 Interrupt To SIO"]
pub type EnblGpio082inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO082INTToSIO` writer - Enable GPIO082 Interrupt To SIO"]
pub type EnblGpio082inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO082 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio082inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio082inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio082inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO082INTTargetRstTolerance` reader - GPIO082 Interrupt Target Reset Tolerance"]
pub type Gpio082inttargetRstToleranceR = crate::BitReader<Gpio082inttargetRstTolerance>;
impl Gpio082inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio082inttargetRstTolerance {
        match self.bits {
            false => Gpio082inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio082inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio082inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio082inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO082INTTargetRstTolerance` writer - GPIO082 Interrupt Target Reset Tolerance"]
pub type Gpio082inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio082inttargetRstTolerance>;
impl<'a, REG> Gpio082inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio082inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio082inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO082INTTargetWrProt` reader - GPIO082 Interrupt Target Write Protection"]
pub type Gpio082inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO082INTTargetWrProt` writer - GPIO082 Interrupt Target Write Protection"]
pub type Gpio082inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO083INTToINT13018` reader - Enable GPIO083 Interrupt To INT#130_18"]
pub type EnblGpio083inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO083INTToINT13018` writer - Enable GPIO083 Interrupt To INT#130_18"]
pub type EnblGpio083inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO083INTToINT13019` reader - Enable GPIO083 Interrupt To INT#130_19"]
pub type EnblGpio083inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO083INTToINT13019` writer - Enable GPIO083 Interrupt To INT#130_19"]
pub type EnblGpio083inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO083INTToINT13020` reader - Enable GPIO083 Interrupt To INT#130_20"]
pub type EnblGpio083inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO083INTToINT13020` writer - Enable GPIO083 Interrupt To INT#130_20"]
pub type EnblGpio083inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO083INTToSIO` reader - Enable GPIO083 Interrupt To SIO"]
pub type EnblGpio083inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO083INTToSIO` writer - Enable GPIO083 Interrupt To SIO"]
pub type EnblGpio083inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO083 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio083inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio083inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio083inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO083INTTargetRstTolerance` reader - GPIO083 Interrupt Target Reset Tolerance"]
pub type Gpio083inttargetRstToleranceR = crate::BitReader<Gpio083inttargetRstTolerance>;
impl Gpio083inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio083inttargetRstTolerance {
        match self.bits {
            false => Gpio083inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio083inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio083inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio083inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO083INTTargetRstTolerance` writer - GPIO083 Interrupt Target Reset Tolerance"]
pub type Gpio083inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio083inttargetRstTolerance>;
impl<'a, REG> Gpio083inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio083inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio083inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO083INTTargetWrProt` reader - GPIO083 Interrupt Target Write Protection"]
pub type Gpio083inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO083INTTargetWrProt` writer - GPIO083 Interrupt Target Write Protection"]
pub type Gpio083inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO080 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio080intto_int13018(&self) -> EnblGpio080inttoInt13018R {
        EnblGpio080inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO080 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio080intto_int13019(&self) -> EnblGpio080inttoInt13019R {
        EnblGpio080inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO080 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio080intto_int13020(&self) -> EnblGpio080inttoInt13020R {
        EnblGpio080inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO080 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio080intto_sio(&self) -> EnblGpio080inttoSioR {
        EnblGpio080inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO080 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio080inttarget_rst_tolerance(&self) -> Gpio080inttargetRstToleranceR {
        Gpio080inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO080 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio080inttarget_wr_prot(&self) -> Gpio080inttargetWrProtR {
        Gpio080inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO081 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio081intto_int13018(&self) -> EnblGpio081inttoInt13018R {
        EnblGpio081inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO081 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio081intto_int13019(&self) -> EnblGpio081inttoInt13019R {
        EnblGpio081inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO081 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio081intto_int13020(&self) -> EnblGpio081inttoInt13020R {
        EnblGpio081inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO081 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio081intto_sio(&self) -> EnblGpio081inttoSioR {
        EnblGpio081inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO081 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio081inttarget_rst_tolerance(&self) -> Gpio081inttargetRstToleranceR {
        Gpio081inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO081 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio081inttarget_wr_prot(&self) -> Gpio081inttargetWrProtR {
        Gpio081inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO082 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio082intto_int13018(&self) -> EnblGpio082inttoInt13018R {
        EnblGpio082inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO082 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio082intto_int13019(&self) -> EnblGpio082inttoInt13019R {
        EnblGpio082inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO082 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio082intto_int13020(&self) -> EnblGpio082inttoInt13020R {
        EnblGpio082inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO082 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio082intto_sio(&self) -> EnblGpio082inttoSioR {
        EnblGpio082inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO082 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio082inttarget_rst_tolerance(&self) -> Gpio082inttargetRstToleranceR {
        Gpio082inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO082 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio082inttarget_wr_prot(&self) -> Gpio082inttargetWrProtR {
        Gpio082inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO083 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio083intto_int13018(&self) -> EnblGpio083inttoInt13018R {
        EnblGpio083inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO083 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio083intto_int13019(&self) -> EnblGpio083inttoInt13019R {
        EnblGpio083inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO083 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio083intto_int13020(&self) -> EnblGpio083inttoInt13020R {
        EnblGpio083inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO083 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio083intto_sio(&self) -> EnblGpio083inttoSioR {
        EnblGpio083inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO083 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio083inttarget_rst_tolerance(&self) -> Gpio083inttargetRstToleranceR {
        Gpio083inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO083 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio083inttarget_wr_prot(&self) -> Gpio083inttargetWrProtR {
        Gpio083inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO080 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio080intto_int13018(&mut self) -> EnblGpio080inttoInt13018W<Gpioa60Spec> {
        EnblGpio080inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO080 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio080intto_int13019(&mut self) -> EnblGpio080inttoInt13019W<Gpioa60Spec> {
        EnblGpio080inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO080 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio080intto_int13020(&mut self) -> EnblGpio080inttoInt13020W<Gpioa60Spec> {
        EnblGpio080inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO080 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio080intto_sio(&mut self) -> EnblGpio080inttoSioW<Gpioa60Spec> {
        EnblGpio080inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa60Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa60Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO080 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio080inttarget_rst_tolerance(&mut self) -> Gpio080inttargetRstToleranceW<Gpioa60Spec> {
        Gpio080inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO080 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio080inttarget_wr_prot(&mut self) -> Gpio080inttargetWrProtW<Gpioa60Spec> {
        Gpio080inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO081 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio081intto_int13018(&mut self) -> EnblGpio081inttoInt13018W<Gpioa60Spec> {
        EnblGpio081inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO081 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio081intto_int13019(&mut self) -> EnblGpio081inttoInt13019W<Gpioa60Spec> {
        EnblGpio081inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO081 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio081intto_int13020(&mut self) -> EnblGpio081inttoInt13020W<Gpioa60Spec> {
        EnblGpio081inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO081 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio081intto_sio(&mut self) -> EnblGpio081inttoSioW<Gpioa60Spec> {
        EnblGpio081inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa60Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa60Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO081 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio081inttarget_rst_tolerance(&mut self) -> Gpio081inttargetRstToleranceW<Gpioa60Spec> {
        Gpio081inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO081 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio081inttarget_wr_prot(&mut self) -> Gpio081inttargetWrProtW<Gpioa60Spec> {
        Gpio081inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO082 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio082intto_int13018(&mut self) -> EnblGpio082inttoInt13018W<Gpioa60Spec> {
        EnblGpio082inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO082 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio082intto_int13019(&mut self) -> EnblGpio082inttoInt13019W<Gpioa60Spec> {
        EnblGpio082inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO082 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio082intto_int13020(&mut self) -> EnblGpio082inttoInt13020W<Gpioa60Spec> {
        EnblGpio082inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO082 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio082intto_sio(&mut self) -> EnblGpio082inttoSioW<Gpioa60Spec> {
        EnblGpio082inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa60Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa60Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO082 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio082inttarget_rst_tolerance(&mut self) -> Gpio082inttargetRstToleranceW<Gpioa60Spec> {
        Gpio082inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO082 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio082inttarget_wr_prot(&mut self) -> Gpio082inttargetWrProtW<Gpioa60Spec> {
        Gpio082inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO083 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio083intto_int13018(&mut self) -> EnblGpio083inttoInt13018W<Gpioa60Spec> {
        EnblGpio083inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO083 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio083intto_int13019(&mut self) -> EnblGpio083inttoInt13019W<Gpioa60Spec> {
        EnblGpio083inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO083 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio083intto_int13020(&mut self) -> EnblGpio083inttoInt13020W<Gpioa60Spec> {
        EnblGpio083inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO083 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio083intto_sio(&mut self) -> EnblGpio083inttoSioW<Gpioa60Spec> {
        EnblGpio083inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa60Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO083 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio083inttarget_rst_tolerance(&mut self) -> Gpio083inttargetRstToleranceW<Gpioa60Spec> {
        Gpio083inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO083 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio083inttarget_wr_prot(&mut self) -> Gpio083inttargetWrProtW<Gpioa60Spec> {
        Gpio083inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa60Spec;
impl crate::RegisterSpec for Gpioa60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa60::R`](R) reader structure"]
impl crate::Readable for Gpioa60Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa60::W`](W) writer structure"]
impl crate::Writable for Gpioa60Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA60 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa60Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
