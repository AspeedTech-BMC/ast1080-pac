#[doc = "Register `I3CPHYCTRLREG074` reader"]
pub type R = crate::R<I3cphyctrlreg074Spec>;
#[doc = "Register `I3CPHYCTRLREG074` writer"]
pub type W = crate::W<I3cphyctrlreg074Spec>;
#[doc = "Field `REGI3CDDRPPLCNT` reader - REG_I3C_DDR_PP_LCNT"]
pub type Regi3cddrpplcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CDDRPPLCNT` writer - REG_I3C_DDR_PP_LCNT"]
pub type Regi3cddrpplcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CDDRPPHCNT` reader - REG_I3C_DDR_PP_HCNT"]
pub type Regi3cddrpphcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CDDRPPHCNT` writer - REG_I3C_DDR_PP_HCNT"]
pub type Regi3cddrpphcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_DDR_PP_LCNT"]
    #[inline(always)]
    pub fn regi3cddrpplcnt(&self) -> Regi3cddrpplcntR {
        Regi3cddrpplcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_DDR_PP_HCNT"]
    #[inline(always)]
    pub fn regi3cddrpphcnt(&self) -> Regi3cddrpphcntR {
        Regi3cddrpphcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_DDR_PP_LCNT"]
    #[inline(always)]
    pub fn regi3cddrpplcnt(&mut self) -> Regi3cddrpplcntW<I3cphyctrlreg074Spec> {
        Regi3cddrpplcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_DDR_PP_HCNT"]
    #[inline(always)]
    pub fn regi3cddrpphcnt(&mut self) -> Regi3cddrpphcntW<I3cphyctrlreg074Spec> {
        Regi3cddrpphcntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_DDR\\_PP\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg074Spec;
impl crate::RegisterSpec for I3cphyctrlreg074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg074::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg074Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg074::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG074 to value 0x0007_0007"]
impl crate::Resettable for I3cphyctrlreg074Spec {
    const RESET_VALUE: u32 = 0x0007_0007;
}
