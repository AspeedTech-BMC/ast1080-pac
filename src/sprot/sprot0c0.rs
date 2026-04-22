#[doc = "Register `SPROT0C0` reader"]
pub type R = crate::R<Sprot0c0Spec>;
#[doc = "Register `SPROT0C0` writer"]
pub type W = crate::W<Sprot0c0Spec>;
#[doc = "Field `SRNGSADR00` reader - SRNG_SADR00"]
pub type Srngsadr00R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR00` writer - SRNG_SADR00"]
pub type Srngsadr00W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE00` reader - SRNG_SIZE00"]
pub type Srngsize00R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE00` writer - SRNG_SIZE00"]
pub type Srngsize00W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR00"]
    #[inline(always)]
    pub fn srngsadr00(&self) -> Srngsadr00R {
        Srngsadr00R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE00"]
    #[inline(always)]
    pub fn srngsize00(&self) -> Srngsize00R {
        Srngsize00R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR00"]
    #[inline(always)]
    pub fn srngsadr00(&mut self) -> Srngsadr00W<Sprot0c0Spec> {
        Srngsadr00W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE00"]
    #[inline(always)]
    pub fn srngsize00(&mut self) -> Srngsize00W<Sprot0c0Spec> {
        Srngsize00W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR00\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0c0Spec;
impl crate::RegisterSpec for Sprot0c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0c0::R`](R) reader structure"]
impl crate::Readable for Sprot0c0Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0c0::W`](W) writer structure"]
impl crate::Writable for Sprot0c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0C0 to value 0"]
impl crate::Resettable for Sprot0c0Spec {}
