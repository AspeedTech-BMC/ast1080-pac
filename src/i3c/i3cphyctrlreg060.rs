#[doc = "Register `I3CPHYCTRLREG060` reader"]
pub type R = crate::R<I3cphyctrlreg060Spec>;
#[doc = "Register `I3CPHYCTRLREG060` writer"]
pub type W = crate::W<I3cphyctrlreg060Spec>;
#[doc = "Field `REGI3CSDR3PPTBITLCNT` reader - REG_I3C_SDR3_PP_TBIT_LCNT"]
pub type Regi3csdr3pptbitlcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR3PPTBITLCNT` writer - REG_I3C_SDR3_PP_TBIT_LCNT"]
pub type Regi3csdr3pptbitlcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CSDR3PPTBITHCNT` reader - REG_I3C_SDR3_PP_TBIT_HCNT"]
pub type Regi3csdr3pptbithcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR3PPTBITHCNT` writer - REG_I3C_SDR3_PP_TBIT_HCNT"]
pub type Regi3csdr3pptbithcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_SDR3_PP_TBIT_LCNT"]
    #[inline(always)]
    pub fn regi3csdr3pptbitlcnt(&self) -> Regi3csdr3pptbitlcntR {
        Regi3csdr3pptbitlcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR3_PP_TBIT_HCNT"]
    #[inline(always)]
    pub fn regi3csdr3pptbithcnt(&self) -> Regi3csdr3pptbithcntR {
        Regi3csdr3pptbithcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_SDR3_PP_TBIT_LCNT"]
    #[inline(always)]
    pub fn regi3csdr3pptbitlcnt(&mut self) -> Regi3csdr3pptbitlcntW<I3cphyctrlreg060Spec> {
        Regi3csdr3pptbitlcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR3_PP_TBIT_HCNT"]
    #[inline(always)]
    pub fn regi3csdr3pptbithcnt(&mut self) -> Regi3csdr3pptbithcntW<I3cphyctrlreg060Spec> {
        Regi3csdr3pptbithcntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_SDR3\\_PP\\_TBIT\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg060Spec;
impl crate::RegisterSpec for I3cphyctrlreg060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg060::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg060Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg060::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG060 to value 0x007c_007c"]
impl crate::Resettable for I3cphyctrlreg060Spec {
    const RESET_VALUE: u32 = 0x007c_007c;
}
