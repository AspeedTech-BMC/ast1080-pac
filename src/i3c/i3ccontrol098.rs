#[doc = "Register `I3CCONTROL098` reader"]
pub type R = crate::R<I3ccontrol098Spec>;
#[doc = "Register `I3CCONTROL098` writer"]
pub type W = crate::W<I3ccontrol098Spec>;
#[doc = "Field `REGRINGRDMADBGHI` reader - REG_RING_RDMA_DBG_HI"]
pub type RegringrdmadbghiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_RING_RDMA_DBG_HI"]
    #[inline(always)]
    pub fn regringrdmadbghi(&self) -> RegringrdmadbghiR {
        RegringrdmadbghiR::new(self.bits)
    }
}
impl W {}
#[doc = "I3C\\_RDMA\\_CTL\\_098\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol098Spec;
impl crate::RegisterSpec for I3ccontrol098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol098::R`](R) reader structure"]
impl crate::Readable for I3ccontrol098Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol098::W`](W) writer structure"]
impl crate::Writable for I3ccontrol098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL098 to value 0"]
impl crate::Resettable for I3ccontrol098Spec {}
