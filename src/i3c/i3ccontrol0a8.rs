#[doc = "Register `I3CCONTROL0A8` reader"]
pub type R = crate::R<I3ccontrol0a8Spec>;
#[doc = "Register `I3CCONTROL0A8` writer"]
pub type W = crate::W<I3ccontrol0a8Spec>;
#[doc = "Field `REGSLVPIDHI` reader - REG_SLV_PID_HI"]
pub type RegslvpidhiR = crate::FieldReader<u16>;
#[doc = "Field `REGSLVPIDHI` writer - REG_SLV_PID_HI"]
pub type RegslvpidhiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_SLV_PID_HI"]
    #[inline(always)]
    pub fn regslvpidhi(&self) -> RegslvpidhiR {
        RegslvpidhiR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_SLV_PID_HI"]
    #[inline(always)]
    pub fn regslvpidhi(&mut self) -> RegslvpidhiW<I3ccontrol0a8Spec> {
        RegslvpidhiW::new(self, 0)
    }
}
#[doc = "I3C\\_SLV\\_CTL\\_0A8\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0a8Spec;
impl crate::RegisterSpec for I3ccontrol0a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0a8::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0a8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0a8::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0A8 to value 0"]
impl crate::Resettable for I3ccontrol0a8Spec {}
