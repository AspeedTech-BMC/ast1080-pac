#[doc = "Register `SPI1E0` reader"]
pub type R = crate::R<Spi1e0Spec>;
#[doc = "Register `SPI1E0` writer"]
pub type W = crate::W<Spi1e0Spec>;
#[doc = "Field `SPISTATUS` reader - SPI_STATUS"]
pub type SpistatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SPI_STATUS"]
    #[inline(always)]
    pub fn spistatus(&self) -> SpistatusR {
        SpistatusR::new(self.bits)
    }
}
impl W {}
#[doc = "SPI Engine Status report\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1e0Spec;
impl crate::RegisterSpec for Spi1e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1e0::R`](R) reader structure"]
impl crate::Readable for Spi1e0Spec {}
#[doc = "`write(|w| ..)` method takes [`spi1e0::W`](W) writer structure"]
impl crate::Writable for Spi1e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1E0 to value 0"]
impl crate::Resettable for Spi1e0Spec {}
