#[doc = "Register `GPIOA4C` reader"]
pub type R = crate::R<Gpioa4cSpec>;
#[doc = "Register `GPIOA4C` writer"]
pub type W = crate::W<Gpioa4cSpec>;
#[doc = "Field `EnblGPIO060INTToINT13018` reader - Enable GPIO060 Interrupt To INT#130_18"]
pub type EnblGpio060inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO060INTToINT13018` writer - Enable GPIO060 Interrupt To INT#130_18"]
pub type EnblGpio060inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO060INTToINT13019` reader - Enable GPIO060 Interrupt To INT#130_19"]
pub type EnblGpio060inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO060INTToINT13019` writer - Enable GPIO060 Interrupt To INT#130_19"]
pub type EnblGpio060inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO060INTToINT13020` reader - Enable GPIO060 Interrupt To INT#130_20"]
pub type EnblGpio060inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO060INTToINT13020` writer - Enable GPIO060 Interrupt To INT#130_20"]
pub type EnblGpio060inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO060INTToSIO` reader - Enable GPIO060 Interrupt To SIO"]
pub type EnblGpio060inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO060INTToSIO` writer - Enable GPIO060 Interrupt To SIO"]
pub type EnblGpio060inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO060 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio060inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio060inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio060inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO060INTTargetRstTolerance` reader - GPIO060 Interrupt Target Reset Tolerance"]
pub type Gpio060inttargetRstToleranceR = crate::BitReader<Gpio060inttargetRstTolerance>;
impl Gpio060inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio060inttargetRstTolerance {
        match self.bits {
            false => Gpio060inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio060inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio060inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio060inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO060INTTargetRstTolerance` writer - GPIO060 Interrupt Target Reset Tolerance"]
pub type Gpio060inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio060inttargetRstTolerance>;
impl<'a, REG> Gpio060inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio060inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio060inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO060INTTargetWrProt` reader - GPIO060 Interrupt Target Write Protection"]
pub type Gpio060inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO060INTTargetWrProt` writer - GPIO060 Interrupt Target Write Protection"]
pub type Gpio060inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO061INTToINT13018` reader - Enable GPIO061 Interrupt To INT#130_18"]
pub type EnblGpio061inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO061INTToINT13018` writer - Enable GPIO061 Interrupt To INT#130_18"]
pub type EnblGpio061inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO061INTToINT13019` reader - Enable GPIO061 Interrupt To INT#130_19"]
pub type EnblGpio061inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO061INTToINT13019` writer - Enable GPIO061 Interrupt To INT#130_19"]
pub type EnblGpio061inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO061INTToINT13020` reader - Enable GPIO061 Interrupt To INT#130_20"]
pub type EnblGpio061inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO061INTToINT13020` writer - Enable GPIO061 Interrupt To INT#130_20"]
pub type EnblGpio061inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO061INTToSIO` reader - Enable GPIO061 Interrupt To SIO"]
pub type EnblGpio061inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO061INTToSIO` writer - Enable GPIO061 Interrupt To SIO"]
pub type EnblGpio061inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO061 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio061inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio061inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio061inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO061INTTargetRstTolerance` reader - GPIO061 Interrupt Target Reset Tolerance"]
pub type Gpio061inttargetRstToleranceR = crate::BitReader<Gpio061inttargetRstTolerance>;
impl Gpio061inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio061inttargetRstTolerance {
        match self.bits {
            false => Gpio061inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio061inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio061inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio061inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO061INTTargetRstTolerance` writer - GPIO061 Interrupt Target Reset Tolerance"]
pub type Gpio061inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio061inttargetRstTolerance>;
impl<'a, REG> Gpio061inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio061inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio061inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO061INTTargetWrProt` reader - GPIO061 Interrupt Target Write Protection"]
pub type Gpio061inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO061INTTargetWrProt` writer - GPIO061 Interrupt Target Write Protection"]
pub type Gpio061inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO062INTToINT13018` reader - Enable GPIO062 Interrupt To INT#130_18"]
pub type EnblGpio062inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO062INTToINT13018` writer - Enable GPIO062 Interrupt To INT#130_18"]
pub type EnblGpio062inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO062INTToINT13019` reader - Enable GPIO062 Interrupt To INT#130_19"]
pub type EnblGpio062inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO062INTToINT13019` writer - Enable GPIO062 Interrupt To INT#130_19"]
pub type EnblGpio062inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO062INTToINT13020` reader - Enable GPIO062 Interrupt To INT#130_20"]
pub type EnblGpio062inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO062INTToINT13020` writer - Enable GPIO062 Interrupt To INT#130_20"]
pub type EnblGpio062inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO062INTToSIO` reader - Enable GPIO062 Interrupt To SIO"]
pub type EnblGpio062inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO062INTToSIO` writer - Enable GPIO062 Interrupt To SIO"]
pub type EnblGpio062inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO062 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio062inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio062inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio062inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO062INTTargetRstTolerance` reader - GPIO062 Interrupt Target Reset Tolerance"]
pub type Gpio062inttargetRstToleranceR = crate::BitReader<Gpio062inttargetRstTolerance>;
impl Gpio062inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio062inttargetRstTolerance {
        match self.bits {
            false => Gpio062inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio062inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio062inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio062inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO062INTTargetRstTolerance` writer - GPIO062 Interrupt Target Reset Tolerance"]
pub type Gpio062inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio062inttargetRstTolerance>;
impl<'a, REG> Gpio062inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio062inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio062inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO062INTTargetWrProt` reader - GPIO062 Interrupt Target Write Protection"]
pub type Gpio062inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO062INTTargetWrProt` writer - GPIO062 Interrupt Target Write Protection"]
pub type Gpio062inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO063INTToINT13018` reader - Enable GPIO063 Interrupt To INT#130_18"]
pub type EnblGpio063inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO063INTToINT13018` writer - Enable GPIO063 Interrupt To INT#130_18"]
pub type EnblGpio063inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO063INTToINT13019` reader - Enable GPIO063 Interrupt To INT#130_19"]
pub type EnblGpio063inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO063INTToINT13019` writer - Enable GPIO063 Interrupt To INT#130_19"]
pub type EnblGpio063inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO063INTToINT13020` reader - Enable GPIO063 Interrupt To INT#130_20"]
pub type EnblGpio063inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO063INTToINT13020` writer - Enable GPIO063 Interrupt To INT#130_20"]
pub type EnblGpio063inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO063INTToSIO` reader - Enable GPIO063 Interrupt To SIO"]
pub type EnblGpio063inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO063INTToSIO` writer - Enable GPIO063 Interrupt To SIO"]
pub type EnblGpio063inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO063 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio063inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio063inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio063inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO063INTTargetRstTolerance` reader - GPIO063 Interrupt Target Reset Tolerance"]
pub type Gpio063inttargetRstToleranceR = crate::BitReader<Gpio063inttargetRstTolerance>;
impl Gpio063inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio063inttargetRstTolerance {
        match self.bits {
            false => Gpio063inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio063inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio063inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio063inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO063INTTargetRstTolerance` writer - GPIO063 Interrupt Target Reset Tolerance"]
pub type Gpio063inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio063inttargetRstTolerance>;
impl<'a, REG> Gpio063inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio063inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio063inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO063INTTargetWrProt` reader - GPIO063 Interrupt Target Write Protection"]
pub type Gpio063inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO063INTTargetWrProt` writer - GPIO063 Interrupt Target Write Protection"]
pub type Gpio063inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO060 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio060intto_int13018(&self) -> EnblGpio060inttoInt13018R {
        EnblGpio060inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO060 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio060intto_int13019(&self) -> EnblGpio060inttoInt13019R {
        EnblGpio060inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO060 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio060intto_int13020(&self) -> EnblGpio060inttoInt13020R {
        EnblGpio060inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO060 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio060intto_sio(&self) -> EnblGpio060inttoSioR {
        EnblGpio060inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO060 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio060inttarget_rst_tolerance(&self) -> Gpio060inttargetRstToleranceR {
        Gpio060inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO060 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio060inttarget_wr_prot(&self) -> Gpio060inttargetWrProtR {
        Gpio060inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO061 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio061intto_int13018(&self) -> EnblGpio061inttoInt13018R {
        EnblGpio061inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO061 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio061intto_int13019(&self) -> EnblGpio061inttoInt13019R {
        EnblGpio061inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO061 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio061intto_int13020(&self) -> EnblGpio061inttoInt13020R {
        EnblGpio061inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO061 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio061intto_sio(&self) -> EnblGpio061inttoSioR {
        EnblGpio061inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO061 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio061inttarget_rst_tolerance(&self) -> Gpio061inttargetRstToleranceR {
        Gpio061inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO061 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio061inttarget_wr_prot(&self) -> Gpio061inttargetWrProtR {
        Gpio061inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO062 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio062intto_int13018(&self) -> EnblGpio062inttoInt13018R {
        EnblGpio062inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO062 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio062intto_int13019(&self) -> EnblGpio062inttoInt13019R {
        EnblGpio062inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO062 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio062intto_int13020(&self) -> EnblGpio062inttoInt13020R {
        EnblGpio062inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO062 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio062intto_sio(&self) -> EnblGpio062inttoSioR {
        EnblGpio062inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO062 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio062inttarget_rst_tolerance(&self) -> Gpio062inttargetRstToleranceR {
        Gpio062inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO062 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio062inttarget_wr_prot(&self) -> Gpio062inttargetWrProtR {
        Gpio062inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO063 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio063intto_int13018(&self) -> EnblGpio063inttoInt13018R {
        EnblGpio063inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO063 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio063intto_int13019(&self) -> EnblGpio063inttoInt13019R {
        EnblGpio063inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO063 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio063intto_int13020(&self) -> EnblGpio063inttoInt13020R {
        EnblGpio063inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO063 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio063intto_sio(&self) -> EnblGpio063inttoSioR {
        EnblGpio063inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO063 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio063inttarget_rst_tolerance(&self) -> Gpio063inttargetRstToleranceR {
        Gpio063inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO063 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio063inttarget_wr_prot(&self) -> Gpio063inttargetWrProtR {
        Gpio063inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO060 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio060intto_int13018(&mut self) -> EnblGpio060inttoInt13018W<Gpioa4cSpec> {
        EnblGpio060inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO060 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio060intto_int13019(&mut self) -> EnblGpio060inttoInt13019W<Gpioa4cSpec> {
        EnblGpio060inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO060 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio060intto_int13020(&mut self) -> EnblGpio060inttoInt13020W<Gpioa4cSpec> {
        EnblGpio060inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO060 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio060intto_sio(&mut self) -> EnblGpio060inttoSioW<Gpioa4cSpec> {
        EnblGpio060inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa4cSpec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa4cSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO060 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio060inttarget_rst_tolerance(&mut self) -> Gpio060inttargetRstToleranceW<Gpioa4cSpec> {
        Gpio060inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO060 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio060inttarget_wr_prot(&mut self) -> Gpio060inttargetWrProtW<Gpioa4cSpec> {
        Gpio060inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO061 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio061intto_int13018(&mut self) -> EnblGpio061inttoInt13018W<Gpioa4cSpec> {
        EnblGpio061inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO061 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio061intto_int13019(&mut self) -> EnblGpio061inttoInt13019W<Gpioa4cSpec> {
        EnblGpio061inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO061 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio061intto_int13020(&mut self) -> EnblGpio061inttoInt13020W<Gpioa4cSpec> {
        EnblGpio061inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO061 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio061intto_sio(&mut self) -> EnblGpio061inttoSioW<Gpioa4cSpec> {
        EnblGpio061inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa4cSpec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa4cSpec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO061 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio061inttarget_rst_tolerance(&mut self) -> Gpio061inttargetRstToleranceW<Gpioa4cSpec> {
        Gpio061inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO061 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio061inttarget_wr_prot(&mut self) -> Gpio061inttargetWrProtW<Gpioa4cSpec> {
        Gpio061inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO062 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio062intto_int13018(&mut self) -> EnblGpio062inttoInt13018W<Gpioa4cSpec> {
        EnblGpio062inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO062 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio062intto_int13019(&mut self) -> EnblGpio062inttoInt13019W<Gpioa4cSpec> {
        EnblGpio062inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO062 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio062intto_int13020(&mut self) -> EnblGpio062inttoInt13020W<Gpioa4cSpec> {
        EnblGpio062inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO062 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio062intto_sio(&mut self) -> EnblGpio062inttoSioW<Gpioa4cSpec> {
        EnblGpio062inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa4cSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa4cSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO062 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio062inttarget_rst_tolerance(&mut self) -> Gpio062inttargetRstToleranceW<Gpioa4cSpec> {
        Gpio062inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO062 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio062inttarget_wr_prot(&mut self) -> Gpio062inttargetWrProtW<Gpioa4cSpec> {
        Gpio062inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO063 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio063intto_int13018(&mut self) -> EnblGpio063inttoInt13018W<Gpioa4cSpec> {
        EnblGpio063inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO063 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio063intto_int13019(&mut self) -> EnblGpio063inttoInt13019W<Gpioa4cSpec> {
        EnblGpio063inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO063 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio063intto_int13020(&mut self) -> EnblGpio063inttoInt13020W<Gpioa4cSpec> {
        EnblGpio063inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO063 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio063intto_sio(&mut self) -> EnblGpio063inttoSioW<Gpioa4cSpec> {
        EnblGpio063inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa4cSpec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO063 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio063inttarget_rst_tolerance(&mut self) -> Gpio063inttargetRstToleranceW<Gpioa4cSpec> {
        Gpio063inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO063 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio063inttarget_wr_prot(&mut self) -> Gpio063inttargetWrProtW<Gpioa4cSpec> {
        Gpio063inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa4cSpec;
impl crate::RegisterSpec for Gpioa4cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa4c::R`](R) reader structure"]
impl crate::Readable for Gpioa4cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioa4c::W`](W) writer structure"]
impl crate::Writable for Gpioa4cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA4C to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa4cSpec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
