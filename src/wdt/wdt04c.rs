#[doc = "Register `WDT04C` reader"]
pub type R = crate::R<Wdt04cSpec>;
#[doc = "Register `WDT04C` writer"]
pub type W = crate::W<Wdt04cSpec>;
#[doc = "Field `ScratchRegiser` reader - Scratch Regiser"]
pub type ScratchRegiserR = crate::FieldReader;
#[doc = "Field `ScratchRegiser` writer - Scratch Regiser"]
pub type ScratchRegiserW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Wr0xEAToClearWDT4C70` reader - Write 0xEA to clear WDT4C\\[7:0\\]"]
pub type Wr0xEatoClearWdt4c70R = crate::FieldReader;
#[doc = "Field `Wr0xEAToClearWDT4C70` writer - Write 0xEA to clear WDT4C\\[7:0\\]"]
pub type Wr0xEatoClearWdt4c70W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Scratch Regiser"]
    #[inline(always)]
    pub fn scratch_regiser(&self) -> ScratchRegiserR {
        ScratchRegiserR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Write 0xEA to clear WDT4C\\[7:0\\]"]
    #[inline(always)]
    pub fn wr0x_eato_clear_wdt4c70(&self) -> Wr0xEatoClearWdt4c70R {
        Wr0xEatoClearWdt4c70R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scratch Regiser"]
    #[inline(always)]
    pub fn scratch_regiser(&mut self) -> ScratchRegiserW<Wdt04cSpec> {
        ScratchRegiserW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Write 0xEA to clear WDT4C\\[7:0\\]"]
    #[inline(always)]
    pub fn wr0x_eato_clear_wdt4c70(&mut self) -> Wr0xEatoClearWdt4c70W<Wdt04cSpec> {
        Wr0xEatoClearWdt4c70W::new(self, 24)
    }
}
#[doc = "WDTn Scratch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt04cSpec;
impl crate::RegisterSpec for Wdt04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt04c::R`](R) reader structure"]
impl crate::Readable for Wdt04cSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt04c::W`](W) writer structure"]
impl crate::Writable for Wdt04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT04C to value 0"]
impl crate::Resettable for Wdt04cSpec {}
