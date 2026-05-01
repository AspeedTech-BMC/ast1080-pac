#[doc = "Register `I3CCONTROL0D0` reader"]
pub type R = crate::R<I3ccontrol0d0Spec>;
#[doc = "Register `I3CCONTROL0D0` writer"]
pub type W = crate::W<I3ccontrol0d0Spec>;
#[doc = "Field `REGGETCAPSTGTCAPS` reader - REG_GETCAPS_TGTCAPS"]
pub type ReggetcapstgtcapsR = crate::FieldReader<u32>;
#[doc = "Field `REGGETCAPSTGTCAPS` writer - REG_GETCAPS_TGTCAPS"]
pub type ReggetcapstgtcapsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_GETCAPS_TGTCAPS"]
    #[inline(always)]
    pub fn reggetcapstgtcaps(&self) -> ReggetcapstgtcapsR {
        ReggetcapstgtcapsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_GETCAPS_TGTCAPS"]
    #[inline(always)]
    pub fn reggetcapstgtcaps(&mut self) -> ReggetcapstgtcapsW<I3ccontrol0d0Spec> {
        ReggetcapstgtcapsW::new(self, 0)
    }
}
#[doc = "I3C\\_SLV\\_CTL\\_0D0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0d0Spec;
impl crate::RegisterSpec for I3ccontrol0d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0d0::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0d0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0d0::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0D0 to value 0"]
impl crate::Resettable for I3ccontrol0d0Spec {}
