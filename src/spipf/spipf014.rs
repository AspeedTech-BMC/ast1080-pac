#[doc = "Register `SPIPF014` reader"]
pub type R = crate::R<Spipf014Spec>;
#[doc = "Register `SPIPF014` writer"]
pub type W = crate::W<Spipf014Spec>;
#[doc = "Field `SizeOfBlockLogDMABuffer` reader - Size of Block Log DMA Buffer"]
pub type SizeOfBlockLogDmabufferR = crate::FieldReader<u32>;
#[doc = "Field `SizeOfBlockLogDMABuffer` writer - Size of Block Log DMA Buffer"]
pub type SizeOfBlockLogDmabufferW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `EnblBlockLogDMA` reader - Enable Block Log DMA"]
pub type EnblBlockLogDmaR = crate::BitReader;
#[doc = "Field `EnblBlockLogDMA` writer - Enable Block Log DMA"]
pub type EnblBlockLogDmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18 - Size of Block Log DMA Buffer"]
    #[inline(always)]
    pub fn size_of_block_log_dmabuffer(&self) -> SizeOfBlockLogDmabufferR {
        SizeOfBlockLogDmabufferR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 31 - Enable Block Log DMA"]
    #[inline(always)]
    pub fn enbl_block_log_dma(&self) -> EnblBlockLogDmaR {
        EnblBlockLogDmaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18 - Size of Block Log DMA Buffer"]
    #[inline(always)]
    pub fn size_of_block_log_dmabuffer(&mut self) -> SizeOfBlockLogDmabufferW<Spipf014Spec> {
        SizeOfBlockLogDmabufferW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable Block Log DMA"]
    #[inline(always)]
    pub fn enbl_block_log_dma(&mut self) -> EnblBlockLogDmaW<Spipf014Spec> {
        EnblBlockLogDmaW::new(self, 31)
    }
}
#[doc = "Block Log DMA Size\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf014Spec;
impl crate::RegisterSpec for Spipf014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf014::R`](R) reader structure"]
impl crate::Readable for Spipf014Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf014::W`](W) writer structure"]
impl crate::Writable for Spipf014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF014 to value 0"]
impl crate::Resettable for Spipf014Spec {}
