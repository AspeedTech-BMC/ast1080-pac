#[doc = "Register `I3CPHYCTRLREG010` reader"]
pub type R = crate::R<I3cphyctrlreg010Spec>;
#[doc = "Register `I3CPHYCTRLREG010` writer"]
pub type W = crate::W<I3cphyctrlreg010Spec>;
#[doc = "Field `REGI2CODFMACKLCNT` reader - REG_I2C_OD_FM_ACK_LCNT"]
pub type Regi2codfmacklcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMACKLCNT` writer - REG_I2C_OD_FM_ACK_LCNT"]
pub type Regi2codfmacklcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI2CODFMACKHCNT` reader - REG_I2C_OD_FM_ACK_HCNT"]
pub type Regi2codfmackhcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMACKHCNT` writer - REG_I2C_OD_FM_ACK_HCNT"]
pub type Regi2codfmackhcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I2C_OD_FM_ACK_LCNT"]
    #[inline(always)]
    pub fn regi2codfmacklcnt(&self) -> Regi2codfmacklcntR {
        Regi2codfmacklcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FM_ACK_HCNT"]
    #[inline(always)]
    pub fn regi2codfmackhcnt(&self) -> Regi2codfmackhcntR {
        Regi2codfmackhcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I2C_OD_FM_ACK_LCNT"]
    #[inline(always)]
    pub fn regi2codfmacklcnt(&mut self) -> Regi2codfmacklcntW<I3cphyctrlreg010Spec> {
        Regi2codfmacklcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FM_ACK_HCNT"]
    #[inline(always)]
    pub fn regi2codfmackhcnt(&mut self) -> Regi2codfmackhcntW<I3cphyctrlreg010Spec> {
        Regi2codfmackhcntW::new(self, 16)
    }
}
#[doc = "CR\\_I2C\\_OD\\_FM\\_ACK\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg010Spec;
impl crate::RegisterSpec for I3cphyctrlreg010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg010::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg010Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg010::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG010 to value 0x0077_0103"]
impl crate::Resettable for I3cphyctrlreg010Spec {
    const RESET_VALUE: u32 = 0x0077_0103;
}
