#[doc = "Register `I3CPHYCTRLREG07C` reader"]
pub type R = crate::R<I3cphyctrlreg07cSpec>;
#[doc = "Register `I3CPHYCTRLREG07C` writer"]
pub type W = crate::W<I3cphyctrlreg07cSpec>;
#[doc = "Field `REGI3CDDRPPSDATBITTRANCNT` reader - REG_I3C_DDR_PP_SDA_TBIT_TRAN_CNT"]
pub type Regi3cddrppsdatbittrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CDDRPPSDATBITTRANCNT` writer - REG_I3C_DDR_PP_SDA_TBIT_TRAN_CNT"]
pub type Regi3cddrppsdatbittrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CDDRPPSDATRANCNT` reader - REG_I3C_DDR_PP_SDA_TRAN_CNT"]
pub type Regi3cddrppsdatrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CDDRPPSDATRANCNT` writer - REG_I3C_DDR_PP_SDA_TRAN_CNT"]
pub type Regi3cddrppsdatrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_DDR_PP_SDA_TBIT_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3cddrppsdatbittrancnt(&self) -> Regi3cddrppsdatbittrancntR {
        Regi3cddrppsdatbittrancntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_DDR_PP_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3cddrppsdatrancnt(&self) -> Regi3cddrppsdatrancntR {
        Regi3cddrppsdatrancntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_DDR_PP_SDA_TBIT_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3cddrppsdatbittrancnt(
        &mut self,
    ) -> Regi3cddrppsdatbittrancntW<I3cphyctrlreg07cSpec> {
        Regi3cddrppsdatbittrancntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_DDR_PP_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3cddrppsdatrancnt(&mut self) -> Regi3cddrppsdatrancntW<I3cphyctrlreg07cSpec> {
        Regi3cddrppsdatrancntW::new(self, 16)
    }
}
#[doc = "CR\\_DDR\\_PP\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg07cSpec;
impl crate::RegisterSpec for I3cphyctrlreg07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg07c::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg07cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg07c::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG07C to value 0x0001_0001"]
impl crate::Resettable for I3cphyctrlreg07cSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
