#[doc = "Register `SPI084` reader"]
pub type R = crate::R<Spi084Spec>;
#[doc = "Register `SPI084` writer"]
pub type W = crate::W<Spi084Spec>;
#[doc = "Field `DMARI` reader - DMA_RI"]
pub type DmariR = crate::FieldReader<u32>;
#[doc = "Field `DMARI` writer - DMA_RI"]
pub type DmariW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA_RI"]
    #[inline(always)]
    pub fn dmari(&self) -> DmariR {
        DmariR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA_RI"]
    #[inline(always)]
    pub fn dmari(&mut self) -> DmariW<Spi084Spec> {
        DmariW::new(self, 0)
    }
}
#[doc = "DMA Flash Side Address\n\nYou can [`read`](crate::Reg::read) this register and get [`spi084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi084Spec;
impl crate::RegisterSpec for Spi084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi084::R`](R) reader structure"]
impl crate::Readable for Spi084Spec {}
#[doc = "`write(|w| ..)` method takes [`spi084::W`](W) writer structure"]
impl crate::Writable for Spi084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI084 to value 0"]
impl crate::Resettable for Spi084Spec {}
