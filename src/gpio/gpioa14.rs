#[doc = "Register `GPIOA14` reader"]
pub type R = crate::R<Gpioa14Spec>;
#[doc = "Register `GPIOA14` writer"]
pub type W = crate::W<Gpioa14Spec>;
#[doc = "Field `EnblGPIO004INTToINT13018` reader - Enable GPIO004 Interrupt To INT#130_18"]
pub type EnblGpio004inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO004INTToINT13018` writer - Enable GPIO004 Interrupt To INT#130_18"]
pub type EnblGpio004inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO004INTToINT13019` reader - Enable GPIO004 Interrupt To INT#130_19"]
pub type EnblGpio004inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO004INTToINT13019` writer - Enable GPIO004 Interrupt To INT#130_19"]
pub type EnblGpio004inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO004INTToINT13020` reader - Enable GPIO004 Interrupt To INT#130_20"]
pub type EnblGpio004inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO004INTToINT13020` writer - Enable GPIO004 Interrupt To INT#130_20"]
pub type EnblGpio004inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO004INTToSIO` reader - Enable GPIO004 Interrupt To SIO"]
pub type EnblGpio004inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO004INTToSIO` writer - Enable GPIO004 Interrupt To SIO"]
pub type EnblGpio004inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO004 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio004inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio004inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio004inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO004INTTargetRstTolerance` reader - GPIO004 Interrupt Target Reset Tolerance"]
pub type Gpio004inttargetRstToleranceR = crate::BitReader<Gpio004inttargetRstTolerance>;
impl Gpio004inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio004inttargetRstTolerance {
        match self.bits {
            false => Gpio004inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio004inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio004inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio004inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO004INTTargetRstTolerance` writer - GPIO004 Interrupt Target Reset Tolerance"]
pub type Gpio004inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio004inttargetRstTolerance>;
impl<'a, REG> Gpio004inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio004inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio004inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO004INTTargetWrProt` reader - GPIO004 Interrupt Target Write Protection"]
pub type Gpio004inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO004INTTargetWrProt` writer - GPIO004 Interrupt Target Write Protection"]
pub type Gpio004inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO005INTToINT13018` reader - Enable GPIO005 Interrupt To INT#130_18"]
pub type EnblGpio005inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO005INTToINT13018` writer - Enable GPIO005 Interrupt To INT#130_18"]
pub type EnblGpio005inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO005INTToINT13019` reader - Enable GPIO005 Interrupt To INT#130_19"]
pub type EnblGpio005inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO005INTToINT13019` writer - Enable GPIO005 Interrupt To INT#130_19"]
pub type EnblGpio005inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO005INTToINT13020` reader - Enable GPIO005 Interrupt To INT#130_20"]
pub type EnblGpio005inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO005INTToINT13020` writer - Enable GPIO005 Interrupt To INT#130_20"]
pub type EnblGpio005inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO005INTToSIO` reader - Enable GPIO005 Interrupt To SIO"]
pub type EnblGpio005inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO005INTToSIO` writer - Enable GPIO005 Interrupt To SIO"]
pub type EnblGpio005inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO005 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio005inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio005inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio005inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO005INTTargetRstTolerance` reader - GPIO005 Interrupt Target Reset Tolerance"]
pub type Gpio005inttargetRstToleranceR = crate::BitReader<Gpio005inttargetRstTolerance>;
impl Gpio005inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio005inttargetRstTolerance {
        match self.bits {
            false => Gpio005inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio005inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio005inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio005inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO005INTTargetRstTolerance` writer - GPIO005 Interrupt Target Reset Tolerance"]
pub type Gpio005inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio005inttargetRstTolerance>;
impl<'a, REG> Gpio005inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio005inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio005inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO005INTTargetWrProt` reader - GPIO005 Interrupt Target Write Protection"]
pub type Gpio005inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO005INTTargetWrProt` writer - GPIO005 Interrupt Target Write Protection"]
pub type Gpio005inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO006INTToINT13018` reader - Enable GPIO006 Interrupt To INT#130_18"]
pub type EnblGpio006inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO006INTToINT13018` writer - Enable GPIO006 Interrupt To INT#130_18"]
pub type EnblGpio006inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO006INTToINT13019` reader - Enable GPIO006 Interrupt To INT#130_19"]
pub type EnblGpio006inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO006INTToINT13019` writer - Enable GPIO006 Interrupt To INT#130_19"]
pub type EnblGpio006inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO006INTToINT13020` reader - Enable GPIO006 Interrupt To INT#130_20"]
pub type EnblGpio006inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO006INTToINT13020` writer - Enable GPIO006 Interrupt To INT#130_20"]
pub type EnblGpio006inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO006INTToSIO` reader - Enable GPIO006 Interrupt To SIO"]
pub type EnblGpio006inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO006INTToSIO` writer - Enable GPIO006 Interrupt To SIO"]
pub type EnblGpio006inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO006 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio006inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio006inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio006inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO006INTTargetRstTolerance` reader - GPIO006 Interrupt Target Reset Tolerance"]
pub type Gpio006inttargetRstToleranceR = crate::BitReader<Gpio006inttargetRstTolerance>;
impl Gpio006inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio006inttargetRstTolerance {
        match self.bits {
            false => Gpio006inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio006inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio006inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio006inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO006INTTargetRstTolerance` writer - GPIO006 Interrupt Target Reset Tolerance"]
pub type Gpio006inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio006inttargetRstTolerance>;
impl<'a, REG> Gpio006inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio006inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio006inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO006INTTargetWrProt` reader - GPIO006 Interrupt Target Write Protection"]
pub type Gpio006inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO006INTTargetWrProt` writer - GPIO006 Interrupt Target Write Protection"]
pub type Gpio006inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO007INTToINT13018` reader - Enable GPIO007 Interrupt To INT#130_18"]
pub type EnblGpio007inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO007INTToINT13018` writer - Enable GPIO007 Interrupt To INT#130_18"]
pub type EnblGpio007inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO007INTToINT13019` reader - Enable GPIO007 Interrupt To INT#130_19"]
pub type EnblGpio007inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO007INTToINT13019` writer - Enable GPIO007 Interrupt To INT#130_19"]
pub type EnblGpio007inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO007INTToINT13020` reader - Enable GPIO007 Interrupt To INT#130_20"]
pub type EnblGpio007inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO007INTToINT13020` writer - Enable GPIO007 Interrupt To INT#130_20"]
pub type EnblGpio007inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO007INTToSIO` reader - Enable GPIO007 Interrupt To SIO"]
pub type EnblGpio007inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO007INTToSIO` writer - Enable GPIO007 Interrupt To SIO"]
pub type EnblGpio007inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO007 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio007inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio007inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio007inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO007INTTargetRstTolerance` reader - GPIO007 Interrupt Target Reset Tolerance"]
pub type Gpio007inttargetRstToleranceR = crate::BitReader<Gpio007inttargetRstTolerance>;
impl Gpio007inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio007inttargetRstTolerance {
        match self.bits {
            false => Gpio007inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio007inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio007inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio007inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO007INTTargetRstTolerance` writer - GPIO007 Interrupt Target Reset Tolerance"]
pub type Gpio007inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio007inttargetRstTolerance>;
impl<'a, REG> Gpio007inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio007inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio007inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO007INTTargetWrProt` reader - GPIO007 Interrupt Target Write Protection"]
pub type Gpio007inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO007INTTargetWrProt` writer - GPIO007 Interrupt Target Write Protection"]
pub type Gpio007inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO004 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio004intto_int13018(&self) -> EnblGpio004inttoInt13018R {
        EnblGpio004inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO004 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio004intto_int13019(&self) -> EnblGpio004inttoInt13019R {
        EnblGpio004inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO004 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio004intto_int13020(&self) -> EnblGpio004inttoInt13020R {
        EnblGpio004inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO004 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio004intto_sio(&self) -> EnblGpio004inttoSioR {
        EnblGpio004inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO004 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio004inttarget_rst_tolerance(&self) -> Gpio004inttargetRstToleranceR {
        Gpio004inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO004 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio004inttarget_wr_prot(&self) -> Gpio004inttargetWrProtR {
        Gpio004inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO005 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio005intto_int13018(&self) -> EnblGpio005inttoInt13018R {
        EnblGpio005inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO005 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio005intto_int13019(&self) -> EnblGpio005inttoInt13019R {
        EnblGpio005inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO005 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio005intto_int13020(&self) -> EnblGpio005inttoInt13020R {
        EnblGpio005inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO005 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio005intto_sio(&self) -> EnblGpio005inttoSioR {
        EnblGpio005inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO005 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio005inttarget_rst_tolerance(&self) -> Gpio005inttargetRstToleranceR {
        Gpio005inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO005 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio005inttarget_wr_prot(&self) -> Gpio005inttargetWrProtR {
        Gpio005inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO006 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio006intto_int13018(&self) -> EnblGpio006inttoInt13018R {
        EnblGpio006inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO006 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio006intto_int13019(&self) -> EnblGpio006inttoInt13019R {
        EnblGpio006inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO006 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio006intto_int13020(&self) -> EnblGpio006inttoInt13020R {
        EnblGpio006inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO006 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio006intto_sio(&self) -> EnblGpio006inttoSioR {
        EnblGpio006inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO006 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio006inttarget_rst_tolerance(&self) -> Gpio006inttargetRstToleranceR {
        Gpio006inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO006 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio006inttarget_wr_prot(&self) -> Gpio006inttargetWrProtR {
        Gpio006inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO007 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio007intto_int13018(&self) -> EnblGpio007inttoInt13018R {
        EnblGpio007inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO007 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio007intto_int13019(&self) -> EnblGpio007inttoInt13019R {
        EnblGpio007inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO007 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio007intto_int13020(&self) -> EnblGpio007inttoInt13020R {
        EnblGpio007inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO007 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio007intto_sio(&self) -> EnblGpio007inttoSioR {
        EnblGpio007inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO007 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio007inttarget_rst_tolerance(&self) -> Gpio007inttargetRstToleranceR {
        Gpio007inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO007 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio007inttarget_wr_prot(&self) -> Gpio007inttargetWrProtR {
        Gpio007inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO004 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio004intto_int13018(&mut self) -> EnblGpio004inttoInt13018W<Gpioa14Spec> {
        EnblGpio004inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO004 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio004intto_int13019(&mut self) -> EnblGpio004inttoInt13019W<Gpioa14Spec> {
        EnblGpio004inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO004 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio004intto_int13020(&mut self) -> EnblGpio004inttoInt13020W<Gpioa14Spec> {
        EnblGpio004inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO004 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio004intto_sio(&mut self) -> EnblGpio004inttoSioW<Gpioa14Spec> {
        EnblGpio004inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa14Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa14Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO004 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio004inttarget_rst_tolerance(&mut self) -> Gpio004inttargetRstToleranceW<Gpioa14Spec> {
        Gpio004inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO004 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio004inttarget_wr_prot(&mut self) -> Gpio004inttargetWrProtW<Gpioa14Spec> {
        Gpio004inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO005 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio005intto_int13018(&mut self) -> EnblGpio005inttoInt13018W<Gpioa14Spec> {
        EnblGpio005inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO005 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio005intto_int13019(&mut self) -> EnblGpio005inttoInt13019W<Gpioa14Spec> {
        EnblGpio005inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO005 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio005intto_int13020(&mut self) -> EnblGpio005inttoInt13020W<Gpioa14Spec> {
        EnblGpio005inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO005 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio005intto_sio(&mut self) -> EnblGpio005inttoSioW<Gpioa14Spec> {
        EnblGpio005inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa14Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa14Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO005 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio005inttarget_rst_tolerance(&mut self) -> Gpio005inttargetRstToleranceW<Gpioa14Spec> {
        Gpio005inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO005 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio005inttarget_wr_prot(&mut self) -> Gpio005inttargetWrProtW<Gpioa14Spec> {
        Gpio005inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO006 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio006intto_int13018(&mut self) -> EnblGpio006inttoInt13018W<Gpioa14Spec> {
        EnblGpio006inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO006 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio006intto_int13019(&mut self) -> EnblGpio006inttoInt13019W<Gpioa14Spec> {
        EnblGpio006inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO006 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio006intto_int13020(&mut self) -> EnblGpio006inttoInt13020W<Gpioa14Spec> {
        EnblGpio006inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO006 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio006intto_sio(&mut self) -> EnblGpio006inttoSioW<Gpioa14Spec> {
        EnblGpio006inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa14Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa14Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO006 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio006inttarget_rst_tolerance(&mut self) -> Gpio006inttargetRstToleranceW<Gpioa14Spec> {
        Gpio006inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO006 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio006inttarget_wr_prot(&mut self) -> Gpio006inttargetWrProtW<Gpioa14Spec> {
        Gpio006inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO007 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio007intto_int13018(&mut self) -> EnblGpio007inttoInt13018W<Gpioa14Spec> {
        EnblGpio007inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO007 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio007intto_int13019(&mut self) -> EnblGpio007inttoInt13019W<Gpioa14Spec> {
        EnblGpio007inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO007 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio007intto_int13020(&mut self) -> EnblGpio007inttoInt13020W<Gpioa14Spec> {
        EnblGpio007inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO007 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio007intto_sio(&mut self) -> EnblGpio007inttoSioW<Gpioa14Spec> {
        EnblGpio007inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa14Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO007 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio007inttarget_rst_tolerance(&mut self) -> Gpio007inttargetRstToleranceW<Gpioa14Spec> {
        Gpio007inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO007 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio007inttarget_wr_prot(&mut self) -> Gpio007inttargetWrProtW<Gpioa14Spec> {
        Gpio007inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa14Spec;
impl crate::RegisterSpec for Gpioa14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa14::R`](R) reader structure"]
impl crate::Readable for Gpioa14Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa14::W`](W) writer structure"]
impl crate::Writable for Gpioa14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA14 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa14Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
