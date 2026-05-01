#[doc = "Register `GPIOA58` reader"]
pub type R = crate::R<Gpioa58Spec>;
#[doc = "Register `GPIOA58` writer"]
pub type W = crate::W<Gpioa58Spec>;
#[doc = "Field `EnblGPIO072INTToINT13018` reader - Enable GPIO072 Interrupt To INT#130_18"]
pub type EnblGpio072inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO072INTToINT13018` writer - Enable GPIO072 Interrupt To INT#130_18"]
pub type EnblGpio072inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO072INTToINT13019` reader - Enable GPIO072 Interrupt To INT#130_19"]
pub type EnblGpio072inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO072INTToINT13019` writer - Enable GPIO072 Interrupt To INT#130_19"]
pub type EnblGpio072inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO072INTToINT13020` reader - Enable GPIO072 Interrupt To INT#130_20"]
pub type EnblGpio072inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO072INTToINT13020` writer - Enable GPIO072 Interrupt To INT#130_20"]
pub type EnblGpio072inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO072INTToSIO` reader - Enable GPIO072 Interrupt To SIO"]
pub type EnblGpio072inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO072INTToSIO` writer - Enable GPIO072 Interrupt To SIO"]
pub type EnblGpio072inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO072 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio072inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio072inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio072inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO072INTTargetRstTolerance` reader - GPIO072 Interrupt Target Reset Tolerance"]
pub type Gpio072inttargetRstToleranceR = crate::BitReader<Gpio072inttargetRstTolerance>;
impl Gpio072inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio072inttargetRstTolerance {
        match self.bits {
            false => Gpio072inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio072inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio072inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio072inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO072INTTargetRstTolerance` writer - GPIO072 Interrupt Target Reset Tolerance"]
pub type Gpio072inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio072inttargetRstTolerance>;
impl<'a, REG> Gpio072inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio072inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio072inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO072INTTargetWrProt` reader - GPIO072 Interrupt Target Write Protection"]
pub type Gpio072inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO072INTTargetWrProt` writer - GPIO072 Interrupt Target Write Protection"]
pub type Gpio072inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO073INTToINT13018` reader - Enable GPIO073 Interrupt To INT#130_18"]
pub type EnblGpio073inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO073INTToINT13018` writer - Enable GPIO073 Interrupt To INT#130_18"]
pub type EnblGpio073inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO073INTToINT13019` reader - Enable GPIO073 Interrupt To INT#130_19"]
pub type EnblGpio073inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO073INTToINT13019` writer - Enable GPIO073 Interrupt To INT#130_19"]
pub type EnblGpio073inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO073INTToINT13020` reader - Enable GPIO073 Interrupt To INT#130_20"]
pub type EnblGpio073inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO073INTToINT13020` writer - Enable GPIO073 Interrupt To INT#130_20"]
pub type EnblGpio073inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO073INTToSIO` reader - Enable GPIO073 Interrupt To SIO"]
pub type EnblGpio073inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO073INTToSIO` writer - Enable GPIO073 Interrupt To SIO"]
pub type EnblGpio073inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO073 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio073inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio073inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio073inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO073INTTargetRstTolerance` reader - GPIO073 Interrupt Target Reset Tolerance"]
pub type Gpio073inttargetRstToleranceR = crate::BitReader<Gpio073inttargetRstTolerance>;
impl Gpio073inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio073inttargetRstTolerance {
        match self.bits {
            false => Gpio073inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio073inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio073inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio073inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO073INTTargetRstTolerance` writer - GPIO073 Interrupt Target Reset Tolerance"]
pub type Gpio073inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio073inttargetRstTolerance>;
impl<'a, REG> Gpio073inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio073inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio073inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO073INTTargetWrProt` reader - GPIO073 Interrupt Target Write Protection"]
pub type Gpio073inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO073INTTargetWrProt` writer - GPIO073 Interrupt Target Write Protection"]
pub type Gpio073inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO074INTToINT13018` reader - Enable GPIO074 Interrupt To INT#130_18"]
pub type EnblGpio074inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO074INTToINT13018` writer - Enable GPIO074 Interrupt To INT#130_18"]
pub type EnblGpio074inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO074INTToINT13019` reader - Enable GPIO074 Interrupt To INT#130_19"]
pub type EnblGpio074inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO074INTToINT13019` writer - Enable GPIO074 Interrupt To INT#130_19"]
pub type EnblGpio074inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO074INTToINT13020` reader - Enable GPIO074 Interrupt To INT#130_20"]
pub type EnblGpio074inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO074INTToINT13020` writer - Enable GPIO074 Interrupt To INT#130_20"]
pub type EnblGpio074inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO074INTToSIO` reader - Enable GPIO074 Interrupt To SIO"]
pub type EnblGpio074inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO074INTToSIO` writer - Enable GPIO074 Interrupt To SIO"]
pub type EnblGpio074inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO074 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio074inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio074inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio074inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO074INTTargetRstTolerance` reader - GPIO074 Interrupt Target Reset Tolerance"]
pub type Gpio074inttargetRstToleranceR = crate::BitReader<Gpio074inttargetRstTolerance>;
impl Gpio074inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio074inttargetRstTolerance {
        match self.bits {
            false => Gpio074inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio074inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio074inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio074inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO074INTTargetRstTolerance` writer - GPIO074 Interrupt Target Reset Tolerance"]
pub type Gpio074inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio074inttargetRstTolerance>;
impl<'a, REG> Gpio074inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio074inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio074inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO074INTTargetWrProt` reader - GPIO074 Interrupt Target Write Protection"]
pub type Gpio074inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO074INTTargetWrProt` writer - GPIO074 Interrupt Target Write Protection"]
pub type Gpio074inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO075INTToINT13018` reader - Enable GPIO075 Interrupt To INT#130_18"]
pub type EnblGpio075inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO075INTToINT13018` writer - Enable GPIO075 Interrupt To INT#130_18"]
pub type EnblGpio075inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO075INTToINT13019` reader - Enable GPIO075 Interrupt To INT#130_19"]
pub type EnblGpio075inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO075INTToINT13019` writer - Enable GPIO075 Interrupt To INT#130_19"]
pub type EnblGpio075inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO075INTToINT13020` reader - Enable GPIO075 Interrupt To INT#130_20"]
pub type EnblGpio075inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO075INTToINT13020` writer - Enable GPIO075 Interrupt To INT#130_20"]
pub type EnblGpio075inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO075INTToSIO` reader - Enable GPIO075 Interrupt To SIO"]
pub type EnblGpio075inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO075INTToSIO` writer - Enable GPIO075 Interrupt To SIO"]
pub type EnblGpio075inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO075 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio075inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio075inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio075inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO075INTTargetRstTolerance` reader - GPIO075 Interrupt Target Reset Tolerance"]
pub type Gpio075inttargetRstToleranceR = crate::BitReader<Gpio075inttargetRstTolerance>;
impl Gpio075inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio075inttargetRstTolerance {
        match self.bits {
            false => Gpio075inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio075inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio075inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio075inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO075INTTargetRstTolerance` writer - GPIO075 Interrupt Target Reset Tolerance"]
pub type Gpio075inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio075inttargetRstTolerance>;
impl<'a, REG> Gpio075inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio075inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio075inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO075INTTargetWrProt` reader - GPIO075 Interrupt Target Write Protection"]
pub type Gpio075inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO075INTTargetWrProt` writer - GPIO075 Interrupt Target Write Protection"]
pub type Gpio075inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO072 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio072intto_int13018(&self) -> EnblGpio072inttoInt13018R {
        EnblGpio072inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO072 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio072intto_int13019(&self) -> EnblGpio072inttoInt13019R {
        EnblGpio072inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO072 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio072intto_int13020(&self) -> EnblGpio072inttoInt13020R {
        EnblGpio072inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO072 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio072intto_sio(&self) -> EnblGpio072inttoSioR {
        EnblGpio072inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO072 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio072inttarget_rst_tolerance(&self) -> Gpio072inttargetRstToleranceR {
        Gpio072inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO072 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio072inttarget_wr_prot(&self) -> Gpio072inttargetWrProtR {
        Gpio072inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO073 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio073intto_int13018(&self) -> EnblGpio073inttoInt13018R {
        EnblGpio073inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO073 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio073intto_int13019(&self) -> EnblGpio073inttoInt13019R {
        EnblGpio073inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO073 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio073intto_int13020(&self) -> EnblGpio073inttoInt13020R {
        EnblGpio073inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO073 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio073intto_sio(&self) -> EnblGpio073inttoSioR {
        EnblGpio073inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO073 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio073inttarget_rst_tolerance(&self) -> Gpio073inttargetRstToleranceR {
        Gpio073inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO073 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio073inttarget_wr_prot(&self) -> Gpio073inttargetWrProtR {
        Gpio073inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO074 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio074intto_int13018(&self) -> EnblGpio074inttoInt13018R {
        EnblGpio074inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO074 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio074intto_int13019(&self) -> EnblGpio074inttoInt13019R {
        EnblGpio074inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO074 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio074intto_int13020(&self) -> EnblGpio074inttoInt13020R {
        EnblGpio074inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO074 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio074intto_sio(&self) -> EnblGpio074inttoSioR {
        EnblGpio074inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO074 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio074inttarget_rst_tolerance(&self) -> Gpio074inttargetRstToleranceR {
        Gpio074inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO074 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio074inttarget_wr_prot(&self) -> Gpio074inttargetWrProtR {
        Gpio074inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO075 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio075intto_int13018(&self) -> EnblGpio075inttoInt13018R {
        EnblGpio075inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO075 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio075intto_int13019(&self) -> EnblGpio075inttoInt13019R {
        EnblGpio075inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO075 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio075intto_int13020(&self) -> EnblGpio075inttoInt13020R {
        EnblGpio075inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO075 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio075intto_sio(&self) -> EnblGpio075inttoSioR {
        EnblGpio075inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO075 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio075inttarget_rst_tolerance(&self) -> Gpio075inttargetRstToleranceR {
        Gpio075inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO075 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio075inttarget_wr_prot(&self) -> Gpio075inttargetWrProtR {
        Gpio075inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO072 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio072intto_int13018(&mut self) -> EnblGpio072inttoInt13018W<Gpioa58Spec> {
        EnblGpio072inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO072 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio072intto_int13019(&mut self) -> EnblGpio072inttoInt13019W<Gpioa58Spec> {
        EnblGpio072inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO072 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio072intto_int13020(&mut self) -> EnblGpio072inttoInt13020W<Gpioa58Spec> {
        EnblGpio072inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO072 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio072intto_sio(&mut self) -> EnblGpio072inttoSioW<Gpioa58Spec> {
        EnblGpio072inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa58Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa58Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO072 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio072inttarget_rst_tolerance(&mut self) -> Gpio072inttargetRstToleranceW<Gpioa58Spec> {
        Gpio072inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO072 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio072inttarget_wr_prot(&mut self) -> Gpio072inttargetWrProtW<Gpioa58Spec> {
        Gpio072inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO073 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio073intto_int13018(&mut self) -> EnblGpio073inttoInt13018W<Gpioa58Spec> {
        EnblGpio073inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO073 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio073intto_int13019(&mut self) -> EnblGpio073inttoInt13019W<Gpioa58Spec> {
        EnblGpio073inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO073 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio073intto_int13020(&mut self) -> EnblGpio073inttoInt13020W<Gpioa58Spec> {
        EnblGpio073inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO073 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio073intto_sio(&mut self) -> EnblGpio073inttoSioW<Gpioa58Spec> {
        EnblGpio073inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa58Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa58Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO073 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio073inttarget_rst_tolerance(&mut self) -> Gpio073inttargetRstToleranceW<Gpioa58Spec> {
        Gpio073inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO073 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio073inttarget_wr_prot(&mut self) -> Gpio073inttargetWrProtW<Gpioa58Spec> {
        Gpio073inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO074 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio074intto_int13018(&mut self) -> EnblGpio074inttoInt13018W<Gpioa58Spec> {
        EnblGpio074inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO074 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio074intto_int13019(&mut self) -> EnblGpio074inttoInt13019W<Gpioa58Spec> {
        EnblGpio074inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO074 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio074intto_int13020(&mut self) -> EnblGpio074inttoInt13020W<Gpioa58Spec> {
        EnblGpio074inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO074 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio074intto_sio(&mut self) -> EnblGpio074inttoSioW<Gpioa58Spec> {
        EnblGpio074inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa58Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa58Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO074 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio074inttarget_rst_tolerance(&mut self) -> Gpio074inttargetRstToleranceW<Gpioa58Spec> {
        Gpio074inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO074 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio074inttarget_wr_prot(&mut self) -> Gpio074inttargetWrProtW<Gpioa58Spec> {
        Gpio074inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO075 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio075intto_int13018(&mut self) -> EnblGpio075inttoInt13018W<Gpioa58Spec> {
        EnblGpio075inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO075 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio075intto_int13019(&mut self) -> EnblGpio075inttoInt13019W<Gpioa58Spec> {
        EnblGpio075inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO075 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio075intto_int13020(&mut self) -> EnblGpio075inttoInt13020W<Gpioa58Spec> {
        EnblGpio075inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO075 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio075intto_sio(&mut self) -> EnblGpio075inttoSioW<Gpioa58Spec> {
        EnblGpio075inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa58Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO075 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio075inttarget_rst_tolerance(&mut self) -> Gpio075inttargetRstToleranceW<Gpioa58Spec> {
        Gpio075inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO075 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio075inttarget_wr_prot(&mut self) -> Gpio075inttargetWrProtW<Gpioa58Spec> {
        Gpio075inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa58Spec;
impl crate::RegisterSpec for Gpioa58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa58::R`](R) reader structure"]
impl crate::Readable for Gpioa58Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa58::W`](W) writer structure"]
impl crate::Writable for Gpioa58Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA58 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa58Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
