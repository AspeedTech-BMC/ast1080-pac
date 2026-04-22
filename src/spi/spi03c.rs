#[doc = "Register `SPI03C` reader"]
pub type R = crate::R<Spi03cSpec>;
#[doc = "Register `SPI03C` writer"]
pub type W = crate::W<Spi03cSpec>;
#[doc = "Field `CE3SADDR` reader - CE3_SADDR"]
pub type Ce3saddrR = crate::FieldReader<u16>;
#[doc = "Field `CE3SADDR` writer - CE3_SADDR"]
pub type Ce3saddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CE3EADDR` reader - CE3_EADDR"]
pub type Ce3eaddrR = crate::FieldReader<u16>;
#[doc = "Field `CE3EADDR` writer - CE3_EADDR"]
pub type Ce3eaddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CE3_SADDR"]
    #[inline(always)]
    pub fn ce3saddr(&self) -> Ce3saddrR {
        Ce3saddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CE3_EADDR"]
    #[inline(always)]
    pub fn ce3eaddr(&self) -> Ce3eaddrR {
        Ce3eaddrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CE3_SADDR"]
    #[inline(always)]
    pub fn ce3saddr(&mut self) -> Ce3saddrW<Spi03cSpec> {
        Ce3saddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - CE3_EADDR"]
    #[inline(always)]
    pub fn ce3eaddr(&mut self) -> Ce3eaddrW<Spi03cSpec> {
        Ce3eaddrW::new(self, 16)
    }
}
#[doc = "CE3 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi03cSpec;
impl crate::RegisterSpec for Spi03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi03c::R`](R) reader structure"]
impl crate::Readable for Spi03cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi03c::W`](W) writer structure"]
impl crate::Writable for Spi03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI03C to value 0"]
impl crate::Resettable for Spi03cSpec {}
