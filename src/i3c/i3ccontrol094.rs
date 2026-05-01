#[doc = "Register `I3CCONTROL094` reader"]
pub type R = crate::R<I3ccontrol094Spec>;
#[doc = "Register `I3CCONTROL094` writer"]
pub type W = crate::W<I3ccontrol094Spec>;
#[doc = "Field `REGRINGRDMADBGLO` reader - REG_RING_RDMA_DBG_LO"]
pub type RegringrdmadbgloR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_RING_RDMA_DBG_LO"]
    #[inline(always)]
    pub fn regringrdmadbglo(&self) -> RegringrdmadbgloR {
        RegringrdmadbgloR::new(self.bits)
    }
}
impl W {}
#[doc = "I3C\\_RDMA\\_CTL\\_094\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol094Spec;
impl crate::RegisterSpec for I3ccontrol094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol094::R`](R) reader structure"]
impl crate::Readable for I3ccontrol094Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol094::W`](W) writer structure"]
impl crate::Writable for I3ccontrol094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL094 to value 0"]
impl crate::Resettable for I3ccontrol094Spec {}
