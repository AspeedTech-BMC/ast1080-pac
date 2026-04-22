#[doc = "Register `SPROT0C8` reader"]
pub type R = crate::R<Sprot0c8Spec>;
#[doc = "Register `SPROT0C8` writer"]
pub type W = crate::W<Sprot0c8Spec>;
#[doc = "Field `SRNGSADR02` reader - SRNG_SADR02"]
pub type Srngsadr02R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR02` writer - SRNG_SADR02"]
pub type Srngsadr02W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE02` reader - SRNG_SIZE02"]
pub type Srngsize02R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE02` writer - SRNG_SIZE02"]
pub type Srngsize02W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR02"]
    #[inline(always)]
    pub fn srngsadr02(&self) -> Srngsadr02R {
        Srngsadr02R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE02"]
    #[inline(always)]
    pub fn srngsize02(&self) -> Srngsize02R {
        Srngsize02R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR02"]
    #[inline(always)]
    pub fn srngsadr02(&mut self) -> Srngsadr02W<Sprot0c8Spec> {
        Srngsadr02W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE02"]
    #[inline(always)]
    pub fn srngsize02(&mut self) -> Srngsize02W<Sprot0c8Spec> {
        Srngsize02W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR02\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0c8Spec;
impl crate::RegisterSpec for Sprot0c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0c8::R`](R) reader structure"]
impl crate::Readable for Sprot0c8Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0c8::W`](W) writer structure"]
impl crate::Writable for Sprot0c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0C8 to value 0"]
impl crate::Resettable for Sprot0c8Spec {}
