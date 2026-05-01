#[doc = "Register `I3CPHYCTRLREG044` reader"]
pub type R = crate::R<I3cphyctrlreg044Spec>;
#[doc = "Register `I3CPHYCTRLREG044` writer"]
pub type W = crate::W<I3cphyctrlreg044Spec>;
#[doc = "Field `REGI3CSDR1PPLCNT` reader - REG_I3C_SDR1_PP_LCNT"]
pub type Regi3csdr1pplcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR1PPLCNT` writer - REG_I3C_SDR1_PP_LCNT"]
pub type Regi3csdr1pplcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CSDR1PPHCNT` reader - REG_I3C_SDR1_PP_HCNT"]
pub type Regi3csdr1pphcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR1PPHCNT` writer - REG_I3C_SDR1_PP_HCNT"]
pub type Regi3csdr1pphcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_SDR1_PP_LCNT"]
    #[inline(always)]
    pub fn regi3csdr1pplcnt(&self) -> Regi3csdr1pplcntR {
        Regi3csdr1pplcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR1_PP_HCNT"]
    #[inline(always)]
    pub fn regi3csdr1pphcnt(&self) -> Regi3csdr1pphcntR {
        Regi3csdr1pphcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_SDR1_PP_LCNT"]
    #[inline(always)]
    pub fn regi3csdr1pplcnt(&mut self) -> Regi3csdr1pplcntW<I3cphyctrlreg044Spec> {
        Regi3csdr1pplcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR1_PP_HCNT"]
    #[inline(always)]
    pub fn regi3csdr1pphcnt(&mut self) -> Regi3csdr1pphcntW<I3cphyctrlreg044Spec> {
        Regi3csdr1pphcntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_SDR1\\_PP\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg044Spec;
impl crate::RegisterSpec for I3cphyctrlreg044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg044::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg044Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg044::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG044 to value 0x003b_0040"]
impl crate::Resettable for I3cphyctrlreg044Spec {
    const RESET_VALUE: u32 = 0x003b_0040;
}
