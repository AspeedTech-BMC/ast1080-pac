#[doc = "Register `SPI090` reader"]
pub type R = crate::R<Spi090Spec>;
#[doc = "Register `SPI090` writer"]
pub type W = crate::W<Spi090Spec>;
#[doc = "Field `DMACHKSUM` reader - DMA_CHKSUM"]
pub type DmachksumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DMA_CHKSUM"]
    #[inline(always)]
    pub fn dmachksum(&self) -> DmachksumR {
        DmachksumR::new(self.bits)
    }
}
impl W {}
#[doc = "CheckSum Calculation Result\n\nYou can [`read`](crate::Reg::read) this register and get [`spi090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi090Spec;
impl crate::RegisterSpec for Spi090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi090::R`](R) reader structure"]
impl crate::Readable for Spi090Spec {}
#[doc = "`write(|w| ..)` method takes [`spi090::W`](W) writer structure"]
impl crate::Writable for Spi090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI090 to value 0"]
impl crate::Resettable for Spi090Spec {}
