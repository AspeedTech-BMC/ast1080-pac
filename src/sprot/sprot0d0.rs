#[doc = "Register `SPROT0D0` reader"]
pub type R = crate::R<Sprot0d0Spec>;
#[doc = "Register `SPROT0D0` writer"]
pub type W = crate::W<Sprot0d0Spec>;
#[doc = "Field `SRNGSADR04` reader - SRNG_SADR04"]
pub type Srngsadr04R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR04` writer - SRNG_SADR04"]
pub type Srngsadr04W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE04` reader - SRNG_SIZE04"]
pub type Srngsize04R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE04` writer - SRNG_SIZE04"]
pub type Srngsize04W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR04"]
    #[inline(always)]
    pub fn srngsadr04(&self) -> Srngsadr04R {
        Srngsadr04R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE04"]
    #[inline(always)]
    pub fn srngsize04(&self) -> Srngsize04R {
        Srngsize04R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR04"]
    #[inline(always)]
    pub fn srngsadr04(&mut self) -> Srngsadr04W<Sprot0d0Spec> {
        Srngsadr04W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE04"]
    #[inline(always)]
    pub fn srngsize04(&mut self) -> Srngsize04W<Sprot0d0Spec> {
        Srngsize04W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR04\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0d0Spec;
impl crate::RegisterSpec for Sprot0d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0d0::R`](R) reader structure"]
impl crate::Readable for Sprot0d0Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0d0::W`](W) writer structure"]
impl crate::Writable for Sprot0d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0D0 to value 0"]
impl crate::Resettable for Sprot0d0Spec {}
