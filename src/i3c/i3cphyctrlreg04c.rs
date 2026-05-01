#[doc = "Register `I3CPHYCTRLREG04C` reader"]
pub type R = crate::R<I3cphyctrlreg04cSpec>;
#[doc = "Register `I3CPHYCTRLREG04C` writer"]
pub type W = crate::W<I3cphyctrlreg04cSpec>;
#[doc = "Field `REGI3CSDR1PPSDATBITTRANCNT` reader - REG_I3C_SDR1_PP_SDA_TBIT_TRAN_CNT"]
pub type Regi3csdr1ppsdatbittrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR1PPSDATBITTRANCNT` writer - REG_I3C_SDR1_PP_SDA_TBIT_TRAN_CNT"]
pub type Regi3csdr1ppsdatbittrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CSDR1PPSDATRANCNT` reader - REG_I3C_SDR1_PP_SDA_TRAN_CNT"]
pub type Regi3csdr1ppsdatrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CSDR1PPSDATRANCNT` writer - REG_I3C_SDR1_PP_SDA_TRAN_CNT"]
pub type Regi3csdr1ppsdatrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_SDR1_PP_SDA_TBIT_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3csdr1ppsdatbittrancnt(&self) -> Regi3csdr1ppsdatbittrancntR {
        Regi3csdr1ppsdatbittrancntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR1_PP_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3csdr1ppsdatrancnt(&self) -> Regi3csdr1ppsdatrancntR {
        Regi3csdr1ppsdatrancntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_SDR1_PP_SDA_TBIT_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3csdr1ppsdatbittrancnt(
        &mut self,
    ) -> Regi3csdr1ppsdatbittrancntW<I3cphyctrlreg04cSpec> {
        Regi3csdr1ppsdatbittrancntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_SDR1_PP_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3csdr1ppsdatrancnt(&mut self) -> Regi3csdr1ppsdatrancntW<I3cphyctrlreg04cSpec> {
        Regi3csdr1ppsdatrancntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_SDR1\\_PP\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg04cSpec;
impl crate::RegisterSpec for I3cphyctrlreg04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg04c::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg04cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg04c::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG04C to value 0x0001_0001"]
impl crate::Resettable for I3cphyctrlreg04cSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
