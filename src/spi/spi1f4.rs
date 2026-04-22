#[doc = "Register `SPI1F4` reader"]
pub type R = crate::R<Spi1f4Spec>;
#[doc = "Register `SPI1F4` writer"]
pub type W = crate::W<Spi1f4Spec>;
#[doc = "Field `WLOCK1` reader - WLOCK1"]
pub type Wlock1R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK1` writer - WLOCK1"]
pub type Wlock1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK1"]
    #[inline(always)]
    pub fn wlock1(&self) -> Wlock1R {
        Wlock1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK1"]
    #[inline(always)]
    pub fn wlock1(&mut self) -> Wlock1W<Spi1f4Spec> {
        Wlock1W::new(self, 0)
    }
}
#[doc = "SPI Write lock Status when SOC RESET is coming\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1f4Spec;
impl crate::RegisterSpec for Spi1f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1f4::R`](R) reader structure"]
impl crate::Readable for Spi1f4Spec {}
#[doc = "`write(|w| ..)` method takes [`spi1f4::W`](W) writer structure"]
impl crate::Writable for Spi1f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1F4 to value 0"]
impl crate::Resettable for Spi1f4Spec {}
