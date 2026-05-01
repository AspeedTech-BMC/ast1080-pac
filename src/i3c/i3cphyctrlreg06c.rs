#[doc = "Register `I3CPHYCTRLREG06C` reader"]
pub type R = crate::R<I3cphyctrlreg06cSpec>;
#[doc = "Register `I3CPHYCTRLREG06C` writer"]
pub type W = crate::W<I3cphyctrlreg06cSpec>;
#[doc = "Field `REGI3CSDR4PPTBITLCNT` reader - REG_I3C_SDR4_PP_TBIT_LCNT"]
pub type Regi3csdr4pptbitlcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR4PPTBITLCNT` writer - REG_I3C_SDR4_PP_TBIT_LCNT"]
pub type Regi3csdr4pptbitlcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CSDR4PPTBITHCNT` reader - REG_I3C_SDR4_PP_TBIT_HCNT"]
pub type Regi3csdr4pptbithcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR4PPTBITHCNT` writer - REG_I3C_SDR4_PP_TBIT_HCNT"]
pub type Regi3csdr4pptbithcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_SDR4_PP_TBIT_LCNT"]
    #[inline(always)]
    pub fn regi3csdr4pptbitlcnt(&self) -> Regi3csdr4pptbitlcntR {
        Regi3csdr4pptbitlcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR4_PP_TBIT_HCNT"]
    #[inline(always)]
    pub fn regi3csdr4pptbithcnt(&self) -> Regi3csdr4pptbithcntR {
        Regi3csdr4pptbithcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_SDR4_PP_TBIT_LCNT"]
    #[inline(always)]
    pub fn regi3csdr4pptbitlcnt(&mut self) -> Regi3csdr4pptbitlcntW<I3cphyctrlreg06cSpec> {
        Regi3csdr4pptbitlcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR4_PP_TBIT_HCNT"]
    #[inline(always)]
    pub fn regi3csdr4pptbithcnt(&mut self) -> Regi3csdr4pptbithcntW<I3cphyctrlreg06cSpec> {
        Regi3csdr4pptbithcntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_SDR4\\_PP\\_TBIT\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg06c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg06c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg06cSpec;
impl crate::RegisterSpec for I3cphyctrlreg06cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg06c::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg06cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg06c::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg06cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG06C to value 0x00f9_00f9"]
impl crate::Resettable for I3cphyctrlreg06cSpec {
    const RESET_VALUE: u32 = 0x00f9_00f9;
}
