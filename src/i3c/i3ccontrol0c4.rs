#[doc = "Register `I3CCONTROL0C4` reader"]
pub type R = crate::R<I3ccontrol0c4Spec>;
#[doc = "Register `I3CCONTROL0C4` writer"]
pub type W = crate::W<I3ccontrol0c4Spec>;
#[doc = "Field `REGGETMXDSMAXRD` reader - REG_GETMXDS_MAXRD"]
pub type ReggetmxdsmaxrdR = crate::FieldReader;
#[doc = "Field `REGGETMXDSMAXRD` writer - REG_GETMXDS_MAXRD"]
pub type ReggetmxdsmaxrdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGGETMXDSMAXWR` reader - REG_GETMXDS_MAXWR"]
pub type ReggetmxdsmaxwrR = crate::FieldReader;
#[doc = "Field `REGGETMXDSMAXWR` writer - REG_GETMXDS_MAXWR"]
pub type ReggetmxdsmaxwrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - REG_GETMXDS_MAXRD"]
    #[inline(always)]
    pub fn reggetmxdsmaxrd(&self) -> ReggetmxdsmaxrdR {
        ReggetmxdsmaxrdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_GETMXDS_MAXWR"]
    #[inline(always)]
    pub fn reggetmxdsmaxwr(&self) -> ReggetmxdsmaxwrR {
        ReggetmxdsmaxwrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_GETMXDS_MAXRD"]
    #[inline(always)]
    pub fn reggetmxdsmaxrd(&mut self) -> ReggetmxdsmaxrdW<I3ccontrol0c4Spec> {
        ReggetmxdsmaxrdW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_GETMXDS_MAXWR"]
    #[inline(always)]
    pub fn reggetmxdsmaxwr(&mut self) -> ReggetmxdsmaxwrW<I3ccontrol0c4Spec> {
        ReggetmxdsmaxwrW::new(self, 8)
    }
}
#[doc = "I3C\\_SLV\\_CTL\\_0C4\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0c4Spec;
impl crate::RegisterSpec for I3ccontrol0c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0c4::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0c4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0c4::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0C4 to value 0"]
impl crate::Resettable for I3ccontrol0c4Spec {}
