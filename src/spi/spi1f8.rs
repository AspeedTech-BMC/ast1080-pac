#[doc = "Register `SPI1F8` reader"]
pub type R = crate::R<Spi1f8Spec>;
#[doc = "Register `SPI1F8` writer"]
pub type W = crate::W<Spi1f8Spec>;
#[doc = "Field `WLOCK2` reader - WLOCK2"]
pub type Wlock2R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK2` writer - WLOCK2"]
pub type Wlock2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK2"]
    #[inline(always)]
    pub fn wlock2(&self) -> Wlock2R {
        Wlock2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK2"]
    #[inline(always)]
    pub fn wlock2(&mut self) -> Wlock2W<Spi1f8Spec> {
        Wlock2W::new(self, 0)
    }
}
#[doc = "SPI Write lock Status for Configure Write until soc reset\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1f8Spec;
impl crate::RegisterSpec for Spi1f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1f8::R`](R) reader structure"]
impl crate::Readable for Spi1f8Spec {}
#[doc = "`write(|w| ..)` method takes [`spi1f8::W`](W) writer structure"]
impl crate::Writable for Spi1f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1F8 to value 0"]
impl crate::Resettable for Spi1f8Spec {}
