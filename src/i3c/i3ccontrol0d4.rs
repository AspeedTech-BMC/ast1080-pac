#[doc = "Register `I3CCONTROL0D4` reader"]
pub type R = crate::R<I3ccontrol0d4Spec>;
#[doc = "Register `I3CCONTROL0D4` writer"]
pub type W = crate::W<I3ccontrol0d4Spec>;
#[doc = "Field `REGGETCAPSCRCAPS` reader - REG_GETCAPS_CRCAPS"]
pub type ReggetcapscrcapsR = crate::FieldReader<u16>;
#[doc = "Field `REGGETCAPSCRCAPS` writer - REG_GETCAPS_CRCAPS"]
pub type ReggetcapscrcapsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGGETCAPSVTCAPS` reader - REG_GETCAPS_VTCAPS"]
pub type ReggetcapsvtcapsR = crate::FieldReader<u16>;
#[doc = "Field `REGGETCAPSVTCAPS` writer - REG_GETCAPS_VTCAPS"]
pub type ReggetcapsvtcapsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_GETCAPS_CRCAPS"]
    #[inline(always)]
    pub fn reggetcapscrcaps(&self) -> ReggetcapscrcapsR {
        ReggetcapscrcapsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_GETCAPS_VTCAPS"]
    #[inline(always)]
    pub fn reggetcapsvtcaps(&self) -> ReggetcapsvtcapsR {
        ReggetcapsvtcapsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_GETCAPS_CRCAPS"]
    #[inline(always)]
    pub fn reggetcapscrcaps(&mut self) -> ReggetcapscrcapsW<I3ccontrol0d4Spec> {
        ReggetcapscrcapsW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_GETCAPS_VTCAPS"]
    #[inline(always)]
    pub fn reggetcapsvtcaps(&mut self) -> ReggetcapsvtcapsW<I3ccontrol0d4Spec> {
        ReggetcapsvtcapsW::new(self, 16)
    }
}
#[doc = "I3C\\_SLV\\_CTL\\_0D4\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0d4Spec;
impl crate::RegisterSpec for I3ccontrol0d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0d4::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0d4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0d4::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0D4 to value 0"]
impl crate::Resettable for I3ccontrol0d4Spec {}
