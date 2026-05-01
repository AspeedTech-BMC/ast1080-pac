#[doc = "Register `I3CPHYCTRLREG020` reader"]
pub type R = crate::R<I3cphyctrlreg020Spec>;
#[doc = "Register `I3CPHYCTRLREG020` writer"]
pub type W = crate::W<I3cphyctrlreg020Spec>;
#[doc = "Field `REGI2CODFMPACKLCNT` reader - REG_I2C_OD_FMP_ACK_LCNT"]
pub type Regi2codfmpacklcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMPACKLCNT` writer - REG_I2C_OD_FMP_ACK_LCNT"]
pub type Regi2codfmpacklcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI2CODFMPACKHCNT` reader - REG_I2C_OD_FMP_ACK_HCNT"]
pub type Regi2codfmpackhcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMPACKHCNT` writer - REG_I2C_OD_FMP_ACK_HCNT"]
pub type Regi2codfmpackhcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I2C_OD_FMP_ACK_LCNT"]
    #[inline(always)]
    pub fn regi2codfmpacklcnt(&self) -> Regi2codfmpacklcntR {
        Regi2codfmpacklcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FMP_ACK_HCNT"]
    #[inline(always)]
    pub fn regi2codfmpackhcnt(&self) -> Regi2codfmpackhcntR {
        Regi2codfmpackhcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I2C_OD_FMP_ACK_LCNT"]
    #[inline(always)]
    pub fn regi2codfmpacklcnt(&mut self) -> Regi2codfmpacklcntW<I3cphyctrlreg020Spec> {
        Regi2codfmpacklcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FMP_ACK_HCNT"]
    #[inline(always)]
    pub fn regi2codfmpackhcnt(&mut self) -> Regi2codfmpackhcntW<I3cphyctrlreg020Spec> {
        Regi2codfmpackhcntW::new(self, 16)
    }
}
#[doc = "CR\\_I2C\\_OD\\_FMP\\_ACK\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg020Spec;
impl crate::RegisterSpec for I3cphyctrlreg020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg020::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg020Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg020::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG020 to value 0x0033_0063"]
impl crate::Resettable for I3cphyctrlreg020Spec {
    const RESET_VALUE: u32 = 0x0033_0063;
}
