#[doc = "Register `GPIOAB8` reader"]
pub type R = crate::R<Gpioab8Spec>;
#[doc = "Register `GPIOAB8` writer"]
pub type W = crate::W<Gpioab8Spec>;
#[doc = "Field `EnblGPIO168INTToINT13018` reader - Enable GPIO168 Interrupt To INT#130_18"]
pub type EnblGpio168inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO168INTToINT13018` writer - Enable GPIO168 Interrupt To INT#130_18"]
pub type EnblGpio168inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO168INTToINT13019` reader - Enable GPIO168 Interrupt To INT#130_19"]
pub type EnblGpio168inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO168INTToINT13019` writer - Enable GPIO168 Interrupt To INT#130_19"]
pub type EnblGpio168inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO168INTToINT13020` reader - Enable GPIO168 Interrupt To INT#130_20"]
pub type EnblGpio168inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO168INTToINT13020` writer - Enable GPIO168 Interrupt To INT#130_20"]
pub type EnblGpio168inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO168INTToSIO` reader - Enable GPIO168 Interrupt To SIO"]
pub type EnblGpio168inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO168INTToSIO` writer - Enable GPIO168 Interrupt To SIO"]
pub type EnblGpio168inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO168 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio168inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio168inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio168inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO168INTTargetRstTolerance` reader - GPIO168 Interrupt Target Reset Tolerance"]
pub type Gpio168inttargetRstToleranceR = crate::BitReader<Gpio168inttargetRstTolerance>;
impl Gpio168inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio168inttargetRstTolerance {
        match self.bits {
            false => Gpio168inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio168inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio168inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio168inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO168INTTargetRstTolerance` writer - GPIO168 Interrupt Target Reset Tolerance"]
pub type Gpio168inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio168inttargetRstTolerance>;
impl<'a, REG> Gpio168inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio168inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio168inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO168INTTargetWrProt` reader - GPIO168 Interrupt Target Write Protection"]
pub type Gpio168inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO168INTTargetWrProt` writer - GPIO168 Interrupt Target Write Protection"]
pub type Gpio168inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO169INTToINT13018` reader - Enable GPIO169 Interrupt To INT#130_18"]
pub type EnblGpio169inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO169INTToINT13018` writer - Enable GPIO169 Interrupt To INT#130_18"]
pub type EnblGpio169inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO169INTToINT13019` reader - Enable GPIO169 Interrupt To INT#130_19"]
pub type EnblGpio169inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO169INTToINT13019` writer - Enable GPIO169 Interrupt To INT#130_19"]
pub type EnblGpio169inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO169INTToINT13020` reader - Enable GPIO169 Interrupt To INT#130_20"]
pub type EnblGpio169inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO169INTToINT13020` writer - Enable GPIO169 Interrupt To INT#130_20"]
pub type EnblGpio169inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO169INTToSIO` reader - Enable GPIO169 Interrupt To SIO"]
pub type EnblGpio169inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO169INTToSIO` writer - Enable GPIO169 Interrupt To SIO"]
pub type EnblGpio169inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO169 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio169inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio169inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio169inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO169INTTargetRstTolerance` reader - GPIO169 Interrupt Target Reset Tolerance"]
pub type Gpio169inttargetRstToleranceR = crate::BitReader<Gpio169inttargetRstTolerance>;
impl Gpio169inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio169inttargetRstTolerance {
        match self.bits {
            false => Gpio169inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio169inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio169inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio169inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO169INTTargetRstTolerance` writer - GPIO169 Interrupt Target Reset Tolerance"]
pub type Gpio169inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio169inttargetRstTolerance>;
impl<'a, REG> Gpio169inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio169inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio169inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO169INTTargetWrProt` reader - GPIO169 Interrupt Target Write Protection"]
pub type Gpio169inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO169INTTargetWrProt` writer - GPIO169 Interrupt Target Write Protection"]
pub type Gpio169inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO170INTToINT13018` reader - Enable GPIO170 Interrupt To INT#130_18"]
pub type EnblGpio170inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO170INTToINT13018` writer - Enable GPIO170 Interrupt To INT#130_18"]
pub type EnblGpio170inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO170INTToINT13019` reader - Enable GPIO170 Interrupt To INT#130_19"]
pub type EnblGpio170inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO170INTToINT13019` writer - Enable GPIO170 Interrupt To INT#130_19"]
pub type EnblGpio170inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO170INTToINT13020` reader - Enable GPIO170 Interrupt To INT#130_20"]
pub type EnblGpio170inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO170INTToINT13020` writer - Enable GPIO170 Interrupt To INT#130_20"]
pub type EnblGpio170inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO170INTToSIO` reader - Enable GPIO170 Interrupt To SIO"]
pub type EnblGpio170inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO170INTToSIO` writer - Enable GPIO170 Interrupt To SIO"]
pub type EnblGpio170inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO170 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio170inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio170inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio170inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO170INTTargetRstTolerance` reader - GPIO170 Interrupt Target Reset Tolerance"]
pub type Gpio170inttargetRstToleranceR = crate::BitReader<Gpio170inttargetRstTolerance>;
impl Gpio170inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio170inttargetRstTolerance {
        match self.bits {
            false => Gpio170inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio170inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio170inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio170inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO170INTTargetRstTolerance` writer - GPIO170 Interrupt Target Reset Tolerance"]
pub type Gpio170inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio170inttargetRstTolerance>;
impl<'a, REG> Gpio170inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio170inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio170inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO170INTTargetWrProt` reader - GPIO170 Interrupt Target Write Protection"]
pub type Gpio170inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO170INTTargetWrProt` writer - GPIO170 Interrupt Target Write Protection"]
pub type Gpio170inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO171INTToINT13018` reader - Enable GPIO171 Interrupt To INT#130_18"]
pub type EnblGpio171inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO171INTToINT13018` writer - Enable GPIO171 Interrupt To INT#130_18"]
pub type EnblGpio171inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO171INTToINT13019` reader - Enable GPIO171 Interrupt To INT#130_19"]
pub type EnblGpio171inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO171INTToINT13019` writer - Enable GPIO171 Interrupt To INT#130_19"]
pub type EnblGpio171inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO171INTToINT13020` reader - Enable GPIO171 Interrupt To INT#130_20"]
pub type EnblGpio171inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO171INTToINT13020` writer - Enable GPIO171 Interrupt To INT#130_20"]
pub type EnblGpio171inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO171INTToSIO` reader - Enable GPIO171 Interrupt To SIO"]
pub type EnblGpio171inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO171INTToSIO` writer - Enable GPIO171 Interrupt To SIO"]
pub type EnblGpio171inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO171 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio171inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio171inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio171inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO171INTTargetRstTolerance` reader - GPIO171 Interrupt Target Reset Tolerance"]
pub type Gpio171inttargetRstToleranceR = crate::BitReader<Gpio171inttargetRstTolerance>;
impl Gpio171inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio171inttargetRstTolerance {
        match self.bits {
            false => Gpio171inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio171inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio171inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio171inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO171INTTargetRstTolerance` writer - GPIO171 Interrupt Target Reset Tolerance"]
pub type Gpio171inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio171inttargetRstTolerance>;
impl<'a, REG> Gpio171inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio171inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio171inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO171INTTargetWrProt` reader - GPIO171 Interrupt Target Write Protection"]
pub type Gpio171inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO171INTTargetWrProt` writer - GPIO171 Interrupt Target Write Protection"]
pub type Gpio171inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO168 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio168intto_int13018(&self) -> EnblGpio168inttoInt13018R {
        EnblGpio168inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO168 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio168intto_int13019(&self) -> EnblGpio168inttoInt13019R {
        EnblGpio168inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO168 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio168intto_int13020(&self) -> EnblGpio168inttoInt13020R {
        EnblGpio168inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO168 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio168intto_sio(&self) -> EnblGpio168inttoSioR {
        EnblGpio168inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO168 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio168inttarget_rst_tolerance(&self) -> Gpio168inttargetRstToleranceR {
        Gpio168inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO168 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio168inttarget_wr_prot(&self) -> Gpio168inttargetWrProtR {
        Gpio168inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO169 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio169intto_int13018(&self) -> EnblGpio169inttoInt13018R {
        EnblGpio169inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO169 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio169intto_int13019(&self) -> EnblGpio169inttoInt13019R {
        EnblGpio169inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO169 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio169intto_int13020(&self) -> EnblGpio169inttoInt13020R {
        EnblGpio169inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO169 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio169intto_sio(&self) -> EnblGpio169inttoSioR {
        EnblGpio169inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO169 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio169inttarget_rst_tolerance(&self) -> Gpio169inttargetRstToleranceR {
        Gpio169inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO169 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio169inttarget_wr_prot(&self) -> Gpio169inttargetWrProtR {
        Gpio169inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO170 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio170intto_int13018(&self) -> EnblGpio170inttoInt13018R {
        EnblGpio170inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO170 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio170intto_int13019(&self) -> EnblGpio170inttoInt13019R {
        EnblGpio170inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO170 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio170intto_int13020(&self) -> EnblGpio170inttoInt13020R {
        EnblGpio170inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO170 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio170intto_sio(&self) -> EnblGpio170inttoSioR {
        EnblGpio170inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO170 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio170inttarget_rst_tolerance(&self) -> Gpio170inttargetRstToleranceR {
        Gpio170inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO170 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio170inttarget_wr_prot(&self) -> Gpio170inttargetWrProtR {
        Gpio170inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO171 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio171intto_int13018(&self) -> EnblGpio171inttoInt13018R {
        EnblGpio171inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO171 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio171intto_int13019(&self) -> EnblGpio171inttoInt13019R {
        EnblGpio171inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO171 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio171intto_int13020(&self) -> EnblGpio171inttoInt13020R {
        EnblGpio171inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO171 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio171intto_sio(&self) -> EnblGpio171inttoSioR {
        EnblGpio171inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO171 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio171inttarget_rst_tolerance(&self) -> Gpio171inttargetRstToleranceR {
        Gpio171inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO171 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio171inttarget_wr_prot(&self) -> Gpio171inttargetWrProtR {
        Gpio171inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO168 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio168intto_int13018(&mut self) -> EnblGpio168inttoInt13018W<Gpioab8Spec> {
        EnblGpio168inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO168 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio168intto_int13019(&mut self) -> EnblGpio168inttoInt13019W<Gpioab8Spec> {
        EnblGpio168inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO168 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio168intto_int13020(&mut self) -> EnblGpio168inttoInt13020W<Gpioab8Spec> {
        EnblGpio168inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO168 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio168intto_sio(&mut self) -> EnblGpio168inttoSioW<Gpioab8Spec> {
        EnblGpio168inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioab8Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioab8Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO168 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio168inttarget_rst_tolerance(&mut self) -> Gpio168inttargetRstToleranceW<Gpioab8Spec> {
        Gpio168inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO168 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio168inttarget_wr_prot(&mut self) -> Gpio168inttargetWrProtW<Gpioab8Spec> {
        Gpio168inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO169 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio169intto_int13018(&mut self) -> EnblGpio169inttoInt13018W<Gpioab8Spec> {
        EnblGpio169inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO169 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio169intto_int13019(&mut self) -> EnblGpio169inttoInt13019W<Gpioab8Spec> {
        EnblGpio169inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO169 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio169intto_int13020(&mut self) -> EnblGpio169inttoInt13020W<Gpioab8Spec> {
        EnblGpio169inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO169 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio169intto_sio(&mut self) -> EnblGpio169inttoSioW<Gpioab8Spec> {
        EnblGpio169inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioab8Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioab8Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO169 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio169inttarget_rst_tolerance(&mut self) -> Gpio169inttargetRstToleranceW<Gpioab8Spec> {
        Gpio169inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO169 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio169inttarget_wr_prot(&mut self) -> Gpio169inttargetWrProtW<Gpioab8Spec> {
        Gpio169inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO170 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio170intto_int13018(&mut self) -> EnblGpio170inttoInt13018W<Gpioab8Spec> {
        EnblGpio170inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO170 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio170intto_int13019(&mut self) -> EnblGpio170inttoInt13019W<Gpioab8Spec> {
        EnblGpio170inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO170 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio170intto_int13020(&mut self) -> EnblGpio170inttoInt13020W<Gpioab8Spec> {
        EnblGpio170inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO170 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio170intto_sio(&mut self) -> EnblGpio170inttoSioW<Gpioab8Spec> {
        EnblGpio170inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioab8Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioab8Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO170 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio170inttarget_rst_tolerance(&mut self) -> Gpio170inttargetRstToleranceW<Gpioab8Spec> {
        Gpio170inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO170 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio170inttarget_wr_prot(&mut self) -> Gpio170inttargetWrProtW<Gpioab8Spec> {
        Gpio170inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO171 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio171intto_int13018(&mut self) -> EnblGpio171inttoInt13018W<Gpioab8Spec> {
        EnblGpio171inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO171 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio171intto_int13019(&mut self) -> EnblGpio171inttoInt13019W<Gpioab8Spec> {
        EnblGpio171inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO171 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio171intto_int13020(&mut self) -> EnblGpio171inttoInt13020W<Gpioab8Spec> {
        EnblGpio171inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO171 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio171intto_sio(&mut self) -> EnblGpio171inttoSioW<Gpioab8Spec> {
        EnblGpio171inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioab8Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO171 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio171inttarget_rst_tolerance(&mut self) -> Gpio171inttargetRstToleranceW<Gpioab8Spec> {
        Gpio171inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO171 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio171inttarget_wr_prot(&mut self) -> Gpio171inttargetWrProtW<Gpioab8Spec> {
        Gpio171inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#42\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioab8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioab8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioab8Spec;
impl crate::RegisterSpec for Gpioab8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioab8::R`](R) reader structure"]
impl crate::Readable for Gpioab8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioab8::W`](W) writer structure"]
impl crate::Writable for Gpioab8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOAB8 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioab8Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
