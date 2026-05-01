#[doc = "Register `GPIOA70` reader"]
pub type R = crate::R<Gpioa70Spec>;
#[doc = "Register `GPIOA70` writer"]
pub type W = crate::W<Gpioa70Spec>;
#[doc = "Field `EnblGPIO096INTToINT13018` reader - Enable GPIO096 Interrupt To INT#130_18"]
pub type EnblGpio096inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO096INTToINT13018` writer - Enable GPIO096 Interrupt To INT#130_18"]
pub type EnblGpio096inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO096INTToINT13019` reader - Enable GPIO096 Interrupt To INT#130_19"]
pub type EnblGpio096inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO096INTToINT13019` writer - Enable GPIO096 Interrupt To INT#130_19"]
pub type EnblGpio096inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO096INTToINT13020` reader - Enable GPIO096 Interrupt To INT#130_20"]
pub type EnblGpio096inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO096INTToINT13020` writer - Enable GPIO096 Interrupt To INT#130_20"]
pub type EnblGpio096inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO096INTToSIO` reader - Enable GPIO096 Interrupt To SIO"]
pub type EnblGpio096inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO096INTToSIO` writer - Enable GPIO096 Interrupt To SIO"]
pub type EnblGpio096inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO096 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio096inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio096inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio096inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO096INTTargetRstTolerance` reader - GPIO096 Interrupt Target Reset Tolerance"]
pub type Gpio096inttargetRstToleranceR = crate::BitReader<Gpio096inttargetRstTolerance>;
impl Gpio096inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio096inttargetRstTolerance {
        match self.bits {
            false => Gpio096inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio096inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio096inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio096inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO096INTTargetRstTolerance` writer - GPIO096 Interrupt Target Reset Tolerance"]
pub type Gpio096inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio096inttargetRstTolerance>;
impl<'a, REG> Gpio096inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio096inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio096inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO096INTTargetWrProt` reader - GPIO096 Interrupt Target Write Protection"]
pub type Gpio096inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO096INTTargetWrProt` writer - GPIO096 Interrupt Target Write Protection"]
pub type Gpio096inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO097INTToINT13018` reader - Enable GPIO097 Interrupt To INT#130_18"]
pub type EnblGpio097inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO097INTToINT13018` writer - Enable GPIO097 Interrupt To INT#130_18"]
pub type EnblGpio097inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO097INTToINT13019` reader - Enable GPIO097 Interrupt To INT#130_19"]
pub type EnblGpio097inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO097INTToINT13019` writer - Enable GPIO097 Interrupt To INT#130_19"]
pub type EnblGpio097inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO097INTToINT13020` reader - Enable GPIO097 Interrupt To INT#130_20"]
pub type EnblGpio097inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO097INTToINT13020` writer - Enable GPIO097 Interrupt To INT#130_20"]
pub type EnblGpio097inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO097INTToSIO` reader - Enable GPIO097 Interrupt To SIO"]
pub type EnblGpio097inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO097INTToSIO` writer - Enable GPIO097 Interrupt To SIO"]
pub type EnblGpio097inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO097 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio097inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio097inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio097inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO097INTTargetRstTolerance` reader - GPIO097 Interrupt Target Reset Tolerance"]
pub type Gpio097inttargetRstToleranceR = crate::BitReader<Gpio097inttargetRstTolerance>;
impl Gpio097inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio097inttargetRstTolerance {
        match self.bits {
            false => Gpio097inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio097inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio097inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio097inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO097INTTargetRstTolerance` writer - GPIO097 Interrupt Target Reset Tolerance"]
pub type Gpio097inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio097inttargetRstTolerance>;
impl<'a, REG> Gpio097inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio097inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio097inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO097INTTargetWrProt` reader - GPIO097 Interrupt Target Write Protection"]
pub type Gpio097inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO097INTTargetWrProt` writer - GPIO097 Interrupt Target Write Protection"]
pub type Gpio097inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO098INTToINT13018` reader - Enable GPIO098 Interrupt To INT#130_18"]
pub type EnblGpio098inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO098INTToINT13018` writer - Enable GPIO098 Interrupt To INT#130_18"]
pub type EnblGpio098inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO098INTToINT13019` reader - Enable GPIO098 Interrupt To INT#130_19"]
pub type EnblGpio098inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO098INTToINT13019` writer - Enable GPIO098 Interrupt To INT#130_19"]
pub type EnblGpio098inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO098INTToINT13020` reader - Enable GPIO098 Interrupt To INT#130_20"]
pub type EnblGpio098inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO098INTToINT13020` writer - Enable GPIO098 Interrupt To INT#130_20"]
pub type EnblGpio098inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO098INTToSIO` reader - Enable GPIO098 Interrupt To SIO"]
pub type EnblGpio098inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO098INTToSIO` writer - Enable GPIO098 Interrupt To SIO"]
pub type EnblGpio098inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO098 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio098inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio098inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio098inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO098INTTargetRstTolerance` reader - GPIO098 Interrupt Target Reset Tolerance"]
pub type Gpio098inttargetRstToleranceR = crate::BitReader<Gpio098inttargetRstTolerance>;
impl Gpio098inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio098inttargetRstTolerance {
        match self.bits {
            false => Gpio098inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio098inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio098inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio098inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO098INTTargetRstTolerance` writer - GPIO098 Interrupt Target Reset Tolerance"]
pub type Gpio098inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio098inttargetRstTolerance>;
impl<'a, REG> Gpio098inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio098inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio098inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO098INTTargetWrProt` reader - GPIO098 Interrupt Target Write Protection"]
pub type Gpio098inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO098INTTargetWrProt` writer - GPIO098 Interrupt Target Write Protection"]
pub type Gpio098inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO099INTToINT13018` reader - Enable GPIO099 Interrupt To INT#130_18"]
pub type EnblGpio099inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO099INTToINT13018` writer - Enable GPIO099 Interrupt To INT#130_18"]
pub type EnblGpio099inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO099INTToINT13019` reader - Enable GPIO099 Interrupt To INT#130_19"]
pub type EnblGpio099inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO099INTToINT13019` writer - Enable GPIO099 Interrupt To INT#130_19"]
pub type EnblGpio099inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO099INTToINT13020` reader - Enable GPIO099 Interrupt To INT#130_20"]
pub type EnblGpio099inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO099INTToINT13020` writer - Enable GPIO099 Interrupt To INT#130_20"]
pub type EnblGpio099inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO099INTToSIO` reader - Enable GPIO099 Interrupt To SIO"]
pub type EnblGpio099inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO099INTToSIO` writer - Enable GPIO099 Interrupt To SIO"]
pub type EnblGpio099inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO099 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio099inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio099inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio099inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO099INTTargetRstTolerance` reader - GPIO099 Interrupt Target Reset Tolerance"]
pub type Gpio099inttargetRstToleranceR = crate::BitReader<Gpio099inttargetRstTolerance>;
impl Gpio099inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio099inttargetRstTolerance {
        match self.bits {
            false => Gpio099inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio099inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio099inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio099inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO099INTTargetRstTolerance` writer - GPIO099 Interrupt Target Reset Tolerance"]
pub type Gpio099inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio099inttargetRstTolerance>;
impl<'a, REG> Gpio099inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio099inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio099inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO099INTTargetWrProt` reader - GPIO099 Interrupt Target Write Protection"]
pub type Gpio099inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO099INTTargetWrProt` writer - GPIO099 Interrupt Target Write Protection"]
pub type Gpio099inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO096 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio096intto_int13018(&self) -> EnblGpio096inttoInt13018R {
        EnblGpio096inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO096 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio096intto_int13019(&self) -> EnblGpio096inttoInt13019R {
        EnblGpio096inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO096 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio096intto_int13020(&self) -> EnblGpio096inttoInt13020R {
        EnblGpio096inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO096 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio096intto_sio(&self) -> EnblGpio096inttoSioR {
        EnblGpio096inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO096 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio096inttarget_rst_tolerance(&self) -> Gpio096inttargetRstToleranceR {
        Gpio096inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO096 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio096inttarget_wr_prot(&self) -> Gpio096inttargetWrProtR {
        Gpio096inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO097 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio097intto_int13018(&self) -> EnblGpio097inttoInt13018R {
        EnblGpio097inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO097 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio097intto_int13019(&self) -> EnblGpio097inttoInt13019R {
        EnblGpio097inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO097 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio097intto_int13020(&self) -> EnblGpio097inttoInt13020R {
        EnblGpio097inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO097 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio097intto_sio(&self) -> EnblGpio097inttoSioR {
        EnblGpio097inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO097 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio097inttarget_rst_tolerance(&self) -> Gpio097inttargetRstToleranceR {
        Gpio097inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO097 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio097inttarget_wr_prot(&self) -> Gpio097inttargetWrProtR {
        Gpio097inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO098 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio098intto_int13018(&self) -> EnblGpio098inttoInt13018R {
        EnblGpio098inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO098 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio098intto_int13019(&self) -> EnblGpio098inttoInt13019R {
        EnblGpio098inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO098 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio098intto_int13020(&self) -> EnblGpio098inttoInt13020R {
        EnblGpio098inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO098 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio098intto_sio(&self) -> EnblGpio098inttoSioR {
        EnblGpio098inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO098 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio098inttarget_rst_tolerance(&self) -> Gpio098inttargetRstToleranceR {
        Gpio098inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO098 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio098inttarget_wr_prot(&self) -> Gpio098inttargetWrProtR {
        Gpio098inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO099 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio099intto_int13018(&self) -> EnblGpio099inttoInt13018R {
        EnblGpio099inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO099 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio099intto_int13019(&self) -> EnblGpio099inttoInt13019R {
        EnblGpio099inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO099 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio099intto_int13020(&self) -> EnblGpio099inttoInt13020R {
        EnblGpio099inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO099 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio099intto_sio(&self) -> EnblGpio099inttoSioR {
        EnblGpio099inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO099 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio099inttarget_rst_tolerance(&self) -> Gpio099inttargetRstToleranceR {
        Gpio099inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO099 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio099inttarget_wr_prot(&self) -> Gpio099inttargetWrProtR {
        Gpio099inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO096 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio096intto_int13018(&mut self) -> EnblGpio096inttoInt13018W<Gpioa70Spec> {
        EnblGpio096inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO096 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio096intto_int13019(&mut self) -> EnblGpio096inttoInt13019W<Gpioa70Spec> {
        EnblGpio096inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO096 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio096intto_int13020(&mut self) -> EnblGpio096inttoInt13020W<Gpioa70Spec> {
        EnblGpio096inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO096 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio096intto_sio(&mut self) -> EnblGpio096inttoSioW<Gpioa70Spec> {
        EnblGpio096inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa70Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa70Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO096 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio096inttarget_rst_tolerance(&mut self) -> Gpio096inttargetRstToleranceW<Gpioa70Spec> {
        Gpio096inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO096 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio096inttarget_wr_prot(&mut self) -> Gpio096inttargetWrProtW<Gpioa70Spec> {
        Gpio096inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO097 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio097intto_int13018(&mut self) -> EnblGpio097inttoInt13018W<Gpioa70Spec> {
        EnblGpio097inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO097 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio097intto_int13019(&mut self) -> EnblGpio097inttoInt13019W<Gpioa70Spec> {
        EnblGpio097inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO097 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio097intto_int13020(&mut self) -> EnblGpio097inttoInt13020W<Gpioa70Spec> {
        EnblGpio097inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO097 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio097intto_sio(&mut self) -> EnblGpio097inttoSioW<Gpioa70Spec> {
        EnblGpio097inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa70Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa70Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO097 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio097inttarget_rst_tolerance(&mut self) -> Gpio097inttargetRstToleranceW<Gpioa70Spec> {
        Gpio097inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO097 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio097inttarget_wr_prot(&mut self) -> Gpio097inttargetWrProtW<Gpioa70Spec> {
        Gpio097inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO098 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio098intto_int13018(&mut self) -> EnblGpio098inttoInt13018W<Gpioa70Spec> {
        EnblGpio098inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO098 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio098intto_int13019(&mut self) -> EnblGpio098inttoInt13019W<Gpioa70Spec> {
        EnblGpio098inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO098 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio098intto_int13020(&mut self) -> EnblGpio098inttoInt13020W<Gpioa70Spec> {
        EnblGpio098inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO098 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio098intto_sio(&mut self) -> EnblGpio098inttoSioW<Gpioa70Spec> {
        EnblGpio098inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa70Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa70Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO098 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio098inttarget_rst_tolerance(&mut self) -> Gpio098inttargetRstToleranceW<Gpioa70Spec> {
        Gpio098inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO098 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio098inttarget_wr_prot(&mut self) -> Gpio098inttargetWrProtW<Gpioa70Spec> {
        Gpio098inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO099 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio099intto_int13018(&mut self) -> EnblGpio099inttoInt13018W<Gpioa70Spec> {
        EnblGpio099inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO099 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio099intto_int13019(&mut self) -> EnblGpio099inttoInt13019W<Gpioa70Spec> {
        EnblGpio099inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO099 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio099intto_int13020(&mut self) -> EnblGpio099inttoInt13020W<Gpioa70Spec> {
        EnblGpio099inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO099 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio099intto_sio(&mut self) -> EnblGpio099inttoSioW<Gpioa70Spec> {
        EnblGpio099inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa70Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO099 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio099inttarget_rst_tolerance(&mut self) -> Gpio099inttargetRstToleranceW<Gpioa70Spec> {
        Gpio099inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO099 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio099inttarget_wr_prot(&mut self) -> Gpio099inttargetWrProtW<Gpioa70Spec> {
        Gpio099inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa70::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa70::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa70Spec;
impl crate::RegisterSpec for Gpioa70Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa70::R`](R) reader structure"]
impl crate::Readable for Gpioa70Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa70::W`](W) writer structure"]
impl crate::Writable for Gpioa70Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA70 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa70Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
