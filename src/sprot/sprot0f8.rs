#[doc = "Register `SPROT0F8` reader"]
pub type R = crate::R<Sprot0f8Spec>;
#[doc = "Register `SPROT0F8` writer"]
pub type W = crate::W<Sprot0f8Spec>;
#[doc = "Field `SRNGSADR14` reader - SRNG_SADR14"]
pub type Srngsadr14R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR14` writer - SRNG_SADR14"]
pub type Srngsadr14W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE14` reader - SRNG_SIZE14"]
pub type Srngsize14R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE14` writer - SRNG_SIZE14"]
pub type Srngsize14W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR14"]
    #[inline(always)]
    pub fn srngsadr14(&self) -> Srngsadr14R {
        Srngsadr14R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE14"]
    #[inline(always)]
    pub fn srngsize14(&self) -> Srngsize14R {
        Srngsize14R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR14"]
    #[inline(always)]
    pub fn srngsadr14(&mut self) -> Srngsadr14W<Sprot0f8Spec> {
        Srngsadr14W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE14"]
    #[inline(always)]
    pub fn srngsize14(&mut self) -> Srngsize14W<Sprot0f8Spec> {
        Srngsize14W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR14\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0f8Spec;
impl crate::RegisterSpec for Sprot0f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0f8::R`](R) reader structure"]
impl crate::Readable for Sprot0f8Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0f8::W`](W) writer structure"]
impl crate::Writable for Sprot0f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0F8 to value 0"]
impl crate::Resettable for Sprot0f8Spec {}
