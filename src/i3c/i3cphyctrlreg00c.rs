#[doc = "Register `I3CPHYCTRLREG00C` reader"]
pub type R = crate::R<I3cphyctrlreg00cSpec>;
#[doc = "Register `I3CPHYCTRLREG00C` writer"]
pub type W = crate::W<I3cphyctrlreg00cSpec>;
#[doc = "Field `REGI2CODFMLCNT` reader - REG_I2C_OD_FM_LCNT"]
pub type Regi2codfmlcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMLCNT` writer - REG_I2C_OD_FM_LCNT"]
pub type Regi2codfmlcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI2CODFMHCNT` reader - REG_I2C_OD_FM_HCNT"]
pub type Regi2codfmhcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI2CODFMHCNT` writer - REG_I2C_OD_FM_HCNT"]
pub type Regi2codfmhcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I2C_OD_FM_LCNT"]
    #[inline(always)]
    pub fn regi2codfmlcnt(&self) -> Regi2codfmlcntR {
        Regi2codfmlcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FM_HCNT"]
    #[inline(always)]
    pub fn regi2codfmhcnt(&self) -> Regi2codfmhcntR {
        Regi2codfmhcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I2C_OD_FM_LCNT"]
    #[inline(always)]
    pub fn regi2codfmlcnt(&mut self) -> Regi2codfmlcntW<I3cphyctrlreg00cSpec> {
        Regi2codfmlcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I2C_OD_FM_HCNT"]
    #[inline(always)]
    pub fn regi2codfmhcnt(&mut self) -> Regi2codfmhcntW<I3cphyctrlreg00cSpec> {
        Regi2codfmhcntW::new(self, 16)
    }
}
#[doc = "CR\\_I2C\\_OD\\_FM\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg00cSpec;
impl crate::RegisterSpec for I3cphyctrlreg00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg00c::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg00cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg00c::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG00C to value 0x0077_0103"]
impl crate::Resettable for I3cphyctrlreg00cSpec {
    const RESET_VALUE: u32 = 0x0077_0103;
}
