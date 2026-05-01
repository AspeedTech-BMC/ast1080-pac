#[doc = "Register `I3CPHYCTRLREG03C` reader"]
pub type R = crate::R<I3cphyctrlreg03cSpec>;
#[doc = "Register `I3CPHYCTRLREG03C` writer"]
pub type W = crate::W<I3cphyctrlreg03cSpec>;
#[doc = "Field `REGI3CSDR0PPTBITLCNT` reader - REG_I3C_SDR0_PP_TBIT_LCNT"]
pub type Regi3csdr0pptbitlcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR0PPTBITLCNT` writer - REG_I3C_SDR0_PP_TBIT_LCNT"]
pub type Regi3csdr0pptbitlcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CSDR0PPTBITHCNT` reader - REG_I3C_SDR0_PP_TBIT_HCNT"]
pub type Regi3csdr0pptbithcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR0PPTBITHCNT` writer - REG_I3C_SDR0_PP_TBIT_HCNT"]
pub type Regi3csdr0pptbithcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_SDR0_PP_TBIT_LCNT"]
    #[inline(always)]
    pub fn regi3csdr0pptbitlcnt(&self) -> Regi3csdr0pptbitlcntR {
        Regi3csdr0pptbitlcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR0_PP_TBIT_HCNT"]
    #[inline(always)]
    pub fn regi3csdr0pptbithcnt(&self) -> Regi3csdr0pptbithcntR {
        Regi3csdr0pptbithcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_SDR0_PP_TBIT_LCNT"]
    #[inline(always)]
    pub fn regi3csdr0pptbitlcnt(&mut self) -> Regi3csdr0pptbitlcntW<I3cphyctrlreg03cSpec> {
        Regi3csdr0pptbitlcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR0_PP_TBIT_HCNT"]
    #[inline(always)]
    pub fn regi3csdr0pptbithcnt(&mut self) -> Regi3csdr0pptbithcntW<I3cphyctrlreg03cSpec> {
        Regi3csdr0pptbithcntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_SDR0\\_PP\\_TBIT\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg03cSpec;
impl crate::RegisterSpec for I3cphyctrlreg03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg03c::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg03cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg03c::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG03C to value 0x0007_0007"]
impl crate::Resettable for I3cphyctrlreg03cSpec {
    const RESET_VALUE: u32 = 0x0007_0007;
}
