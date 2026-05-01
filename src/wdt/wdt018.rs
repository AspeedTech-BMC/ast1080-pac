#[doc = "Register `WDT018` reader"]
pub type R = crate::R<Wdt018Spec>;
#[doc = "Register `WDT018` writer"]
pub type W = crate::W<Wdt018Spec>;
#[doc = "Field `RstWidth` reader - Reset width"]
pub type RstWidthR = crate::FieldReader<u32>;
#[doc = "Field `RstWidth` writer - Reset width"]
pub type RstWidthW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Reset pulse generation trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstPulseGenerationTriggerSel {
    #[doc = "0: trigger by timeout"]
    TriggerByTimeout = 0,
    #[doc = "1: trigger by pre-timeout"]
    TriggerByPretimeout = 1,
}
impl From<RstPulseGenerationTriggerSel> for bool {
    #[inline(always)]
    fn from(variant: RstPulseGenerationTriggerSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstPulseGenerationTriggerSel` reader - Reset pulse generation trigger selection"]
pub type RstPulseGenerationTriggerSelR = crate::BitReader<RstPulseGenerationTriggerSel>;
impl RstPulseGenerationTriggerSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstPulseGenerationTriggerSel {
        match self.bits {
            false => RstPulseGenerationTriggerSel::TriggerByTimeout,
            true => RstPulseGenerationTriggerSel::TriggerByPretimeout,
        }
    }
    #[doc = "trigger by timeout"]
    #[inline(always)]
    pub fn is_trigger_by_timeout(&self) -> bool {
        *self == RstPulseGenerationTriggerSel::TriggerByTimeout
    }
    #[doc = "trigger by pre-timeout"]
    #[inline(always)]
    pub fn is_trigger_by_pretimeout(&self) -> bool {
        *self == RstPulseGenerationTriggerSel::TriggerByPretimeout
    }
}
#[doc = "Field `RstPulseGenerationTriggerSel` writer - Reset pulse generation trigger selection"]
pub type RstPulseGenerationTriggerSelW<'a, REG> =
    crate::BitWriter<'a, REG, RstPulseGenerationTriggerSel>;
impl<'a, REG> RstPulseGenerationTriggerSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "trigger by timeout"]
    #[inline(always)]
    pub fn trigger_by_timeout(self) -> &'a mut crate::W<REG> {
        self.variant(RstPulseGenerationTriggerSel::TriggerByTimeout)
    }
    #[doc = "trigger by pre-timeout"]
    #[inline(always)]
    pub fn trigger_by_pretimeout(self) -> &'a mut crate::W<REG> {
        self.variant(RstPulseGenerationTriggerSel::TriggerByPretimeout)
    }
}
#[doc = "Reset pulse output driving type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstPulseOutputDrivingType {
    #[doc = "0: open-drain"]
    Opendrain = 0,
    #[doc = "1: push-pull"]
    Pushpull = 1,
}
impl From<RstPulseOutputDrivingType> for bool {
    #[inline(always)]
    fn from(variant: RstPulseOutputDrivingType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstPulseOutputDrivingType` reader - Reset pulse output driving type"]
pub type RstPulseOutputDrivingTypeR = crate::BitReader<RstPulseOutputDrivingType>;
impl RstPulseOutputDrivingTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstPulseOutputDrivingType {
        match self.bits {
            false => RstPulseOutputDrivingType::Opendrain,
            true => RstPulseOutputDrivingType::Pushpull,
        }
    }
    #[doc = "open-drain"]
    #[inline(always)]
    pub fn is_opendrain(&self) -> bool {
        *self == RstPulseOutputDrivingType::Opendrain
    }
    #[doc = "push-pull"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == RstPulseOutputDrivingType::Pushpull
    }
}
#[doc = "Field `RstPulseOutputDrivingType` writer - Reset pulse output driving type"]
pub type RstPulseOutputDrivingTypeW<'a, REG> = crate::BitWriter<'a, REG, RstPulseOutputDrivingType>;
impl<'a, REG> RstPulseOutputDrivingTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "open-drain"]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut crate::W<REG> {
        self.variant(RstPulseOutputDrivingType::Opendrain)
    }
    #[doc = "push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(RstPulseOutputDrivingType::Pushpull)
    }
}
#[doc = "Reset pulse polarity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstPulsePolaritySel {
    #[doc = "0: active low output"]
    ActiveLowOutput = 0,
    #[doc = "1: active high output"]
    ActiveHighOutput = 1,
}
impl From<RstPulsePolaritySel> for bool {
    #[inline(always)]
    fn from(variant: RstPulsePolaritySel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstPulsePolaritySel` reader - Reset pulse polarity selection"]
pub type RstPulsePolaritySelR = crate::BitReader<RstPulsePolaritySel>;
impl RstPulsePolaritySelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstPulsePolaritySel {
        match self.bits {
            false => RstPulsePolaritySel::ActiveLowOutput,
            true => RstPulsePolaritySel::ActiveHighOutput,
        }
    }
    #[doc = "active low output"]
    #[inline(always)]
    pub fn is_active_low_output(&self) -> bool {
        *self == RstPulsePolaritySel::ActiveLowOutput
    }
    #[doc = "active high output"]
    #[inline(always)]
    pub fn is_active_high_output(&self) -> bool {
        *self == RstPulsePolaritySel::ActiveHighOutput
    }
}
#[doc = "Field `RstPulsePolaritySel` writer - Reset pulse polarity selection"]
pub type RstPulsePolaritySelW<'a, REG> = crate::BitWriter<'a, REG, RstPulsePolaritySel>;
impl<'a, REG> RstPulsePolaritySelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "active low output"]
    #[inline(always)]
    pub fn active_low_output(self) -> &'a mut crate::W<REG> {
        self.variant(RstPulsePolaritySel::ActiveLowOutput)
    }
    #[doc = "active high output"]
    #[inline(always)]
    pub fn active_high_output(self) -> &'a mut crate::W<REG> {
        self.variant(RstPulsePolaritySel::ActiveHighOutput)
    }
}
impl R {
    #[doc = "Bits 0:19 - Reset width"]
    #[inline(always)]
    pub fn rst_width(&self) -> RstWidthR {
        RstWidthR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:28 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - Reset pulse generation trigger selection"]
    #[inline(always)]
    pub fn rst_pulse_generation_trigger_sel(&self) -> RstPulseGenerationTriggerSelR {
        RstPulseGenerationTriggerSelR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset pulse output driving type"]
    #[inline(always)]
    pub fn rst_pulse_output_driving_type(&self) -> RstPulseOutputDrivingTypeR {
        RstPulseOutputDrivingTypeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reset pulse polarity selection"]
    #[inline(always)]
    pub fn rst_pulse_polarity_sel(&self) -> RstPulsePolaritySelR {
        RstPulsePolaritySelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Reset width"]
    #[inline(always)]
    pub fn rst_width(&mut self) -> RstWidthW<Wdt018Spec> {
        RstWidthW::new(self, 0)
    }
    #[doc = "Bit 29 - Reset pulse generation trigger selection"]
    #[inline(always)]
    pub fn rst_pulse_generation_trigger_sel(
        &mut self,
    ) -> RstPulseGenerationTriggerSelW<Wdt018Spec> {
        RstPulseGenerationTriggerSelW::new(self, 29)
    }
    #[doc = "Bit 30 - Reset pulse output driving type"]
    #[inline(always)]
    pub fn rst_pulse_output_driving_type(&mut self) -> RstPulseOutputDrivingTypeW<Wdt018Spec> {
        RstPulseOutputDrivingTypeW::new(self, 30)
    }
    #[doc = "Bit 31 - Reset pulse polarity selection"]
    #[inline(always)]
    pub fn rst_pulse_polarity_sel(&mut self) -> RstPulsePolaritySelW<Wdt018Spec> {
        RstPulsePolaritySelW::new(self, 31)
    }
}
#[doc = "WDTn Reset Width Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt018Spec;
impl crate::RegisterSpec for Wdt018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt018::R`](R) reader structure"]
impl crate::Readable for Wdt018Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt018::W`](W) writer structure"]
impl crate::Writable for Wdt018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT018 to value 0xff"]
impl crate::Resettable for Wdt018Spec {
    const RESET_VALUE: u32 = 0xff;
}
