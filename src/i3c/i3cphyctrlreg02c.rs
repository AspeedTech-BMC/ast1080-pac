#[doc = "Register `I3CPHYCTRLREG02C` reader"]
pub type R = crate::R<I3cphyctrlreg02cSpec>;
#[doc = "Register `I3CPHYCTRLREG02C` writer"]
pub type W = crate::W<I3cphyctrlreg02cSpec>;
#[doc = "Field `REGI3CODLCNT` reader - REG_I3C_OD_LCNT"]
pub type Regi3codlcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CODLCNT` writer - REG_I3C_OD_LCNT"]
pub type Regi3codlcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CODHCNT` reader - REG_I3C_OD_HCNT"]
pub type Regi3codhcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CODHCNT` writer - REG_I3C_OD_HCNT"]
pub type Regi3codhcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_OD_LCNT"]
    #[inline(always)]
    pub fn regi3codlcnt(&self) -> Regi3codlcntR {
        Regi3codlcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_OD_HCNT"]
    #[inline(always)]
    pub fn regi3codhcnt(&self) -> Regi3codhcntR {
        Regi3codhcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_OD_LCNT"]
    #[inline(always)]
    pub fn regi3codlcnt(&mut self) -> Regi3codlcntW<I3cphyctrlreg02cSpec> {
        Regi3codlcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_OD_HCNT"]
    #[inline(always)]
    pub fn regi3codhcnt(&mut self) -> Regi3codhcntW<I3cphyctrlreg02cSpec> {
        Regi3codhcntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_OD\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg02c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg02c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg02cSpec;
impl crate::RegisterSpec for I3cphyctrlreg02cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg02c::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg02cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg02c::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg02cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG02C to value 0x0013_0027"]
impl crate::Resettable for I3cphyctrlreg02cSpec {
    const RESET_VALUE: u32 = 0x0013_0027;
}
