#[doc = "Register `ADC184` reader"]
pub type R = crate::R<Adc184Spec>;
#[doc = "Register `ADC184` writer"]
pub type W = crate::W<Adc184Spec>;
#[doc = "Field `LowerBound` reader - Lower bound"]
pub type LowerBoundR = crate::FieldReader<u16>;
#[doc = "Field `LowerBound` writer - Lower bound"]
pub type LowerBoundW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `UpperBound` reader - Upper bound"]
pub type UpperBoundR = crate::FieldReader<u16>;
#[doc = "Field `UpperBound` writer - Upper bound"]
pub type UpperBoundW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Hysteresis Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HysteresisCtrl {
    #[doc = "1: Enable hysteresis."]
    EnableHysteresis = 1,
    #[doc = "0: Disable hysteresis."]
    DisableHysteresis = 0,
}
impl From<HysteresisCtrl> for bool {
    #[inline(always)]
    fn from(variant: HysteresisCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HysteresisCtrl` reader - Hysteresis Control"]
pub type HysteresisCtrlR = crate::BitReader<HysteresisCtrl>;
impl HysteresisCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HysteresisCtrl {
        match self.bits {
            true => HysteresisCtrl::EnableHysteresis,
            false => HysteresisCtrl::DisableHysteresis,
        }
    }
    #[doc = "Enable hysteresis."]
    #[inline(always)]
    pub fn is_enable_hysteresis(&self) -> bool {
        *self == HysteresisCtrl::EnableHysteresis
    }
    #[doc = "Disable hysteresis."]
    #[inline(always)]
    pub fn is_disable_hysteresis(&self) -> bool {
        *self == HysteresisCtrl::DisableHysteresis
    }
}
#[doc = "Field `HysteresisCtrl` writer - Hysteresis Control"]
pub type HysteresisCtrlW<'a, REG> = crate::BitWriter<'a, REG, HysteresisCtrl>;
impl<'a, REG> HysteresisCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable hysteresis."]
    #[inline(always)]
    pub fn enable_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(HysteresisCtrl::EnableHysteresis)
    }
    #[doc = "Disable hysteresis."]
    #[inline(always)]
    pub fn disable_hysteresis(self) -> &'a mut crate::W<REG> {
        self.variant(HysteresisCtrl::DisableHysteresis)
    }
}
impl R {
    #[doc = "Bits 0:9 - Lower bound"]
    #[inline(always)]
    pub fn lower_bound(&self) -> LowerBoundR {
        LowerBoundR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - Upper bound"]
    #[inline(always)]
    pub fn upper_bound(&self) -> UpperBoundR {
        UpperBoundR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Hysteresis Control"]
    #[inline(always)]
    pub fn hysteresis_ctrl(&self) -> HysteresisCtrlR {
        HysteresisCtrlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Lower bound"]
    #[inline(always)]
    pub fn lower_bound(&mut self) -> LowerBoundW<Adc184Spec> {
        LowerBoundW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Upper bound"]
    #[inline(always)]
    pub fn upper_bound(&mut self) -> UpperBoundW<Adc184Spec> {
        UpperBoundW::new(self, 16)
    }
    #[doc = "Bit 31 - Hysteresis Control"]
    #[inline(always)]
    pub fn hysteresis_ctrl(&mut self) -> HysteresisCtrlW<Adc184Spec> {
        HysteresisCtrlW::new(self, 31)
    }
}
#[doc = "Hysteresis Control and bound of Channel 13\n\nYou can [`read`](crate::Reg::read) this register and get [`adc184::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc184::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc184Spec;
impl crate::RegisterSpec for Adc184Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc184::R`](R) reader structure"]
impl crate::Readable for Adc184Spec {}
#[doc = "`write(|w| ..)` method takes [`adc184::W`](W) writer structure"]
impl crate::Writable for Adc184Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC184 to value 0"]
impl crate::Resettable for Adc184Spec {}
