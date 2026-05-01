#[doc = "Register `I3CCONTROL088` reader"]
pub type R = crate::R<I3ccontrol088Spec>;
#[doc = "Register `I3CCONTROL088` writer"]
pub type W = crate::W<I3ccontrol088Spec>;
#[doc = "Field `REGRINGWDMADBGHI` reader - REG_RING_WDMA_DBG_HI"]
pub type RegringwdmadbghiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_RING_WDMA_DBG_HI"]
    #[inline(always)]
    pub fn regringwdmadbghi(&self) -> RegringwdmadbghiR {
        RegringwdmadbghiR::new(self.bits)
    }
}
impl W {}
#[doc = "I3C\\_WDMA\\_CTL\\_088\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol088Spec;
impl crate::RegisterSpec for I3ccontrol088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol088::R`](R) reader structure"]
impl crate::Readable for I3ccontrol088Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol088::W`](W) writer structure"]
impl crate::Writable for I3ccontrol088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL088 to value 0"]
impl crate::Resettable for I3ccontrol088Spec {}
