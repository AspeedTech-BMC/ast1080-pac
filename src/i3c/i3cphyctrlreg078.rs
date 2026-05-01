#[doc = "Register `I3CPHYCTRLREG078` reader"]
pub type R = crate::R<I3cphyctrlreg078Spec>;
#[doc = "Register `I3CPHYCTRLREG078` writer"]
pub type W = crate::W<I3cphyctrlreg078Spec>;
#[doc = "Field `REGI3CDDRPPTBITLCNT` reader - REG_I3C_DDR_PP_TBIT_LCNT"]
pub type Regi3cddrpptbitlcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CDDRPPTBITLCNT` writer - REG_I3C_DDR_PP_TBIT_LCNT"]
pub type Regi3cddrpptbitlcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CDDRPPTBITHCNT` reader - REG_I3C_DDR_PP_TBIT_HCNT"]
pub type Regi3cddrpptbithcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CDDRPPTBITHCNT` writer - REG_I3C_DDR_PP_TBIT_HCNT"]
pub type Regi3cddrpptbithcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_DDR_PP_TBIT_LCNT"]
    #[inline(always)]
    pub fn regi3cddrpptbitlcnt(&self) -> Regi3cddrpptbitlcntR {
        Regi3cddrpptbitlcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_DDR_PP_TBIT_HCNT"]
    #[inline(always)]
    pub fn regi3cddrpptbithcnt(&self) -> Regi3cddrpptbithcntR {
        Regi3cddrpptbithcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_DDR_PP_TBIT_LCNT"]
    #[inline(always)]
    pub fn regi3cddrpptbitlcnt(&mut self) -> Regi3cddrpptbitlcntW<I3cphyctrlreg078Spec> {
        Regi3cddrpptbitlcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_DDR_PP_TBIT_HCNT"]
    #[inline(always)]
    pub fn regi3cddrpptbithcnt(&mut self) -> Regi3cddrpptbithcntW<I3cphyctrlreg078Spec> {
        Regi3cddrpptbithcntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_DDR\\_PP\\_TBIT\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg078::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg078::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg078Spec;
impl crate::RegisterSpec for I3cphyctrlreg078Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg078::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg078Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg078::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg078Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG078 to value 0x0007_0007"]
impl crate::Resettable for I3cphyctrlreg078Spec {
    const RESET_VALUE: u32 = 0x0007_0007;
}
