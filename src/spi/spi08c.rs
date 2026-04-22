#[doc = "Register `SPI08C` reader"]
pub type R = crate::R<Spi08cSpec>;
#[doc = "Register `SPI08C` writer"]
pub type W = crate::W<Spi08cSpec>;
#[doc = "Field `DMALEN` reader - DMA_LEN"]
pub type DmalenR = crate::FieldReader<u32>;
#[doc = "Field `DMALEN` writer - DMA_LEN"]
pub type DmalenW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - DMA_LEN"]
    #[inline(always)]
    pub fn dmalen(&self) -> DmalenR {
        DmalenR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - DMA_LEN"]
    #[inline(always)]
    pub fn dmalen(&mut self) -> DmalenW<Spi08cSpec> {
        DmalenW::new(self, 0)
    }
}
#[doc = "DMA Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi08cSpec;
impl crate::RegisterSpec for Spi08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi08c::R`](R) reader structure"]
impl crate::Readable for Spi08cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi08c::W`](W) writer structure"]
impl crate::Writable for Spi08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI08C to value 0"]
impl crate::Resettable for Spi08cSpec {}
