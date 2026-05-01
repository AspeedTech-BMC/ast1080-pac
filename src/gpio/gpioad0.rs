#[doc = "Register `GPIOAD0` reader"]
pub type R = crate::R<Gpioad0Spec>;
#[doc = "Register `GPIOAD0` writer"]
pub type W = crate::W<Gpioad0Spec>;
#[doc = "Field `EnblGPIO192INTToINT13018` reader - Enable GPIO192 Interrupt To INT#130_18"]
pub type EnblGpio192inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO192INTToINT13018` writer - Enable GPIO192 Interrupt To INT#130_18"]
pub type EnblGpio192inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO192INTToINT13019` reader - Enable GPIO192 Interrupt To INT#130_19"]
pub type EnblGpio192inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO192INTToINT13019` writer - Enable GPIO192 Interrupt To INT#130_19"]
pub type EnblGpio192inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO192INTToINT13020` reader - Enable GPIO192 Interrupt To INT#130_20"]
pub type EnblGpio192inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO192INTToINT13020` writer - Enable GPIO192 Interrupt To INT#130_20"]
pub type EnblGpio192inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO192INTToSIO` reader - Enable GPIO192 Interrupt To SIO"]
pub type EnblGpio192inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO192INTToSIO` writer - Enable GPIO192 Interrupt To SIO"]
pub type EnblGpio192inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved19` reader - Reserved"]
pub type Reserved19R = crate::BitReader;
#[doc = "Field `Reserved19` writer - Reserved"]
pub type Reserved19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved18` reader - Reserved"]
pub type Reserved18R = crate::BitReader;
#[doc = "Field `Reserved18` writer - Reserved"]
pub type Reserved18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO192 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio192inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio192inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio192inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO192INTTargetRstTolerance` reader - GPIO192 Interrupt Target Reset Tolerance"]
pub type Gpio192inttargetRstToleranceR = crate::BitReader<Gpio192inttargetRstTolerance>;
impl Gpio192inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio192inttargetRstTolerance {
        match self.bits {
            false => Gpio192inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio192inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio192inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio192inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO192INTTargetRstTolerance` writer - GPIO192 Interrupt Target Reset Tolerance"]
pub type Gpio192inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio192inttargetRstTolerance>;
impl<'a, REG> Gpio192inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio192inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio192inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO192INTTargetWrProt` reader - GPIO192 Interrupt Target Write Protection"]
pub type Gpio192inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO192INTTargetWrProt` writer - GPIO192 Interrupt Target Write Protection"]
pub type Gpio192inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO193INTToINT13018` reader - Enable GPIO193 Interrupt To INT#130_18"]
pub type EnblGpio193inttoInt13018R = crate::BitReader;
#[doc = "Field `EnblGPIO193INTToINT13018` writer - Enable GPIO193 Interrupt To INT#130_18"]
pub type EnblGpio193inttoInt13018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO193INTToINT13019` reader - Enable GPIO193 Interrupt To INT#130_19"]
pub type EnblGpio193inttoInt13019R = crate::BitReader;
#[doc = "Field `EnblGPIO193INTToINT13019` writer - Enable GPIO193 Interrupt To INT#130_19"]
pub type EnblGpio193inttoInt13019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO193INTToINT13020` reader - Enable GPIO193 Interrupt To INT#130_20"]
pub type EnblGpio193inttoInt13020R = crate::BitReader;
#[doc = "Field `EnblGPIO193INTToINT13020` writer - Enable GPIO193 Interrupt To INT#130_20"]
pub type EnblGpio193inttoInt13020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIO193INTToSIO` reader - Enable GPIO193 Interrupt To SIO"]
pub type EnblGpio193inttoSioR = crate::BitReader;
#[doc = "Field `EnblGPIO193INTToSIO` writer - Enable GPIO193 Interrupt To SIO"]
pub type EnblGpio193inttoSioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved17` reader - Reserved"]
pub type Reserved17R = crate::BitReader;
#[doc = "Field `Reserved17` writer - Reserved"]
pub type Reserved17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved16` reader - Reserved"]
pub type Reserved16R = crate::BitReader;
#[doc = "Field `Reserved16` writer - Reserved"]
pub type Reserved16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "GPIO193 Interrupt Target Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio193inttargetRstTolerance {
    #[doc = "0: Interrupt Target is reset by WDT."]
    InterruptTargetIsResetByWdt = 0,
    #[doc = "1: Interrupt Target is NOT reset by WDT."]
    InterruptTargetIsNotResetByWdt = 1,
}
impl From<Gpio193inttargetRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio193inttargetRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO193INTTargetRstTolerance` reader - GPIO193 Interrupt Target Reset Tolerance"]
pub type Gpio193inttargetRstToleranceR = crate::BitReader<Gpio193inttargetRstTolerance>;
impl Gpio193inttargetRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio193inttargetRstTolerance {
        match self.bits {
            false => Gpio193inttargetRstTolerance::InterruptTargetIsResetByWdt,
            true => Gpio193inttargetRstTolerance::InterruptTargetIsNotResetByWdt,
        }
    }
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_reset_by_wdt(&self) -> bool {
        *self == Gpio193inttargetRstTolerance::InterruptTargetIsResetByWdt
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_interrupt_target_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio193inttargetRstTolerance::InterruptTargetIsNotResetByWdt
    }
}
#[doc = "Field `GPIO193INTTargetRstTolerance` writer - GPIO193 Interrupt Target Reset Tolerance"]
pub type Gpio193inttargetRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio193inttargetRstTolerance>;
impl<'a, REG> Gpio193inttargetRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt Target is reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio193inttargetRstTolerance::InterruptTargetIsResetByWdt)
    }
    #[doc = "Interrupt Target is NOT reset by WDT."]
    #[inline(always)]
    pub fn interrupt_target_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio193inttargetRstTolerance::InterruptTargetIsNotResetByWdt)
    }
}
#[doc = "Field `GPIO193INTTargetWrProt` reader - GPIO193 Interrupt Target Write Protection"]
pub type Gpio193inttargetWrProtR = crate::BitReader;
#[doc = "Field `GPIO193INTTargetWrProt` writer - GPIO193 Interrupt Target Write Protection"]
pub type Gpio193inttargetWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved15` reader - Reserved"]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `Reserved15` writer - Reserved"]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved14` reader - Reserved"]
pub type Reserved14R = crate::BitReader;
#[doc = "Field `Reserved14` writer - Reserved"]
pub type Reserved14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved13` reader - Reserved"]
pub type Reserved13R = crate::BitReader;
#[doc = "Field `Reserved13` writer - Reserved"]
pub type Reserved13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved12` reader - Reserved"]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `Reserved12` writer - Reserved"]
pub type Reserved12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved11` reader - Reserved"]
pub type Reserved11R = crate::BitReader;
#[doc = "Field `Reserved11` writer - Reserved"]
pub type Reserved11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved10` reader - Reserved"]
pub type Reserved10R = crate::BitReader;
#[doc = "Field `Reserved10` writer - Reserved"]
pub type Reserved10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `Reserved9` writer - Reserved"]
pub type Reserved9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `Reserved8` writer - Reserved"]
pub type Reserved8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO192 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio192intto_int13018(&self) -> EnblGpio192inttoInt13018R {
        EnblGpio192inttoInt13018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO192 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio192intto_int13019(&self) -> EnblGpio192inttoInt13019R {
        EnblGpio192inttoInt13019R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable GPIO192 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio192intto_int13020(&self) -> EnblGpio192inttoInt13020R {
        EnblGpio192inttoInt13020R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable GPIO192 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio192intto_sio(&self) -> EnblGpio192inttoSioR {
        EnblGpio192inttoSioR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO192 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio192inttarget_rst_tolerance(&self) -> Gpio192inttargetRstToleranceR {
        Gpio192inttargetRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO192 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio192inttarget_wr_prot(&self) -> Gpio192inttargetWrProtR {
        Gpio192inttargetWrProtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable GPIO193 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio193intto_int13018(&self) -> EnblGpio193inttoInt13018R {
        EnblGpio193inttoInt13018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable GPIO193 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio193intto_int13019(&self) -> EnblGpio193inttoInt13019R {
        EnblGpio193inttoInt13019R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable GPIO193 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio193intto_int13020(&self) -> EnblGpio193inttoInt13020R {
        EnblGpio193inttoInt13020R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO193 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio193intto_sio(&self) -> EnblGpio193inttoSioR {
        EnblGpio193inttoSioR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO193 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio193inttarget_rst_tolerance(&self) -> Gpio193inttargetRstToleranceR {
        Gpio193inttargetRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO193 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio193inttarget_wr_prot(&self) -> Gpio193inttargetWrProtR {
        Gpio193inttargetWrProtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO192 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio192intto_int13018(&mut self) -> EnblGpio192inttoInt13018W<Gpioad0Spec> {
        EnblGpio192inttoInt13018W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO192 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio192intto_int13019(&mut self) -> EnblGpio192inttoInt13019W<Gpioad0Spec> {
        EnblGpio192inttoInt13019W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable GPIO192 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio192intto_int13020(&mut self) -> EnblGpio192inttoInt13020W<Gpioad0Spec> {
        EnblGpio192inttoInt13020W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable GPIO192 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio192intto_sio(&mut self) -> EnblGpio192inttoSioW<Gpioad0Spec> {
        EnblGpio192inttoSioW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved19(&mut self) -> Reserved19W<Gpioad0Spec> {
        Reserved19W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved18(&mut self) -> Reserved18W<Gpioad0Spec> {
        Reserved18W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO192 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio192inttarget_rst_tolerance(&mut self) -> Gpio192inttargetRstToleranceW<Gpioad0Spec> {
        Gpio192inttargetRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO192 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio192inttarget_wr_prot(&mut self) -> Gpio192inttargetWrProtW<Gpioad0Spec> {
        Gpio192inttargetWrProtW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable GPIO193 Interrupt To INT#130_18"]
    #[inline(always)]
    pub fn enbl_gpio193intto_int13018(&mut self) -> EnblGpio193inttoInt13018W<Gpioad0Spec> {
        EnblGpio193inttoInt13018W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable GPIO193 Interrupt To INT#130_19"]
    #[inline(always)]
    pub fn enbl_gpio193intto_int13019(&mut self) -> EnblGpio193inttoInt13019W<Gpioad0Spec> {
        EnblGpio193inttoInt13019W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable GPIO193 Interrupt To INT#130_20"]
    #[inline(always)]
    pub fn enbl_gpio193intto_int13020(&mut self) -> EnblGpio193inttoInt13020W<Gpioad0Spec> {
        EnblGpio193inttoInt13020W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable GPIO193 Interrupt To SIO"]
    #[inline(always)]
    pub fn enbl_gpio193intto_sio(&mut self) -> EnblGpio193inttoSioW<Gpioad0Spec> {
        EnblGpio193inttoSioW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved17(&mut self) -> Reserved17W<Gpioad0Spec> {
        Reserved17W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved16(&mut self) -> Reserved16W<Gpioad0Spec> {
        Reserved16W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO193 Interrupt Target Reset Tolerance"]
    #[inline(always)]
    pub fn gpio193inttarget_rst_tolerance(&mut self) -> Gpio193inttargetRstToleranceW<Gpioad0Spec> {
        Gpio193inttargetRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO193 Interrupt Target Write Protection"]
    #[inline(always)]
    pub fn gpio193inttarget_wr_prot(&mut self) -> Gpio193inttargetWrProtW<Gpioad0Spec> {
        Gpio193inttargetWrProtW::new(self, 15)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&mut self) -> Reserved15W<Gpioad0Spec> {
        Reserved15W::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&mut self) -> Reserved14W<Gpioad0Spec> {
        Reserved14W::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&mut self) -> Reserved13W<Gpioad0Spec> {
        Reserved13W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&mut self) -> Reserved12W<Gpioad0Spec> {
        Reserved12W::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&mut self) -> Reserved11W<Gpioad0Spec> {
        Reserved11W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&mut self) -> Reserved10W<Gpioad0Spec> {
        Reserved10W::new(self, 21)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<Gpioad0Spec> {
        Reserved9W::new(self, 22)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<Gpioad0Spec> {
        Reserved8W::new(self, 23)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Gpioad0Spec> {
        Reserved7W::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpioad0Spec> {
        Reserved6W::new(self, 25)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpioad0Spec> {
        Reserved5W::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpioad0Spec> {
        Reserved4W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpioad0Spec> {
        Reserved3W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpioad0Spec> {
        Reserved2W::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpioad0Spec> {
        Reserved1W::new(self, 30)
    }
}
#[doc = "GPIO Interrupt Target Control Register \\#48\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioad0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioad0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioad0Spec;
impl crate::RegisterSpec for Gpioad0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioad0::R`](R) reader structure"]
impl crate::Readable for Gpioad0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioad0::W`](W) writer structure"]
impl crate::Writable for Gpioad0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOAD0 to value 0x1f1f_1f1f"]
impl crate::Resettable for Gpioad0Spec {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
