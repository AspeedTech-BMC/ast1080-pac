#[doc = "Register `SPI1E4` reader"]
pub type R = crate::R<Spi1e4Spec>;
#[doc = "Register `SPI1E4` writer"]
pub type W = crate::W<Spi1e4Spec>;
#[doc = "Field `SPIDIMNT` reader - SPI_DI_MNT"]
pub type SpidimntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SPI_DI_MNT"]
    #[inline(always)]
    pub fn spidimnt(&self) -> SpidimntR {
        SpidimntR::new(self.bits)
    }
}
impl W {}
#[doc = "SPI Data in monitor\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1e4Spec;
impl crate::RegisterSpec for Spi1e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1e4::R`](R) reader structure"]
impl crate::Readable for Spi1e4Spec {}
#[doc = "`write(|w| ..)` method takes [`spi1e4::W`](W) writer structure"]
impl crate::Writable for Spi1e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1E4 to value 0"]
impl crate::Resettable for Spi1e4Spec {}
