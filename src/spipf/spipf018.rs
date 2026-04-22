#[doc = "Register `SPIPF018` reader"]
pub type R = crate::R<Spipf018Spec>;
#[doc = "Register `SPIPF018` writer"]
pub type W = crate::W<Spipf018Spec>;
#[doc = "Field `BlockLogDMAWrPointer` reader - Block Log DMA Write Pointer"]
pub type BlockLogDmawrPointerR = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:17 - Block Log DMA Write Pointer"]
    #[inline(always)]
    pub fn block_log_dmawr_pointer(&self) -> BlockLogDmawrPointerR {
        BlockLogDmawrPointerR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {}
#[doc = "Block Log DMA Write Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf018Spec;
impl crate::RegisterSpec for Spipf018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf018::R`](R) reader structure"]
impl crate::Readable for Spipf018Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf018::W`](W) writer structure"]
impl crate::Writable for Spipf018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF018 to value 0"]
impl crate::Resettable for Spipf018Spec {}
