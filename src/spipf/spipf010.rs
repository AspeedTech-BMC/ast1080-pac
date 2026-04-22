#[doc = "Register `SPIPF010` reader"]
pub type R = crate::R<Spipf010Spec>;
#[doc = "Register `SPIPF010` writer"]
pub type W = crate::W<Spipf010Spec>;
#[doc = "Field `BlockLogDMABaseAddr` reader - Block Log DMA Base Address"]
pub type BlockLogDmabaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `BlockLogDMABaseAddr` writer - Block Log DMA Base Address"]
pub type BlockLogDmabaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Block Log DMA Base Address"]
    #[inline(always)]
    pub fn block_log_dmabase_addr(&self) -> BlockLogDmabaseAddrR {
        BlockLogDmabaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Block Log DMA Base Address"]
    #[inline(always)]
    pub fn block_log_dmabase_addr(&mut self) -> BlockLogDmabaseAddrW<Spipf010Spec> {
        BlockLogDmabaseAddrW::new(self, 2)
    }
}
#[doc = "Block Log DMA Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf010Spec;
impl crate::RegisterSpec for Spipf010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf010::R`](R) reader structure"]
impl crate::Readable for Spipf010Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf010::W`](W) writer structure"]
impl crate::Writable for Spipf010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF010 to value 0"]
impl crate::Resettable for Spipf010Spec {}
