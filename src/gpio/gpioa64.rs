#[doc = "Register `GPIOA64` reader"]
pub type R = crate::R<Gpioa64Spec>;
#[doc = "Register `GPIOA64` writer"]
pub type W = crate::W<Gpioa64Spec>;
#[doc = "Field `EnblGPIO084INTToINT13018` reader - Enable GPIO084 Interrupt To INT#130_18"]
pub type EnblGpio084inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO084INTToINT13018` writer - Enable GPIO084 Interrupt To INT#130_18"]
pub type EnblGpio084inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO084INTToINT13019` reader - Enable GPIO084 Interrupt To INT#130_19"]
pub type EnblGpio084inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO084INTToINT13019` writer - Enable GPIO084 Interrupt To INT#130_19"]
pub type EnblGpio084inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO084INTToINT13020` reader - Enable GPIO084 Interrupt To INT#130_20"]
pub type EnblGpio084inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO084INTToINT13020` writer - Enable GPIO084 Interrupt To INT#130_20"]
pub type EnblGpio084inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO084INTToSIO` reader - Enable GPIO084 Interrupt To SIO"]
pub type EnblGpio084inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO084INTToSIO` writer - Enable GPIO084 Interrupt To SIO"]
pub type EnblGpio084inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO084 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio084inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio084inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio084inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO084INTTargetRstTolerance` reader - GPIO084 Interrupt Target Reset Tolerance"]
pub type Gpio084inttargetRstToleranceR = crate::BitReader<Gpio084inttargetRstTolerance>;
impl Gpio084inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio084inttargetRstTolerance {
        match self.bits {
            false => Gpio084inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio084inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio084inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio084inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO084INTTargetRstTolerance` writer - GPIO084 Interrupt Target Reset Tolerance"]
pub type Gpio084inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio084inttargetRstTolerance>;
impl<'a, REG> Gpio084inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio084inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio084inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO084INTTargetWrProt` reader - GPIO084 Interrupt Target Write Protection"]
pub type Gpio084inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO084INTTargetWrProt` writer - GPIO084 Interrupt Target Write Protection"]
pub type Gpio084inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO085INTToINT13018` reader - Enable GPIO085 Interrupt To INT#130_18"]
pub type EnblGpio085inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO085INTToINT13018` writer - Enable GPIO085 Interrupt To INT#130_18"]
pub type EnblGpio085inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO085INTToINT13019` reader - Enable GPIO085 Interrupt To INT#130_19"]
pub type EnblGpio085inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO085INTToINT13019` writer - Enable GPIO085 Interrupt To INT#130_19"]
pub type EnblGpio085inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO085INTToINT13020` reader - Enable GPIO085 Interrupt To INT#130_20"]
pub type EnblGpio085inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO085INTToINT13020` writer - Enable GPIO085 Interrupt To INT#130_20"]
pub type EnblGpio085inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO085INTToSIO` reader - Enable GPIO085 Interrupt To SIO"]
pub type EnblGpio085inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO085INTToSIO` writer - Enable GPIO085 Interrupt To SIO"]
pub type EnblGpio085inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO085 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio085inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio085inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio085inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO085INTTargetRstTolerance` reader - GPIO085 Interrupt Target Reset Tolerance"]
pub type Gpio085inttargetRstToleranceR = crate::BitReader<Gpio085inttargetRstTolerance>;
impl Gpio085inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio085inttargetRstTolerance {
        match self.bits {
            false => Gpio085inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio085inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio085inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio085inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO085INTTargetRstTolerance` writer - GPIO085 Interrupt Target Reset Tolerance"]
pub type Gpio085inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio085inttargetRstTolerance>;
impl<'a, REG> Gpio085inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio085inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio085inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO085INTTargetWrProt` reader - GPIO085 Interrupt Target Write Protection"]
pub type Gpio085inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO085INTTargetWrProt` writer - GPIO085 Interrupt Target Write Protection"]
pub type Gpio085inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO086INTToINT13018` reader - Enable GPIO086 Interrupt To INT#130_18"]
pub type EnblGpio086inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO086INTToINT13018` writer - Enable GPIO086 Interrupt To INT#130_18"]
pub type EnblGpio086inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO086INTToINT13019` reader - Enable GPIO086 Interrupt To INT#130_19"]
pub type EnblGpio086inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO086INTToINT13019` writer - Enable GPIO086 Interrupt To INT#130_19"]
pub type EnblGpio086inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO086INTToINT13020` reader - Enable GPIO086 Interrupt To INT#130_20"]
pub type EnblGpio086inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO086INTToINT13020` writer - Enable GPIO086 Interrupt To INT#130_20"]
pub type EnblGpio086inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO086INTToSIO` reader - Enable GPIO086 Interrupt To SIO"]
pub type EnblGpio086inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO086INTToSIO` writer - Enable GPIO086 Interrupt To SIO"]
pub type EnblGpio086inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO086 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio086inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio086inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio086inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO086INTTargetRstTolerance` reader - GPIO086 Interrupt Target Reset Tolerance"]
pub type Gpio086inttargetRstToleranceR = crate::BitReader<Gpio086inttargetRstTolerance>;
impl Gpio086inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio086inttargetRstTolerance {
        match self.bits {
            false => Gpio086inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio086inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio086inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio086inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO086INTTargetRstTolerance` writer - GPIO086 Interrupt Target Reset Tolerance"]
pub type Gpio086inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio086inttargetRstTolerance>;
impl<'a, REG> Gpio086inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio086inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio086inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO086INTTargetWrProt` reader - GPIO086 Interrupt Target Write Protection"]
pub type Gpio086inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO086INTTargetWrProt` writer - GPIO086 Interrupt Target Write Protection"]
pub type Gpio086inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO087INTToINT13018` reader - Enable GPIO087 Interrupt To INT#130_18"]
pub type EnblGpio087inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO087INTToINT13018` writer - Enable GPIO087 Interrupt To INT#130_18"]
pub type EnblGpio087inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO087INTToINT13019` reader - Enable GPIO087 Interrupt To INT#130_19"]
pub type EnblGpio087inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO087INTToINT13019` writer - Enable GPIO087 Interrupt To INT#130_19"]
pub type EnblGpio087inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO087INTToINT13020` reader - Enable GPIO087 Interrupt To INT#130_20"]
pub type EnblGpio087inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO087INTToINT13020` writer - Enable GPIO087 Interrupt To INT#130_20"]
pub type EnblGpio087inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO087INTToSIO` reader - Enable GPIO087 Interrupt To SIO"]
pub type EnblGpio087inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO087INTToSIO` writer - Enable GPIO087 Interrupt To SIO"]
pub type EnblGpio087inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO087 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio087inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio087inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio087inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO087INTTargetRstTolerance` reader - GPIO087 Interrupt Target Reset Tolerance"]
pub type Gpio087inttargetRstToleranceR = crate::BitReader<Gpio087inttargetRstTolerance>;
impl Gpio087inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio087inttargetRstTolerance {
        match self.bits {
            false => Gpio087inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio087inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio087inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio087inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO087INTTargetRstTolerance` writer - GPIO087 Interrupt Target Reset Tolerance"]
pub type Gpio087inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio087inttargetRstTolerance>;
impl<'a, REG> Gpio087inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio087inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio087inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO087INTTargetWrProt` reader - GPIO087 Interrupt Target Write Protection"]
pub type Gpio087inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO087INTTargetWrProt` writer - GPIO087 Interrupt Target Write Protection"]
pub type Gpio087inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO084 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio084intto_int13018(&self) -> EnblGpio084inttoInt13018R {
        EnblGpio084inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO084 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio084intto_int13019(&self) -> EnblGpio084inttoInt13019R {
        EnblGpio084inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO084 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio084intto_int13020(&self) -> EnblGpio084inttoInt13020R {
        EnblGpio084inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO084 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio084intto_sio(&self) -> EnblGpio084inttoSioR {
        EnblGpio084inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO084 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio084inttarget_rst_tolerance(&self) -> Gpio084inttargetRstToleranceR {
        Gpio084inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO084 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio084inttarget_wr_prot(&self) -> Gpio084inttargetWrProtR {
        Gpio084inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO085 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio085intto_int13018(&self) -> EnblGpio085inttoInt13018R {
        EnblGpio085inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO085 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio085intto_int13019(&self) -> EnblGpio085inttoInt13019R {
        EnblGpio085inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO085 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio085intto_int13020(&self) -> EnblGpio085inttoInt13020R {
        EnblGpio085inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO085 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio085intto_sio(&self) -> EnblGpio085inttoSioR {
        EnblGpio085inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO085 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio085inttarget_rst_tolerance(&self) -> Gpio085inttargetRstToleranceR {
        Gpio085inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO085 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio085inttarget_wr_prot(&self) -> Gpio085inttargetWrProtR {
        Gpio085inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO086 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio086intto_int13018(&self) -> EnblGpio086inttoInt13018R {
        EnblGpio086inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO086 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio086intto_int13019(&self) -> EnblGpio086inttoInt13019R {
        EnblGpio086inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO086 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio086intto_int13020(&self) -> EnblGpio086inttoInt13020R {
        EnblGpio086inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO086 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio086intto_sio(&self) -> EnblGpio086inttoSioR {
        EnblGpio086inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO086 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio086inttarget_rst_tolerance(&self) -> Gpio086inttargetRstToleranceR {
        Gpio086inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO086 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio086inttarget_wr_prot(&self) -> Gpio086inttargetWrProtR {
        Gpio086inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO087 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio087intto_int13018(&self) -> EnblGpio087inttoInt13018R {
        EnblGpio087inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO087 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio087intto_int13019(&self) -> EnblGpio087inttoInt13019R {
        EnblGpio087inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO087 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio087intto_int13020(&self) -> EnblGpio087inttoInt13020R {
        EnblGpio087inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO087 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio087intto_sio(&self) -> EnblGpio087inttoSioR {
        EnblGpio087inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO087 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio087inttarget_rst_tolerance(&self) -> Gpio087inttargetRstToleranceR {
        Gpio087inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO087 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio087inttarget_wr_prot(&self) -> Gpio087inttargetWrProtR {
        Gpio087inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO084 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio084intto_int13018(&mut self) -> EnblGpio084inttoInt13018W<Gpioa64Spec> {
        EnblGpio084inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO084 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio084intto_int13019(&mut self) -> EnblGpio084inttoInt13019W<Gpioa64Spec> {
        EnblGpio084inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO084 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio084intto_int13020(&mut self) -> EnblGpio084inttoInt13020W<Gpioa64Spec> {
        EnblGpio084inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO084 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio084intto_sio(&mut self) -> EnblGpio084inttoSioW<Gpioa64Spec> {
        EnblGpio084inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa64Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa64Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO084 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio084inttarget_rst_tolerance(&mut self) -> Gpio084inttargetRstToleranceW<Gpioa64Spec> {
        Gpio084inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO084 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio084inttarget_wr_prot(&mut self) -> Gpio084inttargetWrProtW<Gpioa64Spec> {
        Gpio084inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO085 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio085intto_int13018(&mut self) -> EnblGpio085inttoInt13018W<Gpioa64Spec> {
        EnblGpio085inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO085 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio085intto_int13019(&mut self) -> EnblGpio085inttoInt13019W<Gpioa64Spec> {
        EnblGpio085inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO085 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio085intto_int13020(&mut self) -> EnblGpio085inttoInt13020W<Gpioa64Spec> {
        EnblGpio085inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO085 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio085intto_sio(&mut self) -> EnblGpio085inttoSioW<Gpioa64Spec> {
        EnblGpio085inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa64Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa64Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO085 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio085inttarget_rst_tolerance(&mut self) -> Gpio085inttargetRstToleranceW<Gpioa64Spec> {
        Gpio085inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO085 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio085inttarget_wr_prot(&mut self) -> Gpio085inttargetWrProtW<Gpioa64Spec> {
        Gpio085inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO086 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio086intto_int13018(&mut self) -> EnblGpio086inttoInt13018W<Gpioa64Spec> {
        EnblGpio086inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO086 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio086intto_int13019(&mut self) -> EnblGpio086inttoInt13019W<Gpioa64Spec> {
        EnblGpio086inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO086 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio086intto_int13020(&mut self) -> EnblGpio086inttoInt13020W<Gpioa64Spec> {
        EnblGpio086inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO086 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio086intto_sio(&mut self) -> EnblGpio086inttoSioW<Gpioa64Spec> {
        EnblGpio086inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa64Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa64Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO086 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio086inttarget_rst_tolerance(&mut self) -> Gpio086inttargetRstToleranceW<Gpioa64Spec> {
        Gpio086inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO086 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio086inttarget_wr_prot(&mut self) -> Gpio086inttargetWrProtW<Gpioa64Spec> {
        Gpio086inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO087 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio087intto_int13018(&mut self) -> EnblGpio087inttoInt13018W<Gpioa64Spec> {
        EnblGpio087inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO087 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio087intto_int13019(&mut self) -> EnblGpio087inttoInt13019W<Gpioa64Spec> {
        EnblGpio087inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO087 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio087intto_int13020(&mut self) -> EnblGpio087inttoInt13020W<Gpioa64Spec> {
        EnblGpio087inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO087 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio087intto_sio(&mut self) -> EnblGpio087inttoSioW<Gpioa64Spec> {
        EnblGpio087inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa64Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO087 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio087inttarget_rst_tolerance(&mut self) -> Gpio087inttargetRstToleranceW<Gpioa64Spec> {
        Gpio087inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO087 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio087inttarget_wr_prot(&mut self) -> Gpio087inttargetWrProtW<Gpioa64Spec> {
        Gpio087inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa64Spec;
impl crate::RegisterSpec for Gpioa64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa64::R`](R) reader structure"]
impl crate::Readable for Gpioa64Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa64::W`](W) writer structure"]
impl crate::Writable for Gpioa64Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA64 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa64Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
