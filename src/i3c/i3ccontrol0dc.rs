#[doc = "Register `I3CCONTROL0DC` reader"]
pub type R = crate::R<I3ccontrol0dcSpec>;
#[doc = "Register `I3CCONTROL0DC` writer"]
pub type W = crate::W<I3ccontrol0dcSpec>;
#[doc = "Field `REGRXQUEWPTR` reader - REG_RX_QUE_WPTR"]
pub type RegrxquewptrR = crate::FieldReader;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `REGRXQUERPTR` reader - REG_RX_QUE_RPTR"]
pub type RegrxquerptrR = crate::FieldReader;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `REGIBIDATQUEWPTR` reader - REG_IBI_DAT_QUE_WPTR"]
pub type RegibidatquewptrR = crate::FieldReader;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGIBIDATQUERPTR` reader - REG_IBI_DAT_QUE_RPTR"]
pub type RegibidatquerptrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - REG_RX_QUE_WPTR"]
    #[inline(always)]
    pub fn regrxquewptr(&self) -> RegrxquewptrR {
        RegrxquewptrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - REG_RX_QUE_RPTR"]
    #[inline(always)]
    pub fn regrxquerptr(&self) -> RegrxquerptrR {
        RegrxquerptrR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - REG_IBI_DAT_QUE_WPTR"]
    #[inline(always)]
    pub fn regibidatquewptr(&self) -> RegibidatquewptrR {
        RegibidatquewptrR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - REG_IBI_DAT_QUE_RPTR"]
    #[inline(always)]
    pub fn regibidatquerptr(&self) -> RegibidatquerptrR {
        RegibidatquerptrR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {}
#[doc = "I3C\\_QUEUE\\_PTR\\_0DC\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0dcSpec;
impl crate::RegisterSpec for I3ccontrol0dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0dc::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0dcSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0dc::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0DC to value 0"]
impl crate::Resettable for I3ccontrol0dcSpec {}
