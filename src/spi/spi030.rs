#[doc = "Register `SPI030` reader"]
pub type R = crate::R<Spi030Spec>;
#[doc = "Register `SPI030` writer"]
pub type W = crate::W<Spi030Spec>;
#[doc = "Field `CE0SADDR` reader - CE0_SADDR"]
pub type Ce0saddrR = crate::FieldReader<u16>;
#[doc = "Field `CE0SADDR` writer - CE0_SADDR"]
pub type Ce0saddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CE0EADDR` reader - CE0_EADDR"]
pub type Ce0eaddrR = crate::FieldReader<u16>;
#[doc = "Field `CE0EADDR` writer - CE0_EADDR"]
pub type Ce0eaddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CE0_SADDR"]
    #[inline(always)]
    pub fn ce0saddr(&self) -> Ce0saddrR {
        Ce0saddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CE0_EADDR"]
    #[inline(always)]
    pub fn ce0eaddr(&self) -> Ce0eaddrR {
        Ce0eaddrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CE0_SADDR"]
    #[inline(always)]
    pub fn ce0saddr(&mut self) -> Ce0saddrW<Spi030Spec> {
        Ce0saddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - CE0_EADDR"]
    #[inline(always)]
    pub fn ce0eaddr(&mut self) -> Ce0eaddrW<Spi030Spec> {
        Ce0eaddrW::new(self, 16)
    }
}
#[doc = "CE0 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi030Spec;
impl crate::RegisterSpec for Spi030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi030::R`](R) reader structure"]
impl crate::Readable for Spi030Spec {}
#[doc = "`write(|w| ..)` method takes [`spi030::W`](W) writer structure"]
impl crate::Writable for Spi030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI030 to value 0x0ff0_0000"]
impl crate::Resettable for Spi030Spec {
    const RESET_VALUE: u32 = 0x0ff0_0000;
}
