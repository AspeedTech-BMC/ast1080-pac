#[doc = "Register `SPI1E8` reader"]
pub type R = crate::R<Spi1e8Spec>;
#[doc = "Register `SPI1E8` writer"]
pub type W = crate::W<Spi1e8Spec>;
#[doc = "Field `SOCRST` reader - SOC_RST"]
pub type SocrstR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SOC_RST"]
    #[inline(always)]
    pub fn socrst(&self) -> SocrstR {
        SocrstR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "SPI SOC RESET information\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1e8Spec;
impl crate::RegisterSpec for Spi1e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1e8::R`](R) reader structure"]
impl crate::Readable for Spi1e8Spec {}
#[doc = "`write(|w| ..)` method takes [`spi1e8::W`](W) writer structure"]
impl crate::Writable for Spi1e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1E8 to value 0"]
impl crate::Resettable for Spi1e8Spec {}
