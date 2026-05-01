#[doc = "Register `I3CCONTROL0CC` reader"]
pub type R = crate::R<I3ccontrol0ccSpec>;
#[doc = "Register `I3CCONTROL0CC` writer"]
pub type W = crate::W<I3ccontrol0ccSpec>;
#[doc = "Field `REGGETSTATUSTGTSTAT` reader - REG_GETSTATUS_TGTSTAT"]
pub type ReggetstatustgtstatR = crate::FieldReader<u16>;
#[doc = "Field `REGGETSTATUSTGTSTAT` writer - REG_GETSTATUS_TGTSTAT"]
pub type ReggetstatustgtstatW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGGETSTATUSPRECR` reader - REG_GETSTATUS_PRECR"]
pub type ReggetstatusprecrR = crate::FieldReader<u16>;
#[doc = "Field `REGGETSTATUSPRECR` writer - REG_GETSTATUS_PRECR"]
pub type ReggetstatusprecrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_GETSTATUS_TGTSTAT"]
    #[inline(always)]
    pub fn reggetstatustgtstat(&self) -> ReggetstatustgtstatR {
        ReggetstatustgtstatR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_GETSTATUS_PRECR"]
    #[inline(always)]
    pub fn reggetstatusprecr(&self) -> ReggetstatusprecrR {
        ReggetstatusprecrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_GETSTATUS_TGTSTAT"]
    #[inline(always)]
    pub fn reggetstatustgtstat(&mut self) -> ReggetstatustgtstatW<I3ccontrol0ccSpec> {
        ReggetstatustgtstatW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_GETSTATUS_PRECR"]
    #[inline(always)]
    pub fn reggetstatusprecr(&mut self) -> ReggetstatusprecrW<I3ccontrol0ccSpec> {
        ReggetstatusprecrW::new(self, 16)
    }
}
#[doc = "I3C\\_SLV\\_CTL\\_0CC\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0ccSpec;
impl crate::RegisterSpec for I3ccontrol0ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0cc::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0ccSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0cc::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0CC to value 0"]
impl crate::Resettable for I3ccontrol0ccSpec {}
