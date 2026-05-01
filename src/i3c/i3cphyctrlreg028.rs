#[doc = "Register `I3CPHYCTRLREG028` reader"]
pub type R = crate::R<I3cphyctrlreg028Spec>;
#[doc = "Register `I3CPHYCTRLREG028` writer"]
pub type W = crate::W<I3cphyctrlreg028Spec>;
#[doc = "Field `REGI3CODSTOPHCNT` reader - REG_I3C_OD_STOP_HCNT"]
pub type Regi3codstophcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CODSTOPHCNT` writer - REG_I3C_OD_STOP_HCNT"]
pub type Regi3codstophcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGI3CODSTARTHCNT` reader - REG_I3C_OD_START_HCNT"]
pub type Regi3codstarthcntR = crate::FieldReader<u16>;
#[doc = "Field `REGI3CODSTARTHCNT` writer - REG_I3C_OD_START_HCNT"]
pub type Regi3codstarthcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_I3C_OD_STOP_HCNT"]
    #[inline(always)]
    pub fn regi3codstophcnt(&self) -> Regi3codstophcntR {
        Regi3codstophcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_I3C_OD_START_HCNT"]
    #[inline(always)]
    pub fn regi3codstarthcnt(&self) -> Regi3codstarthcntR {
        Regi3codstarthcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_I3C_OD_STOP_HCNT"]
    #[inline(always)]
    pub fn regi3codstophcnt(&mut self) -> Regi3codstophcntW<I3cphyctrlreg028Spec> {
        Regi3codstophcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_I3C_OD_START_HCNT"]
    #[inline(always)]
    pub fn regi3codstarthcnt(&mut self) -> Regi3codstarthcntW<I3cphyctrlreg028Spec> {
        Regi3codstarthcntW::new(self, 16)
    }
}
#[doc = "CR\\_I3C\\_OD\\_STA\\_STO\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg028Spec;
impl crate::RegisterSpec for I3cphyctrlreg028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg028::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg028Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg028::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG028 to value 0x0007_0007"]
impl crate::Resettable for I3cphyctrlreg028Spec {
    const RESET_VALUE: u32 = 0x0007_0007;
}
