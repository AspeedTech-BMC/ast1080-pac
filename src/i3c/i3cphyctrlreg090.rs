#[doc = "Register `I3CPHYCTRLREG090` reader"]
pub type R = crate::R<I3cphyctrlreg090Spec>;
#[doc = "Register `I3CPHYCTRLREG090` writer"]
pub type W = crate::W<I3cphyctrlreg090Spec>;
#[doc = "Field `REGTGDDRSDATRANCNT` reader - REG_TG_DDR_SDA_TRAN_CNT"]
pub type RegtgddrsdatrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGTGDDRSDATRANCNT` writer - REG_TG_DDR_SDA_TRAN_CNT"]
pub type RegtgddrsdatrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGTGI3CSDATRANCNT` reader - REG_TG_I3C_SDA_TRAN_CNT"]
pub type Regtgi3csdatrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGTGI3CSDATRANCNT` writer - REG_TG_I3C_SDA_TRAN_CNT"]
pub type Regtgi3csdatrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_TG_DDR_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regtgddrsdatrancnt(&self) -> RegtgddrsdatrancntR {
        RegtgddrsdatrancntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_TG_I3C_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regtgi3csdatrancnt(&self) -> Regtgi3csdatrancntR {
        Regtgi3csdatrancntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_TG_DDR_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regtgddrsdatrancnt(&mut self) -> RegtgddrsdatrancntW<I3cphyctrlreg090Spec> {
        RegtgddrsdatrancntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_TG_I3C_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regtgi3csdatrancnt(&mut self) -> Regtgi3csdatrancntW<I3cphyctrlreg090Spec> {
        Regtgi3csdatrancntW::new(self, 16)
    }
}
#[doc = "TG\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg090Spec;
impl crate::RegisterSpec for I3cphyctrlreg090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg090::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg090Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg090::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG090 to value 0"]
impl crate::Resettable for I3cphyctrlreg090Spec {}
