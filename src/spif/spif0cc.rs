#[doc = "Register `SPIF0CC` reader"]
pub type R = crate::R<Spif0ccSpec>;
#[doc = "Register `SPIF0CC` writer"]
pub type W = crate::W<Spif0ccSpec>;
#[doc = "Field `WTABLE19` reader - WTABLE19"]
pub type Wtable19R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE19` writer - WTABLE19"]
pub type Wtable19W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE19"]
    #[inline(always)]
    pub fn wtable19(&self) -> Wtable19R {
        Wtable19R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE19"]
    #[inline(always)]
    pub fn wtable19(&mut self) -> Wtable19W<Spif0ccSpec> {
        Wtable19W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE19\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0ccSpec;
impl crate::RegisterSpec for Spif0ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0cc::R`](R) reader structure"]
impl crate::Readable for Spif0ccSpec {}
#[doc = "`write(|w| ..)` method takes [`spif0cc::W`](W) writer structure"]
impl crate::Writable for Spif0ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0CC to value 0"]
impl crate::Resettable for Spif0ccSpec {}
