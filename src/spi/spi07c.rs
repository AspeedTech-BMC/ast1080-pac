#[doc = "Register `SPI07C` reader"]
pub type R = crate::R<Spi07cSpec>;
#[doc = "Register `SPI07C` writer"]
pub type W = crate::W<Spi07cSpec>;
#[doc = "Field `DMAROHI` reader - DMA_RO_HI"]
pub type DmarohiR = crate::FieldReader;
#[doc = "Field `DMAROHI` writer - DMA_RO_HI"]
pub type DmarohiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA_RO_HI"]
    #[inline(always)]
    pub fn dmarohi(&self) -> DmarohiR {
        DmarohiR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA_RO_HI"]
    #[inline(always)]
    pub fn dmarohi(&mut self) -> DmarohiW<Spi07cSpec> {
        DmarohiW::new(self, 0)
    }
}
#[doc = "DMA DRAM Side Address High Part\n\nYou can [`read`](crate::Reg::read) this register and get [`spi07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi07cSpec;
impl crate::RegisterSpec for Spi07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi07c::R`](R) reader structure"]
impl crate::Readable for Spi07cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi07c::W`](W) writer structure"]
impl crate::Writable for Spi07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI07C to value 0"]
impl crate::Resettable for Spi07cSpec {}
