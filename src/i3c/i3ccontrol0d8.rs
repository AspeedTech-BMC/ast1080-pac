#[doc = "Register `I3CCONTROL0D8` reader"]
pub type R = crate::R<I3ccontrol0d8Spec>;
#[doc = "Register `I3CCONTROL0D8` writer"]
pub type W = crate::W<I3ccontrol0d8Spec>;
#[doc = "Field `REGCMDQUEWPTR` reader - REG_CMD_QUE_WPTR"]
pub type RegcmdquewptrR = crate::FieldReader;
#[doc = "Field `REGCMDQUERPTR` reader - REG_CMD_QUE_RPTR"]
pub type RegcmdquerptrR = crate::FieldReader;
#[doc = "Field `REGRESPQUEWPTR` reader - REG_RESP_QUE_WPTR"]
pub type RegrespquewptrR = crate::FieldReader;
#[doc = "Field `REGRESPQUERPTR` reader - REG_RESP_QUE_RPTR"]
pub type RegrespquerptrR = crate::FieldReader;
#[doc = "Field `REGIBISTSQUEWPTR` reader - REG_IBI_STS_QUE_WPTR"]
pub type RegibistsquewptrR = crate::FieldReader;
#[doc = "Field `REGIBISTSQUERPTR` reader - REG_IBI_STS_QUE_RPTR"]
pub type RegibistsquerptrR = crate::FieldReader;
#[doc = "Field `REGTXQUEWPTR` reader - REG_TX_QUE_WPTR"]
pub type RegtxquewptrR = crate::FieldReader;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGTXQUERPTR` reader - REG_TX_QUE_RPTR"]
pub type RegtxquerptrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - REG_CMD_QUE_WPTR"]
    #[inline(always)]
    pub fn regcmdquewptr(&self) -> RegcmdquewptrR {
        RegcmdquewptrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - REG_CMD_QUE_RPTR"]
    #[inline(always)]
    pub fn regcmdquerptr(&self) -> RegcmdquerptrR {
        RegcmdquerptrR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - REG_RESP_QUE_WPTR"]
    #[inline(always)]
    pub fn regrespquewptr(&self) -> RegrespquewptrR {
        RegrespquewptrR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - REG_RESP_QUE_RPTR"]
    #[inline(always)]
    pub fn regrespquerptr(&self) -> RegrespquerptrR {
        RegrespquerptrR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - REG_IBI_STS_QUE_WPTR"]
    #[inline(always)]
    pub fn regibistsquewptr(&self) -> RegibistsquewptrR {
        RegibistsquewptrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - REG_IBI_STS_QUE_RPTR"]
    #[inline(always)]
    pub fn regibistsquerptr(&self) -> RegibistsquerptrR {
        RegibistsquerptrR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:16 - REG_TX_QUE_WPTR"]
    #[inline(always)]
    pub fn regtxquewptr(&self) -> RegtxquewptrR {
        RegtxquewptrR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:24 - REG_TX_QUE_RPTR"]
    #[inline(always)]
    pub fn regtxquerptr(&self) -> RegtxquerptrR {
        RegtxquerptrR::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
impl W {}
#[doc = "I3C\\_QUEUE\\_PTR\\_0D8\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0d8Spec;
impl crate::RegisterSpec for I3ccontrol0d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0d8::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0d8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0d8::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0D8 to value 0"]
impl crate::Resettable for I3ccontrol0d8Spec {}
