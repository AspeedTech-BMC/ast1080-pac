#[doc = "Register `SPROT0D8` reader"]
pub type R = crate::R<Sprot0d8Spec>;
#[doc = "Register `SPROT0D8` writer"]
pub type W = crate::W<Sprot0d8Spec>;
#[doc = "Field `SRNGSADR06` reader - SRNG_SADR06"]
pub type Srngsadr06R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR06` writer - SRNG_SADR06"]
pub type Srngsadr06W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE06` reader - SRNG_SIZE06"]
pub type Srngsize06R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE06` writer - SRNG_SIZE06"]
pub type Srngsize06W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR06"]
    #[inline(always)]
    pub fn srngsadr06(&self) -> Srngsadr06R {
        Srngsadr06R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE06"]
    #[inline(always)]
    pub fn srngsize06(&self) -> Srngsize06R {
        Srngsize06R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR06"]
    #[inline(always)]
    pub fn srngsadr06(&mut self) -> Srngsadr06W<Sprot0d8Spec> {
        Srngsadr06W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE06"]
    #[inline(always)]
    pub fn srngsize06(&mut self) -> Srngsize06W<Sprot0d8Spec> {
        Srngsize06W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR06\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0d8Spec;
impl crate::RegisterSpec for Sprot0d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0d8::R`](R) reader structure"]
impl crate::Readable for Sprot0d8Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0d8::W`](W) writer structure"]
impl crate::Writable for Sprot0d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0D8 to value 0"]
impl crate::Resettable for Sprot0d8Spec {}
