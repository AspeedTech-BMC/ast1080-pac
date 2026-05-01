#[doc = "Register `I3CPHYCTRLREG030` reader"]
pub type R = crate::R<I3cphyctrlreg030Spec>;
#[doc = "Register `I3CPHYCTRLREG030` writer"]
pub type W = crate::W<I3cphyctrlreg030Spec>;
#[doc = "Field `REGI3CODACKLCNT` reader - REG_I3C_OD_ACK_LCNT"]
pub type Regi3codacklcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CODACKLCNT` writer - REG_I3C_OD_ACK_LCNT"]
pub type Regi3codacklcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CODACKHCNT` reader - REG_I3C_OD_ACK_HCNT"]
pub type Regi3codackhcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CODACKHCNT` writer - REG_I3C_OD_ACK_HCNT"]
pub type Regi3codackhcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_OD_ACK_LCNT"]
    #[inline(always)]
    pub fn regi3codacklcnt(&self) -> Regi3codacklcntR {
        Regi3codacklcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_OD_ACK_HCNT"]
    #[inline(always)]
    pub fn regi3codackhcnt(&self) -> Regi3codackhcntR {
        Regi3codackhcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_OD_ACK_LCNT"]
    #[inline(always)]
    pub fn regi3codacklcnt(&mut self) -> Regi3codacklcntW<I3cphyctrlreg030Spec> {
        Regi3codacklcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_OD_ACK_HCNT"]
    #[inline(always)]
    pub fn regi3codackhcnt(&mut self) -> Regi3codackhcntW<I3cphyctrlreg030Spec> {
        Regi3codackhcntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_OD\\_ACK\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg030Spec;
impl crate::RegisterSpec for I3cphyctrlreg030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg030::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg030Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg030::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG030 to value 0x0013_0027"]
impl crate::Resettable for I3cphyctrlreg030Spec {
    const RESET_VALUE: u32 = 0x0013_0027;
}
