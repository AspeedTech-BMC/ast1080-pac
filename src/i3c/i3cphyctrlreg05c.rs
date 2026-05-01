#[doc = "Register `I3CPHYCTRLREG05C` reader"]
pub type R = crate::R<I3cphyctrlreg05cSpec>;
#[doc = "Register `I3CPHYCTRLREG05C` writer"]
pub type W = crate::W<I3cphyctrlreg05cSpec>;
#[doc = "Field `REGI3CSDR3PPLCNT` reader - REG_I3C_SDR3_PP_LCNT"]
pub type Regi3csdr3pplcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR3PPLCNT` writer - REG_I3C_SDR3_PP_LCNT"]
pub type Regi3csdr3pplcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CSDR3PPHCNT` reader - REG_I3C_SDR3_PP_HCNT"]
pub type Regi3csdr3pphcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR3PPHCNT` writer - REG_I3C_SDR3_PP_HCNT"]
pub type Regi3csdr3pphcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_SDR3_PP_LCNT"]
    #[inline(always)]
    pub fn regi3csdr3pplcnt(&self) -> Regi3csdr3pplcntR {
        Regi3csdr3pplcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR3_PP_HCNT"]
    #[inline(always)]
    pub fn regi3csdr3pphcnt(&self) -> Regi3csdr3pphcntR {
        Regi3csdr3pphcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_SDR3_PP_LCNT"]
    #[inline(always)]
    pub fn regi3csdr3pplcnt(&mut self) -> Regi3csdr3pplcntW<I3cphyctrlreg05cSpec> {
        Regi3csdr3pplcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR3_PP_HCNT"]
    #[inline(always)]
    pub fn regi3csdr3pphcnt(&mut self) -> Regi3csdr3pphcntW<I3cphyctrlreg05cSpec> {
        Regi3csdr3pphcntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_SDR3\\_PP\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg05c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg05c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg05cSpec;
impl crate::RegisterSpec for I3cphyctrlreg05cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg05c::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg05cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg05c::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg05cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG05C to value 0x007c_007c"]
impl crate::Resettable for I3cphyctrlreg05cSpec {
    const RESET_VALUE: u32 = 0x007c_007c;
}
