#[doc = "Register `GPIOA20` reader"]
pub type R = crate::R<Gpioa20Spec>;
#[doc = "Register `GPIOA20` writer"]
pub type W = crate::W<Gpioa20Spec>;
#[doc = "Field `EnblGPIO016INTToINT13018` reader - Enable GPIO016 Interrupt To INT#130_18"]
pub type EnblGpio016inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO016INTToINT13018` writer - Enable GPIO016 Interrupt To INT#130_18"]
pub type EnblGpio016inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO016INTToINT13019` reader - Enable GPIO016 Interrupt To INT#130_19"]
pub type EnblGpio016inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO016INTToINT13019` writer - Enable GPIO016 Interrupt To INT#130_19"]
pub type EnblGpio016inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO016INTToINT13020` reader - Enable GPIO016 Interrupt To INT#130_20"]
pub type EnblGpio016inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO016INTToINT13020` writer - Enable GPIO016 Interrupt To INT#130_20"]
pub type EnblGpio016inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO016INTToSIO` reader - Enable GPIO016 Interrupt To SIO"]
pub type EnblGpio016inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO016INTToSIO` writer - Enable GPIO016 Interrupt To SIO"]
pub type EnblGpio016inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO016 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio016inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio016inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio016inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO016INTTargetRstTolerance` reader - GPIO016 Interrupt Target Reset Tolerance"]
pub type Gpio016inttargetRstToleranceR = crate::BitReader<Gpio016inttargetRstTolerance>;
impl Gpio016inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio016inttargetRstTolerance {
        match self.bits {
            false => Gpio016inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio016inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio016inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio016inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO016INTTargetRstTolerance` writer - GPIO016 Interrupt Target Reset Tolerance"]
pub type Gpio016inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio016inttargetRstTolerance>;
impl<'a, REG> Gpio016inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio016inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio016inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO016INTTargetWrProt` reader - GPIO016 Interrupt Target Write Protection"]
pub type Gpio016inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO016INTTargetWrProt` writer - GPIO016 Interrupt Target Write Protection"]
pub type Gpio016inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO017INTToINT13018` reader - Enable GPIO017 Interrupt To INT#130_18"]
pub type EnblGpio017inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO017INTToINT13018` writer - Enable GPIO017 Interrupt To INT#130_18"]
pub type EnblGpio017inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO017INTToINT13019` reader - Enable GPIO017 Interrupt To INT#130_19"]
pub type EnblGpio017inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO017INTToINT13019` writer - Enable GPIO017 Interrupt To INT#130_19"]
pub type EnblGpio017inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO017INTToINT13020` reader - Enable GPIO017 Interrupt To INT#130_20"]
pub type EnblGpio017inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO017INTToINT13020` writer - Enable GPIO017 Interrupt To INT#130_20"]
pub type EnblGpio017inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO017INTToSIO` reader - Enable GPIO017 Interrupt To SIO"]
pub type EnblGpio017inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO017INTToSIO` writer - Enable GPIO017 Interrupt To SIO"]
pub type EnblGpio017inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO017 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio017inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio017inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio017inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO017INTTargetRstTolerance` reader - GPIO017 Interrupt Target Reset Tolerance"]
pub type Gpio017inttargetRstToleranceR = crate::BitReader<Gpio017inttargetRstTolerance>;
impl Gpio017inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio017inttargetRstTolerance {
        match self.bits {
            false => Gpio017inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio017inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio017inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio017inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO017INTTargetRstTolerance` writer - GPIO017 Interrupt Target Reset Tolerance"]
pub type Gpio017inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio017inttargetRstTolerance>;
impl<'a, REG> Gpio017inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio017inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio017inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO017INTTargetWrProt` reader - GPIO017 Interrupt Target Write Protection"]
pub type Gpio017inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO017INTTargetWrProt` writer - GPIO017 Interrupt Target Write Protection"]
pub type Gpio017inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO018INTToINT13018` reader - Enable GPIO018 Interrupt To INT#130_18"]
pub type EnblGpio018inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO018INTToINT13018` writer - Enable GPIO018 Interrupt To INT#130_18"]
pub type EnblGpio018inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO018INTToINT13019` reader - Enable GPIO018 Interrupt To INT#130_19"]
pub type EnblGpio018inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO018INTToINT13019` writer - Enable GPIO018 Interrupt To INT#130_19"]
pub type EnblGpio018inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO018INTToINT13020` reader - Enable GPIO018 Interrupt To INT#130_20"]
pub type EnblGpio018inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO018INTToINT13020` writer - Enable GPIO018 Interrupt To INT#130_20"]
pub type EnblGpio018inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO018INTToSIO` reader - Enable GPIO018 Interrupt To SIO"]
pub type EnblGpio018inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO018INTToSIO` writer - Enable GPIO018 Interrupt To SIO"]
pub type EnblGpio018inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO018 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio018inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio018inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio018inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO018INTTargetRstTolerance` reader - GPIO018 Interrupt Target Reset Tolerance"]
pub type Gpio018inttargetRstToleranceR = crate::BitReader<Gpio018inttargetRstTolerance>;
impl Gpio018inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio018inttargetRstTolerance {
        match self.bits {
            false => Gpio018inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio018inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio018inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio018inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO018INTTargetRstTolerance` writer - GPIO018 Interrupt Target Reset Tolerance"]
pub type Gpio018inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio018inttargetRstTolerance>;
impl<'a, REG> Gpio018inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio018inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio018inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO018INTTargetWrProt` reader - GPIO018 Interrupt Target Write Protection"]
pub type Gpio018inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO018INTTargetWrProt` writer - GPIO018 Interrupt Target Write Protection"]
pub type Gpio018inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO019INTToINT13018` reader - Enable GPIO019 Interrupt To INT#130_18"]
pub type EnblGpio019inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO019INTToINT13018` writer - Enable GPIO019 Interrupt To INT#130_18"]
pub type EnblGpio019inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO019INTToINT13019` reader - Enable GPIO019 Interrupt To INT#130_19"]
pub type EnblGpio019inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO019INTToINT13019` writer - Enable GPIO019 Interrupt To INT#130_19"]
pub type EnblGpio019inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO019INTToINT13020` reader - Enable GPIO019 Interrupt To INT#130_20"]
pub type EnblGpio019inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO019INTToINT13020` writer - Enable GPIO019 Interrupt To INT#130_20"]
pub type EnblGpio019inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO019INTToSIO` reader - Enable GPIO019 Interrupt To SIO"]
pub type EnblGpio019inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO019INTToSIO` writer - Enable GPIO019 Interrupt To SIO"]
pub type EnblGpio019inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO019 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio019inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio019inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio019inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO019INTTargetRstTolerance` reader - GPIO019 Interrupt Target Reset Tolerance"]
pub type Gpio019inttargetRstToleranceR = crate::BitReader<Gpio019inttargetRstTolerance>;
impl Gpio019inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio019inttargetRstTolerance {
        match self.bits {
            false => Gpio019inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio019inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio019inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio019inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO019INTTargetRstTolerance` writer - GPIO019 Interrupt Target Reset Tolerance"]
pub type Gpio019inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio019inttargetRstTolerance>;
impl<'a, REG> Gpio019inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio019inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio019inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO019INTTargetWrProt` reader - GPIO019 Interrupt Target Write Protection"]
pub type Gpio019inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO019INTTargetWrProt` writer - GPIO019 Interrupt Target Write Protection"]
pub type Gpio019inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO016 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio016intto_int13018(&self) -> EnblGpio016inttoInt13018R {
        EnblGpio016inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO016 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio016intto_int13019(&self) -> EnblGpio016inttoInt13019R {
        EnblGpio016inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO016 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio016intto_int13020(&self) -> EnblGpio016inttoInt13020R {
        EnblGpio016inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO016 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio016intto_sio(&self) -> EnblGpio016inttoSioR {
        EnblGpio016inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO016 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio016inttarget_rst_tolerance(&self) -> Gpio016inttargetRstToleranceR {
        Gpio016inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO016 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio016inttarget_wr_prot(&self) -> Gpio016inttargetWrProtR {
        Gpio016inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO017 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio017intto_int13018(&self) -> EnblGpio017inttoInt13018R {
        EnblGpio017inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO017 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio017intto_int13019(&self) -> EnblGpio017inttoInt13019R {
        EnblGpio017inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO017 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio017intto_int13020(&self) -> EnblGpio017inttoInt13020R {
        EnblGpio017inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO017 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio017intto_sio(&self) -> EnblGpio017inttoSioR {
        EnblGpio017inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO017 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio017inttarget_rst_tolerance(&self) -> Gpio017inttargetRstToleranceR {
        Gpio017inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO017 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio017inttarget_wr_prot(&self) -> Gpio017inttargetWrProtR {
        Gpio017inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO018 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio018intto_int13018(&self) -> EnblGpio018inttoInt13018R {
        EnblGpio018inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO018 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio018intto_int13019(&self) -> EnblGpio018inttoInt13019R {
        EnblGpio018inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO018 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio018intto_int13020(&self) -> EnblGpio018inttoInt13020R {
        EnblGpio018inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO018 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio018intto_sio(&self) -> EnblGpio018inttoSioR {
        EnblGpio018inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO018 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio018inttarget_rst_tolerance(&self) -> Gpio018inttargetRstToleranceR {
        Gpio018inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO018 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio018inttarget_wr_prot(&self) -> Gpio018inttargetWrProtR {
        Gpio018inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO019 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio019intto_int13018(&self) -> EnblGpio019inttoInt13018R {
        EnblGpio019inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO019 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio019intto_int13019(&self) -> EnblGpio019inttoInt13019R {
        EnblGpio019inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO019 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio019intto_int13020(&self) -> EnblGpio019inttoInt13020R {
        EnblGpio019inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO019 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio019intto_sio(&self) -> EnblGpio019inttoSioR {
        EnblGpio019inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO019 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio019inttarget_rst_tolerance(&self) -> Gpio019inttargetRstToleranceR {
        Gpio019inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO019 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio019inttarget_wr_prot(&self) -> Gpio019inttargetWrProtR {
        Gpio019inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO016 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio016intto_int13018(&mut self) -> EnblGpio016inttoInt13018W<Gpioa20Spec> {
        EnblGpio016inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO016 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio016intto_int13019(&mut self) -> EnblGpio016inttoInt13019W<Gpioa20Spec> {
        EnblGpio016inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO016 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio016intto_int13020(&mut self) -> EnblGpio016inttoInt13020W<Gpioa20Spec> {
        EnblGpio016inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO016 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio016intto_sio(&mut self) -> EnblGpio016inttoSioW<Gpioa20Spec> {
        EnblGpio016inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa20Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa20Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO016 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio016inttarget_rst_tolerance(&mut self) -> Gpio016inttargetRstToleranceW<Gpioa20Spec> {
        Gpio016inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO016 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio016inttarget_wr_prot(&mut self) -> Gpio016inttargetWrProtW<Gpioa20Spec> {
        Gpio016inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO017 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio017intto_int13018(&mut self) -> EnblGpio017inttoInt13018W<Gpioa20Spec> {
        EnblGpio017inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO017 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio017intto_int13019(&mut self) -> EnblGpio017inttoInt13019W<Gpioa20Spec> {
        EnblGpio017inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO017 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio017intto_int13020(&mut self) -> EnblGpio017inttoInt13020W<Gpioa20Spec> {
        EnblGpio017inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO017 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio017intto_sio(&mut self) -> EnblGpio017inttoSioW<Gpioa20Spec> {
        EnblGpio017inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa20Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa20Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO017 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio017inttarget_rst_tolerance(&mut self) -> Gpio017inttargetRstToleranceW<Gpioa20Spec> {
        Gpio017inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO017 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio017inttarget_wr_prot(&mut self) -> Gpio017inttargetWrProtW<Gpioa20Spec> {
        Gpio017inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO018 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio018intto_int13018(&mut self) -> EnblGpio018inttoInt13018W<Gpioa20Spec> {
        EnblGpio018inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO018 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio018intto_int13019(&mut self) -> EnblGpio018inttoInt13019W<Gpioa20Spec> {
        EnblGpio018inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO018 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio018intto_int13020(&mut self) -> EnblGpio018inttoInt13020W<Gpioa20Spec> {
        EnblGpio018inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO018 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio018intto_sio(&mut self) -> EnblGpio018inttoSioW<Gpioa20Spec> {
        EnblGpio018inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa20Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa20Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO018 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio018inttarget_rst_tolerance(&mut self) -> Gpio018inttargetRstToleranceW<Gpioa20Spec> {
        Gpio018inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO018 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio018inttarget_wr_prot(&mut self) -> Gpio018inttargetWrProtW<Gpioa20Spec> {
        Gpio018inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO019 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio019intto_int13018(&mut self) -> EnblGpio019inttoInt13018W<Gpioa20Spec> {
        EnblGpio019inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO019 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio019intto_int13019(&mut self) -> EnblGpio019inttoInt13019W<Gpioa20Spec> {
        EnblGpio019inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO019 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio019intto_int13020(&mut self) -> EnblGpio019inttoInt13020W<Gpioa20Spec> {
        EnblGpio019inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO019 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio019intto_sio(&mut self) -> EnblGpio019inttoSioW<Gpioa20Spec> {
        EnblGpio019inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa20Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO019 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio019inttarget_rst_tolerance(&mut self) -> Gpio019inttargetRstToleranceW<Gpioa20Spec> {
        Gpio019inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO019 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio019inttarget_wr_prot(&mut self) -> Gpio019inttargetWrProtW<Gpioa20Spec> {
        Gpio019inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa20Spec;
impl crate::RegisterSpec for Gpioa20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa20::R`](R) reader structure"]
impl crate::Readable for Gpioa20Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa20::W`](W) writer structure"]
impl crate::Writable for Gpioa20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA20 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa20Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
