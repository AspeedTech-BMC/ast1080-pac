#[doc = "Register `GPIOAA8` reader"]
pub type R = crate::R<Gpioaa8Spec>;
#[doc = "Register `GPIOAA8` writer"]
pub type W = crate::W<Gpioaa8Spec>;
#[doc = "Field `EnblGPIO152INTToINT13018` reader - Enable GPIO152 Interrupt To INT#130_18"]
pub type EnblGpio152inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO152INTToINT13018` writer - Enable GPIO152 Interrupt To INT#130_18"]
pub type EnblGpio152inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO152INTToINT13019` reader - Enable GPIO152 Interrupt To INT#130_19"]
pub type EnblGpio152inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO152INTToINT13019` writer - Enable GPIO152 Interrupt To INT#130_19"]
pub type EnblGpio152inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO152INTToINT13020` reader - Enable GPIO152 Interrupt To INT#130_20"]
pub type EnblGpio152inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO152INTToINT13020` writer - Enable GPIO152 Interrupt To INT#130_20"]
pub type EnblGpio152inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO152INTToSIO` reader - Enable GPIO152 Interrupt To SIO"]
pub type EnblGpio152inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO152INTToSIO` writer - Enable GPIO152 Interrupt To SIO"]
pub type EnblGpio152inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO152 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio152inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio152inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio152inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO152INTTargetRstTolerance` reader - GPIO152 Interrupt Target Reset Tolerance"]
pub type Gpio152inttargetRstToleranceR = crate::BitReader<Gpio152inttargetRstTolerance>;
impl Gpio152inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio152inttargetRstTolerance {
        match self.bits {
            false => Gpio152inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio152inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio152inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio152inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO152INTTargetRstTolerance` writer - GPIO152 Interrupt Target Reset Tolerance"]
pub type Gpio152inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio152inttargetRstTolerance>;
impl<'a, REG> Gpio152inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio152inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio152inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO152INTTargetWrProt` reader - GPIO152 Interrupt Target Write Protection"]
pub type Gpio152inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO152INTTargetWrProt` writer - GPIO152 Interrupt Target Write Protection"]
pub type Gpio152inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO153INTToINT13018` reader - Enable GPIO153 Interrupt To INT#130_18"]
pub type EnblGpio153inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO153INTToINT13018` writer - Enable GPIO153 Interrupt To INT#130_18"]
pub type EnblGpio153inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO153INTToINT13019` reader - Enable GPIO153 Interrupt To INT#130_19"]
pub type EnblGpio153inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO153INTToINT13019` writer - Enable GPIO153 Interrupt To INT#130_19"]
pub type EnblGpio153inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO153INTToINT13020` reader - Enable GPIO153 Interrupt To INT#130_20"]
pub type EnblGpio153inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO153INTToINT13020` writer - Enable GPIO153 Interrupt To INT#130_20"]
pub type EnblGpio153inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO153INTToSIO` reader - Enable GPIO153 Interrupt To SIO"]
pub type EnblGpio153inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO153INTToSIO` writer - Enable GPIO153 Interrupt To SIO"]
pub type EnblGpio153inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO153 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio153inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio153inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio153inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO153INTTargetRstTolerance` reader - GPIO153 Interrupt Target Reset Tolerance"]
pub type Gpio153inttargetRstToleranceR = crate::BitReader<Gpio153inttargetRstTolerance>;
impl Gpio153inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio153inttargetRstTolerance {
        match self.bits {
            false => Gpio153inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio153inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio153inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio153inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO153INTTargetRstTolerance` writer - GPIO153 Interrupt Target Reset Tolerance"]
pub type Gpio153inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio153inttargetRstTolerance>;
impl<'a, REG> Gpio153inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio153inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio153inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO153INTTargetWrProt` reader - GPIO153 Interrupt Target Write Protection"]
pub type Gpio153inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO153INTTargetWrProt` writer - GPIO153 Interrupt Target Write Protection"]
pub type Gpio153inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO154INTToINT13018` reader - Enable GPIO154 Interrupt To INT#130_18"]
pub type EnblGpio154inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO154INTToINT13018` writer - Enable GPIO154 Interrupt To INT#130_18"]
pub type EnblGpio154inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO154INTToINT13019` reader - Enable GPIO154 Interrupt To INT#130_19"]
pub type EnblGpio154inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO154INTToINT13019` writer - Enable GPIO154 Interrupt To INT#130_19"]
pub type EnblGpio154inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO154INTToINT13020` reader - Enable GPIO154 Interrupt To INT#130_20"]
pub type EnblGpio154inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO154INTToINT13020` writer - Enable GPIO154 Interrupt To INT#130_20"]
pub type EnblGpio154inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO154INTToSIO` reader - Enable GPIO154 Interrupt To SIO"]
pub type EnblGpio154inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO154INTToSIO` writer - Enable GPIO154 Interrupt To SIO"]
pub type EnblGpio154inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO154 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio154inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio154inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio154inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO154INTTargetRstTolerance` reader - GPIO154 Interrupt Target Reset Tolerance"]
pub type Gpio154inttargetRstToleranceR = crate::BitReader<Gpio154inttargetRstTolerance>;
impl Gpio154inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio154inttargetRstTolerance {
        match self.bits {
            false => Gpio154inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio154inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio154inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio154inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO154INTTargetRstTolerance` writer - GPIO154 Interrupt Target Reset Tolerance"]
pub type Gpio154inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio154inttargetRstTolerance>;
impl<'a, REG> Gpio154inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio154inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio154inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO154INTTargetWrProt` reader - GPIO154 Interrupt Target Write Protection"]
pub type Gpio154inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO154INTTargetWrProt` writer - GPIO154 Interrupt Target Write Protection"]
pub type Gpio154inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO155INTToINT13018` reader - Enable GPIO155 Interrupt To INT#130_18"]
pub type EnblGpio155inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO155INTToINT13018` writer - Enable GPIO155 Interrupt To INT#130_18"]
pub type EnblGpio155inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO155INTToINT13019` reader - Enable GPIO155 Interrupt To INT#130_19"]
pub type EnblGpio155inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO155INTToINT13019` writer - Enable GPIO155 Interrupt To INT#130_19"]
pub type EnblGpio155inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO155INTToINT13020` reader - Enable GPIO155 Interrupt To INT#130_20"]
pub type EnblGpio155inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO155INTToINT13020` writer - Enable GPIO155 Interrupt To INT#130_20"]
pub type EnblGpio155inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO155INTToSIO` reader - Enable GPIO155 Interrupt To SIO"]
pub type EnblGpio155inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO155INTToSIO` writer - Enable GPIO155 Interrupt To SIO"]
pub type EnblGpio155inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO155 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio155inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio155inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio155inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO155INTTargetRstTolerance` reader - GPIO155 Interrupt Target Reset Tolerance"]
pub type Gpio155inttargetRstToleranceR = crate::BitReader<Gpio155inttargetRstTolerance>;
impl Gpio155inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio155inttargetRstTolerance {
        match self.bits {
            false => Gpio155inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio155inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio155inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio155inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO155INTTargetRstTolerance` writer - GPIO155 Interrupt Target Reset Tolerance"]
pub type Gpio155inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio155inttargetRstTolerance>;
impl<'a, REG> Gpio155inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio155inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio155inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO155INTTargetWrProt` reader - GPIO155 Interrupt Target Write Protection"]
pub type Gpio155inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO155INTTargetWrProt` writer - GPIO155 Interrupt Target Write Protection"]
pub type Gpio155inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO152 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio152intto_int13018(&self) -> EnblGpio152inttoInt13018R {
        EnblGpio152inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO152 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio152intto_int13019(&self) -> EnblGpio152inttoInt13019R {
        EnblGpio152inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO152 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio152intto_int13020(&self) -> EnblGpio152inttoInt13020R {
        EnblGpio152inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO152 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio152intto_sio(&self) -> EnblGpio152inttoSioR {
        EnblGpio152inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO152 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio152inttarget_rst_tolerance(&self) -> Gpio152inttargetRstToleranceR {
        Gpio152inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO152 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio152inttarget_wr_prot(&self) -> Gpio152inttargetWrProtR {
        Gpio152inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO153 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio153intto_int13018(&self) -> EnblGpio153inttoInt13018R {
        EnblGpio153inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO153 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio153intto_int13019(&self) -> EnblGpio153inttoInt13019R {
        EnblGpio153inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO153 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio153intto_int13020(&self) -> EnblGpio153inttoInt13020R {
        EnblGpio153inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO153 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio153intto_sio(&self) -> EnblGpio153inttoSioR {
        EnblGpio153inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO153 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio153inttarget_rst_tolerance(&self) -> Gpio153inttargetRstToleranceR {
        Gpio153inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO153 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio153inttarget_wr_prot(&self) -> Gpio153inttargetWrProtR {
        Gpio153inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO154 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio154intto_int13018(&self) -> EnblGpio154inttoInt13018R {
        EnblGpio154inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO154 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio154intto_int13019(&self) -> EnblGpio154inttoInt13019R {
        EnblGpio154inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO154 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio154intto_int13020(&self) -> EnblGpio154inttoInt13020R {
        EnblGpio154inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO154 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio154intto_sio(&self) -> EnblGpio154inttoSioR {
        EnblGpio154inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO154 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio154inttarget_rst_tolerance(&self) -> Gpio154inttargetRstToleranceR {
        Gpio154inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO154 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio154inttarget_wr_prot(&self) -> Gpio154inttargetWrProtR {
        Gpio154inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO155 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio155intto_int13018(&self) -> EnblGpio155inttoInt13018R {
        EnblGpio155inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO155 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio155intto_int13019(&self) -> EnblGpio155inttoInt13019R {
        EnblGpio155inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO155 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio155intto_int13020(&self) -> EnblGpio155inttoInt13020R {
        EnblGpio155inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO155 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio155intto_sio(&self) -> EnblGpio155inttoSioR {
        EnblGpio155inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO155 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio155inttarget_rst_tolerance(&self) -> Gpio155inttargetRstToleranceR {
        Gpio155inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO155 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio155inttarget_wr_prot(&self) -> Gpio155inttargetWrProtR {
        Gpio155inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO152 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio152intto_int13018(&mut self) -> EnblGpio152inttoInt13018W<Gpioaa8Spec> {
        EnblGpio152inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO152 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio152intto_int13019(&mut self) -> EnblGpio152inttoInt13019W<Gpioaa8Spec> {
        EnblGpio152inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO152 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio152intto_int13020(&mut self) -> EnblGpio152inttoInt13020W<Gpioaa8Spec> {
        EnblGpio152inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO152 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio152intto_sio(&mut self) -> EnblGpio152inttoSioW<Gpioaa8Spec> {
        EnblGpio152inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioaa8Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioaa8Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO152 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio152inttarget_rst_tolerance(&mut self) -> Gpio152inttargetRstToleranceW<Gpioaa8Spec> {
        Gpio152inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO152 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio152inttarget_wr_prot(&mut self) -> Gpio152inttargetWrProtW<Gpioaa8Spec> {
        Gpio152inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO153 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio153intto_int13018(&mut self) -> EnblGpio153inttoInt13018W<Gpioaa8Spec> {
        EnblGpio153inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO153 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio153intto_int13019(&mut self) -> EnblGpio153inttoInt13019W<Gpioaa8Spec> {
        EnblGpio153inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO153 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio153intto_int13020(&mut self) -> EnblGpio153inttoInt13020W<Gpioaa8Spec> {
        EnblGpio153inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO153 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio153intto_sio(&mut self) -> EnblGpio153inttoSioW<Gpioaa8Spec> {
        EnblGpio153inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioaa8Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioaa8Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO153 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio153inttarget_rst_tolerance(&mut self) -> Gpio153inttargetRstToleranceW<Gpioaa8Spec> {
        Gpio153inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO153 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio153inttarget_wr_prot(&mut self) -> Gpio153inttargetWrProtW<Gpioaa8Spec> {
        Gpio153inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO154 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio154intto_int13018(&mut self) -> EnblGpio154inttoInt13018W<Gpioaa8Spec> {
        EnblGpio154inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO154 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio154intto_int13019(&mut self) -> EnblGpio154inttoInt13019W<Gpioaa8Spec> {
        EnblGpio154inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO154 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio154intto_int13020(&mut self) -> EnblGpio154inttoInt13020W<Gpioaa8Spec> {
        EnblGpio154inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO154 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio154intto_sio(&mut self) -> EnblGpio154inttoSioW<Gpioaa8Spec> {
        EnblGpio154inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioaa8Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioaa8Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO154 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio154inttarget_rst_tolerance(&mut self) -> Gpio154inttargetRstToleranceW<Gpioaa8Spec> {
        Gpio154inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO154 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio154inttarget_wr_prot(&mut self) -> Gpio154inttargetWrProtW<Gpioaa8Spec> {
        Gpio154inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO155 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio155intto_int13018(&mut self) -> EnblGpio155inttoInt13018W<Gpioaa8Spec> {
        EnblGpio155inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO155 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio155intto_int13019(&mut self) -> EnblGpio155inttoInt13019W<Gpioaa8Spec> {
        EnblGpio155inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO155 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio155intto_int13020(&mut self) -> EnblGpio155inttoInt13020W<Gpioaa8Spec> {
        EnblGpio155inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO155 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio155intto_sio(&mut self) -> EnblGpio155inttoSioW<Gpioaa8Spec> {
        EnblGpio155inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioaa8Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO155 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio155inttarget_rst_tolerance(&mut self) -> Gpio155inttargetRstToleranceW<Gpioaa8Spec> {
        Gpio155inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO155 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio155inttarget_wr_prot(&mut self) -> Gpio155inttargetWrProtW<Gpioaa8Spec> {
        Gpio155inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#38\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioaa8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioaa8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioaa8Spec;
impl crate::RegisterSpec for Gpioaa8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioaa8::R`](R) reader structure"]
impl crate::Readable for Gpioaa8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioaa8::W`](W) writer structure"]
impl crate::Writable for Gpioaa8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOAA8 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioaa8Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
