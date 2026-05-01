#[doc = "Register `I3CCONTROL0A4` reader"]
pub type R = crate::R<I3ccontrol0a4Spec>;
#[doc = "Register `I3CCONTROL0A4` writer"]
pub type W = crate::W<I3ccontrol0a4Spec>;
#[doc = "Field `REGSLVPID` reader - REG_SLV_PID"]
pub type RegslvpidR = crate::FieldReader<u32>;
#[doc = "Field `REGSLVPID` writer - REG_SLV_PID"]
pub type RegslvpidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SLV_PID"]
    #[inline(always)]
    pub fn regslvpid(&self) -> RegslvpidR {
        RegslvpidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SLV_PID"]
    #[inline(always)]
    pub fn regslvpid(&mut self) -> RegslvpidW<I3ccontrol0a4Spec> {
        RegslvpidW::new(self, 0)
    }
}
#[doc = "I3C\\_SLV\\_CTL\\_0A4\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0a4Spec;
impl crate::RegisterSpec for I3ccontrol0a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0a4::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0a4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0a4::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0A4 to value 0"]
impl crate::Resettable for I3ccontrol0a4Spec {}
