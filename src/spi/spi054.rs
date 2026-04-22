#[doc = "Register `SPI054` reader"]
pub type R = crate::R<Spi054Spec>;
#[doc = "Register `SPI054` writer"]
pub type W = crate::W<Spi054Spec>;
#[doc = "Field `DUMYCMD` reader - DUMYCMD"]
pub type DumycmdR = crate::FieldReader;
#[doc = "Field `DUMYCMD` writer - DUMYCMD"]
pub type DumycmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `ADR4KBND` reader - ADR4KBND"]
pub type Adr4kbndR = crate::BitReader;
#[doc = "Field `ADR4KBND` writer - ADR4KBND"]
pub type Adr4kbndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `MODESEL` reader - MODE_SEL"]
pub type ModeselR = crate::FieldReader;
#[doc = "Field `MODESEL` writer - MODE_SEL"]
pub type ModeselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - DUMYCMD"]
    #[inline(always)]
    pub fn dumycmd(&self) -> DumycmdR {
        DumycmdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - ADR4KBND"]
    #[inline(always)]
    pub fn adr4kbnd(&self) -> Adr4kbndR {
        Adr4kbndR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - MODE_SEL"]
    #[inline(always)]
    pub fn modesel(&self) -> ModeselR {
        ModeselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DUMYCMD"]
    #[inline(always)]
    pub fn dumycmd(&mut self) -> DumycmdW<Spi054Spec> {
        DumycmdW::new(self, 0)
    }
    #[doc = "Bit 16 - ADR4KBND"]
    #[inline(always)]
    pub fn adr4kbnd(&mut self) -> Adr4kbndW<Spi054Spec> {
        Adr4kbndW::new(self, 16)
    }
    #[doc = "Bits 24:27 - MODE_SEL"]
    #[inline(always)]
    pub fn modesel(&mut self) -> ModeselW<Spi054Spec> {
        ModeselW::new(self, 24)
    }
}
#[doc = "SPI Dummy Cycle Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi054::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi054::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi054Spec;
impl crate::RegisterSpec for Spi054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi054::R`](R) reader structure"]
impl crate::Readable for Spi054Spec {}
#[doc = "`write(|w| ..)` method takes [`spi054::W`](W) writer structure"]
impl crate::Writable for Spi054Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI054 to value 0x0001_0000"]
impl crate::Resettable for Spi054Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
