#[doc = "Register `I3CPHYCTRLREG014` reader"]
pub type R = crate::R<I3cphyctrlreg014Spec>;
#[doc = "Register `I3CPHYCTRLREG014` writer"]
pub type W = crate::W<I3cphyctrlreg014Spec>;
#[doc = "Field `REGI2CODFMSDAACKTRANCNT` reader - REG_I2C_OD_FM_SDA_ACK_TRAN_CNT"]
pub type Regi2codfmsdaacktrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMSDAACKTRANCNT` writer - REG_I2C_OD_FM_SDA_ACK_TRAN_CNT"]
pub type Regi2codfmsdaacktrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI2CODFMSDATRANCNT` reader - REG_I2C_OD_FM_SDA_TRAN_CNT"]
pub type Regi2codfmsdatrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMSDATRANCNT` writer - REG_I2C_OD_FM_SDA_TRAN_CNT"]
pub type Regi2codfmsdatrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I2C_OD_FM_SDA_ACK_TRAN_CNT"]
    #[inline(always)]
    pub fn regi2codfmsdaacktrancnt(&self) -> Regi2codfmsdaacktrancntR {
        Regi2codfmsdaacktrancntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FM_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi2codfmsdatrancnt(&self) -> Regi2codfmsdatrancntR {
        Regi2codfmsdatrancntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I2C_OD_FM_SDA_ACK_TRAN_CNT"]
    #[inline(always)]
    pub fn regi2codfmsdaacktrancnt(&mut self) -> Regi2codfmsdaacktrancntW<I3cphyctrlreg014Spec> {
        Regi2codfmsdaacktrancntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FM_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi2codfmsdatrancnt(&mut self) -> Regi2codfmsdatrancntW<I3cphyctrlreg014Spec> {
        Regi2codfmsdatrancntW::new(self, 16)
    }
}
#[doc = "CR\\_I2C\\_OD\\_FM\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg014Spec;
impl crate::RegisterSpec for I3cphyctrlreg014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg014::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg014Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg014::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG014 to value 0x0001_0001"]
impl crate::Resettable for I3cphyctrlreg014Spec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
