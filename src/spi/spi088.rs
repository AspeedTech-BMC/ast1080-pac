#[doc = "Register `SPI088` reader"]
pub type R = crate::R<Spi088Spec>;
#[doc = "Register `SPI088` writer"]
pub type W = crate::W<Spi088Spec>;
#[doc = "Field `DMARO` reader - DMA_RO"]
pub type DmaroR = crate::FieldReader<u32>;
#[doc = "Field `DMARO` writer - DMA_RO"]
pub type DmaroW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA_RO"]
    #[inline(always)]
    pub fn dmaro(&self) -> DmaroR {
        DmaroR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA_RO"]
    #[inline(always)]
    pub fn dmaro(&mut self) -> DmaroW<Spi088Spec> {
        DmaroW::new(self, 0)
    }
}
#[doc = "DMA DRAM Side Address\n\nYou can [`read`](crate::Reg::read) this register and get [`spi088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi088Spec;
impl crate::RegisterSpec for Spi088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi088::R`](R) reader structure"]
impl crate::Readable for Spi088Spec {}
#[doc = "`write(|w| ..)` method takes [`spi088::W`](W) writer structure"]
impl crate::Writable for Spi088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI088 to value 0"]
impl crate::Resettable for Spi088Spec {}
