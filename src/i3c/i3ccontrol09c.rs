#[doc = "Register `I3CCONTROL09C` reader"]
pub type R = crate::R<I3ccontrol09cSpec>;
#[doc = "Register `I3CCONTROL09C` writer"]
pub type W = crate::W<I3ccontrol09cSpec>;
#[doc = "Field `REGRINGPROCESSSTATE` reader - REG_RING_PROCESS_STATE"]
pub type RegringprocessstateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - REG_RING_PROCESS_STATE"]
    #[inline(always)]
    pub fn regringprocessstate(&self) -> RegringprocessstateR {
        RegringprocessstateR::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "I3C\\_RING\\_CTL\\_09C\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol09c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol09c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol09cSpec;
impl crate::RegisterSpec for I3ccontrol09cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol09c::R`](R) reader structure"]
impl crate::Readable for I3ccontrol09cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol09c::W`](W) writer structure"]
impl crate::Writable for I3ccontrol09cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL09C to value 0"]
impl crate::Resettable for I3ccontrol09cSpec {}
