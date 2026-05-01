#[doc = "Register `I3CPHYCTRLREG01C` reader"]
pub type R = crate::R<I3cphyctrlreg01cSpec>;
#[doc = "Register `I3CPHYCTRLREG01C` writer"]
pub type W = crate::W<I3cphyctrlreg01cSpec>;
#[doc = "Field `REGI2CODFMPLCNT` reader - REG_I2C_OD_FMP_LCNT"]
pub type Regi2codfmplcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMPLCNT` writer - REG_I2C_OD_FMP_LCNT"]
pub type Regi2codfmplcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI2CODFMPHCNT` reader - REG_I2C_OD_FMP_HCNT"]
pub type Regi2codfmphcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMPHCNT` writer - REG_I2C_OD_FMP_HCNT"]
pub type Regi2codfmphcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I2C_OD_FMP_LCNT"]
    #[inline(always)]
    pub fn regi2codfmplcnt(&self) -> Regi2codfmplcntR {
        Regi2codfmplcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FMP_HCNT"]
    #[inline(always)]
    pub fn regi2codfmphcnt(&self) -> Regi2codfmphcntR {
        Regi2codfmphcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I2C_OD_FMP_LCNT"]
    #[inline(always)]
    pub fn regi2codfmplcnt(&mut self) -> Regi2codfmplcntW<I3cphyctrlreg01cSpec> {
        Regi2codfmplcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FMP_HCNT"]
    #[inline(always)]
    pub fn regi2codfmphcnt(&mut self) -> Regi2codfmphcntW<I3cphyctrlreg01cSpec> {
        Regi2codfmphcntW::new(self, 16)
    }
}
#[doc = "CR\\_I2C\\_OD\\_FMP\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg01cSpec;
impl crate::RegisterSpec for I3cphyctrlreg01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg01c::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg01cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg01c::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG01C to value 0x0033_0063"]
impl crate::Resettable for I3cphyctrlreg01cSpec {
    const RESET_VALUE: u32 = 0x0033_0063;
}
