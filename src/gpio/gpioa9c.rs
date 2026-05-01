#[doc = "Register `GPIOA9C` reader"]
pub type R = crate::R<Gpioa9cSpec>;
#[doc = "Register `GPIOA9C` writer"]
pub type W = crate::W<Gpioa9cSpec>;
#[doc = "Field `EnblGPIO140INTToINT13018` reader - Enable GPIO140 Interrupt To INT#130_18"]
pub type EnblGpio140inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO140INTToINT13018` writer - Enable GPIO140 Interrupt To INT#130_18"]
pub type EnblGpio140inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO140INTToINT13019` reader - Enable GPIO140 Interrupt To INT#130_19"]
pub type EnblGpio140inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO140INTToINT13019` writer - Enable GPIO140 Interrupt To INT#130_19"]
pub type EnblGpio140inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO140INTToINT13020` reader - Enable GPIO140 Interrupt To INT#130_20"]
pub type EnblGpio140inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO140INTToINT13020` writer - Enable GPIO140 Interrupt To INT#130_20"]
pub type EnblGpio140inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO140INTToSIO` reader - Enable GPIO140 Interrupt To SIO"]
pub type EnblGpio140inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO140INTToSIO` writer - Enable GPIO140 Interrupt To SIO"]
pub type EnblGpio140inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO140 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio140inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio140inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio140inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO140INTTargetRstTolerance` reader - GPIO140 Interrupt Target Reset Tolerance"]
pub type Gpio140inttargetRstToleranceR = crate::BitReader<Gpio140inttargetRstTolerance>;
impl Gpio140inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio140inttargetRstTolerance {
        match self.bits {
            false => Gpio140inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio140inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio140inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio140inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO140INTTargetRstTolerance` writer - GPIO140 Interrupt Target Reset Tolerance"]
pub type Gpio140inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio140inttargetRstTolerance>;
impl<'a, REG> Gpio140inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio140inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio140inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO140INTTargetWrProt` reader - GPIO140 Interrupt Target Write Protection"]
pub type Gpio140inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO140INTTargetWrProt` writer - GPIO140 Interrupt Target Write Protection"]
pub type Gpio140inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO141INTToINT13018` reader - Enable GPIO141 Interrupt To INT#130_18"]
pub type EnblGpio141inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO141INTToINT13018` writer - Enable GPIO141 Interrupt To INT#130_18"]
pub type EnblGpio141inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO141INTToINT13019` reader - Enable GPIO141 Interrupt To INT#130_19"]
pub type EnblGpio141inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO141INTToINT13019` writer - Enable GPIO141 Interrupt To INT#130_19"]
pub type EnblGpio141inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO141INTToINT13020` reader - Enable GPIO141 Interrupt To INT#130_20"]
pub type EnblGpio141inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO141INTToINT13020` writer - Enable GPIO141 Interrupt To INT#130_20"]
pub type EnblGpio141inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO141INTToSIO` reader - Enable GPIO141 Interrupt To SIO"]
pub type EnblGpio141inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO141INTToSIO` writer - Enable GPIO141 Interrupt To SIO"]
pub type EnblGpio141inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO141 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio141inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio141inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio141inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO141INTTargetRstTolerance` reader - GPIO141 Interrupt Target Reset Tolerance"]
pub type Gpio141inttargetRstToleranceR = crate::BitReader<Gpio141inttargetRstTolerance>;
impl Gpio141inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio141inttargetRstTolerance {
        match self.bits {
            false => Gpio141inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio141inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio141inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio141inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO141INTTargetRstTolerance` writer - GPIO141 Interrupt Target Reset Tolerance"]
pub type Gpio141inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio141inttargetRstTolerance>;
impl<'a, REG> Gpio141inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio141inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio141inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO141INTTargetWrProt` reader - GPIO141 Interrupt Target Write Protection"]
pub type Gpio141inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO141INTTargetWrProt` writer - GPIO141 Interrupt Target Write Protection"]
pub type Gpio141inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO142INTToINT13018` reader - Enable GPIO142 Interrupt To INT#130_18"]
pub type EnblGpio142inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO142INTToINT13018` writer - Enable GPIO142 Interrupt To INT#130_18"]
pub type EnblGpio142inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO142INTToINT13019` reader - Enable GPIO142 Interrupt To INT#130_19"]
pub type EnblGpio142inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO142INTToINT13019` writer - Enable GPIO142 Interrupt To INT#130_19"]
pub type EnblGpio142inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO142INTToINT13020` reader - Enable GPIO142 Interrupt To INT#130_20"]
pub type EnblGpio142inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO142INTToINT13020` writer - Enable GPIO142 Interrupt To INT#130_20"]
pub type EnblGpio142inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO142INTToSIO` reader - Enable GPIO142 Interrupt To SIO"]
pub type EnblGpio142inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO142INTToSIO` writer - Enable GPIO142 Interrupt To SIO"]
pub type EnblGpio142inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO142 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio142inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio142inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio142inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO142INTTargetRstTolerance` reader - GPIO142 Interrupt Target Reset Tolerance"]
pub type Gpio142inttargetRstToleranceR = crate::BitReader<Gpio142inttargetRstTolerance>;
impl Gpio142inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio142inttargetRstTolerance {
        match self.bits {
            false => Gpio142inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio142inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio142inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio142inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO142INTTargetRstTolerance` writer - GPIO142 Interrupt Target Reset Tolerance"]
pub type Gpio142inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio142inttargetRstTolerance>;
impl<'a, REG> Gpio142inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio142inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio142inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO142INTTargetWrProt` reader - GPIO142 Interrupt Target Write Protection"]
pub type Gpio142inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO142INTTargetWrProt` writer - GPIO142 Interrupt Target Write Protection"]
pub type Gpio142inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO143INTToINT13018` reader - Enable GPIO143 Interrupt To INT#130_18"]
pub type EnblGpio143inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO143INTToINT13018` writer - Enable GPIO143 Interrupt To INT#130_18"]
pub type EnblGpio143inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO143INTToINT13019` reader - Enable GPIO143 Interrupt To INT#130_19"]
pub type EnblGpio143inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO143INTToINT13019` writer - Enable GPIO143 Interrupt To INT#130_19"]
pub type EnblGpio143inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO143INTToINT13020` reader - Enable GPIO143 Interrupt To INT#130_20"]
pub type EnblGpio143inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO143INTToINT13020` writer - Enable GPIO143 Interrupt To INT#130_20"]
pub type EnblGpio143inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO143INTToSIO` reader - Enable GPIO143 Interrupt To SIO"]
pub type EnblGpio143inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO143INTToSIO` writer - Enable GPIO143 Interrupt To SIO"]
pub type EnblGpio143inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO143 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio143inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio143inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio143inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO143INTTargetRstTolerance` reader - GPIO143 Interrupt Target Reset Tolerance"]
pub type Gpio143inttargetRstToleranceR = crate::BitReader<Gpio143inttargetRstTolerance>;
impl Gpio143inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio143inttargetRstTolerance {
        match self.bits {
            false => Gpio143inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio143inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio143inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio143inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO143INTTargetRstTolerance` writer - GPIO143 Interrupt Target Reset Tolerance"]
pub type Gpio143inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio143inttargetRstTolerance>;
impl<'a, REG> Gpio143inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio143inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio143inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO143INTTargetWrProt` reader - GPIO143 Interrupt Target Write Protection"]
pub type Gpio143inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO143INTTargetWrProt` writer - GPIO143 Interrupt Target Write Protection"]
pub type Gpio143inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO140 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio140intto_int13018(&self) -> EnblGpio140inttoInt13018R {
        EnblGpio140inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO140 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio140intto_int13019(&self) -> EnblGpio140inttoInt13019R {
        EnblGpio140inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO140 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio140intto_int13020(&self) -> EnblGpio140inttoInt13020R {
        EnblGpio140inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO140 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio140intto_sio(&self) -> EnblGpio140inttoSioR {
        EnblGpio140inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO140 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio140inttarget_rst_tolerance(&self) -> Gpio140inttargetRstToleranceR {
        Gpio140inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO140 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio140inttarget_wr_prot(&self) -> Gpio140inttargetWrProtR {
        Gpio140inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO141 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio141intto_int13018(&self) -> EnblGpio141inttoInt13018R {
        EnblGpio141inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO141 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio141intto_int13019(&self) -> EnblGpio141inttoInt13019R {
        EnblGpio141inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO141 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio141intto_int13020(&self) -> EnblGpio141inttoInt13020R {
        EnblGpio141inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO141 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio141intto_sio(&self) -> EnblGpio141inttoSioR {
        EnblGpio141inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO141 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio141inttarget_rst_tolerance(&self) -> Gpio141inttargetRstToleranceR {
        Gpio141inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO141 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio141inttarget_wr_prot(&self) -> Gpio141inttargetWrProtR {
        Gpio141inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO142 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio142intto_int13018(&self) -> EnblGpio142inttoInt13018R {
        EnblGpio142inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO142 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio142intto_int13019(&self) -> EnblGpio142inttoInt13019R {
        EnblGpio142inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO142 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio142intto_int13020(&self) -> EnblGpio142inttoInt13020R {
        EnblGpio142inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO142 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio142intto_sio(&self) -> EnblGpio142inttoSioR {
        EnblGpio142inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO142 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio142inttarget_rst_tolerance(&self) -> Gpio142inttargetRstToleranceR {
        Gpio142inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO142 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio142inttarget_wr_prot(&self) -> Gpio142inttargetWrProtR {
        Gpio142inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO143 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio143intto_int13018(&self) -> EnblGpio143inttoInt13018R {
        EnblGpio143inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO143 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio143intto_int13019(&self) -> EnblGpio143inttoInt13019R {
        EnblGpio143inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO143 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio143intto_int13020(&self) -> EnblGpio143inttoInt13020R {
        EnblGpio143inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO143 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio143intto_sio(&self) -> EnblGpio143inttoSioR {
        EnblGpio143inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO143 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio143inttarget_rst_tolerance(&self) -> Gpio143inttargetRstToleranceR {
        Gpio143inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO143 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio143inttarget_wr_prot(&self) -> Gpio143inttargetWrProtR {
        Gpio143inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO140 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio140intto_int13018(&mut self) -> EnblGpio140inttoInt13018W<Gpioa9cSpec> {
        EnblGpio140inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO140 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio140intto_int13019(&mut self) -> EnblGpio140inttoInt13019W<Gpioa9cSpec> {
        EnblGpio140inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO140 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio140intto_int13020(&mut self) -> EnblGpio140inttoInt13020W<Gpioa9cSpec> {
        EnblGpio140inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO140 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio140intto_sio(&mut self) -> EnblGpio140inttoSioW<Gpioa9cSpec> {
        EnblGpio140inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa9cSpec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa9cSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO140 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio140inttarget_rst_tolerance(&mut self) -> Gpio140inttargetRstToleranceW<Gpioa9cSpec> {
        Gpio140inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO140 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio140inttarget_wr_prot(&mut self) -> Gpio140inttargetWrProtW<Gpioa9cSpec> {
        Gpio140inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO141 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio141intto_int13018(&mut self) -> EnblGpio141inttoInt13018W<Gpioa9cSpec> {
        EnblGpio141inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO141 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio141intto_int13019(&mut self) -> EnblGpio141inttoInt13019W<Gpioa9cSpec> {
        EnblGpio141inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO141 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio141intto_int13020(&mut self) -> EnblGpio141inttoInt13020W<Gpioa9cSpec> {
        EnblGpio141inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO141 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio141intto_sio(&mut self) -> EnblGpio141inttoSioW<Gpioa9cSpec> {
        EnblGpio141inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa9cSpec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa9cSpec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO141 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio141inttarget_rst_tolerance(&mut self) -> Gpio141inttargetRstToleranceW<Gpioa9cSpec> {
        Gpio141inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO141 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio141inttarget_wr_prot(&mut self) -> Gpio141inttargetWrProtW<Gpioa9cSpec> {
        Gpio141inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO142 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio142intto_int13018(&mut self) -> EnblGpio142inttoInt13018W<Gpioa9cSpec> {
        EnblGpio142inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO142 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio142intto_int13019(&mut self) -> EnblGpio142inttoInt13019W<Gpioa9cSpec> {
        EnblGpio142inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO142 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio142intto_int13020(&mut self) -> EnblGpio142inttoInt13020W<Gpioa9cSpec> {
        EnblGpio142inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO142 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio142intto_sio(&mut self) -> EnblGpio142inttoSioW<Gpioa9cSpec> {
        EnblGpio142inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa9cSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa9cSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO142 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio142inttarget_rst_tolerance(&mut self) -> Gpio142inttargetRstToleranceW<Gpioa9cSpec> {
        Gpio142inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO142 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio142inttarget_wr_prot(&mut self) -> Gpio142inttargetWrProtW<Gpioa9cSpec> {
        Gpio142inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO143 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio143intto_int13018(&mut self) -> EnblGpio143inttoInt13018W<Gpioa9cSpec> {
        EnblGpio143inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO143 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio143intto_int13019(&mut self) -> EnblGpio143inttoInt13019W<Gpioa9cSpec> {
        EnblGpio143inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO143 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio143intto_int13020(&mut self) -> EnblGpio143inttoInt13020W<Gpioa9cSpec> {
        EnblGpio143inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO143 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio143intto_sio(&mut self) -> EnblGpio143inttoSioW<Gpioa9cSpec> {
        EnblGpio143inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa9cSpec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO143 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio143inttarget_rst_tolerance(&mut self) -> Gpio143inttargetRstToleranceW<Gpioa9cSpec> {
        Gpio143inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO143 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio143inttarget_wr_prot(&mut self) -> Gpio143inttargetWrProtW<Gpioa9cSpec> {
        Gpio143inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa9c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa9c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa9cSpec;
impl crate::RegisterSpec for Gpioa9cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa9c::R`](R) reader structure"]
impl crate::Readable for Gpioa9cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioa9c::W`](W) writer structure"]
impl crate::Writable for Gpioa9cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA9C to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa9cSpec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
