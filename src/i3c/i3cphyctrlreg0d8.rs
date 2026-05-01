#[doc = "Register `I3CPHYCTRLREG0D8` reader"]
pub type R = crate::R<I3cphyctrlreg0d8Spec>;
#[doc = "Register `I3CPHYCTRLREG0D8` writer"]
pub type W = crate::W<I3cphyctrlreg0d8Spec>;
#[doc = "Field `REGTGI2CSDATRANCNT` reader - REG_TG_I2C_SDA_TRAN_CNT"]
pub type Regtgi2csdatrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGTGI2CSDATRANCNT` writer - REG_TG_I2C_SDA_TRAN_CNT"]
pub type Regtgi2csdatrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGSTOPSDAHCNT` reader - REG_STOP_SDAH_CNT"]
pub type RegstopsdahcntR = crate::FieldReader<u16>;
#[doc = "Field `REGSTOPSDAHCNT` writer - REG_STOP_SDAH_CNT"]
pub type RegstopsdahcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_TG_I2C_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regtgi2csdatrancnt(&self) -> Regtgi2csdatrancntR {
        Regtgi2csdatrancntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_STOP_SDAH_CNT"]
    #[inline(always)]
    pub fn regstopsdahcnt(&self) -> RegstopsdahcntR {
        RegstopsdahcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_TG_I2C_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regtgi2csdatrancnt(&mut self) -> Regtgi2csdatrancntW<I3cphyctrlreg0d8Spec> {
        Regtgi2csdatrancntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_STOP_SDAH_CNT"]
    #[inline(always)]
    pub fn regstopsdahcnt(&mut self) -> RegstopsdahcntW<I3cphyctrlreg0d8Spec> {
        RegstopsdahcntW::new(self, 16)
    }
}
#[doc = "SCL\\_SDA\\_TIMMIING\\_CNT\\_ADDITIONAL\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0d8Spec;
impl crate::RegisterSpec for I3cphyctrlreg0d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0d8::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0d8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0d8::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0D8 to value 0"]
impl crate::Resettable for I3cphyctrlreg0d8Spec {}
