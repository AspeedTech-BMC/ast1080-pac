#[doc = "Register `I3CPHYCTRLREG064` reader"]
pub type R = crate::R<I3cphyctrlreg064Spec>;
#[doc = "Register `I3CPHYCTRLREG064` writer"]
pub type W = crate::W<I3cphyctrlreg064Spec>;
#[doc = "Field `REGI3CSDR3PPSDATBITTRANCNT` reader - REG_I3C_SDR3_PP_SDA_TBIT_TRAN_CNT"]
pub type Regi3csdr3ppsdatbittrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR3PPSDATBITTRANCNT` writer - REG_I3C_SDR3_PP_SDA_TBIT_TRAN_CNT"]
pub type Regi3csdr3ppsdatbittrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CSDR3PPSDATRANCNT` reader - REG_I3C_SDR3_PP_SDA_TRAN_CNT"]
pub type Regi3csdr3ppsdatrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR3PPSDATRANCNT` writer - REG_I3C_SDR3_PP_SDA_TRAN_CNT"]
pub type Regi3csdr3ppsdatrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_SDR3_PP_SDA_TBIT_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3csdr3ppsdatbittrancnt(&self) -> Regi3csdr3ppsdatbittrancntR {
        Regi3csdr3ppsdatbittrancntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR3_PP_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3csdr3ppsdatrancnt(&self) -> Regi3csdr3ppsdatrancntR {
        Regi3csdr3ppsdatrancntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_SDR3_PP_SDA_TBIT_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3csdr3ppsdatbittrancnt(
        &mut self,
    ) -> Regi3csdr3ppsdatbittrancntW<I3cphyctrlreg064Spec> {
        Regi3csdr3ppsdatbittrancntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR3_PP_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3csdr3ppsdatrancnt(&mut self) -> Regi3csdr3ppsdatrancntW<I3cphyctrlreg064Spec> {
        Regi3csdr3ppsdatrancntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_SDR3\\_PP\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg064Spec;
impl crate::RegisterSpec for I3cphyctrlreg064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg064::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg064Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg064::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG064 to value 0x0001_0001"]
impl crate::Resettable for I3cphyctrlreg064Spec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
