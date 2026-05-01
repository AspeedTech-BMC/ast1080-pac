#[doc = "Register `I3CPHYCTRLREG058` reader"]
pub type R = crate::R<I3cphyctrlreg058Spec>;
#[doc = "Register `I3CPHYCTRLREG058` writer"]
pub type W = crate::W<I3cphyctrlreg058Spec>;
#[doc = "Field `REGI3CSDR2PPSDATBITTRANCNT` reader - REG_I3C_SDR2_PP_SDA_TBIT_TRAN_CNT"]
pub type Regi3csdr2ppsdatbittrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR2PPSDATBITTRANCNT` writer - REG_I3C_SDR2_PP_SDA_TBIT_TRAN_CNT"]
pub type Regi3csdr2ppsdatbittrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CSDR2PPSDATRANCNT` reader - REG_I3C_SDR2_PP_SDA_TRAN_CNT"]
pub type Regi3csdr2ppsdatrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR2PPSDATRANCNT` writer - REG_I3C_SDR2_PP_SDA_TRAN_CNT"]
pub type Regi3csdr2ppsdatrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_SDR2_PP_SDA_TBIT_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3csdr2ppsdatbittrancnt(&self) -> Regi3csdr2ppsdatbittrancntR {
        Regi3csdr2ppsdatbittrancntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR2_PP_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3csdr2ppsdatrancnt(&self) -> Regi3csdr2ppsdatrancntR {
        Regi3csdr2ppsdatrancntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_SDR2_PP_SDA_TBIT_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3csdr2ppsdatbittrancnt(
        &mut self,
    ) -> Regi3csdr2ppsdatbittrancntW<I3cphyctrlreg058Spec> {
        Regi3csdr2ppsdatbittrancntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR2_PP_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3csdr2ppsdatrancnt(&mut self) -> Regi3csdr2ppsdatrancntW<I3cphyctrlreg058Spec> {
        Regi3csdr2ppsdatrancntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_SDR2\\_PP\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg058::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg058::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg058Spec;
impl crate::RegisterSpec for I3cphyctrlreg058Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg058::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg058Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg058::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg058Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG058 to value 0x0001_0001"]
impl crate::Resettable for I3cphyctrlreg058Spec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
