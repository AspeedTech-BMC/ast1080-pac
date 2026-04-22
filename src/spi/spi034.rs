#[doc = "Register `SPI034` reader"]
pub type R = crate::R<Spi034Spec>;
#[doc = "Register `SPI034` writer"]
pub type W = crate::W<Spi034Spec>;
#[doc = "Field `CE1SADDR` reader - CE1_SADDR"]
pub type Ce1saddrR = crate::FieldReader<u16>;
#[doc = "Field `CE1SADDR` writer - CE1_SADDR"]
pub type Ce1saddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CE1EADDR` reader - CE1_EADDR"]
pub type Ce1eaddrR = crate::FieldReader<u16>;
#[doc = "Field `CE1EADDR` writer - CE1_EADDR"]
pub type Ce1eaddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CE1_SADDR"]
    #[inline(always)]
    pub fn ce1saddr(&self) -> Ce1saddrR {
        Ce1saddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CE1_EADDR"]
    #[inline(always)]
    pub fn ce1eaddr(&self) -> Ce1eaddrR {
        Ce1eaddrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CE1_SADDR"]
    #[inline(always)]
    pub fn ce1saddr(&mut self) -> Ce1saddrW<Spi034Spec> {
        Ce1saddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - CE1_EADDR"]
    #[inline(always)]
    pub fn ce1eaddr(&mut self) -> Ce1eaddrW<Spi034Spec> {
        Ce1eaddrW::new(self, 16)
    }
}
#[doc = "CE1 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi034Spec;
impl crate::RegisterSpec for Spi034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi034::R`](R) reader structure"]
impl crate::Readable for Spi034Spec {}
#[doc = "`write(|w| ..)` method takes [`spi034::W`](W) writer structure"]
impl crate::Writable for Spi034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI034 to value 0"]
impl crate::Resettable for Spi034Spec {}
