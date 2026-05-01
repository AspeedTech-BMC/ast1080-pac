#[doc = "Register `I3CPHYCTRLREG024` reader"]
pub type R = crate::R<I3cphyctrlreg024Spec>;
#[doc = "Register `I3CPHYCTRLREG024` writer"]
pub type W = crate::W<I3cphyctrlreg024Spec>;
#[doc = "Field `REGI2CODFMPSDAACKTRANCNT` reader - REG_I2C_OD_FMP_SDA_ACK_TRAN_CNT"]
pub type Regi2codfmpsdaacktrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMPSDAACKTRANCNT` writer - REG_I2C_OD_FMP_SDA_ACK_TRAN_CNT"]
pub type Regi2codfmpsdaacktrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI2CODFMPSDATRANCNT` reader - REG_I2C_OD_FMP_SDA_TRAN_CNT"]
pub type Regi2codfmpsdatrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMPSDATRANCNT` writer - REG_I2C_OD_FMP_SDA_TRAN_CNT"]
pub type Regi2codfmpsdatrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I2C_OD_FMP_SDA_ACK_TRAN_CNT"]
    #[inline(always)]
    pub fn regi2codfmpsdaacktrancnt(&self) -> Regi2codfmpsdaacktrancntR {
        Regi2codfmpsdaacktrancntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FMP_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi2codfmpsdatrancnt(&self) -> Regi2codfmpsdatrancntR {
        Regi2codfmpsdatrancntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I2C_OD_FMP_SDA_ACK_TRAN_CNT"]
    #[inline(always)]
    pub fn regi2codfmpsdaacktrancnt(&mut self) -> Regi2codfmpsdaacktrancntW<I3cphyctrlreg024Spec> {
        Regi2codfmpsdaacktrancntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FMP_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi2codfmpsdatrancnt(&mut self) -> Regi2codfmpsdatrancntW<I3cphyctrlreg024Spec> {
        Regi2codfmpsdatrancntW::new(self, 16)
    }
}
#[doc = "CR\\_I2C\\_OD\\_FMP\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg024Spec;
impl crate::RegisterSpec for I3cphyctrlreg024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg024::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg024Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg024::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG024 to value 0x0001_0001"]
impl crate::Resettable for I3cphyctrlreg024Spec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
