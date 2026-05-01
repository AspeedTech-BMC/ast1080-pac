#[doc = "Register `I3CPHYCTRLREG0D4` reader"]
pub type R = crate::R<I3cphyctrlreg0d4Spec>;
#[doc = "Register `I3CPHYCTRLREG0D4` writer"]
pub type W = crate::W<I3cphyctrlreg0d4Spec>;
#[doc = "Field `REGI2CSPIKEFILTEREN` reader - REG_I2C_SPIKE_FILTER_EN"]
pub type Regi2cspikefilterenR = crate::FieldReader;
#[doc = "Field `REGI2CSPIKEFILTEREN` writer - REG_I2C_SPIKE_FILTER_EN"]
pub type Regi2cspikefilterenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGI3CSPIKEFILTEREN` reader - REG_I3C_SPIKE_FILTER_EN"]
pub type Regi3cspikefilterenR = crate::FieldReader;
#[doc = "Field `REGI3CSPIKEFILTEREN` writer - REG_I3C_SPIKE_FILTER_EN"]
pub type Regi3cspikefilterenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGI3CSPIKEFILTEROFFAFTER7ERNW0EN` reader - REG_I3C_SPIKE_FILTER_OFF_AFTER_7E_RNW0_EN"]
pub type Regi3cspikefilteroffafter7ernw0enR = crate::BitReader;
#[doc = "Field `REGI3CSPIKEFILTEROFFAFTER7ERNW0EN` writer - REG_I3C_SPIKE_FILTER_OFF_AFTER_7E_RNW0_EN"]
pub type Regi3cspikefilteroffafter7ernw0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGI3CSPIKEFILTERONAFTERTGRSTEN` reader - REG_I3C_SPIKE_FILTER_ON_AFTER_TG_RST_EN"]
pub type Regi3cspikefilteronaftertgrstenR = crate::BitReader;
#[doc = "Field `REGI3CSPIKEFILTERONAFTERTGRSTEN` writer - REG_I3C_SPIKE_FILTER_ON_AFTER_TG_RST_EN"]
pub type Regi3cspikefilteronaftertgrstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CSPIKEFILTERCNT` reader - REG_I3C_SPIKE_FILTER_CNT"]
pub type Regi3cspikefiltercntR = crate::FieldReader;
#[doc = "Field `REGI3CSPIKEFILTERCNT` writer - REG_I3C_SPIKE_FILTER_CNT"]
pub type Regi3cspikefiltercntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGI2CSPIKEFILTERCNT` reader - REG_I2C_SPIKE_FILTER_CNT"]
pub type Regi2cspikefiltercntR = crate::FieldReader;
#[doc = "Field `REGI2CSPIKEFILTERCNT` writer - REG_I2C_SPIKE_FILTER_CNT"]
pub type Regi2cspikefiltercntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - REG_I2C_SPIKE_FILTER_EN"]
    #[inline(always)]
    pub fn regi2cspikefilteren(&self) -> Regi2cspikefilterenR {
        Regi2cspikefilterenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - REG_I3C_SPIKE_FILTER_EN"]
    #[inline(always)]
    pub fn regi3cspikefilteren(&self) -> Regi3cspikefilterenR {
        Regi3cspikefilterenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - REG_I3C_SPIKE_FILTER_OFF_AFTER_7E_RNW0_EN"]
    #[inline(always)]
    pub fn regi3cspikefilteroffafter7ernw0en(&self) -> Regi3cspikefilteroffafter7ernw0enR {
        Regi3cspikefilteroffafter7ernw0enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_I3C_SPIKE_FILTER_ON_AFTER_TG_RST_EN"]
    #[inline(always)]
    pub fn regi3cspikefilteronaftertgrsten(&self) -> Regi3cspikefilteronaftertgrstenR {
        Regi3cspikefilteronaftertgrstenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - REG_I3C_SPIKE_FILTER_CNT"]
    #[inline(always)]
    pub fn regi3cspikefiltercnt(&self) -> Regi3cspikefiltercntR {
        Regi3cspikefiltercntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_I2C_SPIKE_FILTER_CNT"]
    #[inline(always)]
    pub fn regi2cspikefiltercnt(&self) -> Regi2cspikefiltercntR {
        Regi2cspikefiltercntR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - REG_I2C_SPIKE_FILTER_EN"]
    #[inline(always)]
    pub fn regi2cspikefilteren(&mut self) -> Regi2cspikefilterenW<I3cphyctrlreg0d4Spec> {
        Regi2cspikefilterenW::new(self, 0)
    }
    #[doc = "Bits 2:3 - REG_I3C_SPIKE_FILTER_EN"]
    #[inline(always)]
    pub fn regi3cspikefilteren(&mut self) -> Regi3cspikefilterenW<I3cphyctrlreg0d4Spec> {
        Regi3cspikefilterenW::new(self, 2)
    }
    #[doc = "Bit 4 - REG_I3C_SPIKE_FILTER_OFF_AFTER_7E_RNW0_EN"]
    #[inline(always)]
    pub fn regi3cspikefilteroffafter7ernw0en(
        &mut self,
    ) -> Regi3cspikefilteroffafter7ernw0enW<I3cphyctrlreg0d4Spec> {
        Regi3cspikefilteroffafter7ernw0enW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_I3C_SPIKE_FILTER_ON_AFTER_TG_RST_EN"]
    #[inline(always)]
    pub fn regi3cspikefilteronaftertgrsten(
        &mut self,
    ) -> Regi3cspikefilteronaftertgrstenW<I3cphyctrlreg0d4Spec> {
        Regi3cspikefilteronaftertgrstenW::new(self, 5)
    }
    #[doc = "Bits 8:15 - REG_I3C_SPIKE_FILTER_CNT"]
    #[inline(always)]
    pub fn regi3cspikefiltercnt(&mut self) -> Regi3cspikefiltercntW<I3cphyctrlreg0d4Spec> {
        Regi3cspikefiltercntW::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_I2C_SPIKE_FILTER_CNT"]
    #[inline(always)]
    pub fn regi2cspikefiltercnt(&mut self) -> Regi2cspikefiltercntW<I3cphyctrlreg0d4Spec> {
        Regi2cspikefiltercntW::new(self, 16)
    }
}
#[doc = "SPIKE\\_FILTER\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0d4Spec;
impl crate::RegisterSpec for I3cphyctrlreg0d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0d4::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0d4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0d4::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0D4 to value 0"]
impl crate::Resettable for I3cphyctrlreg0d4Spec {}
