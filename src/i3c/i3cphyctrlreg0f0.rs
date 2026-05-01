#[doc = "Register `I3CPHYCTRLREG0F0` reader"]
pub type R = crate::R<I3cphyctrlreg0f0Spec>;
#[doc = "Register `I3CPHYCTRLREG0F0` writer"]
pub type W = crate::W<I3cphyctrlreg0f0Spec>;
#[doc = "Field `REGCRI2CSDACONTWAIT` reader - REG_CR_I2C_SDA_CONT_WAIT"]
pub type Regcri2csdacontwaitR = crate::FieldReader<u16>;
#[doc = "Field `REGCRI2CSDACONTWAIT` writer - REG_CR_I2C_SDA_CONT_WAIT"]
pub type Regcri2csdacontwaitW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGTGI2CSDACONTWAIT` reader - REG_TG_I2C_SDA_CONT_WAIT"]
pub type Regtgi2csdacontwaitR = crate::FieldReader<u16>;
#[doc = "Field `REGTGI2CSDACONTWAIT` writer - REG_TG_I2C_SDA_CONT_WAIT"]
pub type Regtgi2csdacontwaitW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_CR_I2C_SDA_CONT_WAIT"]
    #[inline(always)]
    pub fn regcri2csdacontwait(&self) -> Regcri2csdacontwaitR {
        Regcri2csdacontwaitR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_TG_I2C_SDA_CONT_WAIT"]
    #[inline(always)]
    pub fn regtgi2csdacontwait(&self) -> Regtgi2csdacontwaitR {
        Regtgi2csdacontwaitR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_CR_I2C_SDA_CONT_WAIT"]
    #[inline(always)]
    pub fn regcri2csdacontwait(&mut self) -> Regcri2csdacontwaitW<I3cphyctrlreg0f0Spec> {
        Regcri2csdacontwaitW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_TG_I2C_SDA_CONT_WAIT"]
    #[inline(always)]
    pub fn regtgi2csdacontwait(&mut self) -> Regtgi2csdacontwaitW<I3cphyctrlreg0f0Spec> {
        Regtgi2csdacontwaitW::new(self, 16)
    }
}
#[doc = "BUS\\_CONTENTION\\_CNT2\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0f0Spec;
impl crate::RegisterSpec for I3cphyctrlreg0f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0f0::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0f0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0f0::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0F0 to value 0"]
impl crate::Resettable for I3cphyctrlreg0f0Spec {}
