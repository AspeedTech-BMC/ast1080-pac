#[doc = "Register `SPROT0C4` reader"]
pub type R = crate::R<Sprot0c4Spec>;
#[doc = "Register `SPROT0C4` writer"]
pub type W = crate::W<Sprot0c4Spec>;
#[doc = "Field `SRNGSADR01` reader - SRNG_SADR01"]
pub type Srngsadr01R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR01` writer - SRNG_SADR01"]
pub type Srngsadr01W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE01` reader - SRNG_SIZE01"]
pub type Srngsize01R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE01` writer - SRNG_SIZE01"]
pub type Srngsize01W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR01"]
    #[inline(always)]
    pub fn srngsadr01(&self) -> Srngsadr01R {
        Srngsadr01R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE01"]
    #[inline(always)]
    pub fn srngsize01(&self) -> Srngsize01R {
        Srngsize01R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR01"]
    #[inline(always)]
    pub fn srngsadr01(&mut self) -> Srngsadr01W<Sprot0c4Spec> {
        Srngsadr01W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE01"]
    #[inline(always)]
    pub fn srngsize01(&mut self) -> Srngsize01W<Sprot0c4Spec> {
        Srngsize01W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR01\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0c4Spec;
impl crate::RegisterSpec for Sprot0c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0c4::R`](R) reader structure"]
impl crate::Readable for Sprot0c4Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0c4::W`](W) writer structure"]
impl crate::Writable for Sprot0c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0C4 to value 0"]
impl crate::Resettable for Sprot0c4Spec {}
