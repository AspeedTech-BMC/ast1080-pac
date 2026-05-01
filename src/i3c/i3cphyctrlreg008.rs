#[doc = "Register `I3CPHYCTRLREG008` reader"]
pub type R = crate::R<I3cphyctrlreg008Spec>;
#[doc = "Register `I3CPHYCTRLREG008` writer"]
pub type W = crate::W<I3cphyctrlreg008Spec>;
#[doc = "Field `REGI2CODFMSTOPHCNT` reader - REG_I2C_OD_FM_STOP_HCNT"]
pub type Regi2codfmstophcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMSTOPHCNT` writer - REG_I2C_OD_FM_STOP_HCNT"]
pub type Regi2codfmstophcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI2CODFMSTARTHCNT` reader - REG_I2C_OD_FM_START_HCNT"]
pub type Regi2codfmstarthcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMSTARTHCNT` writer - REG_I2C_OD_FM_START_HCNT"]
pub type Regi2codfmstarthcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I2C_OD_FM_STOP_HCNT"]
    #[inline(always)]
    pub fn regi2codfmstophcnt(&self) -> Regi2codfmstophcntR {
        Regi2codfmstophcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FM_START_HCNT"]
    #[inline(always)]
    pub fn regi2codfmstarthcnt(&self) -> Regi2codfmstarthcntR {
        Regi2codfmstarthcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I2C_OD_FM_STOP_HCNT"]
    #[inline(always)]
    pub fn regi2codfmstophcnt(&mut self) -> Regi2codfmstophcntW<I3cphyctrlreg008Spec> {
        Regi2codfmstophcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FM_START_HCNT"]
    #[inline(always)]
    pub fn regi2codfmstarthcnt(&mut self) -> Regi2codfmstarthcntW<I3cphyctrlreg008Spec> {
        Regi2codfmstarthcntW::new(self, 16)
    }
}
#[doc = "CR\\_I2C\\_OD\\_FM\\_STA\\_STO\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg008Spec;
impl crate::RegisterSpec for I3cphyctrlreg008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg008::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg008Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg008::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG008 to value 0x0077_0103"]
impl crate::Resettable for I3cphyctrlreg008Spec {
    const RESET_VALUE: u32 = 0x0077_0103;
}
