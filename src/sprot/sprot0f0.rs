#[doc = "Register `SPROT0F0` reader"]
pub type R = crate::R<Sprot0f0Spec>;
#[doc = "Register `SPROT0F0` writer"]
pub type W = crate::W<Sprot0f0Spec>;
#[doc = "Field `SRNGSADR12` reader - SRNG_SADR12"]
pub type Srngsadr12R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR12` writer - SRNG_SADR12"]
pub type Srngsadr12W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE12` reader - SRNG_SIZE12"]
pub type Srngsize12R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE12` writer - SRNG_SIZE12"]
pub type Srngsize12W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR12"]
    #[inline(always)]
    pub fn srngsadr12(&self) -> Srngsadr12R {
        Srngsadr12R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE12"]
    #[inline(always)]
    pub fn srngsize12(&self) -> Srngsize12R {
        Srngsize12R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR12"]
    #[inline(always)]
    pub fn srngsadr12(&mut self) -> Srngsadr12W<Sprot0f0Spec> {
        Srngsadr12W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE12"]
    #[inline(always)]
    pub fn srngsize12(&mut self) -> Srngsize12W<Sprot0f0Spec> {
        Srngsize12W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR12\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0f0Spec;
impl crate::RegisterSpec for Sprot0f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0f0::R`](R) reader structure"]
impl crate::Readable for Sprot0f0Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0f0::W`](W) writer structure"]
impl crate::Writable for Sprot0f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0F0 to value 0"]
impl crate::Resettable for Sprot0f0Spec {}
