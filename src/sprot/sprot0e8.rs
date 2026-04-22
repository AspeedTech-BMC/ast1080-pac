#[doc = "Register `SPROT0E8` reader"]
pub type R = crate::R<Sprot0e8Spec>;
#[doc = "Register `SPROT0E8` writer"]
pub type W = crate::W<Sprot0e8Spec>;
#[doc = "Field `SRNGSADR10` reader - SRNG_SADR10"]
pub type Srngsadr10R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR10` writer - SRNG_SADR10"]
pub type Srngsadr10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE10` reader - SRNG_SIZE10"]
pub type Srngsize10R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE10` writer - SRNG_SIZE10"]
pub type Srngsize10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR10"]
    #[inline(always)]
    pub fn srngsadr10(&self) -> Srngsadr10R {
        Srngsadr10R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE10"]
    #[inline(always)]
    pub fn srngsize10(&self) -> Srngsize10R {
        Srngsize10R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR10"]
    #[inline(always)]
    pub fn srngsadr10(&mut self) -> Srngsadr10W<Sprot0e8Spec> {
        Srngsadr10W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE10"]
    #[inline(always)]
    pub fn srngsize10(&mut self) -> Srngsize10W<Sprot0e8Spec> {
        Srngsize10W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR10\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0e8Spec;
impl crate::RegisterSpec for Sprot0e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0e8::R`](R) reader structure"]
impl crate::Readable for Sprot0e8Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0e8::W`](W) writer structure"]
impl crate::Writable for Sprot0e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0E8 to value 0"]
impl crate::Resettable for Sprot0e8Spec {}
