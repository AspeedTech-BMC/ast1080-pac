#[doc = "Register `I3CPHYCTRLREG048` reader"]
pub type R = crate::R<I3cphyctrlreg048Spec>;
#[doc = "Register `I3CPHYCTRLREG048` writer"]
pub type W = crate::W<I3cphyctrlreg048Spec>;
#[doc = "Field `REGI3CSDR1PPTBITLCNT` reader - REG_I3C_SDR1_PP_TBIT_LCNT"]
pub type Regi3csdr1pptbitlcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR1PPTBITLCNT` writer - REG_I3C_SDR1_PP_TBIT_LCNT"]
pub type Regi3csdr1pptbitlcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CSDR1PPTBITHCNT` reader - REG_I3C_SDR1_PP_TBIT_HCNT"]
pub type Regi3csdr1pptbithcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR1PPTBITHCNT` writer - REG_I3C_SDR1_PP_TBIT_HCNT"]
pub type Regi3csdr1pptbithcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_SDR1_PP_TBIT_LCNT"]
    #[inline(always)]
    pub fn regi3csdr1pptbitlcnt(&self) -> Regi3csdr1pptbitlcntR {
        Regi3csdr1pptbitlcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR1_PP_TBIT_HCNT"]
    #[inline(always)]
    pub fn regi3csdr1pptbithcnt(&self) -> Regi3csdr1pptbithcntR {
        Regi3csdr1pptbithcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_SDR1_PP_TBIT_LCNT"]
    #[inline(always)]
    pub fn regi3csdr1pptbitlcnt(&mut self) -> Regi3csdr1pptbitlcntW<I3cphyctrlreg048Spec> {
        Regi3csdr1pptbitlcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR1_PP_TBIT_HCNT"]
    #[inline(always)]
    pub fn regi3csdr1pptbithcnt(&mut self) -> Regi3csdr1pptbithcntW<I3cphyctrlreg048Spec> {
        Regi3csdr1pptbithcntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_SDR1\\_PP\\_TBIT\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg048Spec;
impl crate::RegisterSpec for I3cphyctrlreg048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg048::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg048Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg048::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG048 to value 0x003b_0040"]
impl crate::Resettable for I3cphyctrlreg048Spec {
    const RESET_VALUE: u32 = 0x003b_0040;
}
