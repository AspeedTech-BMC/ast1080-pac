#[doc = "Register `SPI1FC` reader"]
pub type R = crate::R<Spi1fcSpec>;
#[doc = "Register `SPI1FC` writer"]
pub type W = crate::W<Spi1fcSpec>;
#[doc = "Field `WLOCKX` reader - WLOCKX"]
pub type WlockxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCKX"]
    #[inline(always)]
    pub fn wlockx(&self) -> WlockxR {
        WlockxR::new(self.bits)
    }
}
impl W {}
#[doc = "SPI Write lock status for Configure Write\n\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1fcSpec;
impl crate::RegisterSpec for Spi1fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1fc::R`](R) reader structure"]
impl crate::Readable for Spi1fcSpec {}
#[doc = "`write(|w| ..)` method takes [`spi1fc::W`](W) writer structure"]
impl crate::Writable for Spi1fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1FC to value 0"]
impl crate::Resettable for Spi1fcSpec {}
