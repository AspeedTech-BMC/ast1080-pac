#[doc = "Register `GPIOA38` reader"]
pub type R = crate::R<Gpioa38Spec>;
#[doc = "Register `GPIOA38` writer"]
pub type W = crate::W<Gpioa38Spec>;
#[doc = "Field `EnblGPIO040INTToINT13018` reader - Enable GPIO040 Interrupt To INT#130_18"]
pub type EnblGpio040inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO040INTToINT13018` writer - Enable GPIO040 Interrupt To INT#130_18"]
pub type EnblGpio040inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO040INTToINT13019` reader - Enable GPIO040 Interrupt To INT#130_19"]
pub type EnblGpio040inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO040INTToINT13019` writer - Enable GPIO040 Interrupt To INT#130_19"]
pub type EnblGpio040inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO040INTToINT13020` reader - Enable GPIO040 Interrupt To INT#130_20"]
pub type EnblGpio040inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO040INTToINT13020` writer - Enable GPIO040 Interrupt To INT#130_20"]
pub type EnblGpio040inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO040INTToSIO` reader - Enable GPIO040 Interrupt To SIO"]
pub type EnblGpio040inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO040INTToSIO` writer - Enable GPIO040 Interrupt To SIO"]
pub type EnblGpio040inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO040 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio040inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio040inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio040inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO040INTTargetRstTolerance` reader - GPIO040 Interrupt Target Reset Tolerance"]
pub type Gpio040inttargetRstToleranceR = crate::BitReader<Gpio040inttargetRstTolerance>;
impl Gpio040inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio040inttargetRstTolerance {
        match self.bits {
            false => Gpio040inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio040inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio040inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio040inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO040INTTargetRstTolerance` writer - GPIO040 Interrupt Target Reset Tolerance"]
pub type Gpio040inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio040inttargetRstTolerance>;
impl<'a, REG> Gpio040inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio040inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio040inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO040INTTargetWrProt` reader - GPIO040 Interrupt Target Write Protection"]
pub type Gpio040inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO040INTTargetWrProt` writer - GPIO040 Interrupt Target Write Protection"]
pub type Gpio040inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO041INTToINT13018` reader - Enable GPIO041 Interrupt To INT#130_18"]
pub type EnblGpio041inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO041INTToINT13018` writer - Enable GPIO041 Interrupt To INT#130_18"]
pub type EnblGpio041inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO041INTToINT13019` reader - Enable GPIO041 Interrupt To INT#130_19"]
pub type EnblGpio041inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO041INTToINT13019` writer - Enable GPIO041 Interrupt To INT#130_19"]
pub type EnblGpio041inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO041INTToINT13020` reader - Enable GPIO041 Interrupt To INT#130_20"]
pub type EnblGpio041inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO041INTToINT13020` writer - Enable GPIO041 Interrupt To INT#130_20"]
pub type EnblGpio041inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO041INTToSIO` reader - Enable GPIO041 Interrupt To SIO"]
pub type EnblGpio041inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO041INTToSIO` writer - Enable GPIO041 Interrupt To SIO"]
pub type EnblGpio041inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO041 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio041inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio041inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio041inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO041INTTargetRstTolerance` reader - GPIO041 Interrupt Target Reset Tolerance"]
pub type Gpio041inttargetRstToleranceR = crate::BitReader<Gpio041inttargetRstTolerance>;
impl Gpio041inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio041inttargetRstTolerance {
        match self.bits {
            false => Gpio041inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio041inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio041inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio041inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO041INTTargetRstTolerance` writer - GPIO041 Interrupt Target Reset Tolerance"]
pub type Gpio041inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio041inttargetRstTolerance>;
impl<'a, REG> Gpio041inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio041inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio041inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO041INTTargetWrProt` reader - GPIO041 Interrupt Target Write Protection"]
pub type Gpio041inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO041INTTargetWrProt` writer - GPIO041 Interrupt Target Write Protection"]
pub type Gpio041inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO042INTToINT13018` reader - Enable GPIO042 Interrupt To INT#130_18"]
pub type EnblGpio042inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO042INTToINT13018` writer - Enable GPIO042 Interrupt To INT#130_18"]
pub type EnblGpio042inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO042INTToINT13019` reader - Enable GPIO042 Interrupt To INT#130_19"]
pub type EnblGpio042inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO042INTToINT13019` writer - Enable GPIO042 Interrupt To INT#130_19"]
pub type EnblGpio042inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO042INTToINT13020` reader - Enable GPIO042 Interrupt To INT#130_20"]
pub type EnblGpio042inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO042INTToINT13020` writer - Enable GPIO042 Interrupt To INT#130_20"]
pub type EnblGpio042inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO042INTToSIO` reader - Enable GPIO042 Interrupt To SIO"]
pub type EnblGpio042inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO042INTToSIO` writer - Enable GPIO042 Interrupt To SIO"]
pub type EnblGpio042inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO042 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio042inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio042inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio042inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO042INTTargetRstTolerance` reader - GPIO042 Interrupt Target Reset Tolerance"]
pub type Gpio042inttargetRstToleranceR = crate::BitReader<Gpio042inttargetRstTolerance>;
impl Gpio042inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio042inttargetRstTolerance {
        match self.bits {
            false => Gpio042inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio042inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio042inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio042inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO042INTTargetRstTolerance` writer - GPIO042 Interrupt Target Reset Tolerance"]
pub type Gpio042inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio042inttargetRstTolerance>;
impl<'a, REG> Gpio042inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio042inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio042inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO042INTTargetWrProt` reader - GPIO042 Interrupt Target Write Protection"]
pub type Gpio042inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO042INTTargetWrProt` writer - GPIO042 Interrupt Target Write Protection"]
pub type Gpio042inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO043INTToINT13018` reader - Enable GPIO043 Interrupt To INT#130_18"]
pub type EnblGpio043inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO043INTToINT13018` writer - Enable GPIO043 Interrupt To INT#130_18"]
pub type EnblGpio043inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO043INTToINT13019` reader - Enable GPIO043 Interrupt To INT#130_19"]
pub type EnblGpio043inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO043INTToINT13019` writer - Enable GPIO043 Interrupt To INT#130_19"]
pub type EnblGpio043inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO043INTToINT13020` reader - Enable GPIO043 Interrupt To INT#130_20"]
pub type EnblGpio043inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO043INTToINT13020` writer - Enable GPIO043 Interrupt To INT#130_20"]
pub type EnblGpio043inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO043INTToSIO` reader - Enable GPIO043 Interrupt To SIO"]
pub type EnblGpio043inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO043INTToSIO` writer - Enable GPIO043 Interrupt To SIO"]
pub type EnblGpio043inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO043 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio043inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio043inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio043inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO043INTTargetRstTolerance` reader - GPIO043 Interrupt Target Reset Tolerance"]
pub type Gpio043inttargetRstToleranceR = crate::BitReader<Gpio043inttargetRstTolerance>;
impl Gpio043inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio043inttargetRstTolerance {
        match self.bits {
            false => Gpio043inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio043inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio043inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio043inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO043INTTargetRstTolerance` writer - GPIO043 Interrupt Target Reset Tolerance"]
pub type Gpio043inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio043inttargetRstTolerance>;
impl<'a, REG> Gpio043inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio043inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio043inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO043INTTargetWrProt` reader - GPIO043 Interrupt Target Write Protection"]
pub type Gpio043inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO043INTTargetWrProt` writer - GPIO043 Interrupt Target Write Protection"]
pub type Gpio043inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO040 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio040intto_int13018(&self) -> EnblGpio040inttoInt13018R {
        EnblGpio040inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO040 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio040intto_int13019(&self) -> EnblGpio040inttoInt13019R {
        EnblGpio040inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO040 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio040intto_int13020(&self) -> EnblGpio040inttoInt13020R {
        EnblGpio040inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO040 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio040intto_sio(&self) -> EnblGpio040inttoSioR {
        EnblGpio040inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO040 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio040inttarget_rst_tolerance(&self) -> Gpio040inttargetRstToleranceR {
        Gpio040inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO040 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio040inttarget_wr_prot(&self) -> Gpio040inttargetWrProtR {
        Gpio040inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO041 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio041intto_int13018(&self) -> EnblGpio041inttoInt13018R {
        EnblGpio041inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO041 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio041intto_int13019(&self) -> EnblGpio041inttoInt13019R {
        EnblGpio041inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO041 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio041intto_int13020(&self) -> EnblGpio041inttoInt13020R {
        EnblGpio041inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO041 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio041intto_sio(&self) -> EnblGpio041inttoSioR {
        EnblGpio041inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO041 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio041inttarget_rst_tolerance(&self) -> Gpio041inttargetRstToleranceR {
        Gpio041inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO041 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio041inttarget_wr_prot(&self) -> Gpio041inttargetWrProtR {
        Gpio041inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO042 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio042intto_int13018(&self) -> EnblGpio042inttoInt13018R {
        EnblGpio042inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO042 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio042intto_int13019(&self) -> EnblGpio042inttoInt13019R {
        EnblGpio042inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO042 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio042intto_int13020(&self) -> EnblGpio042inttoInt13020R {
        EnblGpio042inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO042 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio042intto_sio(&self) -> EnblGpio042inttoSioR {
        EnblGpio042inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO042 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio042inttarget_rst_tolerance(&self) -> Gpio042inttargetRstToleranceR {
        Gpio042inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO042 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio042inttarget_wr_prot(&self) -> Gpio042inttargetWrProtR {
        Gpio042inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO043 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio043intto_int13018(&self) -> EnblGpio043inttoInt13018R {
        EnblGpio043inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO043 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio043intto_int13019(&self) -> EnblGpio043inttoInt13019R {
        EnblGpio043inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO043 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio043intto_int13020(&self) -> EnblGpio043inttoInt13020R {
        EnblGpio043inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO043 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio043intto_sio(&self) -> EnblGpio043inttoSioR {
        EnblGpio043inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO043 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio043inttarget_rst_tolerance(&self) -> Gpio043inttargetRstToleranceR {
        Gpio043inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO043 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio043inttarget_wr_prot(&self) -> Gpio043inttargetWrProtR {
        Gpio043inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO040 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio040intto_int13018(&mut self) -> EnblGpio040inttoInt13018W<Gpioa38Spec> {
        EnblGpio040inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO040 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio040intto_int13019(&mut self) -> EnblGpio040inttoInt13019W<Gpioa38Spec> {
        EnblGpio040inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO040 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio040intto_int13020(&mut self) -> EnblGpio040inttoInt13020W<Gpioa38Spec> {
        EnblGpio040inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO040 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio040intto_sio(&mut self) -> EnblGpio040inttoSioW<Gpioa38Spec> {
        EnblGpio040inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa38Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa38Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO040 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio040inttarget_rst_tolerance(&mut self) -> Gpio040inttargetRstToleranceW<Gpioa38Spec> {
        Gpio040inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO040 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio040inttarget_wr_prot(&mut self) -> Gpio040inttargetWrProtW<Gpioa38Spec> {
        Gpio040inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO041 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio041intto_int13018(&mut self) -> EnblGpio041inttoInt13018W<Gpioa38Spec> {
        EnblGpio041inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO041 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio041intto_int13019(&mut self) -> EnblGpio041inttoInt13019W<Gpioa38Spec> {
        EnblGpio041inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO041 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio041intto_int13020(&mut self) -> EnblGpio041inttoInt13020W<Gpioa38Spec> {
        EnblGpio041inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO041 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio041intto_sio(&mut self) -> EnblGpio041inttoSioW<Gpioa38Spec> {
        EnblGpio041inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa38Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa38Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO041 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio041inttarget_rst_tolerance(&mut self) -> Gpio041inttargetRstToleranceW<Gpioa38Spec> {
        Gpio041inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO041 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio041inttarget_wr_prot(&mut self) -> Gpio041inttargetWrProtW<Gpioa38Spec> {
        Gpio041inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO042 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio042intto_int13018(&mut self) -> EnblGpio042inttoInt13018W<Gpioa38Spec> {
        EnblGpio042inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO042 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio042intto_int13019(&mut self) -> EnblGpio042inttoInt13019W<Gpioa38Spec> {
        EnblGpio042inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO042 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio042intto_int13020(&mut self) -> EnblGpio042inttoInt13020W<Gpioa38Spec> {
        EnblGpio042inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO042 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio042intto_sio(&mut self) -> EnblGpio042inttoSioW<Gpioa38Spec> {
        EnblGpio042inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa38Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa38Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO042 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio042inttarget_rst_tolerance(&mut self) -> Gpio042inttargetRstToleranceW<Gpioa38Spec> {
        Gpio042inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO042 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio042inttarget_wr_prot(&mut self) -> Gpio042inttargetWrProtW<Gpioa38Spec> {
        Gpio042inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO043 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio043intto_int13018(&mut self) -> EnblGpio043inttoInt13018W<Gpioa38Spec> {
        EnblGpio043inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO043 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio043intto_int13019(&mut self) -> EnblGpio043inttoInt13019W<Gpioa38Spec> {
        EnblGpio043inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO043 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio043intto_int13020(&mut self) -> EnblGpio043inttoInt13020W<Gpioa38Spec> {
        EnblGpio043inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO043 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio043intto_sio(&mut self) -> EnblGpio043inttoSioW<Gpioa38Spec> {
        EnblGpio043inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa38Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO043 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio043inttarget_rst_tolerance(&mut self) -> Gpio043inttargetRstToleranceW<Gpioa38Spec> {
        Gpio043inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO043 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio043inttarget_wr_prot(&mut self) -> Gpio043inttargetWrProtW<Gpioa38Spec> {
        Gpio043inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa38Spec;
impl crate::RegisterSpec for Gpioa38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa38::R`](R) reader structure"]
impl crate::Readable for Gpioa38Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa38::W`](W) writer structure"]
impl crate::Writable for Gpioa38Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA38 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa38Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
