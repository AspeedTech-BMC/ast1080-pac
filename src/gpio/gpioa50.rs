#[doc = "Register `GPIOA50` reader"]
pub type R = crate::R<Gpioa50Spec>;
#[doc = "Register `GPIOA50` writer"]
pub type W = crate::W<Gpioa50Spec>;
#[doc = "Field `EnblGPIO064INTToINT13018` reader - Enable GPIO064 Interrupt To INT#130_18"]
pub type EnblGpio064inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO064INTToINT13018` writer - Enable GPIO064 Interrupt To INT#130_18"]
pub type EnblGpio064inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO064INTToINT13019` reader - Enable GPIO064 Interrupt To INT#130_19"]
pub type EnblGpio064inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO064INTToINT13019` writer - Enable GPIO064 Interrupt To INT#130_19"]
pub type EnblGpio064inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO064INTToINT13020` reader - Enable GPIO064 Interrupt To INT#130_20"]
pub type EnblGpio064inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO064INTToINT13020` writer - Enable GPIO064 Interrupt To INT#130_20"]
pub type EnblGpio064inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO064INTToSIO` reader - Enable GPIO064 Interrupt To SIO"]
pub type EnblGpio064inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO064INTToSIO` writer - Enable GPIO064 Interrupt To SIO"]
pub type EnblGpio064inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO064 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio064inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio064inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio064inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO064INTTargetRstTolerance` reader - GPIO064 Interrupt Target Reset Tolerance"]
pub type Gpio064inttargetRstToleranceR = crate::BitReader<Gpio064inttargetRstTolerance>;
impl Gpio064inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio064inttargetRstTolerance {
        match self.bits {
            false => Gpio064inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio064inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio064inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio064inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO064INTTargetRstTolerance` writer - GPIO064 Interrupt Target Reset Tolerance"]
pub type Gpio064inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio064inttargetRstTolerance>;
impl<'a, REG> Gpio064inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio064inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio064inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO064INTTargetWrProt` reader - GPIO064 Interrupt Target Write Protection"]
pub type Gpio064inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO064INTTargetWrProt` writer - GPIO064 Interrupt Target Write Protection"]
pub type Gpio064inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO065INTToINT13018` reader - Enable GPIO065 Interrupt To INT#130_18"]
pub type EnblGpio065inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO065INTToINT13018` writer - Enable GPIO065 Interrupt To INT#130_18"]
pub type EnblGpio065inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO065INTToINT13019` reader - Enable GPIO065 Interrupt To INT#130_19"]
pub type EnblGpio065inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO065INTToINT13019` writer - Enable GPIO065 Interrupt To INT#130_19"]
pub type EnblGpio065inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO065INTToINT13020` reader - Enable GPIO065 Interrupt To INT#130_20"]
pub type EnblGpio065inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO065INTToINT13020` writer - Enable GPIO065 Interrupt To INT#130_20"]
pub type EnblGpio065inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO065INTToSIO` reader - Enable GPIO065 Interrupt To SIO"]
pub type EnblGpio065inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO065INTToSIO` writer - Enable GPIO065 Interrupt To SIO"]
pub type EnblGpio065inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO065 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio065inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio065inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio065inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO065INTTargetRstTolerance` reader - GPIO065 Interrupt Target Reset Tolerance"]
pub type Gpio065inttargetRstToleranceR = crate::BitReader<Gpio065inttargetRstTolerance>;
impl Gpio065inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio065inttargetRstTolerance {
        match self.bits {
            false => Gpio065inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio065inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio065inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio065inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO065INTTargetRstTolerance` writer - GPIO065 Interrupt Target Reset Tolerance"]
pub type Gpio065inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio065inttargetRstTolerance>;
impl<'a, REG> Gpio065inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio065inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio065inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO065INTTargetWrProt` reader - GPIO065 Interrupt Target Write Protection"]
pub type Gpio065inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO065INTTargetWrProt` writer - GPIO065 Interrupt Target Write Protection"]
pub type Gpio065inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO066INTToINT13018` reader - Enable GPIO066 Interrupt To INT#130_18"]
pub type EnblGpio066inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO066INTToINT13018` writer - Enable GPIO066 Interrupt To INT#130_18"]
pub type EnblGpio066inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO066INTToINT13019` reader - Enable GPIO066 Interrupt To INT#130_19"]
pub type EnblGpio066inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO066INTToINT13019` writer - Enable GPIO066 Interrupt To INT#130_19"]
pub type EnblGpio066inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO066INTToINT13020` reader - Enable GPIO066 Interrupt To INT#130_20"]
pub type EnblGpio066inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO066INTToINT13020` writer - Enable GPIO066 Interrupt To INT#130_20"]
pub type EnblGpio066inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO066INTToSIO` reader - Enable GPIO066 Interrupt To SIO"]
pub type EnblGpio066inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO066INTToSIO` writer - Enable GPIO066 Interrupt To SIO"]
pub type EnblGpio066inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO066 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio066inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio066inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio066inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO066INTTargetRstTolerance` reader - GPIO066 Interrupt Target Reset Tolerance"]
pub type Gpio066inttargetRstToleranceR = crate::BitReader<Gpio066inttargetRstTolerance>;
impl Gpio066inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio066inttargetRstTolerance {
        match self.bits {
            false => Gpio066inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio066inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio066inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio066inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO066INTTargetRstTolerance` writer - GPIO066 Interrupt Target Reset Tolerance"]
pub type Gpio066inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio066inttargetRstTolerance>;
impl<'a, REG> Gpio066inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio066inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio066inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO066INTTargetWrProt` reader - GPIO066 Interrupt Target Write Protection"]
pub type Gpio066inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO066INTTargetWrProt` writer - GPIO066 Interrupt Target Write Protection"]
pub type Gpio066inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO067INTToINT13018` reader - Enable GPIO067 Interrupt To INT#130_18"]
pub type EnblGpio067inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO067INTToINT13018` writer - Enable GPIO067 Interrupt To INT#130_18"]
pub type EnblGpio067inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO067INTToINT13019` reader - Enable GPIO067 Interrupt To INT#130_19"]
pub type EnblGpio067inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO067INTToINT13019` writer - Enable GPIO067 Interrupt To INT#130_19"]
pub type EnblGpio067inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO067INTToINT13020` reader - Enable GPIO067 Interrupt To INT#130_20"]
pub type EnblGpio067inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO067INTToINT13020` writer - Enable GPIO067 Interrupt To INT#130_20"]
pub type EnblGpio067inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO067INTToSIO` reader - Enable GPIO067 Interrupt To SIO"]
pub type EnblGpio067inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO067INTToSIO` writer - Enable GPIO067 Interrupt To SIO"]
pub type EnblGpio067inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO067 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio067inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio067inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio067inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO067INTTargetRstTolerance` reader - GPIO067 Interrupt Target Reset Tolerance"]
pub type Gpio067inttargetRstToleranceR = crate::BitReader<Gpio067inttargetRstTolerance>;
impl Gpio067inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio067inttargetRstTolerance {
        match self.bits {
            false => Gpio067inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio067inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio067inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio067inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO067INTTargetRstTolerance` writer - GPIO067 Interrupt Target Reset Tolerance"]
pub type Gpio067inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio067inttargetRstTolerance>;
impl<'a, REG> Gpio067inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio067inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio067inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO067INTTargetWrProt` reader - GPIO067 Interrupt Target Write Protection"]
pub type Gpio067inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO067INTTargetWrProt` writer - GPIO067 Interrupt Target Write Protection"]
pub type Gpio067inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO064 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio064intto_int13018(&self) -> EnblGpio064inttoInt13018R {
        EnblGpio064inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO064 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio064intto_int13019(&self) -> EnblGpio064inttoInt13019R {
        EnblGpio064inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO064 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio064intto_int13020(&self) -> EnblGpio064inttoInt13020R {
        EnblGpio064inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO064 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio064intto_sio(&self) -> EnblGpio064inttoSioR {
        EnblGpio064inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO064 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio064inttarget_rst_tolerance(&self) -> Gpio064inttargetRstToleranceR {
        Gpio064inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO064 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio064inttarget_wr_prot(&self) -> Gpio064inttargetWrProtR {
        Gpio064inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO065 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio065intto_int13018(&self) -> EnblGpio065inttoInt13018R {
        EnblGpio065inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO065 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio065intto_int13019(&self) -> EnblGpio065inttoInt13019R {
        EnblGpio065inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO065 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio065intto_int13020(&self) -> EnblGpio065inttoInt13020R {
        EnblGpio065inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO065 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio065intto_sio(&self) -> EnblGpio065inttoSioR {
        EnblGpio065inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO065 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio065inttarget_rst_tolerance(&self) -> Gpio065inttargetRstToleranceR {
        Gpio065inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO065 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio065inttarget_wr_prot(&self) -> Gpio065inttargetWrProtR {
        Gpio065inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO066 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio066intto_int13018(&self) -> EnblGpio066inttoInt13018R {
        EnblGpio066inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO066 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio066intto_int13019(&self) -> EnblGpio066inttoInt13019R {
        EnblGpio066inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO066 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio066intto_int13020(&self) -> EnblGpio066inttoInt13020R {
        EnblGpio066inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO066 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio066intto_sio(&self) -> EnblGpio066inttoSioR {
        EnblGpio066inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO066 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio066inttarget_rst_tolerance(&self) -> Gpio066inttargetRstToleranceR {
        Gpio066inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO066 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio066inttarget_wr_prot(&self) -> Gpio066inttargetWrProtR {
        Gpio066inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO067 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio067intto_int13018(&self) -> EnblGpio067inttoInt13018R {
        EnblGpio067inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO067 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio067intto_int13019(&self) -> EnblGpio067inttoInt13019R {
        EnblGpio067inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO067 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio067intto_int13020(&self) -> EnblGpio067inttoInt13020R {
        EnblGpio067inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO067 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio067intto_sio(&self) -> EnblGpio067inttoSioR {
        EnblGpio067inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO067 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio067inttarget_rst_tolerance(&self) -> Gpio067inttargetRstToleranceR {
        Gpio067inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO067 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio067inttarget_wr_prot(&self) -> Gpio067inttargetWrProtR {
        Gpio067inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO064 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio064intto_int13018(&mut self) -> EnblGpio064inttoInt13018W<Gpioa50Spec> {
        EnblGpio064inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO064 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio064intto_int13019(&mut self) -> EnblGpio064inttoInt13019W<Gpioa50Spec> {
        EnblGpio064inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO064 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio064intto_int13020(&mut self) -> EnblGpio064inttoInt13020W<Gpioa50Spec> {
        EnblGpio064inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO064 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio064intto_sio(&mut self) -> EnblGpio064inttoSioW<Gpioa50Spec> {
        EnblGpio064inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa50Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa50Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO064 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio064inttarget_rst_tolerance(&mut self) -> Gpio064inttargetRstToleranceW<Gpioa50Spec> {
        Gpio064inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO064 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio064inttarget_wr_prot(&mut self) -> Gpio064inttargetWrProtW<Gpioa50Spec> {
        Gpio064inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO065 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio065intto_int13018(&mut self) -> EnblGpio065inttoInt13018W<Gpioa50Spec> {
        EnblGpio065inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO065 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio065intto_int13019(&mut self) -> EnblGpio065inttoInt13019W<Gpioa50Spec> {
        EnblGpio065inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO065 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio065intto_int13020(&mut self) -> EnblGpio065inttoInt13020W<Gpioa50Spec> {
        EnblGpio065inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO065 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio065intto_sio(&mut self) -> EnblGpio065inttoSioW<Gpioa50Spec> {
        EnblGpio065inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa50Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa50Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO065 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio065inttarget_rst_tolerance(&mut self) -> Gpio065inttargetRstToleranceW<Gpioa50Spec> {
        Gpio065inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO065 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio065inttarget_wr_prot(&mut self) -> Gpio065inttargetWrProtW<Gpioa50Spec> {
        Gpio065inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO066 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio066intto_int13018(&mut self) -> EnblGpio066inttoInt13018W<Gpioa50Spec> {
        EnblGpio066inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO066 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio066intto_int13019(&mut self) -> EnblGpio066inttoInt13019W<Gpioa50Spec> {
        EnblGpio066inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO066 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio066intto_int13020(&mut self) -> EnblGpio066inttoInt13020W<Gpioa50Spec> {
        EnblGpio066inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO066 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio066intto_sio(&mut self) -> EnblGpio066inttoSioW<Gpioa50Spec> {
        EnblGpio066inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa50Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa50Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO066 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio066inttarget_rst_tolerance(&mut self) -> Gpio066inttargetRstToleranceW<Gpioa50Spec> {
        Gpio066inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO066 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio066inttarget_wr_prot(&mut self) -> Gpio066inttargetWrProtW<Gpioa50Spec> {
        Gpio066inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO067 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio067intto_int13018(&mut self) -> EnblGpio067inttoInt13018W<Gpioa50Spec> {
        EnblGpio067inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO067 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio067intto_int13019(&mut self) -> EnblGpio067inttoInt13019W<Gpioa50Spec> {
        EnblGpio067inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO067 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio067intto_int13020(&mut self) -> EnblGpio067inttoInt13020W<Gpioa50Spec> {
        EnblGpio067inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO067 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio067intto_sio(&mut self) -> EnblGpio067inttoSioW<Gpioa50Spec> {
        EnblGpio067inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa50Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO067 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio067inttarget_rst_tolerance(&mut self) -> Gpio067inttargetRstToleranceW<Gpioa50Spec> {
        Gpio067inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO067 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio067inttarget_wr_prot(&mut self) -> Gpio067inttargetWrProtW<Gpioa50Spec> {
        Gpio067inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa50Spec;
impl crate::RegisterSpec for Gpioa50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa50::R`](R) reader structure"]
impl crate::Readable for Gpioa50Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa50::W`](W) writer structure"]
impl crate::Writable for Gpioa50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA50 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa50Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
