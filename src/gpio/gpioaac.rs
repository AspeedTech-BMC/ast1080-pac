#[doc = "Register `GPIOAAC` reader"]
pub type R = crate::R<GpioaacSpec>;
#[doc = "Register `GPIOAAC` writer"]
pub type W = crate::W<GpioaacSpec>;
#[doc = "Field `EnblGPIO156INTToINT13018` reader - Enable GPIO156 Interrupt To INT#130_18"]
pub type EnblGpio156inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO156INTToINT13018` writer - Enable GPIO156 Interrupt To INT#130_18"]
pub type EnblGpio156inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO156INTToINT13019` reader - Enable GPIO156 Interrupt To INT#130_19"]
pub type EnblGpio156inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO156INTToINT13019` writer - Enable GPIO156 Interrupt To INT#130_19"]
pub type EnblGpio156inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO156INTToINT13020` reader - Enable GPIO156 Interrupt To INT#130_20"]
pub type EnblGpio156inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO156INTToINT13020` writer - Enable GPIO156 Interrupt To INT#130_20"]
pub type EnblGpio156inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO156INTToSIO` reader - Enable GPIO156 Interrupt To SIO"]
pub type EnblGpio156inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO156INTToSIO` writer - Enable GPIO156 Interrupt To SIO"]
pub type EnblGpio156inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO156 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio156inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio156inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio156inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO156INTTargetRstTolerance` reader - GPIO156 Interrupt Target Reset Tolerance"]
pub type Gpio156inttargetRstToleranceR = crate::BitReader<Gpio156inttargetRstTolerance>;
impl Gpio156inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio156inttargetRstTolerance {
        match self.bits {
            false => Gpio156inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio156inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio156inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio156inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO156INTTargetRstTolerance` writer - GPIO156 Interrupt Target Reset Tolerance"]
pub type Gpio156inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio156inttargetRstTolerance>;
impl<'a, REG> Gpio156inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio156inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio156inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO156INTTargetWrProt` reader - GPIO156 Interrupt Target Write Protection"]
pub type Gpio156inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO156INTTargetWrProt` writer - GPIO156 Interrupt Target Write Protection"]
pub type Gpio156inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO157INTToINT13018` reader - Enable GPIO157 Interrupt To INT#130_18"]
pub type EnblGpio157inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO157INTToINT13018` writer - Enable GPIO157 Interrupt To INT#130_18"]
pub type EnblGpio157inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO157INTToINT13019` reader - Enable GPIO157 Interrupt To INT#130_19"]
pub type EnblGpio157inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO157INTToINT13019` writer - Enable GPIO157 Interrupt To INT#130_19"]
pub type EnblGpio157inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO157INTToINT13020` reader - Enable GPIO157 Interrupt To INT#130_20"]
pub type EnblGpio157inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO157INTToINT13020` writer - Enable GPIO157 Interrupt To INT#130_20"]
pub type EnblGpio157inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO157INTToSIO` reader - Enable GPIO157 Interrupt To SIO"]
pub type EnblGpio157inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO157INTToSIO` writer - Enable GPIO157 Interrupt To SIO"]
pub type EnblGpio157inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO157 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio157inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio157inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio157inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO157INTTargetRstTolerance` reader - GPIO157 Interrupt Target Reset Tolerance"]
pub type Gpio157inttargetRstToleranceR = crate::BitReader<Gpio157inttargetRstTolerance>;
impl Gpio157inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio157inttargetRstTolerance {
        match self.bits {
            false => Gpio157inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio157inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio157inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio157inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO157INTTargetRstTolerance` writer - GPIO157 Interrupt Target Reset Tolerance"]
pub type Gpio157inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio157inttargetRstTolerance>;
impl<'a, REG> Gpio157inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio157inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio157inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO157INTTargetWrProt` reader - GPIO157 Interrupt Target Write Protection"]
pub type Gpio157inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO157INTTargetWrProt` writer - GPIO157 Interrupt Target Write Protection"]
pub type Gpio157inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO158INTToINT13018` reader - Enable GPIO158 Interrupt To INT#130_18"]
pub type EnblGpio158inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO158INTToINT13018` writer - Enable GPIO158 Interrupt To INT#130_18"]
pub type EnblGpio158inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO158INTToINT13019` reader - Enable GPIO158 Interrupt To INT#130_19"]
pub type EnblGpio158inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO158INTToINT13019` writer - Enable GPIO158 Interrupt To INT#130_19"]
pub type EnblGpio158inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO158INTToINT13020` reader - Enable GPIO158 Interrupt To INT#130_20"]
pub type EnblGpio158inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO158INTToINT13020` writer - Enable GPIO158 Interrupt To INT#130_20"]
pub type EnblGpio158inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO158INTToSIO` reader - Enable GPIO158 Interrupt To SIO"]
pub type EnblGpio158inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO158INTToSIO` writer - Enable GPIO158 Interrupt To SIO"]
pub type EnblGpio158inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO158 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio158inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio158inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio158inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO158INTTargetRstTolerance` reader - GPIO158 Interrupt Target Reset Tolerance"]
pub type Gpio158inttargetRstToleranceR = crate::BitReader<Gpio158inttargetRstTolerance>;
impl Gpio158inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio158inttargetRstTolerance {
        match self.bits {
            false => Gpio158inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio158inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio158inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio158inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO158INTTargetRstTolerance` writer - GPIO158 Interrupt Target Reset Tolerance"]
pub type Gpio158inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio158inttargetRstTolerance>;
impl<'a, REG> Gpio158inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio158inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio158inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO158INTTargetWrProt` reader - GPIO158 Interrupt Target Write Protection"]
pub type Gpio158inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO158INTTargetWrProt` writer - GPIO158 Interrupt Target Write Protection"]
pub type Gpio158inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO159INTToINT13018` reader - Enable GPIO159 Interrupt To INT#130_18"]
pub type EnblGpio159inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO159INTToINT13018` writer - Enable GPIO159 Interrupt To INT#130_18"]
pub type EnblGpio159inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO159INTToINT13019` reader - Enable GPIO159 Interrupt To INT#130_19"]
pub type EnblGpio159inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO159INTToINT13019` writer - Enable GPIO159 Interrupt To INT#130_19"]
pub type EnblGpio159inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO159INTToINT13020` reader - Enable GPIO159 Interrupt To INT#130_20"]
pub type EnblGpio159inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO159INTToINT13020` writer - Enable GPIO159 Interrupt To INT#130_20"]
pub type EnblGpio159inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO159INTToSIO` reader - Enable GPIO159 Interrupt To SIO"]
pub type EnblGpio159inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO159INTToSIO` writer - Enable GPIO159 Interrupt To SIO"]
pub type EnblGpio159inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO159 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio159inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio159inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio159inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO159INTTargetRstTolerance` reader - GPIO159 Interrupt Target Reset Tolerance"]
pub type Gpio159inttargetRstToleranceR = crate::BitReader<Gpio159inttargetRstTolerance>;
impl Gpio159inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio159inttargetRstTolerance {
        match self.bits {
            false => Gpio159inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio159inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio159inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio159inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO159INTTargetRstTolerance` writer - GPIO159 Interrupt Target Reset Tolerance"]
pub type Gpio159inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio159inttargetRstTolerance>;
impl<'a, REG> Gpio159inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio159inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio159inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO159INTTargetWrProt` reader - GPIO159 Interrupt Target Write Protection"]
pub type Gpio159inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO159INTTargetWrProt` writer - GPIO159 Interrupt Target Write Protection"]
pub type Gpio159inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO156 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio156intto_int13018(&self) -> EnblGpio156inttoInt13018R {
        EnblGpio156inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO156 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio156intto_int13019(&self) -> EnblGpio156inttoInt13019R {
        EnblGpio156inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO156 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio156intto_int13020(&self) -> EnblGpio156inttoInt13020R {
        EnblGpio156inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO156 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio156intto_sio(&self) -> EnblGpio156inttoSioR {
        EnblGpio156inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO156 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio156inttarget_rst_tolerance(&self) -> Gpio156inttargetRstToleranceR {
        Gpio156inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO156 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio156inttarget_wr_prot(&self) -> Gpio156inttargetWrProtR {
        Gpio156inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO157 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio157intto_int13018(&self) -> EnblGpio157inttoInt13018R {
        EnblGpio157inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO157 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio157intto_int13019(&self) -> EnblGpio157inttoInt13019R {
        EnblGpio157inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO157 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio157intto_int13020(&self) -> EnblGpio157inttoInt13020R {
        EnblGpio157inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO157 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio157intto_sio(&self) -> EnblGpio157inttoSioR {
        EnblGpio157inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO157 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio157inttarget_rst_tolerance(&self) -> Gpio157inttargetRstToleranceR {
        Gpio157inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO157 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio157inttarget_wr_prot(&self) -> Gpio157inttargetWrProtR {
        Gpio157inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO158 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio158intto_int13018(&self) -> EnblGpio158inttoInt13018R {
        EnblGpio158inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO158 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio158intto_int13019(&self) -> EnblGpio158inttoInt13019R {
        EnblGpio158inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO158 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio158intto_int13020(&self) -> EnblGpio158inttoInt13020R {
        EnblGpio158inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO158 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio158intto_sio(&self) -> EnblGpio158inttoSioR {
        EnblGpio158inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO158 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio158inttarget_rst_tolerance(&self) -> Gpio158inttargetRstToleranceR {
        Gpio158inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO158 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio158inttarget_wr_prot(&self) -> Gpio158inttargetWrProtR {
        Gpio158inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO159 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio159intto_int13018(&self) -> EnblGpio159inttoInt13018R {
        EnblGpio159inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO159 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio159intto_int13019(&self) -> EnblGpio159inttoInt13019R {
        EnblGpio159inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO159 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio159intto_int13020(&self) -> EnblGpio159inttoInt13020R {
        EnblGpio159inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO159 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio159intto_sio(&self) -> EnblGpio159inttoSioR {
        EnblGpio159inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO159 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio159inttarget_rst_tolerance(&self) -> Gpio159inttargetRstToleranceR {
        Gpio159inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO159 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio159inttarget_wr_prot(&self) -> Gpio159inttargetWrProtR {
        Gpio159inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO156 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio156intto_int13018(&mut self) -> EnblGpio156inttoInt13018W<GpioaacSpec> {
        EnblGpio156inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO156 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio156intto_int13019(&mut self) -> EnblGpio156inttoInt13019W<GpioaacSpec> {
        EnblGpio156inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO156 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio156intto_int13020(&mut self) -> EnblGpio156inttoInt13020W<GpioaacSpec> {
        EnblGpio156inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO156 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio156intto_sio(&mut self) -> EnblGpio156inttoSioW<GpioaacSpec> {
        EnblGpio156inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<GpioaacSpec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<GpioaacSpec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO156 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio156inttarget_rst_tolerance(&mut self) -> Gpio156inttargetRstToleranceW<GpioaacSpec> {
        Gpio156inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO156 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio156inttarget_wr_prot(&mut self) -> Gpio156inttargetWrProtW<GpioaacSpec> {
        Gpio156inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO157 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio157intto_int13018(&mut self) -> EnblGpio157inttoInt13018W<GpioaacSpec> {
        EnblGpio157inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO157 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio157intto_int13019(&mut self) -> EnblGpio157inttoInt13019W<GpioaacSpec> {
        EnblGpio157inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO157 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio157intto_int13020(&mut self) -> EnblGpio157inttoInt13020W<GpioaacSpec> {
        EnblGpio157inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO157 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio157intto_sio(&mut self) -> EnblGpio157inttoSioW<GpioaacSpec> {
        EnblGpio157inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<GpioaacSpec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<GpioaacSpec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO157 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio157inttarget_rst_tolerance(&mut self) -> Gpio157inttargetRstToleranceW<GpioaacSpec> {
        Gpio157inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO157 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio157inttarget_wr_prot(&mut self) -> Gpio157inttargetWrProtW<GpioaacSpec> {
        Gpio157inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO158 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio158intto_int13018(&mut self) -> EnblGpio158inttoInt13018W<GpioaacSpec> {
        EnblGpio158inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO158 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio158intto_int13019(&mut self) -> EnblGpio158inttoInt13019W<GpioaacSpec> {
        EnblGpio158inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO158 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio158intto_int13020(&mut self) -> EnblGpio158inttoInt13020W<GpioaacSpec> {
        EnblGpio158inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO158 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio158intto_sio(&mut self) -> EnblGpio158inttoSioW<GpioaacSpec> {
        EnblGpio158inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<GpioaacSpec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<GpioaacSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO158 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio158inttarget_rst_tolerance(&mut self) -> Gpio158inttargetRstToleranceW<GpioaacSpec> {
        Gpio158inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO158 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio158inttarget_wr_prot(&mut self) -> Gpio158inttargetWrProtW<GpioaacSpec> {
        Gpio158inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO159 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio159intto_int13018(&mut self) -> EnblGpio159inttoInt13018W<GpioaacSpec> {
        EnblGpio159inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO159 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio159intto_int13019(&mut self) -> EnblGpio159inttoInt13019W<GpioaacSpec> {
        EnblGpio159inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO159 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio159intto_int13020(&mut self) -> EnblGpio159inttoInt13020W<GpioaacSpec> {
        EnblGpio159inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO159 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio159intto_sio(&mut self) -> EnblGpio159inttoSioW<GpioaacSpec> {
        EnblGpio159inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<GpioaacSpec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO159 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio159inttarget_rst_tolerance(&mut self) -> Gpio159inttargetRstToleranceW<GpioaacSpec> {
        Gpio159inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO159 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio159inttarget_wr_prot(&mut self) -> Gpio159inttargetWrProtW<GpioaacSpec> {
        Gpio159inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#39\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioaac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioaac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioaacSpec;
impl crate::RegisterSpec for GpioaacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioaac::R`](R) reader structure"]
impl crate::Readable for GpioaacSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioaac::W`](W) writer structure"]
impl crate::Writable for GpioaacSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOAAC to value 0x1f1f_1f1f"]
impl crate::Resettable for GpioaacSpec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
