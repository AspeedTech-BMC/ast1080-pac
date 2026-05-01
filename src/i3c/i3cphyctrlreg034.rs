#[doc = "Register `I3CPHYCTRLREG034` reader"]
pub type R = crate::R<I3cphyctrlreg034Spec>;
#[doc = "Register `I3CPHYCTRLREG034` writer"]
pub type W = crate::W<I3cphyctrlreg034Spec>;
#[doc = "Field `REGI3CODSDAACKTRANCNT` reader - REG_I3C_OD_SDA_ACK_TRAN_CNT"]
pub type Regi3codsdaacktrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CODSDAACKTRANCNT` writer - REG_I3C_OD_SDA_ACK_TRAN_CNT"]
pub type Regi3codsdaacktrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CODSDATRANCNT` reader - REG_I3C_OD_SDA_TRAN_CNT"]
pub type Regi3codsdatrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CODSDATRANCNT` writer - REG_I3C_OD_SDA_TRAN_CNT"]
pub type Regi3codsdatrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_OD_SDA_ACK_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3codsdaacktrancnt(&self) -> Regi3codsdaacktrancntR {
        Regi3codsdaacktrancntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_OD_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3codsdatrancnt(&self) -> Regi3codsdatrancntR {
        Regi3codsdatrancntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_OD_SDA_ACK_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3codsdaacktrancnt(&mut self) -> Regi3codsdaacktrancntW<I3cphyctrlreg034Spec> {
        Regi3codsdaacktrancntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_OD_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3codsdatrancnt(&mut self) -> Regi3codsdatrancntW<I3cphyctrlreg034Spec> {
        Regi3codsdatrancntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_OD\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg034Spec;
impl crate::RegisterSpec for I3cphyctrlreg034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg034::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg034Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg034::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG034 to value 0x0001_0001"]
impl crate::Resettable for I3cphyctrlreg034Spec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
