#[doc = "Register `SPI1F0` reader"]
pub type R = crate::R<Spi1f0Spec>;
#[doc = "Register `SPI1F0` writer"]
pub type W = crate::W<Spi1f0Spec>;
#[doc = "Field `WLOCK0` reader - WLOCK0"]
pub type Wlock0R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK0` writer - WLOCK0"]
pub type Wlock0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK0"]
    #[inline(always)]
    pub fn wlock0(&self) -> Wlock0R {
        Wlock0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK0"]
    #[inline(always)]
    pub fn wlock0(&mut self) -> Wlock0W<Spi1f0Spec> {
        Wlock0W::new(self, 0)
    }
}
#[doc = "SPI Write lock Status for Configure Write until SRST\\#\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1f0Spec;
impl crate::RegisterSpec for Spi1f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1f0::R`](R) reader structure"]
impl crate::Readable for Spi1f0Spec {}
#[doc = "`write(|w| ..)` method takes [`spi1f0::W`](W) writer structure"]
impl crate::Writable for Spi1f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1F0 to value 0"]
impl crate::Resettable for Spi1f0Spec {}
