#[doc = "Register `GPIOA74` reader"]
pub type R = crate::R<Gpioa74Spec>;
#[doc = "Register `GPIOA74` writer"]
pub type W = crate::W<Gpioa74Spec>;
#[doc = "Field `EnblGPIO100INTToINT13018` reader - Enable GPIO100 Interrupt To INT#130_18"]
pub type EnblGpio100inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO100INTToINT13018` writer - Enable GPIO100 Interrupt To INT#130_18"]
pub type EnblGpio100inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO100INTToINT13019` reader - Enable GPIO100 Interrupt To INT#130_19"]
pub type EnblGpio100inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO100INTToINT13019` writer - Enable GPIO100 Interrupt To INT#130_19"]
pub type EnblGpio100inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO100INTToINT13020` reader - Enable GPIO100 Interrupt To INT#130_20"]
pub type EnblGpio100inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO100INTToINT13020` writer - Enable GPIO100 Interrupt To INT#130_20"]
pub type EnblGpio100inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO100INTToSIO` reader - Enable GPIO100 Interrupt To SIO"]
pub type EnblGpio100inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO100INTToSIO` writer - Enable GPIO100 Interrupt To SIO"]
pub type EnblGpio100inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO100 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio100inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio100inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio100inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO100INTTargetRstTolerance` reader - GPIO100 Interrupt Target Reset Tolerance"]
pub type Gpio100inttargetRstToleranceR = crate::BitReader<Gpio100inttargetRstTolerance>;
impl Gpio100inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio100inttargetRstTolerance {
        match self.bits {
            false => Gpio100inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio100inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio100inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio100inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO100INTTargetRstTolerance` writer - GPIO100 Interrupt Target Reset Tolerance"]
pub type Gpio100inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio100inttargetRstTolerance>;
impl<'a, REG> Gpio100inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio100inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio100inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO100INTTargetWrProt` reader - GPIO100 Interrupt Target Write Protection"]
pub type Gpio100inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO100INTTargetWrProt` writer - GPIO100 Interrupt Target Write Protection"]
pub type Gpio100inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO101INTToINT13018` reader - Enable GPIO101 Interrupt To INT#130_18"]
pub type EnblGpio101inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO101INTToINT13018` writer - Enable GPIO101 Interrupt To INT#130_18"]
pub type EnblGpio101inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO101INTToINT13019` reader - Enable GPIO101 Interrupt To INT#130_19"]
pub type EnblGpio101inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO101INTToINT13019` writer - Enable GPIO101 Interrupt To INT#130_19"]
pub type EnblGpio101inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO101INTToINT13020` reader - Enable GPIO101 Interrupt To INT#130_20"]
pub type EnblGpio101inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO101INTToINT13020` writer - Enable GPIO101 Interrupt To INT#130_20"]
pub type EnblGpio101inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO101INTToSIO` reader - Enable GPIO101 Interrupt To SIO"]
pub type EnblGpio101inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO101INTToSIO` writer - Enable GPIO101 Interrupt To SIO"]
pub type EnblGpio101inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO101 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio101inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio101inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio101inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO101INTTargetRstTolerance` reader - GPIO101 Interrupt Target Reset Tolerance"]
pub type Gpio101inttargetRstToleranceR = crate::BitReader<Gpio101inttargetRstTolerance>;
impl Gpio101inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio101inttargetRstTolerance {
        match self.bits {
            false => Gpio101inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio101inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio101inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio101inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO101INTTargetRstTolerance` writer - GPIO101 Interrupt Target Reset Tolerance"]
pub type Gpio101inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio101inttargetRstTolerance>;
impl<'a, REG> Gpio101inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio101inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio101inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO101INTTargetWrProt` reader - GPIO101 Interrupt Target Write Protection"]
pub type Gpio101inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO101INTTargetWrProt` writer - GPIO101 Interrupt Target Write Protection"]
pub type Gpio101inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO102INTToINT13018` reader - Enable GPIO102 Interrupt To INT#130_18"]
pub type EnblGpio102inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO102INTToINT13018` writer - Enable GPIO102 Interrupt To INT#130_18"]
pub type EnblGpio102inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO102INTToINT13019` reader - Enable GPIO102 Interrupt To INT#130_19"]
pub type EnblGpio102inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO102INTToINT13019` writer - Enable GPIO102 Interrupt To INT#130_19"]
pub type EnblGpio102inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO102INTToINT13020` reader - Enable GPIO102 Interrupt To INT#130_20"]
pub type EnblGpio102inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO102INTToINT13020` writer - Enable GPIO102 Interrupt To INT#130_20"]
pub type EnblGpio102inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO102INTToSIO` reader - Enable GPIO102 Interrupt To SIO"]
pub type EnblGpio102inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO102INTToSIO` writer - Enable GPIO102 Interrupt To SIO"]
pub type EnblGpio102inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO102 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio102inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio102inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio102inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO102INTTargetRstTolerance` reader - GPIO102 Interrupt Target Reset Tolerance"]
pub type Gpio102inttargetRstToleranceR = crate::BitReader<Gpio102inttargetRstTolerance>;
impl Gpio102inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio102inttargetRstTolerance {
        match self.bits {
            false => Gpio102inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio102inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio102inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio102inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO102INTTargetRstTolerance` writer - GPIO102 Interrupt Target Reset Tolerance"]
pub type Gpio102inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio102inttargetRstTolerance>;
impl<'a, REG> Gpio102inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio102inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio102inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO102INTTargetWrProt` reader - GPIO102 Interrupt Target Write Protection"]
pub type Gpio102inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO102INTTargetWrProt` writer - GPIO102 Interrupt Target Write Protection"]
pub type Gpio102inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO103INTToINT13018` reader - Enable GPIO103 Interrupt To INT#130_18"]
pub type EnblGpio103inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO103INTToINT13018` writer - Enable GPIO103 Interrupt To INT#130_18"]
pub type EnblGpio103inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO103INTToINT13019` reader - Enable GPIO103 Interrupt To INT#130_19"]
pub type EnblGpio103inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO103INTToINT13019` writer - Enable GPIO103 Interrupt To INT#130_19"]
pub type EnblGpio103inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO103INTToINT13020` reader - Enable GPIO103 Interrupt To INT#130_20"]
pub type EnblGpio103inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO103INTToINT13020` writer - Enable GPIO103 Interrupt To INT#130_20"]
pub type EnblGpio103inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO103INTToSIO` reader - Enable GPIO103 Interrupt To SIO"]
pub type EnblGpio103inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO103INTToSIO` writer - Enable GPIO103 Interrupt To SIO"]
pub type EnblGpio103inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO103 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio103inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio103inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio103inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO103INTTargetRstTolerance` reader - GPIO103 Interrupt Target Reset Tolerance"]
pub type Gpio103inttargetRstToleranceR = crate::BitReader<Gpio103inttargetRstTolerance>;
impl Gpio103inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio103inttargetRstTolerance {
        match self.bits {
            false => Gpio103inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio103inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio103inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio103inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO103INTTargetRstTolerance` writer - GPIO103 Interrupt Target Reset Tolerance"]
pub type Gpio103inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio103inttargetRstTolerance>;
impl<'a, REG> Gpio103inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio103inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio103inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO103INTTargetWrProt` reader - GPIO103 Interrupt Target Write Protection"]
pub type Gpio103inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO103INTTargetWrProt` writer - GPIO103 Interrupt Target Write Protection"]
pub type Gpio103inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO100 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio100intto_int13018(&self) -> EnblGpio100inttoInt13018R {
        EnblGpio100inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO100 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio100intto_int13019(&self) -> EnblGpio100inttoInt13019R {
        EnblGpio100inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO100 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio100intto_int13020(&self) -> EnblGpio100inttoInt13020R {
        EnblGpio100inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO100 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio100intto_sio(&self) -> EnblGpio100inttoSioR {
        EnblGpio100inttoSioR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - GPIO100 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio100inttarget_rst_tolerance(&self) -> Gpio100inttargetRstToleranceR {
        Gpio100inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO100 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio100inttarget_wr_prot(&self) -> Gpio100inttargetWrProtR {
        Gpio100inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO101 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio101intto_int13018(&self) -> EnblGpio101inttoInt13018R {
        EnblGpio101inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO101 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio101intto_int13019(&self) -> EnblGpio101inttoInt13019R {
        EnblGpio101inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO101 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio101intto_int13020(&self) -> EnblGpio101inttoInt13020R {
        EnblGpio101inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO101 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio101intto_sio(&self) -> EnblGpio101inttoSioR {
        EnblGpio101inttoSioR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - GPIO101 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio101inttarget_rst_tolerance(&self) -> Gpio101inttargetRstToleranceR {
        Gpio101inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO101 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio101inttarget_wr_prot(&self) -> Gpio101inttargetWrProtR {
        Gpio101inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable GPIO102 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio102intto_int13018(&self) -> EnblGpio102inttoInt13018R {
        EnblGpio102inttoInt13018R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO102 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio102intto_int13019(&self) -> EnblGpio102inttoInt13019R {
        EnblGpio102inttoInt13019R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO102 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio102intto_int13020(&self) -> EnblGpio102inttoInt13020R {
        EnblGpio102inttoInt13020R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO102 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio102intto_sio(&self) -> EnblGpio102inttoSioR {
        EnblGpio102inttoSioR::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 22 - GPIO102 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio102inttarget_rst_tolerance(&self) -> Gpio102inttargetRstToleranceR {
        Gpio102inttargetRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO102 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio102inttarget_wr_prot(&self) -> Gpio102inttargetWrProtR {
        Gpio102inttargetWrProtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO103 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio103intto_int13018(&self) -> EnblGpio103inttoInt13018R {
        EnblGpio103inttoInt13018R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable GPIO103 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio103intto_int13019(&self) -> EnblGpio103inttoInt13019R {
        EnblGpio103inttoInt13019R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable GPIO103 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio103intto_int13020(&self) -> EnblGpio103inttoInt13020R {
        EnblGpio103inttoInt13020R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable GPIO103 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio103intto_sio(&self) -> EnblGpio103inttoSioR {
        EnblGpio103inttoSioR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO103 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio103inttarget_rst_tolerance(&self) -> Gpio103inttargetRstToleranceR {
        Gpio103inttargetRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO103 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio103inttarget_wr_prot(&self) -> Gpio103inttargetWrProtR {
        Gpio103inttargetWrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO100 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio100intto_int13018(&mut self) -> EnblGpio100inttoInt13018W<Gpioa74Spec> {
        EnblGpio100inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO100 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio100intto_int13019(&mut self) -> EnblGpio100inttoInt13019W<Gpioa74Spec> {
        EnblGpio100inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO100 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio100intto_int13020(&mut self) -> EnblGpio100inttoInt13020W<Gpioa74Spec> {
        EnblGpio100inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO100 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio100intto_sio(&mut self) -> EnblGpio100inttoSioW<Gpioa74Spec> {
        EnblGpio100inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioa74Spec> {
        Reserved7W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioa74Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO100 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio100inttarget_rst_tolerance(&mut self) -> Gpio100inttargetRstToleranceW<Gpioa74Spec> {
        Gpio100inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO100 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio100inttarget_wr_prot(&mut self) -> Gpio100inttargetWrProtW<Gpioa74Spec> {
        Gpio100inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO101 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio101intto_int13018(&mut self) -> EnblGpio101inttoInt13018W<Gpioa74Spec> {
        EnblGpio101inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO101 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio101intto_int13019(&mut self) -> EnblGpio101inttoInt13019W<Gpioa74Spec> {
        EnblGpio101inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO101 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio101intto_int13020(&mut self) -> EnblGpio101inttoInt13020W<Gpioa74Spec> {
        EnblGpio101inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO101 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio101intto_sio(&mut self) -> EnblGpio101inttoSioW<Gpioa74Spec> {
        EnblGpio101inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioa74Spec> {
        Reserved5W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioa74Spec> {
        Reserved4W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO101 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio101inttarget_rst_tolerance(&mut self) -> Gpio101inttargetRstToleranceW<Gpioa74Spec> {
        Gpio101inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO101 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio101inttarget_wr_prot(&mut self) -> Gpio101inttargetWrProtW<Gpioa74Spec> {
        Gpio101inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable GPIO102 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio102intto_int13018(&mut self) -> EnblGpio102inttoInt13018W<Gpioa74Spec> {
        EnblGpio102inttoInt13018W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO102 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio102intto_int13019(&mut self) -> EnblGpio102inttoInt13019W<Gpioa74Spec> {
        EnblGpio102inttoInt13019W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO102 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio102intto_int13020(&mut self) -> EnblGpio102inttoInt13020W<Gpioa74Spec> {
        EnblGpio102inttoInt13020W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO102 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio102intto_sio(&mut self) -> EnblGpio102inttoSioW<Gpioa74Spec> {
        EnblGpio102inttoSioW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioa74Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioa74Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO102 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio102inttarget_rst_tolerance(&mut self) -> Gpio102inttargetRstToleranceW<Gpioa74Spec> {
        Gpio102inttargetRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO102 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio102inttarget_wr_prot(&mut self) -> Gpio102inttargetWrProtW<Gpioa74Spec> {
        Gpio102inttargetWrProtW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO103 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio103intto_int13018(&mut self) -> EnblGpio103inttoInt13018W<Gpioa74Spec> {
        EnblGpio103inttoInt13018W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable GPIO103 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio103intto_int13019(&mut self) -> EnblGpio103inttoInt13019W<Gpioa74Spec> {
        EnblGpio103inttoInt13019W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable GPIO103 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio103intto_int13020(&mut self) -> EnblGpio103inttoInt13020W<Gpioa74Spec> {
        EnblGpio103inttoInt13020W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable GPIO103 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio103intto_sio(&mut self) -> EnblGpio103inttoSioW<Gpioa74Spec> {
        EnblGpio103inttoSioW::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioa74Spec> {
        Reserved1W::new(self, 28)
    }
    #[doc = "Bit 30 - GPIO103 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio103inttarget_rst_tolerance(&mut self) -> Gpio103inttargetRstToleranceW<Gpioa74Spec> {
        Gpio103inttargetRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO103 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio103inttarget_wr_prot(&mut self) -> Gpio103inttargetWrProtW<Gpioa74Spec> {
        Gpio103inttargetWrProtW::new(self, 31)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa74::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa74::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioa74Spec;
impl crate::RegisterSpec for Gpioa74Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa74::R`](R) reader structure"]
impl crate::Readable for Gpioa74Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioa74::W`](W) writer structure"]
impl crate::Writable for Gpioa74Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOA74 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioa74Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
