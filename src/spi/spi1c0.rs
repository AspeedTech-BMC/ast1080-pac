#[doc = "Register `SPI1C0` reader"]
pub type R = crate::R<Spi1c0Spec>;
#[doc = "Register `SPI1C0` writer"]
pub type W = crate::W<Spi1c0Spec>;
#[doc = "Field `DMAFFLEN` reader - DMAFFLEN"]
pub type DmafflenR = crate::FieldReader;
#[doc = "Field `DMAFFLEN` writer - DMAFFLEN"]
pub type DmafflenW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - DMAFFLEN"]
    #[inline(always)]
    pub fn dmafflen(&self) -> DmafflenR {
        DmafflenR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMAFFLEN"]
    #[inline(always)]
    pub fn dmafflen(&mut self) -> DmafflenW<Spi1c0Spec> {
        DmafflenW::new(self, 0)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi1c0Spec;
impl crate::RegisterSpec for Spi1c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi1c0::R`](R) reader structure"]
impl crate::Readable for Spi1c0Spec {}
#[doc = "`write(|w| ..)` method takes [`spi1c0::W`](W) writer structure"]
impl crate::Writable for Spi1c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI1C0 to value 0"]
impl crate::Resettable for Spi1c0Spec {}
