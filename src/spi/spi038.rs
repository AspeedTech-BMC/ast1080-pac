#[doc = "Register `SPI038` reader"]
pub type R = crate::R<Spi038Spec>;
#[doc = "Register `SPI038` writer"]
pub type W = crate::W<Spi038Spec>;
#[doc = "Field `CE2SADDR` reader - CE2_SADDR"]
pub type Ce2saddrR = crate::FieldReader<u16>;
#[doc = "Field `CE2SADDR` writer - CE2_SADDR"]
pub type Ce2saddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CE2EADDR` reader - CE2_EADDR"]
pub type Ce2eaddrR = crate::FieldReader<u16>;
#[doc = "Field `CE2EADDR` writer - CE2_EADDR"]
pub type Ce2eaddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CE2_SADDR"]
    #[inline(always)]
    pub fn ce2saddr(&self) -> Ce2saddrR {
        Ce2saddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CE2_EADDR"]
    #[inline(always)]
    pub fn ce2eaddr(&self) -> Ce2eaddrR {
        Ce2eaddrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CE2_SADDR"]
    #[inline(always)]
    pub fn ce2saddr(&mut self) -> Ce2saddrW<Spi038Spec> {
        Ce2saddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - CE2_EADDR"]
    #[inline(always)]
    pub fn ce2eaddr(&mut self) -> Ce2eaddrW<Spi038Spec> {
        Ce2eaddrW::new(self, 16)
    }
}
#[doc = "CE2 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi038Spec;
impl crate::RegisterSpec for Spi038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi038::R`](R) reader structure"]
impl crate::Readable for Spi038Spec {}
#[doc = "`write(|w| ..)` method takes [`spi038::W`](W) writer structure"]
impl crate::Writable for Spi038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI038 to value 0"]
impl crate::Resettable for Spi038Spec {}
