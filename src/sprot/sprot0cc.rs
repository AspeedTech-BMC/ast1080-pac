#[doc = "Register `SPROT0CC` reader"]
pub type R = crate::R<Sprot0ccSpec>;
#[doc = "Register `SPROT0CC` writer"]
pub type W = crate::W<Sprot0ccSpec>;
#[doc = "Field `SRNGSADR03` reader - SRNG_SADR03"]
pub type Srngsadr03R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR03` writer - SRNG_SADR03"]
pub type Srngsadr03W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE03` reader - SRNG_SIZE03"]
pub type Srngsize03R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE03` writer - SRNG_SIZE03"]
pub type Srngsize03W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR03"]
    #[inline(always)]
    pub fn srngsadr03(&self) -> Srngsadr03R {
        Srngsadr03R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE03"]
    #[inline(always)]
    pub fn srngsize03(&self) -> Srngsize03R {
        Srngsize03R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR03"]
    #[inline(always)]
    pub fn srngsadr03(&mut self) -> Srngsadr03W<Sprot0ccSpec> {
        Srngsadr03W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE03"]
    #[inline(always)]
    pub fn srngsize03(&mut self) -> Srngsize03W<Sprot0ccSpec> {
        Srngsize03W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR03\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0ccSpec;
impl crate::RegisterSpec for Sprot0ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0cc::R`](R) reader structure"]
impl crate::Readable for Sprot0ccSpec {}
#[doc = "`write(|w| ..)` method takes [`sprot0cc::W`](W) writer structure"]
impl crate::Writable for Sprot0ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0CC to value 0"]
impl crate::Resettable for Sprot0ccSpec {}
