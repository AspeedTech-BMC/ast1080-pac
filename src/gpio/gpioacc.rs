#[doc = "Register `GPIOACC` reader"]
pub type R = crate::R<GpioaccSpec>;
#[doc = "Register `GPIOACC` writer"]
pub type W = crate::W<GpioaccSpec>;
#[doc = "Field `EnblGPIO188INTToINT13018` reader - Enable GPIO188 Interrupt To INT#130_18"]
pub type EnblGpio188inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO188INTToINT13018` writer - Enable GPIO188 Interrupt To INT#130_18"]
pub type EnblGpio188inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO188INTToINT13019` reader - Enable GPIO188 Interrupt To INT#130_19"]
pub type EnblGpio188inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO188INTToINT13019` writer - Enable GPIO188 Interrupt To INT#130_19"]
pub type EnblGpio188inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO188INTToINT13020` reader - Enable GPIO188 Interrupt To INT#130_20"]
pub type EnblGpio188inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO188INTToINT13020` writer - Enable GPIO188 Interrupt To INT#130_20"]
pub type EnblGpio188inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO188INTToSIO` reader - Enable GPIO188 Interrupt To SIO"]
pub type EnblGpio188inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO188INTToSIO` writer - Enable GPIO188 Interrupt To SIO"]
pub type EnblGpio188inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO188 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio188inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio188inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio188inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO188INTTargetRstTolerance` reader - GPIO188 Interrupt Target Reset Tolerance"]
pub type Gpio188inttargetRstToleranceR = crate::BitReader<Gpio188inttargetRstTolerance>;
impl Gpio188inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio188inttargetRstTolerance {
        match self.bits {
            false => Gpio188inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio188inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio188inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio188inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO188INTTargetRstTolerance` writer - GPIO188 Interrupt Target Reset Tolerance"]
pub type Gpio188inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio188inttargetRstTolerance>;
impl<'a, REG> Gpio188inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio188inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio188inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO188INTTargetWrProt` reader - GPIO188 Interrupt Target Write Protection"]
pub type Gpio188inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO188INTTargetWrProt` writer - GPIO188 Interrupt Target Write Protection"]
pub type Gpio188inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO189INTToINT13018` reader - Enable GPIO189 Interrupt To INT#130_18"]
pub type EnblGpio189inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO189INTToINT13018` writer - Enable GPIO189 Interrupt To INT#130_18"]
pub type EnblGpio189inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO189INTToINT13019` reader - Enable GPIO189 Interrupt To INT#130_19"]
pub type EnblGpio189inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO189INTToINT13019` writer - Enable GPIO189 Interrupt To INT#130_19"]
pub type EnblGpio189inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO189INTToINT13020` reader - Enable GPIO189 Interrupt To INT#130_20"]
pub type EnblGpio189inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO189INTToINT13020` writer - Enable GPIO189 Interrupt To INT#130_20"]
pub type EnblGpio189inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO189INTToSIO` reader - Enable GPIO189 Interrupt To SIO"]
pub type EnblGpio189inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO189INTToSIO` writer - Enable GPIO189 Interrupt To SIO"]
pub type EnblGpio189inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO189 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio189inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio189inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio189inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO189INTTargetRstTolerance` reader - GPIO189 Interrupt Target Reset Tolerance"]
pub type Gpio189inttargetRstToleranceR = crate::BitReader<Gpio189inttargetRstTolerance>;
impl Gpio189inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio189inttargetRstTolerance {
        match self.bits {
            false => Gpio189inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio189inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio189inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio189inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO189INTTargetRstTolerance` writer - GPIO189 Interrupt Target Reset Tolerance"]
pub type Gpio189inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio189inttargetRstTolerance>;
impl<'a, REG> Gpio189inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio189inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio189inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO189INTTargetWrProt` reader - GPIO189 Interrupt Target Write Protection"]
pub type Gpio189inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO189INTTargetWrProt` writer - GPIO189 Interrupt Target Write Protection"]
pub type Gpio189inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO190INTToINT13018` reader - Enable GPIO190 Interrupt To INT#130_18"]
pub type EnblGpio190inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO190INTToINT13018` writer - Enable GPIO190 Interrupt To INT#130_18"]
pub type EnblGpio190inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO190INTToINT13019` reader - Enable GPIO190 Interrupt To INT#130_19"]
pub type EnblGpio190inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO190INTToINT13019` writer - Enable GPIO190 Interrupt To INT#130_19"]
pub type EnblGpio190inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO190INTToINT13020` reader - Enable GPIO190 Interrupt To INT#130_20"]
pub type EnblGpio190inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO190INTToINT13020` writer - Enable GPIO190 Interrupt To INT#130_20"]
pub type EnblGpio190inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO190INTToSIO` reader - Enable GPIO190 Interrupt To SIO"]
pub type EnblGpio190inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO190INTToSIO` writer - Enable GPIO190 Interrupt To SIO"]
pub type EnblGpio190inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO190 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio190inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio190inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio190inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO190INTTargetRstTolerance` reader - GPIO190 Interrupt Target Reset Tolerance"]
pub type Gpio190inttargetRstToleranceR = crate::BitReader<Gpio190inttargetRstTolerance>;
impl Gpio190inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio190inttargetRstTolerance {
        match self.bits {
            false => Gpio190inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio190inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio190inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio190inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO190INTTargetRstTolerance` writer - GPIO190 Interrupt Target Reset Tolerance"]
pub type Gpio190inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio190inttargetRstTolerance>;
impl<'a, REG> Gpio190inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio190inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio190inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO190INTTargetWrProt` reader - GPIO190 Interrupt Target Write Protection"]
pub type Gpio190inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO190INTTargetWrProt` writer - GPIO190 Interrupt Target Write Protection"]
pub type Gpio190inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO191INTToINT13018` reader - Enable GPIO191 Interrupt To INT#130_18"]
pub type EnblGpio191inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO191INTToINT13018` writer - Enable GPIO191 Interrupt To INT#130_18"]
pub type EnblGpio191inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO191INTToINT13019` reader - Enable GPIO191 Interrupt To INT#130_19"]
pub type EnblGpio191inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO191INTToINT13019` writer - Enable GPIO191 Interrupt To INT#130_19"]
pub type EnblGpio191inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO191INTToINT13020` reader - Enable GPIO191 Interrupt To INT#130_20"]
pub type EnblGpio191inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO191INTToINT13020` writer - Enable GPIO191 Interrupt To INT#130_20"]
pub type EnblGpio191inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO191INTToSIO` reader - Enable GPIO191 Interrupt To SIO"]
pub type EnblGpio191inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO191INTToSIO` writer - Enable GPIO191 Interrupt To SIO"]
pub type EnblGpio191inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO191 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio191inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio191inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio191inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO191INTTargetRstTolerance` reader - GPIO191 Interrupt Target Reset Tolerance"]
pub type Gpio191inttargetRstToleranceR = crate::BitReader<Gpio191inttargetRstTolerance>;
impl Gpio191inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio191inttargetRstTolerance {
        match self.bits {
            false => Gpio191inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio191inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio191inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio191inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO191INTTargetRstTolerance` writer - GPIO191 Interrupt Target Reset Tolerance"]
pub type Gpio191inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio191inttargetRstTolerance>;
impl<'a, REG> Gpio191inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio191inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio191inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO191INTTargetWrProt` reader - GPIO191 Interrupt Target Write Protection"]
pub type Gpio191inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO191INTTargetWrProt` writer - GPIO191 Interrupt Target Write Protection"]
pub type Gpio191inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO188 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio188intto_int13018(&self) -> EnblGpio188inttoInt13018R {
        EnblGpio188inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO188 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio188intto_int13019(&self) -> EnblGpio188inttoInt13019R {
        EnblGpio188inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO188 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio188intto_int13020(&self) -> EnblGpio188inttoInt13020R {
        EnblGpio188inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO188 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio188intto_sio(&self) -> EnblGpio188inttoSioR {
        EnblGpio188inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO188 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio188inttarget_rst_tolerance(&self) -> Gpio188inttargetRstToleranceR {
        Gpio188inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO188 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio188inttarget_wr_prot(&self) -> Gpio188inttargetWrProtR {
        Gpio188inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO189 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio189intto_int13018(&self) -> EnblGpio189inttoInt13018R {
        EnblGpio189inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO189 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio189intto_int13019(&self) -> EnblGpio189inttoInt13019R {
        EnblGpio189inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO189 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio189intto_int13020(&self) -> EnblGpio189inttoInt13020R {
        EnblGpio189inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO189 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio189intto_sio(&self) -> EnblGpio189inttoSioR {
        EnblGpio189inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO189 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio189inttarget_rst_tolerance(&self) -> Gpio189inttargetRstToleranceR {
        Gpio189inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO189 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio189inttarget_wr_prot(&self) -> Gpio189inttargetWrProtR {
        Gpio189inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO190 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio190intto_int13018(&self) -> EnblGpio190inttoInt13018R {
        EnblGpio190inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO190 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio190intto_int13019(&self) -> EnblGpio190inttoInt13019R {
        EnblGpio190inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO190 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio190intto_int13020(&self) -> EnblGpio190inttoInt13020R {
        EnblGpio190inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO190 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio190intto_sio(&self) -> EnblGpio190inttoSioR {
        EnblGpio190inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO190 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio190inttarget_rst_tolerance(&self) -> Gpio190inttargetRstToleranceR {
        Gpio190inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO190 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio190inttarget_wr_prot(&self) -> Gpio190inttargetWrProtR {
        Gpio190inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO191 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio191intto_int13018(&self) -> EnblGpio191inttoInt13018R {
        EnblGpio191inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO191 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio191intto_int13019(&self) -> EnblGpio191inttoInt13019R {
        EnblGpio191inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO191 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio191intto_int13020(&self) -> EnblGpio191inttoInt13020R {
        EnblGpio191inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO191 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio191intto_sio(&self) -> EnblGpio191inttoSioR {
        EnblGpio191inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO191 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio191inttarget_rst_tolerance(&self) -> Gpio191inttargetRstToleranceR {
        Gpio191inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO191 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio191inttarget_wr_prot(&self) -> Gpio191inttargetWrProtR {
        Gpio191inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO188 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio188intto_int13018(&mut self) -> EnblGpio188inttoInt13018W<GpioaccSpec> {
        EnblGpio188inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO188 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio188intto_int13019(&mut self) -> EnblGpio188inttoInt13019W<GpioaccSpec> {
        EnblGpio188inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO188 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio188intto_int13020(&mut self) -> EnblGpio188inttoInt13020W<GpioaccSpec> {
        EnblGpio188inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO188 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio188intto_sio(&mut self) -> EnblGpio188inttoSioW<GpioaccSpec> {
        EnblGpio188inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<GpioaccSpec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<GpioaccSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO188 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio188inttarget_rst_tolerance(&mut self) -> Gpio188inttargetRstToleranceW<GpioaccSpec> {
        Gpio188inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO188 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio188inttarget_wr_prot(&mut self) -> Gpio188inttargetWrProtW<GpioaccSpec> {
        Gpio188inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO189 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio189intto_int13018(&mut self) -> EnblGpio189inttoInt13018W<GpioaccSpec> {
        EnblGpio189inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO189 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio189intto_int13019(&mut self) -> EnblGpio189inttoInt13019W<GpioaccSpec> {
        EnblGpio189inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO189 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio189intto_int13020(&mut self) -> EnblGpio189inttoInt13020W<GpioaccSpec> {
        EnblGpio189inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO189 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio189intto_sio(&mut self) -> EnblGpio189inttoSioW<GpioaccSpec> {
        EnblGpio189inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<GpioaccSpec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<GpioaccSpec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO189 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio189inttarget_rst_tolerance(&mut self) -> Gpio189inttargetRstToleranceW<GpioaccSpec> {
        Gpio189inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO189 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio189inttarget_wr_prot(&mut self) -> Gpio189inttargetWrProtW<GpioaccSpec> {
        Gpio189inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO190 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio190intto_int13018(&mut self) -> EnblGpio190inttoInt13018W<GpioaccSpec> {
        EnblGpio190inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO190 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio190intto_int13019(&mut self) -> EnblGpio190inttoInt13019W<GpioaccSpec> {
        EnblGpio190inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO190 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio190intto_int13020(&mut self) -> EnblGpio190inttoInt13020W<GpioaccSpec> {
        EnblGpio190inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO190 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio190intto_sio(&mut self) -> EnblGpio190inttoSioW<GpioaccSpec> {
        EnblGpio190inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<GpioaccSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<GpioaccSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO190 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio190inttarget_rst_tolerance(&mut self) -> Gpio190inttargetRstToleranceW<GpioaccSpec> {
        Gpio190inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO190 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio190inttarget_wr_prot(&mut self) -> Gpio190inttargetWrProtW<GpioaccSpec> {
        Gpio190inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO191 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio191intto_int13018(&mut self) -> EnblGpio191inttoInt13018W<GpioaccSpec> {
        EnblGpio191inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO191 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio191intto_int13019(&mut self) -> EnblGpio191inttoInt13019W<GpioaccSpec> {
        EnblGpio191inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO191 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio191intto_int13020(&mut self) -> EnblGpio191inttoInt13020W<GpioaccSpec> {
        EnblGpio191inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO191 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio191intto_sio(&mut self) -> EnblGpio191inttoSioW<GpioaccSpec> {
        EnblGpio191inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<GpioaccSpec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO191 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio191inttarget_rst_tolerance(&mut self) -> Gpio191inttargetRstToleranceW<GpioaccSpec> {
        Gpio191inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO191 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio191inttarget_wr_prot(&mut self) -> Gpio191inttargetWrProtW<GpioaccSpec> {
        Gpio191inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#47\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioacc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioacc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioaccSpec;
impl crate::RegisterSpec for GpioaccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioacc::R`](R) reader structure"]
impl crate::Readable for GpioaccSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioacc::W`](W) writer structure"]
impl crate::Writable for GpioaccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOACC to value 0x1f1f_1f1f"]
impl crate::Resettable for GpioaccSpec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
